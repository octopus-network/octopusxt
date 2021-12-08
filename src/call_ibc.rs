use crate::ibc_node;
use core::time::Duration;
use ibc::events::IbcEvent;
use ibc::ics02_client::client_consensus::AnyConsensusState;
use ibc::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::ics03_connection::connection::{ConnectionEnd, Counterparty, IdentifiedConnectionEnd};
use ibc::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::ics04_channel::error::Error as Ics04Error;
use ibc::ics04_channel::packet::{Packet, Receipt, Sequence};
use ibc::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use ibc::Height as ICSHeight;
use ibc_proto::ibc::core::channel::v1::PacketState;

use codec::{Decode, Encode};
use core::str::FromStr;
use jsonrpsee::types::to_json_value;
use prost_types::Any;
use sp_keyring::AccountKeyring;
use subxt::{Client, EventSubscription, PairSigner};
use tendermint_proto::Protobuf;
use tokio::time::sleep;

/// Subscribe ibc events
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
        // tracing::info!("In substrate: [subscribe_events] >> variant: {:?}", variant);
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

                use ibc::ics02_client::events::Attributes;
                events.push(IbcEvent::CreateClient(
                    ibc::ics02_client::events::CreateClient(Attributes {
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

                use ibc::ics02_client::events::Attributes;
                events.push(IbcEvent::UpdateClient(
                    ibc::ics02_client::events::UpdateClient {
                        common: Attributes {
                            height: height.to_ibc_height(),
                            client_id: client_id.to_ibc_client_id(),
                            client_type: client_type.to_ibc_client_type(),
                            consensus_height: consensus_height.to_ibc_height(),
                        },
                        header: None,
                    },
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

                use ibc::ics02_client::events::Attributes;
                events.push(IbcEvent::ClientMisbehaviour(
                    ibc::ics02_client::events::ClientMisbehaviour(Attributes {
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

                use ibc::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenInitConnection(
                    ibc::ics03_connection::events::OpenInit(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenTryConnection(
                    ibc::ics03_connection::events::OpenTry(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenAckConnection(
                    ibc::ics03_connection::events::OpenAck(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics03_connection::events::Attributes;
                events.push(IbcEvent::OpenConfirmConnection(
                    ibc::ics03_connection::events::OpenConfirm(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics04_channel::events::Attributes;
                events.push(IbcEvent::OpenInitChannel(
                    ibc::ics04_channel::events::OpenInit(Attributes {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id: counterparty_channel_id,
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics04_channel::events::Attributes;
                events.push(IbcEvent::OpenTryChannel(
                    ibc::ics04_channel::events::OpenTry(Attributes {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id: counterparty_channel_id,
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics04_channel::events::Attributes;
                events.push(IbcEvent::OpenAckChannel(
                    ibc::ics04_channel::events::OpenAck(Attributes {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id: counterparty_channel_id,
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics04_channel::events::Attributes;
                events.push(IbcEvent::OpenConfirmChannel(
                    ibc::ics04_channel::events::OpenConfirm(Attributes {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id: counterparty_channel_id,
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics04_channel::events::Attributes;
                events.push(IbcEvent::CloseInitChannel(
                    ibc::ics04_channel::events::CloseInit(Attributes {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id: counterparty_channel_id,
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
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

                use ibc::ics04_channel::events::Attributes;
                events.push(IbcEvent::CloseConfirmChannel(
                    ibc::ics04_channel::events::CloseConfirm(Attributes {
                        height: height.to_ibc_height(),
                        port_id: port_id.to_ibc_port_id(),
                        channel_id: channel_id,
                        connection_id: connection_id.to_ibc_connection_id(),
                        counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                        counterparty_channel_id: counterparty_channel_id,
                    }),
                ));
                sleep(Duration::from_secs(10)).await;
                break;
            }
            "SendPacket" => {
                let event = <ibc_node::ibc::events::SendPacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> SendPacket Event");

                let height = event.0;
                let packet = event.1;

                events.push(IbcEvent::SendPacket(
                    ibc::ics04_channel::events::SendPacket {
                        height: height.to_ibc_height(),
                        packet: packet.to_ibc_packet(),
                    },
                ));
                sleep(Duration::from_secs(10)).await;
                break;
            }
            "ReceivePacket" => {
                let event = <ibc_node::ibc::events::ReceivePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();

                tracing::info!("In call_ibc: [substrate_events] >> ReceivePacket Event");

                let height = event.0;
                let packet = event.1;

                events.push(IbcEvent::ReceivePacket(
                    ibc::ics04_channel::events::ReceivePacket {
                        height: height.to_ibc_height(),
                        packet: packet.to_ibc_packet(),
                    },
                ));

                sleep(Duration::from_secs(10)).await;
                break;
            }
            "WriteAcknowledgement" => {
                let event = <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> WriteAcknowledgement Event");


                let height = event.0;
                let packet = event.1;

                let ack = event.2;

                events.push(IbcEvent::WriteAcknowledgement(
                    ibc::ics04_channel::events::WriteAcknowledgement {
                        height: height.to_ibc_height(),
                        packet: packet.to_ibc_packet(),
                        ack: ack,
                    },
                ));

                sleep(Duration::from_secs(10)).await;
                break;
            }
            "AcknowledgePacket" => {
                let event = <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> AcknowledgePacket Event");


                let height = event.0;
                let packet = event.1;

                events.push(IbcEvent::AcknowledgePacket(
                    ibc::ics04_channel::events::AcknowledgePacket {
                        height: height.to_ibc_height(),
                        packet: packet.to_ibc_packet(),
                    },
                ));

                sleep(Duration::from_secs(10)).await;
                break;
            }
            "TimeoutPacket" => {
                let event = <ibc_node::ibc::events::TimeoutPacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> TimeoutPacket Event");


                let height = event.0;
                let packet = event.1;

                events.push(IbcEvent::TimeoutPacket(
                    ibc::ics04_channel::events::TimeoutPacket {
                        height: height.to_ibc_height(),
                        packet: packet.to_ibc_packet(),
                    },
                ));

                sleep(Duration::from_secs(10)).await;
                break;
            }
            "TimeoutOnClosePacket" => {
                let event = <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
                )
                .unwrap();
                tracing::info!("In call_ibc: [substrate_events] >> TimeoutOnClosePacket Event");

                let height = event.0;
                let packet = event.1;

                events.push(IbcEvent::TimeoutOnClosePacket(
                    ibc::ics04_channel::events::TimeoutOnClosePacket {
                        height: height.to_ibc_height(),
                        packet: packet.to_ibc_packet(),
                    },
                ));

                sleep(Duration::from_secs(10)).await;
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
                sleep(Duration::from_secs(10)).await;
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
                sleep(Duration::from_secs(10));
                break;
            }
            "ExtrinsicSuccess" => {
                let event = <ibc_node::system::events::ExtrinsicSuccess as codec::Decode>::decode(
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

/// get connectionEnd according by connection_identifier and read Connections StorageMaps
pub async fn get_connection_end(
    connection_identifier: &ConnectionId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<ConnectionEnd, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connection_end]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();
    tracing::info!(
        "In call_ibc : [get_connection_end] >> block_hash: {:?}",
        block_hash
    );

    let data = api
        .storage()
        .ibc()
        .connections(connection_identifier.as_bytes().to_vec(), Some(block_hash))
        .await?;

    let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

    Ok(connection_end)
}

/// get channelEnd according by port_identifier, channel_identifier and read Channles StorageMaps
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

    let rpc_client = client.rpc().client.clone();

    let params = &[
        to_json_value(port_id.as_bytes())?,
        to_json_value(channel_id.as_bytes())?,
    ];
    tracing::info!(
        "in call_ibc: [get_channel_end] >> params: {:?}",
        params.clone()
    );

    let data: Vec<u8> = rpc_client.request("get_channel_end", params).await?;

    tracing::info!(
        "in call_ibc: [get_channel_end] >> data >> {:?}",
        data.clone()
    );

    let channel_end = ChannelEnd::decode_vec(&*data).unwrap();
    tracing::info!(
        "in call_ibc: [get_channel_end] >> channel_end >> {:?}",
        channel_end.clone()
    );

    Ok(channel_end)
}

/// get packet receipt by port_id, channel_id and sequence
pub async fn get_packet_receipt(
    port_id: &PortId,
    channel_id: &ChannelId,
    seq: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Receipt, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc : [get_packet_receipt]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();
    tracing::info!(
        "In call_ibc : [get_packet_receipt] >> block_hash: {:?}",
        block_hash
    );

    let _seq = u64::from(*seq).encode();

    let data = api
        .storage()
        .ibc()
        .packet_receipt(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            _seq,
            Some(block_hash),
        )
        .await?;

    let _data = String::from_utf8(data).unwrap();
    if _data.eq("Ok") {
        Ok(Receipt::Ok)
    } else {
        Err(format!("unrecognized packet receipt: {:?}", _data).into())
    }
}

/// get send packet event by port_id, channel_id and sequence
pub async fn get_send_packet_event(
    port_id: &PortId,
    channel_id: &ChannelId,
    seq: &Sequence,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Packet, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_send_packet_event]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_send_packet_event] >> block_hash: {:?}",
        block_hash
    );

    let data = api
        .storage()
        .ibc()
        .send_packet_event(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            u64::from(*seq),
            Some(block_hash),
        )
        .await?;

    let packet = Packet::decode_vec(&*data).unwrap();
    Ok(packet)
}

/// get client_state according by client_id, and read ClientStates StoraageMap
pub async fn get_client_state(
    client_id: &ClientId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<AnyClientState, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc : [get_client_state]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .client_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;

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
pub async fn get_client_consensus(
    client_id: &ClientId,
    height: ICSHeight,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<AnyConsensusState, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_client_consensus]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();

    tracing::info!(
        "In call_ibc: [get_client_consensus] >> block_hash: {:?}",
        block_hash
    );

    let data = api
        .storage()
        .ibc()
        .consensus_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;

    // get the height consensus_state
    let mut consensus_state = vec![];
    for item in data.iter() {
        if item.0 == height.encode_vec().unwrap() {
            consensus_state = item.1.clone();
        }
    }

    let consensus_state = AnyConsensusState::decode_vec(&*consensus_state).unwrap();

    Ok(consensus_state)
}

pub async fn get_consensus_state_with_height(
    client_id: &ClientId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<(ICSHeight, AnyConsensusState)>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_consensus_state_with_height]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_client_consensus] >> block_hash: {:?}",
        block_hash
    );

    // vector<height, consensus_state>
    let ret: Vec<(Vec<u8>, Vec<u8>)> = api
        .storage()
        .ibc()
        .consensus_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;

    let mut result = vec![];
    for (height, consensus_state) in ret.iter() {
        let height = ICSHeight::decode_vec(&*height).unwrap();
        let consensus_state = AnyConsensusState::decode_vec(&*consensus_state).unwrap();
        result.push((height, consensus_state));
    }

    Ok(result)
}

pub async fn get_unreceipt_packet(
    port_id: &PortId,
    channel_id: &ChannelId,
    seqs: Vec<u64>,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_receipt_packet]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();

    tracing::info!(
        "In call_ibc: [get_receipt_packet] >> block_hash: {:?}",
        block_hash
    );

    let mut result = Vec::new();

    let pair = seqs
        .into_iter()
        .map(|seq| {
            (
                port_id.clone().as_bytes().to_vec(),
                channel_id.clone().as_bytes().to_vec(),
                (seq.encode(), seq),
            )
        })
        .collect::<Vec<_>>();

    for (port_id, channel_id, (seq_u8, seq)) in pair.into_iter() {
        let ret: Vec<u8> = api
            .storage()
            .ibc()
            .packet_receipt(port_id, channel_id, seq_u8, Some(block_hash.clone()))
            .await?;
        if ret.is_empty() {
            result.push(seq);
        }
    }

    Ok(result)
}

/// get key-value pair (client_identifier, client_state) construct IdentifieredAnyClientstate
pub async fn get_clients(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedAnyClientState>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_clients]");

    let rpc_client = client.rpc().client.clone();

    let ret: Vec<(Vec<u8>, Vec<u8>)> = rpc_client
        .request("get_identified_any_client_state", &[])
        .await?;

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
pub async fn get_connections(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedConnectionEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connections]");

    let rpc_client = client.rpc().client.clone();

    let ret: Vec<(Vec<u8>, Vec<u8>)> = rpc_client
        .request("get_identified_connection_end", &[])
        .await?;

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
pub async fn get_channels(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_channels]");

    let rpc_client = client.rpc().client.clone();

    let ret: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> = rpc_client.request("get_channel_end", &[]).await?;

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
pub async fn get_commitment_packet_state(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_commitment_packet_state]");

    let rpc_client = client.rpc().client.clone();

    let ret: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = rpc_client
        .request("get_packet_commitment_state", &[])
        .await?;

    let mut result = vec![];

    for (port_id, channel_id, seq, data) in ret.into_iter() {
        let port_id = String::from_utf8(port_id).unwrap();
        let channel_id = String::from_utf8(channel_id).unwrap();
        let mut seq: &[u8] = &seq;
        let seq = u64::decode(&mut seq).unwrap();
        let packet_state = PacketState {
            port_id: port_id,
            channel_id: channel_id,
            sequence: seq,
            data,
        };
        result.push(packet_state);
    }

    Ok(result)
}

/// get packet commitment by port_id, channel_id and sequence to verify if the ack has been received by the sending chain
pub async fn get_packet_commitment(
    port_id: &PortId,
    channel_id: &ChannelId,
    seq: u64,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_packet_commitment]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_packet_commitment] >> block_hash: {:?}",
        block_hash
    );

    let _seq = seq.encode();

    let data = api
        .storage()
        .ibc()
        .packet_commitment(
            port_id.as_bytes().to_vec(),
            channel_id.as_bytes().to_vec(),
            _seq,
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        Err(Box::new(Ics04Error::packet_commitment_not_found(Sequence(
            seq,
        ))))
    } else {
        Ok(data)
    }
}

/// get get_commitment_packet_state
pub async fn get_acknowledge_packet_state(
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_acknowledge_packet_state]");

    let rpc_client = client.rpc().client.clone();

    let ret: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = rpc_client
        .request("get_packet_acknowledge_state", &[])
        .await?;

    let mut result = vec![];

    for (port_id, channel_id, seq, data) in ret.into_iter() {
        let port_id = String::from_utf8(port_id).unwrap();
        let channel_id = String::from_utf8(channel_id).unwrap();
        let mut seq: &[u8] = &seq;
        let seq = u64::decode(&mut seq).unwrap();
        let packet_state = PacketState {
            port_id: port_id,
            channel_id: channel_id,
            sequence: seq,
            data,
        };
        result.push(packet_state);
    }

    Ok(result)
}

/// get connection_identifier vector according by client_identifier
pub async fn get_client_connections(
    client_id: ClientId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<ConnectionId>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_client_connections]");
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();
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
        return Ok(Vec::new());
    }

    let mut result = vec![];

    let connection_id_str = String::from_utf8(connection_id.clone()).unwrap();
    let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

    result.push(connection_id);

    Ok(result)
}

pub async fn get_connection_channels(
    connection_id: ConnectionId,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connection_channels]");

    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash = block_header.hash();

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

pub async fn deliver(
    msg: Vec<Any>,
    client: Client<ibc_node::DefaultConfig>,
) -> Result<subxt::sp_core::H256, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [deliver]");

    let msg: Vec<ibc_node::runtime_types::pallet_ibc::Any> =
        msg.into_iter().map(|val| val.into()).collect();

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let result = api
        .tx()
        .ibc()
        .deliver(msg, 0)
        .sign_and_submit(&signer)
        .await?;

    tracing::info!("deliver result: {:?}", result);

    Ok(result)
}
