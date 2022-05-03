use crate::ibc_node;
use ibc::core::ics02_client::client_consensus::AnyConsensusState;
use ibc::core::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::core::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd};
use ibc::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::core::ics04_channel::packet::{Packet, Receipt, Sequence};
use ibc::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use ibc::events::IbcEvent;
use ibc::Height as ICSHeight;
use ibc_proto::ibc::core::channel::v1::PacketState;

use beefy_merkle_tree::Hash;
use core::str::FromStr;
use ibc_proto::google::protobuf::Any;
use jsonrpsee::types::to_json_value;
use sp_core::storage::StorageKey;
use sp_keyring::AccountKeyring;
use subxt::storage::{StorageEntry, StorageKeyPrefix};
use subxt::BeefySubscription;
use subxt::RawEvent;
use subxt::SignedCommitment;
use subxt::{BlockNumber, Client, EventSubscription, PairSigner};
use tendermint_proto::Protobuf;

/// Subscribe ibc events
/// Maybe in the future call ocw
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = subscribe_ibc_event(client).await?;
/// ```
///
pub async fn subscribe_ibc_event(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IbcEvent>, Box<dyn std::error::Error>> {
    const COUNTER_SYSTEM_EVENT: i32 = 8;
    tracing::info!("In call_ibc: [subscribe_events]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<ibc_node::DefaultConfig>::new(sub, decoder);

    let mut events = Vec::new();
    let mut counter_system_event = 0;
    while let Some(raw_event) = sub.next().await {
        if let Err(err) = raw_event {
            println!(
                "In call_ibc: [subscribe_events] >> raw_event error: {:?}",
                err
            );
            continue;
        }
        let raw_event = raw_event.unwrap();
        let variant = raw_event.variant;
        tracing::info!("In substrate: [subscribe_events] >> variant: {:?}", variant);
        match variant.as_str() {
            "CreateClient" => {
                let event = <ibc_node::ibc::events::CreateClient as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> CreateClient Event");

                let height = event.0;
                let client_id = event.1;
                let client_type = event.2;
                let consensus_height = event.3;

                use ibc::core::ics02_client::events::Attributes;
                events.push(IbcEvent::CreateClient(
                    ibc::core::ics02_client::events::CreateClient::from(Attributes {
                        height: height.to_ibc_height(),
                        client_id: client_id.to_ibc_client_id(),
                        client_type: client_type.to_ibc_client_type(),
                        consensus_height: consensus_height.to_ibc_height(),
                    }),
                ));
                break;
            }
            "UpdateClient" => {
                let event = <ibc_node::ibc::events::UpdateClient as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> UpdateClient Event");

                let height = event.0;
                let client_id = event.1;
                let client_type = event.2;
                let consensus_height = event.3;

                use ibc::core::ics02_client::events::Attributes;
                events.push(IbcEvent::UpdateClient(
                    ibc::core::ics02_client::events::UpdateClient::from(Attributes {
                        height: height.to_ibc_height(),
                        client_id: client_id.to_ibc_client_id(),
                        client_type: client_type.to_ibc_client_type(),
                        consensus_height: consensus_height.to_ibc_height(),
                    }),
                ));
                // break;
            }
            "ClientMisbehaviour" => {
                let event = <ibc_node::ibc::events::ClientMisbehaviour as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> ClientMisbehaviour Event");

                let height = event.0;
                let client_id = event.1;
                let client_type = event.2;
                let consensus_height = event.3;

                use ibc::core::ics02_client::events::Attributes;
                events.push(IbcEvent::ClientMisbehaviour(
                    ibc::core::ics02_client::events::ClientMisbehaviour::from(Attributes {
                        height: height.to_ibc_height(),
                        client_id: client_id.to_ibc_client_id(),
                        client_type: client_type.to_ibc_client_type(),
                        consensus_height: consensus_height.to_ibc_height(),
                    }),
                ));
                // break;
            }
            "OpenInitConnection" => {
                let event = <ibc_node::ibc::events::OpenInitConnection as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenInitConnection Event");

                let height = event.0;
                let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                let client_id = event.2;
                let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                let counterparty_client_id = event.4;

                use ibc::core::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenInitConnection(
                    ibc::core::ics03_connection::events::OpenInit::from(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                break;
            }
            "OpenTryConnection" => {
                let event = <ibc_node::ibc::events::OpenTryConnection as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenTryConnection Event");

                let height = event.0;
                let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                let client_id = event.2;
                let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                let counterparty_client_id = event.4;

                use ibc::core::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenTryConnection(
                    ibc::core::ics03_connection::events::OpenTry::from(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                break;
            }
            "OpenAckConnection" => {
                let event = <ibc_node::ibc::events::OpenAckConnection as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenAckConnection Event");

                let height = event.0;
                let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                let client_id = event.2;
                let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                let counterparty_client_id = event.4;

                use ibc::core::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenAckConnection(
                    ibc::core::ics03_connection::events::OpenAck::from(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                break;
            }
            "OpenConfirmConnection" => {
                let event =
                    <ibc_node::ibc::events::OpenConfirmConnection as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenConfirmConnection Event");

                let height = event.0;
                let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                let client_id = event.2;
                let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                let counterparty_client_id = event.4;

                use ibc::core::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenConfirmConnection(
                    ibc::core::ics03_connection::events::OpenConfirm::from(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                break;
            }

            "OpenInitChannel" => {
                let event = <ibc_node::ibc::events::OpenInitChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenInitChannel Event");

                let height = event.0;
                let port_id = event.1;
                let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                let connection_id = event.3;
                let counterparty_port_id = event.4;
                let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                events.push(IbcEvent::OpenInitChannel(
                    ibc::core::ics04_channel::events::OpenInit {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    },
                ));
                break;
            }
            "OpenTryChannel" => {
                let event = <ibc_node::ibc::events::OpenTryChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenTryChannel Event");

                let height = event.0;
                let port_id = event.1;
                let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                let connection_id = event.3;
                let counterparty_port_id = event.4;
                let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                events.push(IbcEvent::OpenTryChannel(
                    ibc::core::ics04_channel::events::OpenTry {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    },
                ));
                break;
            }
            "OpenAckChannel" => {
                let event = <ibc_node::ibc::events::OpenAckChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenAckChannel Event");

                let height = event.0;
                let port_id = event.1;
                let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                let connection_id = event.3;
                let counterparty_port_id = event.4;
                let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                events.push(IbcEvent::OpenAckChannel(
                    ibc::core::ics04_channel::events::OpenAck {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    },
                ));
                break;
            }
            "OpenConfirmChannel" => {
                let event = <ibc_node::ibc::events::OpenConfirmChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> OpenConfirmChannel Event");

                let height = event.0;
                let port_id = event.1;
                let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                let connection_id = event.3;
                let counterparty_port_id = event.4;
                let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                events.push(IbcEvent::OpenConfirmChannel(
                    ibc::core::ics04_channel::events::OpenConfirm {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    },
                ));
                break;
            }
            "CloseInitChannel" => {
                let event = <ibc_node::ibc::events::CloseInitChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> CloseInitChannel Event");

                let height = event.0;
                let port_id = event.1;
                let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                let connection_id = event.3;
                let counterparty_port_id = event.4;
                let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                events.push(IbcEvent::CloseInitChannel(
                    ibc::core::ics04_channel::events::CloseInit {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id.unwrap_or_default(),
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    },
                ));
                break;
            }
            "CloseConfirmChannel" => {
                let event = <ibc_node::ibc::events::CloseConfirmChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();

                tracing::info!("In call_ibc: [subscribe_events] >> CloseConfirmChannel Event");

                let height = event.0;
                let port_id = event.1;
                let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                let connection_id = event.3;
                let counterparty_port_id = event.4;
                let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                events.push(IbcEvent::CloseConfirmChannel(
                    ibc::core::ics04_channel::events::CloseConfirm {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    },
                ));
                break;
            }
            "SendPacket" => {
                let event = <ibc_node::ibc::events::SendPacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> SendPacket Event");

                let send_packet = ibc::core::ics04_channel::events::SendPacket {
                    height: event.0.to_ibc_height(),
                    packet: event.1.to_ibc_packet(),
                };

                events.push(IbcEvent::SendPacket(send_packet));
                break;
            }
            "ReceivePacket" => {
                let event = <ibc_node::ibc::events::ReceivePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();

                tracing::info!("In call_ibc: [substrate_events] >> ReceivePacket Event");

                let receive_packet = ibc::core::ics04_channel::events::ReceivePacket {
                    height: event.0.to_ibc_height(),
                    packet: event.1.to_ibc_packet(),
                };

                events.push(IbcEvent::ReceivePacket(receive_packet));

                break;
            }
            "WriteAcknowledgement" => {
                let event = <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> WriteAcknowledgement Event");

                let write_acknowledgement =
                    ibc::core::ics04_channel::events::WriteAcknowledgement {
                        height: event.0.to_ibc_height(),
                        packet: event.1.to_ibc_packet(),
                        ack: event.2,
                    };

                events.push(IbcEvent::WriteAcknowledgement(write_acknowledgement));

                break;
            }
            "AcknowledgePacket" => {
                let event = <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> AcknowledgePacket Event");

                let acknowledge_packet = ibc::core::ics04_channel::events::AcknowledgePacket {
                    height: event.0.to_ibc_height(),
                    packet: event.1.to_ibc_packet(),
                };

                events.push(IbcEvent::AcknowledgePacket(acknowledge_packet));

                break;
            }
            "TimeoutPacket" => {
                let event = <ibc_node::ibc::events::TimeoutPacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> TimeoutPacket Event");

                let timeout_packet = ibc::core::ics04_channel::events::TimeoutPacket {
                    height: event.0.to_ibc_height(),
                    packet: event.1.to_ibc_packet(),
                };

                events.push(IbcEvent::TimeoutPacket(timeout_packet));

                break;
            }
            "TimeoutOnClosePacket" => {
                let event = <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> TimeoutOnClosePacket Event");

                let timeout_on_close_packet =
                    ibc::core::ics04_channel::events::TimeoutOnClosePacket {
                        height: event.0.to_ibc_height(),
                        packet: event.1.to_ibc_packet(),
                    };

                events.push(IbcEvent::TimeoutOnClosePacket(timeout_on_close_packet));

                break;
            }
            "Empty" => {
                let event = <ibc_node::ibc::events::Empty as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();

                tracing::info!("in call_ibc: [substrate_events] >> Empty Event");

                let data = String::from_utf8(event.0).unwrap();

                events.push(IbcEvent::Empty(data));
                break;
            }
            "ChainError" => {
                let event = <ibc_node::ibc::events::ChainError as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();

                tracing::info!("in call_ibc: [substrate_events] >> ChainError Event");

                let data = String::from_utf8(event.0).unwrap();

                events.push(IbcEvent::Empty(data));
                break;
            }
            "ExtrinsicSuccess" => {
                let _event = <ibc_node::system::events::ExtrinsicSuccess as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [subscribe_events] >> ExtrinsicSuccess ");

                if counter_system_event < COUNTER_SYSTEM_EVENT {
                    tracing::info!(
                        "In call_ibc: [subscribe_events] >> counter_system_event: {:?}",
                        counter_system_event
                    );
                    counter_system_event += 1;
                } else {
                    tracing::info!(
                        "In call_ibc: [subscribe_events] >> counter_system_event: {:?}",
                        counter_system_event
                    );
                    break;
                }
            }
            _ => {
                continue;
            }
        }
    }
    Ok(events)
}

/// convert substrate event to ibc event
pub fn from_substrate_event_to_ibc_event(raw_events: Vec<RawEvent>) -> Vec<IbcEvent> {
    fn inner_convert_event(raw_events: Vec<RawEvent>, module: &str) -> Vec<RawEvent> {
        raw_events
            .into_iter()
            .filter(|raw| raw.pallet == module)
            .map(|raw| raw)
            .collect::<Vec<_>>()
    }

    let ret = inner_convert_event(raw_events, "Ibc");

    ret.into_iter()
        .map(|raw_event| {
            let variant = raw_event.variant;
            let ibc_event = match variant.as_str() {
                "CreateClient" => {
                    let event = <ibc_node::ibc::events::CreateClient as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> CreateClient Event");

                    let height = event.0;
                    let client_id = event.1;
                    let client_type = event.2;
                    let consensus_height = event.3;

                    use ibc::core::ics02_client::events::Attributes;
                    IbcEvent::CreateClient(ibc::core::ics02_client::events::CreateClient::from(
                        Attributes {
                            height: height.to_ibc_height(),
                            client_id: client_id.to_ibc_client_id(),
                            client_type: client_type.to_ibc_client_type(),
                            consensus_height: consensus_height.to_ibc_height(),
                        },
                    ))
                }
                "UpdateClient" => {
                    let event = <ibc_node::ibc::events::UpdateClient as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> UpdateClient Event");

                    let height = event.0;
                    let client_id = event.1;
                    let client_type = event.2;
                    let consensus_height = event.3;

                    use ibc::core::ics02_client::events::Attributes;
                    IbcEvent::UpdateClient(ibc::core::ics02_client::events::UpdateClient::from(
                        Attributes {
                            height: height.to_ibc_height(),
                            client_id: client_id.to_ibc_client_id(),
                            client_type: client_type.to_ibc_client_type(),
                            consensus_height: consensus_height.to_ibc_height(),
                        },
                    ))
                }
                "ClientMisbehaviour" => {
                    let event =
                        <ibc_node::ibc::events::ClientMisbehaviour as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> ClientMisbehaviour Event");

                    let height = event.0;
                    let client_id = event.1;
                    let client_type = event.2;
                    let consensus_height = event.3;

                    use ibc::core::ics02_client::events::Attributes;
                    IbcEvent::ClientMisbehaviour(
                        ibc::core::ics02_client::events::ClientMisbehaviour::from(Attributes {
                            height: height.to_ibc_height(),
                            client_id: client_id.to_ibc_client_id(),
                            client_type: client_type.to_ibc_client_type(),
                            consensus_height: consensus_height.to_ibc_height(),
                        }),
                    )
                }
                "OpenInitConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenInitConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> OpenInitConnection Event");

                    let height = event.0;
                    let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                    let client_id = event.2;
                    let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                    let counterparty_client_id = event.4;

                    use ibc::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenInitConnection(
                        ibc::core::ics03_connection::events::OpenInit::from(Attributes {
                            height: height.to_ibc_height(),
                            connection_id,
                            client_id: client_id.to_ibc_client_id(),
                            counterparty_connection_id,
                            counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                        }),
                    )
                }
                "OpenTryConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenTryConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> OpenTryConnection Event");

                    let height = event.0;
                    let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                    let client_id = event.2;
                    let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                    let counterparty_client_id = event.4;

                    use ibc::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenTryConnection(ibc::core::ics03_connection::events::OpenTry::from(
                        Attributes {
                            height: height.to_ibc_height(),
                            connection_id,
                            client_id: client_id.to_ibc_client_id(),
                            counterparty_connection_id,
                            counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                        },
                    ))
                }
                "OpenAckConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenAckConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> OpenAckConnection Event");

                    let height = event.0;
                    let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                    let client_id = event.2;
                    let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                    let counterparty_client_id = event.4;

                    use ibc::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenAckConnection(ibc::core::ics03_connection::events::OpenAck::from(
                        Attributes {
                            height: height.to_ibc_height(),
                            connection_id,
                            client_id: client_id.to_ibc_client_id(),
                            counterparty_connection_id,
                            counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                        },
                    ))
                }
                "OpenConfirmConnection" => {
                    let event =
                        <ibc_node::ibc::events::OpenConfirmConnection as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!(
                        "In call_ibc: [subscribe_events] >> OpenConfirmConnection Event"
                    );

                    let height = event.0;
                    let connection_id = event.1.map(|val| val.to_ibc_connection_id());
                    let client_id = event.2;
                    let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
                    let counterparty_client_id = event.4;

                    use ibc::core::ics03_connection::events::Attributes;
                    IbcEvent::OpenConfirmConnection(
                        ibc::core::ics03_connection::events::OpenConfirm::from(Attributes {
                            height: height.to_ibc_height(),
                            connection_id,
                            client_id: client_id.to_ibc_client_id(),
                            counterparty_connection_id,
                            counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                        }),
                    )
                }

                "OpenInitChannel" => {
                    let event = <ibc_node::ibc::events::OpenInitChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> OpenInitChannel Event");

                    let height = event.0;
                    let port_id = event.1;
                    let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                    let connection_id = event.3;
                    let counterparty_port_id = event.4;
                    let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                    IbcEvent::OpenInitChannel(ibc::core::ics04_channel::events::OpenInit {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    })
                }
                "OpenTryChannel" => {
                    let event = <ibc_node::ibc::events::OpenTryChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> OpenTryChannel Event");

                    let height = event.0;
                    let port_id = event.1;
                    let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                    let connection_id = event.3;
                    let counterparty_port_id = event.4;
                    let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                    IbcEvent::OpenTryChannel(ibc::core::ics04_channel::events::OpenTry {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    })
                }
                "OpenAckChannel" => {
                    let event = <ibc_node::ibc::events::OpenAckChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> OpenAckChannel Event");

                    let height = event.0;
                    let port_id = event.1;
                    let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                    let connection_id = event.3;
                    let counterparty_port_id = event.4;
                    let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                    IbcEvent::OpenAckChannel(ibc::core::ics04_channel::events::OpenAck {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    })
                }
                "OpenConfirmChannel" => {
                    let event =
                        <ibc_node::ibc::events::OpenConfirmChannel as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> OpenConfirmChannel Event");

                    let height = event.0;
                    let port_id = event.1;
                    let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                    let connection_id = event.3;
                    let counterparty_port_id = event.4;
                    let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                    IbcEvent::OpenConfirmChannel(ibc::core::ics04_channel::events::OpenConfirm {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    })
                }
                "CloseInitChannel" => {
                    let event = <ibc_node::ibc::events::CloseInitChannel as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [subscribe_events] >> CloseInitChannel Event");

                    let height = event.0;
                    let port_id = event.1;
                    let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                    let connection_id = event.3;
                    let counterparty_port_id = event.4;
                    let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                    IbcEvent::CloseInitChannel(ibc::core::ics04_channel::events::CloseInit {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id.unwrap_or_default(),
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    })
                }
                "CloseConfirmChannel" => {
                    let event =
                        <ibc_node::ibc::events::CloseConfirmChannel as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();

                    tracing::info!("In call_ibc: [subscribe_events] >> CloseConfirmChannel Event");

                    let height = event.0;
                    let port_id = event.1;
                    let channel_id = event.2.map(|val| val.to_ibc_channel_id());
                    let connection_id = event.3;
                    let counterparty_port_id = event.4;
                    let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

                    IbcEvent::CloseConfirmChannel(ibc::core::ics04_channel::events::CloseConfirm {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id,
                    })
                }
                "SendPacket" => {
                    let event = <ibc_node::ibc::events::SendPacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [substrate_events] >> SendPacket Event");

                    let send_packet = ibc::core::ics04_channel::events::SendPacket {
                        height: event.0.to_ibc_height(),
                        packet: event.1.to_ibc_packet(),
                    };

                    IbcEvent::SendPacket(send_packet)
                }
                "ReceivePacket" => {
                    let event = <ibc_node::ibc::events::ReceivePacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();

                    tracing::info!("In call_ibc: [substrate_events] >> ReceivePacket Event");

                    let receive_packet = ibc::core::ics04_channel::events::ReceivePacket {
                        height: event.0.to_ibc_height(),
                        packet: event.1.to_ibc_packet(),
                    };

                    IbcEvent::ReceivePacket(receive_packet)
                }
                "WriteAcknowledgement" => {
                    let event =
                        <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [substrate_events] >> WriteAcknowledgement Event");

                    let write_acknowledgement =
                        ibc::core::ics04_channel::events::WriteAcknowledgement {
                            height: event.0.to_ibc_height(),
                            packet: event.1.to_ibc_packet(),
                            ack: event.2,
                        };

                    IbcEvent::WriteAcknowledgement(write_acknowledgement)
                }
                "AcknowledgePacket" => {
                    let event =
                        <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [substrate_events] >> AcknowledgePacket Event");

                    let acknowledge_packet = ibc::core::ics04_channel::events::AcknowledgePacket {
                        height: event.0.to_ibc_height(),
                        packet: event.1.to_ibc_packet(),
                    };

                    IbcEvent::AcknowledgePacket(acknowledge_packet)
                }
                "TimeoutPacket" => {
                    let event = <ibc_node::ibc::events::TimeoutPacket as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();
                    tracing::info!("In call_ibc: [substrate_events] >> TimeoutPacket Event");

                    let timeout_packet = ibc::core::ics04_channel::events::TimeoutPacket {
                        height: event.0.to_ibc_height(),
                        packet: event.1.to_ibc_packet(),
                    };

                    IbcEvent::TimeoutPacket(timeout_packet)
                }
                "TimeoutOnClosePacket" => {
                    let event =
                        <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(
                            &mut &raw_event.data[..],
                        )
                        .unwrap();
                    tracing::info!("In call_ibc: [substrate_events] >> TimeoutOnClosePacket Event");

                    let timeout_on_close_packet =
                        ibc::core::ics04_channel::events::TimeoutOnClosePacket {
                            height: event.0.to_ibc_height(),
                            packet: event.1.to_ibc_packet(),
                        };

                    IbcEvent::TimeoutOnClosePacket(timeout_on_close_packet)
                }
                "Empty" => {
                    let event = <ibc_node::ibc::events::Empty as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();

                    tracing::info!("in call_ibc: [substrate_events] >> Empty Event");

                    let data = String::from_utf8(event.0).unwrap();

                    IbcEvent::Empty(data)
                }
                "ChainError" => {
                    let event = <ibc_node::ibc::events::ChainError as codec::Decode>::decode(
                        &mut &raw_event.data[..],
                    )
                    .unwrap();

                    tracing::info!("in call_ibc: [substrate_events] >> ChainError Event");

                    let data = String::from_utf8(event.0).unwrap();

                    IbcEvent::Empty(data)
                }
                _ => unimplemented!(),
            };
            ibc_event
        })
        .collect::<Vec<_>>()
}

/// Subscribe beefy justifiactions
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = subscribe_beefy(client).await?;
/// ```
///
pub async fn subscribe_beefy(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<SignedCommitment, Box<dyn std::error::Error>> {
    tracing::info!("In call_ibc: [subscribe_beefy_justifications]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();
    let sub = api.client.rpc().subscribe_beefy_justifications().await?;
    let mut sub = BeefySubscription::new(sub);
    let raw = sub.next().await.unwrap();

    Ok(raw)
}

/// get latest height used by subscribe_blocks
///  
/// # Usage example
///
/// ```rust
/// let api = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = get_latest_height(api).await?;
/// ```
///
pub async fn get_latest_height(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<u64, Box<dyn std::error::Error>> {
    tracing::info!("In call_ibc: [get_latest_height]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut blocks = api.client.rpc().subscribe_finalized_blocks().await?;

    let height = match blocks.next().await {
        Ok(Some(header)) => header.number as u64,
        Ok(None) => {
            tracing::info!("In call_ibc: [get_latest_height] >> None");
            0
        }
        Err(err) => {
            tracing::info!(" In call_ibc: [get_latest_height] >> error: {:?} ", err);
            0
        }
    };
    tracing::info!("In call_ibc: [get_latest_height] >> height: {:?}", height);
    Ok(height)
}

/// get connectionEnd according by connection_identifier and read Connections StorageMaps
///
/// # Usage example
///
/// ```rust
/// let api = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let conection_id = ConnectionId::default();
/// let result = get_connection_end(&conection_id, api).await?;
/// ```
///
pub async fn get_connection_end(
    connection_identifier: &ConnectionId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<ConnectionEnd, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connection_end]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc : [get_connection_end] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .connections(connection_identifier.as_bytes().to_vec(), Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_connection_end is empty! by connection_identifier = ({})",
            connection_identifier
        )));
    }

    let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

    Ok(connection_end)
}

/// get channelEnd according by port_identifier, channel_identifier and read Channles StorageMaps
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let prot_id =PortId::default();
/// let channel_id = ChannelId::default();
/// let result = get_channel_end(&port_id, &channel_id, client).await?;
/// ```
///
pub async fn get_channel_end(
    port_id: &PortId,
    channel_id: &ChannelId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<ChannelEnd, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_channel_end]");
    tracing::info!(
        "in call_ibc: [get_channel_end] >> port_id: {:?}, channel_id: {:?}",
        port_id.clone(),
        channel_id.clone()
    );

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_channelend] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .channels(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_channel_end is empty by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )));
    }

    tracing::info!("in call_ibc: [get_channel_end] >> data >> {:?}", data);

    let channel_end = ChannelEnd::decode_vec(&*data).unwrap();
    tracing::info!(
        "in call_ibc: [get_channel_end] >> channel_end >> {:?}",
        channel_end
    );

    Ok(channel_end)
}

/// get packet receipt by port_id, channel_id and sequence
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let prot_id =PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(0);
/// let result = get_packet_receipt(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
pub async fn get_packet_receipt(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Receipt, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc : [get_packet_receipt]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc : [get_packet_receipt] >> block_hash: {:?}",
        block_hash
    );

    let sequence = u64::from(*sequence);

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_receipt(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            sequence,
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )));
    }

    let _data = String::from_utf8(data).unwrap();
    if _data.eq("Ok") {
        Ok(Receipt::Ok)
    } else {
        Err(format!("unrecognized packet receipt: {:?}", _data).into())
    }
}

/// get packet receipt by port_id, channel_id and sequence
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let prot_id =PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(0);
/// let result = get_packet_receipt_vec(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
pub async fn get_packet_receipt_vec(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc : [get_packet_receipt]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc : [get_packet_receipt] >> block_hash: {:?}",
        block_hash
    );

    let sequence = u64::from(*sequence);

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_receipt(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            sequence,
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )));
    }

    Ok(data)
}

/// get send packet event by port_id, channel_id and sequence
/// (port_id, channel_id, sequence), packet)
///
/// # Usage example
///
/// ```rust
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let prot_id =PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(0);
/// let result = get_send_packet_event(&port_id, &channel_id, &sequence, client).await?;
///
/// ```
pub async fn get_send_packet_event(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Packet, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_send_packet_event]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_send_packet_event] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .send_packet_event(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            u64::from(*sequence),
            Some(block_hash),
        )
        .await?;
    if data.is_empty() {
        return Err(Box::from(format!(
            "get_send_packet_event is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )));
    }

    let packet = Packet::decode_vec(&*data).unwrap();
    Ok(packet)
}

/// (port_id, channel_id, sequence), ackHash)
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let prot_id =PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(0);
/// let result = get_write_ack_packet_event(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
pub async fn get_write_ack_packet_event(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_write_ack_packet_event]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_write_ack_packet_event] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .write_ack_packet_event(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            u64::from(*sequence),
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_write_ack_packet_event is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )));
    }

    Ok(data)
}

/// get client_state according by client_id, and read ClientStates StoraageMap
///  
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let client_id = ClientId::default();
/// let result = get_client_state(&client_id, client).await?;
/// ```
///
pub async fn get_client_state(
    client_id: &ClientId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<AnyClientState, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc : [get_client_state]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .client_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_client_state is empty! by client_id = ({})",
            client_id
        )));
    }

    tracing::info!("in call_ibc: [get_client_state]: client_state: {:?}", data);

    let client_state = AnyClientState::decode_vec(&*data).unwrap();
    tracing::info!(
        "in call_ibc: [get_client_state]: any_client_state : {:?}",
        client_state
    );

    Ok(client_state)
}

/// get appoint height consensus_state according by client_identifier and height
/// and read ConsensusStates StoreageMap
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let client_id = ClientId::default();
/// let height = ICSHeight::default();
/// let result = get_client_consensus(&client_id, &height, client).await?;
/// ```
///
pub async fn get_client_consensus(
    client_id: &ClientId,
    height: &ICSHeight,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<AnyConsensusState, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_client_consensus]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();

    tracing::info!(
        "In call_ibc: [get_client_consensus] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<(Vec<u8>, Vec<u8>)> = api
        .storage()
        .ibc()
        .consensus_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;
    if data.is_empty() {
        return Err(Box::from(format!(
            "get_client_consensus is empty! by client_id = ({}), height = ({})",
            client_id, height
        )));
    }
    tracing::info!("call_ibc: [consensus_state] >> data >> {:?}", data);

    // get the height consensus_state
    let mut consensus_state = vec![];
    for item in data.iter() {
        if item.0 == height.encode_vec().unwrap() {
            consensus_state = item.1.clone();
        }
    }

    println!(
        "call_ibc: [consensus_state] >> consensus_state >> {:?}",
        consensus_state
    );

    let consensus_state = if consensus_state.is_empty() {
        // TODO
        AnyConsensusState::Grandpa(
            ibc::clients::ics10_grandpa::consensus_state::ConsensusState::default(),
        )
    } else {
        AnyConsensusState::decode_vec(&*consensus_state).unwrap()
    };

    Ok(consensus_state)
}

/// get consensus state with height
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let client_id = ClientId::default();
/// let result = get_consensus_state_with_height(&client_id, client).await?;
/// ```
///
pub async fn get_consensus_state_with_height(
    client_id: &ClientId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<(ICSHeight, AnyConsensusState)>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_consensus_state_with_height]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_client_consensus] >> block_hash: {:?}",
        block_hash
    );

    // vector<height, consensus_state>
    let data: Vec<(Vec<u8>, Vec<u8>)> = api
        .storage()
        .ibc()
        .consensus_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_consensus_state_with_height is empty! by client_id = ({})",
            client_id
        )));
    }

    let mut result = vec![];
    for (height, consensus_state) in data.iter() {
        let height = ICSHeight::decode_vec(&*height).unwrap();
        let consensus_state = AnyConsensusState::decode_vec(&*consensus_state).unwrap();
        result.push((height, consensus_state));
    }

    Ok(result)
}

/// get  unreceipt packet
///  # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let port_id =PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = vec![Sequence::from(12),Sequence::from(13)];
/// let result = get_unreceipt_packet(&port_id, &channel_id, sequence, client).await?;
/// ```
///
pub async fn get_unreceipt_packet(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequences: Vec<Sequence>,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_receipt_packet]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();

    tracing::info!(
        "In call_ibc: [get_receipt_packet] >> block_hash: {:?}",
        block_hash
    );

    let mut result = Vec::new();

    let pair = sequences.into_iter().map(|sequence| {
        (
            port_id.clone().as_bytes().to_vec(),
            channel_id.clone().as_bytes().to_vec(),
            u64::from(sequence),
        )
    });

    for (port_id, channel_id, sequence) in pair {
        let data: Vec<u8> = api
            .storage()
            .ibc()
            .packet_receipt(port_id, channel_id, sequence, Some(block_hash))
            .await?;
        if data.is_empty() {
            result.push(sequence);
        }
    }

    Ok(result)
}

/// get key-value pair (client_identifier, client_state) construct IdentifieredAnyClientstate
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = get_clients(client).await?;
/// ```
///
pub async fn get_clients(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedAnyClientState>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_clients]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!("In call_ibc: [get_clients] >> block_hash: {:?}", block_hash);

    // vector key-value
    let mut ret = vec![];

    // get client_state Keys
    let client_states_keys: Vec<Vec<u8>> = api
        .storage()
        .ibc()
        .client_states_keys(Some(block_hash))
        .await?;
    if client_states_keys.is_empty() {
        return Err(Box::from(
            "get_clients: get empty client_states_keys".to_string(),
        ));
    }

    // enumate every item get client_state value
    for key in client_states_keys {
        // get client_state value
        let client_states_value: Vec<u8> = api
            .storage()
            .ibc()
            .client_states(key.clone(), Some(block_hash))
            .await?;
        // store key-value
        ret.push((key.clone(), client_states_value));
    }

    let mut result = vec![];

    for (client_id, client_state) in ret.iter() {
        let client_id_str = String::from_utf8(client_id.clone()).unwrap();
        let client_id = ClientId::from_str(client_id_str.as_str()).unwrap();

        let client_state = AnyClientState::decode_vec(&*client_state).unwrap();

        result.push(IdentifiedAnyClientState::new(client_id, client_state));
    }

    Ok(result)
}

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = get_connections(client).await?;
/// ```
///
pub async fn get_connections(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedConnectionEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connctions]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_connctions] >> block_hash: {:?}",
        block_hash
    );

    let mut ret = vec![];

    // get connection_keys
    let connection_keys: Vec<Vec<u8>> = api
        .storage()
        .ibc()
        .connections_keys(Some(block_hash))
        .await?;

    if connection_keys.is_empty() {
        return Err(Box::from(
            "get_connections: get empty connection_keys".to_string(),
        ));
    }

    for key in connection_keys {
        // get connectons value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .connections(key.clone(), Some(block_hash))
            .await?;

        // store key-value
        ret.push((key.clone(), value.clone()));
    }

    let mut result = vec![];

    for (connection_id, connection_end) in ret.iter() {
        let connection_id_str = String::from_utf8(connection_id.clone()).unwrap();
        let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

        let connnection_end = ConnectionEnd::decode_vec(&*connection_end).unwrap();

        result.push(IdentifiedConnectionEnd::new(connection_id, connnection_end));
    }

    Ok(result)
}

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
///
/// # Usage example
///
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = get_channels(client).await?;
/// ```
///
pub async fn get_channels(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_channels]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_channels] >> block_hash: {:?}",
        block_hash
    );

    // vector key-value
    let mut ret = vec![];

    let channels_keys: Vec<(Vec<u8>, Vec<u8>)> =
        api.storage().ibc().channels_keys(Some(block_hash)).await?;

    if channels_keys.is_empty() {
        return Err(Box::from(
            "get_channels: get empty channels_keys".to_string(),
        ));
    }

    for key in channels_keys {
        // get value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .channels(key.0.clone(), key.1.clone(), Some(block_hash))
            .await?;

        // store key-value
        ret.push((key.0.clone(), key.1.clone(), value));
    }
    let mut result = vec![];

    for (port_id, channel_id, channel_end) in ret.iter() {
        let port_id_str = String::from_utf8(port_id.clone()).unwrap();
        let port_id = PortId::from_str(port_id_str.as_str()).unwrap();

        let channel_id_str = String::from_utf8(channel_id.clone()).unwrap();
        let channel_id = ChannelId::from_str(channel_id_str.as_str()).unwrap();

        let channel_end = ChannelEnd::decode_vec(&*channel_end).unwrap();

        result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
    }

    Ok(result)
}

/// get get_commitment_packet_state
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = get_commitment_packet_state(client).await?;
/// ```
///
pub async fn get_commitment_packet_state(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_commitment_packet_state]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_commitment_packet_state] >> block_hash: {:?}",
        block_hash
    );

    let mut ret = vec![];

    let packet_commitments_keys: Vec<(Vec<u8>, Vec<u8>, u64)> = api
        .storage()
        .ibc()
        .packet_commitment_keys(Some(block_hash))
        .await?;

    for key in packet_commitments_keys {
        // get value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .packet_commitment(key.0.clone(), key.1.clone(), key.2, Some(block_hash))
            .await?;

        // store key-value
        ret.push((key.0.clone(), key.1.clone(), key.2, value));
    }

    let mut result = vec![];

    for (port_id, channel_id, sequence, data) in ret.into_iter() {
        let port_id = String::from_utf8(port_id).unwrap();
        let channel_id = String::from_utf8(channel_id).unwrap();

        let packet_state = PacketState {
            port_id,
            channel_id,
            sequence,
            data,
        };
        result.push(packet_state);
    }

    Ok(result)
}

/// get packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
///
///  # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let port_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(23);
/// let result = get_packet_commitment(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
pub async fn get_packet_commitment(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_packet_commitment]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_packet_commitment] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_commitment(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            u64::from(*sequence),
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_packet_commitment is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )));
    } else {
        Ok(data)
    }
}

/// get packet acknowlegement by port_id, channel_id and sequence to verify if the packet has been received by the target chain
///
///  # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let port_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(12);
/// let result = get_packet_ack(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
pub async fn get_packet_ack(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_packet_ack]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_packet_ack] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .acknowledgements(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            u64::from(*sequence),
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_packet_ack is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )));
    } else {
        Ok(data)
    }
}

/// get packet receipt by port_id, channel_id and sequence
///  # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let prot_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let result = get_next_sequence_recv(&prot_id, &channel_id, client).await?;
/// ```
///
pub async fn get_next_sequence_recv(
    port_id: &PortId,
    channel_id: &ChannelId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_next_sequence_recv]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_next_sequence_recv] >> block_hash: {:?}",
        block_hash
    );

    let sequence: u64 = api
        .storage()
        .ibc()
        .next_sequence_recv(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            Some(block_hash),
        )
        .await?;

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_commitment(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            sequence,
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_next_sequence_recv is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )));
    } else {
        Ok(data)
    }
}

/// get get_commitment_packet_state
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let result = get_acknowledge_packet_state(client).await?;
/// ```
///
pub async fn get_acknowledge_packet_state(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_acknowledge_packet_state]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_acknowledge_packet_state] >> block_hash: {:?}",
        block_hash
    );

    let mut ret = vec![];

    let acknowledgements_keys: Vec<(Vec<u8>, Vec<u8>, u64)> = api
        .storage()
        .ibc()
        .acknowledgements_keys(Some(block_hash))
        .await?;

    for key in acknowledgements_keys {
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .acknowledgements(key.0.clone(), key.1.clone(), key.2, Some(block_hash))
            .await?;

        ret.push((key.0.clone(), key.1.clone(), key.2, value));
    }

    let mut result = vec![];

    for (port_id, channel_id, sequence, data) in ret.into_iter() {
        let port_id = String::from_utf8(port_id).unwrap();
        let channel_id = String::from_utf8(channel_id).unwrap();

        let packet_state = PacketState {
            port_id,
            channel_id,
            sequence,
            data,
        };
        result.push(packet_state);
    }

    Ok(result)
}

/// get connection_identifier vector according by client_identifier
///
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let client_id = ClientId::default();
/// let result = get_client_connections(&client_id, client).await?;
/// ```
///
pub async fn get_client_connections(
    client_id: &ClientId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<ConnectionId>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_client_connections]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_client_connections] >> block_hash: {:?}",
        block_hash
    );

    // client_id <-> connection_id
    let connection_id: Vec<u8> = api
        .storage()
        .ibc()
        .connection_client(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;

    if connection_id.is_empty() {
        return Err(Box::from(format!(
            "get_client_connections is empty! by client_id = ({})",
            client_id
        )));
    }

    let mut result = vec![];

    let connection_id_str = String::from_utf8(connection_id).unwrap();
    let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

    result.push(connection_id);

    Ok(result)
}

///  # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let connection = ConnectionId::default();
/// let result = get_connection_channels(&connection, client).await?;
/// ```
///
pub async fn get_connection_channels(
    connection_id: &ConnectionId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connection_channels]");

    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();

    tracing::info!(
        "In call_ibc: [get_client_connections] >> block_hash: {:?}",
        block_hash
    );

    // connection_id <-> Ve<(port_id, channel_id)>
    let channel_id_and_port_id: Vec<(Vec<u8>, Vec<u8>)> = api
        .storage()
        .ibc()
        .channels_connection(connection_id.as_bytes().to_vec(), Some(block_hash))
        .await?;

    if channel_id_and_port_id.is_empty() {
        return Err(Box::from(format!(
            "get_connection_channels is empty! by connection_id = ({})",
            connection_id
        )));
    }

    let mut result = vec![];

    for (port_id, channel_id) in channel_id_and_port_id.iter() {
        // get port_id
        let port_id = String::from_utf8(port_id.clone()).unwrap();
        let port_id = PortId::from_str(port_id.as_str()).unwrap();

        // get channel_id
        let channel_id = String::from_utf8(channel_id.clone()).unwrap();
        let channel_id = ChannelId::from_str(channel_id.as_str()).unwrap();

        // get channel_end
        let channel_end = get_channel_end(&port_id, &channel_id, client.clone()).await?;

        result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
    }

    Ok(result)
}

/// ibc protocol core function, ics26 deliver function
/// this function will dispatch msg to process
///
///  # Usage example
///
/// ```rust
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let msg = vec![Any::default()];
/// let result = deliver(msg, client).await?;
/// ```
/// return block_hash, extrinsic_hash, and event
pub async fn deliver(
    msg: Vec<Any>,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<(subxt::sp_core::H256, subxt::sp_core::H256, Vec<RawEvent>), Box<dyn std::error::Error>>
{
    tracing::info!("in call_ibc: [deliver]");

    let msg: Vec<ibc_node::runtime_types::pallet_ibc::Any> = msg
        .into_iter()
        .map(|value| ibc_node::runtime_types::pallet_ibc::Any {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        })
        .collect();

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let result = api
        .tx()
        .ibc()
        .deliver(msg, 0)
        .sign_and_submit_then_watch(&signer)
        .await?;

    tracing::info!("deliver block hash = {:?}", result.block);
    tracing::info!("deliver extrinsic: {:?}", result.extrinsic);
    tracing::info!("deliver events : {:?}", result.events);

    Ok((result.block, result.extrinsic, result.events))
}

/// # get_mmr_leaf_and_mmr_proof
///
/// This get_mmr_leaf_and_mmr_proof api generate form generateProof api
/// generateProof(leafIndex: u64, at?: BlockHash): MmrLeafProof
/// interface: api.rpc.mmr.generateProof
/// jsonrpc: mmr_generateProof
/// summary: Generate MMR proof for given leaf index.
///
/// Return value a tuple (mmr_leaf, mmr_proof)
///
/// # Usage example
///
/// ```rust
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let block_number = Some(BlockNumber::from(12));
/// let block_hash = None;
/// let result = get_mmr_leaf_and_mmr_proof(block_number, block_hash, client).await?;
/// ```
///
pub async fn get_mmr_leaf_and_mmr_proof(
    block_number: Option<BlockNumber>,
    block_hash: Option<sp_core::H256>,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<(String, Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc [get_mmr_leaf_and_mmr_proof]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    // need to use `to_json_value` to convert the params to json value
    // need make sure mmr_generate_proof index is u64
    let params = &[to_json_value(block_number)?, to_json_value(block_hash)?];
    let generate_proof: pallet_mmr_rpc::LeafProof<String> = api
        .client
        .rpc()
        .client
        .request("mmr_generateProof", params)
        .await?;

    tracing::info!("info generate_proof : {:?}", generate_proof);
    // return block_hash, mmr_leaf, mmr_proof
    Ok((
        generate_proof.block_hash,
        generate_proof.leaf.0,
        generate_proof.proof.0,
    ))
}

/// get header by block hash
///
/// # Usage example
///
/// ```rust
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let block_hash = None;
/// let result = get_header_by_block_hash(block_hash, client).await?;
/// ```
///
pub async fn get_header_by_block_hash(
    block_hash: Option<sp_core::H256>,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<ibc::clients::ics10_grandpa::help::BlockHeader, Box<dyn std::error::Error>> {
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let header: subxt::sp_runtime::generic::Header<u32, subxt::sp_runtime::traits::BlakeTwo256> =
        api.client.rpc().header(block_hash).await?.unwrap();
    tracing::info!("header = {:?}", header);

    let header = convert_substrate_header_to_ibc_header(header);
    tracing::info!("convert header = {:?}", header);

    Ok(header.into())
}

/// get header by block number
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<ibc_node::DefaultConfig>().await?;
/// let block_number = Some(BlockNumber::from(2));
/// let result = get_header_by_block_number(block_number, client).await?;
/// ```
///
pub async fn get_header_by_block_number(
    block_number: Option<BlockNumber>,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<ibc::clients::ics10_grandpa::help::BlockHeader, Box<dyn std::error::Error>> {
    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();
    let block_hash = api.client.rpc().block_hash(block_number).await?;
    let header: subxt::sp_runtime::generic::Header<u32, subxt::sp_runtime::traits::BlakeTwo256> =
        api.client.rpc().header(block_hash).await?.unwrap();
    println!("header before = {:?}", header);
    tracing::info!("header = {:?}", header);

    let header = convert_substrate_header_to_ibc_header(header);
    println!("header after = {:?}", header);
    tracing::info!("convert header = {:?}", header);

    Ok(header.into())
}

/// convert substrate Header to Ibc Header
pub fn convert_substrate_header_to_ibc_header(
    header: subxt::sp_runtime::generic::Header<u32, subxt::sp_runtime::traits::BlakeTwo256>,
) -> beefy_light_client::header::Header {
    beefy_light_client::header::Header {
        parent_hash: Hash::from(header.parent_hash),
        number: header.number,
        state_root: Hash::from(header.state_root),
        extrinsics_root: Hash::from(header.extrinsics_root),
        digest: convert_substrate_digest_to_beefy_light_client_digest(header.digest),
    }
}

fn convert_substrate_digest_to_beefy_light_client_digest(
    digest: sp_runtime::Digest,
) -> beefy_light_client::header::Digest {
    beefy_light_client::header::Digest {
        logs: digest
            .logs
            .into_iter()
            .map(convert_substrate_digest_item_to_beefy_light_client_digest_item)
            .collect(),
    }
}

fn convert_substrate_digest_item_to_beefy_light_client_digest_item(
    digest_item: sp_runtime::DigestItem,
) -> beefy_light_client::header::DigestItem {
    match digest_item {
        sp_runtime::DigestItem::PreRuntime(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::PreRuntime(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Consensus(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::Consensus(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Seal(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::Seal(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Other(value) => {
            beefy_light_client::header::DigestItem::Other(value)
        }
        sp_runtime::DigestItem::RuntimeEnvironmentUpdated => {
            beefy_light_client::header::DigestItem::RuntimeEnvironmentUpdated
        }
    }
}

/// # Usage example
///
/// ```rust
///     let storage_entry = ibc_node::ibc::storage::ClientStates("10-grandpa-0".as_bytes().to_vec());
///     let storage_key = get_storage_key(&storage_entry);
///     println!("key = {:?}", storage_key);
/// ```
pub fn get_storage_key<F: StorageEntry>(store: &F) -> StorageKey {
    let prefix = StorageKeyPrefix::new::<F>();
    store.key().final_key(prefix)
}
