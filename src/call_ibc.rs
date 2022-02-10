use crate::ibc_node;
use core::time::Duration;
use ibc::events::IbcEvent;
use ibc::ics02_client::client_consensus::AnyConsensusState;
use ibc::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd};
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
use sp_core::storage::StorageKey;
use sp_keyring::AccountKeyring;
use subxt::storage::{StorageEntry, StorageKeyPrefix};
use subxt::BeefySubscription;
use subxt::ClientBuilder;
use subxt::SignedCommitment;
use subxt::{BlockNumber, Client, EventSubscription, PairSigner};
use tendermint_proto::Protobuf;
use tokio::time::sleep;
use beefy_merkle_tree::Hash;

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
                sleep(Duration::from_secs(10)).await;
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

#[tokio::test]
async fn test_subscribe_ibc_event() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

/// Subscribe beefy justifiactions
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

#[tokio::test]
async fn test_get_latest_height() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
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

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc : [get_connection_end] >> block_hash: {:?}",
        block_hash
    );

    let data = api
        .storage()
        .ibc()
        .connections(connection_identifier.as_bytes().to_vec(), Some(block_hash))
        .await?;

    assert!(!data.is_empty());

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

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_channelend] >> block_hash: {:?}",
        block_hash
    );

    let data: Vec<u8> = loop {
        let data: Vec<u8> = api
            .storage()
            .ibc()
            .channels(
                port_id.as_bytes().to_vec(),
                channel_id.as_bytes().to_vec(),
                Some(block_hash),
            )
            .await?;

        if !data.is_empty() {
            break data;
        } else {
            continue;
        }
    };
    assert!(!data.is_empty());

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

    let block_hash: sp_core::H256 = block_header.hash();
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
    assert!(!data.is_empty());

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

    let block_hash: sp_core::H256 = block_header.hash();
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
    assert!(!data.is_empty());

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

    let block_hash: sp_core::H256 = block_header.hash();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .client_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;
    assert!(!data.is_empty());

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

    let block_hash: sp_core::H256 = block_header.hash();

    tracing::info!(
        "In call_ibc: [get_client_consensus] >> block_hash: {:?}",
        block_hash
    );

    let data = api
        .storage()
        .ibc()
        .consensus_states(client_id.as_bytes().to_vec(), Some(block_hash))
        .await?;
    assert!(!data.is_empty());

    // get the height consensus_state
    let mut consensus_state = vec![];
    for item in data.iter() {
        if item.0 == height.encode_vec().unwrap() {
            consensus_state = item.1.clone();
        }
    }

    println!("call_ibc: [consensus_state] >> consensus_state >> {:?}", consensus_state);

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

    let block_hash: sp_core::H256 = block_header.hash();
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
    assert!(!ret.is_empty());

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

    let block_hash: sp_core::H256 = block_header.hash();

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
    assert!(!client_states_keys.is_empty());

    // enumate every item get client_state value
    for key in client_states_keys {
        // get client_state value
        let client_states_value: Vec<u8> = api
            .storage()
            .ibc()
            .client_states(key.clone(), Some(block_hash))
            .await?;
        assert!(!client_states_value.is_empty());
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
    assert!(!connection_keys.is_empty());

    for key in connection_keys {
        // get connectons value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .connections(key.clone(), Some(block_hash))
            .await?;
        assert!(!value.is_empty());
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
    assert!(!channels_keys.is_empty());

    for key in channels_keys {
        // get value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .channels(key.0.clone(), key.1.clone(), Some(block_hash))
            .await?;
        assert!(!value.is_empty());
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
    let packet_commitments_keys: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> = api
        .storage()
        .ibc()
        .packet_commitment_keys(Some(block_hash))
        .await?;
    assert!(!packet_commitments_keys.is_empty());

    for key in packet_commitments_keys {
        // get value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .packet_commitment(
                key.0.clone(),
                key.1.clone(),
                key.2.clone(),
                Some(block_hash),
            )
            .await?;
        assert!(!!value.is_empty());
        // store key-value
        ret.push((key.0.clone(), key.1.clone(), key.2.clone(), value));
    }

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

    let block_hash: sp_core::H256 = block_header.hash();
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

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: sp_core::H256 = block_header.hash();
    tracing::info!(
        "In call_ibc: [get_acknowledge_packet_state] >> block_hash: {:?}",
        block_hash
    );

    let mut ret = vec![];

    let acknowledgements_keys: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> = api
        .storage()
        .ibc()
        .acknowledgements_keys(Some(block_hash))
        .await?;
    assert!(!acknowledgements_keys.is_empty());

    for key in acknowledgements_keys {
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .acknowledgements(
                key.0.clone(),
                key.1.clone(),
                key.2.clone(),
                Some(block_hash),
            )
            .await?;
        assert!(!value.is_empty());
        ret.push((key.0.clone(), key.1.clone(), key.2.clone(), value));
    }

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

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let result = api
        .tx()
        .ibc()
        .deliver(msg, 0)
        .sign_and_submit(&signer)
        .await?;

    tracing::info!("deliver result: {:?}", result);

    Ok(result)
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
pub async fn get_mmr_leaf_and_mmr_proof(
    block_number: u64,
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
    // return mmr_leaf, mmr_proof
    Ok((
        generate_proof.block_hash,
        generate_proof.leaf.0,
        generate_proof.proof.0,
    ))
}

/// get header by block hash
pub async fn get_header_by_block_hash(
    client: Client<ibc_node::DefaultConfig>,
    block_hash: Option<sp_core::H256>,
) -> Result<ibc::ics10_grandpa::help::BlockHeader, Box<dyn std::error::Error>> {
    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let header: subxt::sp_runtime::generic::Header<u32, subxt::sp_runtime::traits::BlakeTwo256> =
        api.client.rpc().header(block_hash).await?.unwrap();
    tracing::info!("header = {:?}", header);

    let header = convert_substrate_header_to_ibc_header(header);
    tracing::info!("convert header = {:?}", header);

    Ok(header.into())
}

/// get header by block number
pub async fn get_header_by_block_number(
    client: Client<ibc_node::DefaultConfig>,
    block_number: Option<BlockNumber>,
) -> Result<ibc::ics10_grandpa::help::BlockHeader, Box<dyn std::error::Error>> {
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

fn convert_substrate_digest_to_beefy_light_client_digest(digest: sp_runtime::Digest<sp_core::H256>)
 -> beefy_light_client::header::Digest {
    beefy_light_client::header::Digest {
        logs: digest.logs.into_iter().map(|value | convert_substrate_digest_item_to_beefy_light_client_digest_item(value)).collect(),
    }
}

fn convert_substrate_digest_item_to_beefy_light_client_digest_item(digest_item: sp_runtime::DigestItem<sp_core::H256>)
 -> beefy_light_client::header::DigestItem {
    match digest_item {
        sp_runtime::DigestItem::ChangesTrieRoot(value ) => {
            beefy_light_client::header::DigestItem::ChangesTrieRoot(Hash::from(value))
        },
        sp_runtime::DigestItem::PreRuntime(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::PreRuntime(consensus_engine_id, value)
        },
        sp_runtime::DigestItem::Consensus(consensus_engine_id, value ) => {
            beefy_light_client::header::DigestItem::Consensus(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Seal(consensus_engine_id, value ) => {
            beefy_light_client::header::DigestItem::Seal(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::ChangesTrieSignal(changes_trie_signal) => {
            beefy_light_client::header::DigestItem::ChangesTrieSignal(convert_changes_trie_signal(changes_trie_signal))
        }
        sp_runtime::DigestItem::Other(value ) => {
            beefy_light_client::header::DigestItem::Other(value)
        }
        sp_runtime::DigestItem::RuntimeEnvironmentUpdated => beefy_light_client::header::DigestItem::RuntimeEnvironmentUpdated,
    }
}

fn convert_changes_trie_signal(value: sp_runtime::generic::ChangesTrieSignal) -> beefy_light_client::header::ChangesTrieSignal {
    match value {
        sp_runtime::generic::ChangesTrieSignal::NewConfiguration(value) => {
            if value.is_some() {
                beefy_light_client::header::ChangesTrieSignal::NewConfiguration(Some(convert_changes_trie_configuration(value.unwrap())))
            } else {
                beefy_light_client::header::ChangesTrieSignal::NewConfiguration(None)
            }
        }
    }
}

fn convert_changes_trie_configuration(value: sp_core::ChangesTrieConfiguration) -> beefy_light_client::header::ChangesTrieConfiguration {
    beefy_light_client::header::ChangesTrieConfiguration {
        digest_interval: value.digest_interval,
        digest_levels: value.digest_levels
    }
}

pub fn get_storage_key<F: StorageEntry>(store: &F) -> StorageKey {
    let prefix = StorageKeyPrefix::new::<F>();
    let key = store.key().final_key(prefix);
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibc_node;
    use subxt::ClientBuilder;

    // test API get_block_header
    // use `cargo test -- --captuer` can print content
    #[tokio::test]
    async fn test_get_block_header() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let block_number = Some(BlockNumber::from(3));
        let header = get_header_by_block_number(client, block_number).await?;

        println!("convert header = {:?}", header);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_key() -> Result<(), Box<dyn std::error::Error>> {
        use subxt::StorageEntry;
        let ibc = crate::ibc_node::ibc::storage::ClientStatesKeys;
        let result = ibc.key();

        Ok(())
    }

    #[tokio::test]
    async fn test_get_mmr_leaf_and_mmr_proof() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        let api = client
            .clone()
            .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

        let block_number = 22;

        let block_hash: sp_core::H256 = api
            .client
            .rpc()
            .block_hash(Some(BlockNumber::from(block_number)))
            .await?
            .unwrap();

        println!("block_hash = {:?}", block_hash);

        let result =
            get_mmr_leaf_and_mmr_proof((block_number - 1) as u64, Some(block_hash), client).await?;

        println!("result = {:?}", result);

        Ok(())
    }

    // add unit test for get storage key
    #[test]
    fn test_get_storage_key() {
        let _ibc = crate::ibc_node::ibc::storage::ClientStates(vec![1, 2, 3]);

        let ibc = crate::ibc_node::ibc::storage::ClientStatesKeys;

        // let result = get_storage_key(&ibc);
        // println!("key = {:?}", result);

        let _ibc = crate::ibc_node::ibc::storage::ConsensusStates(vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::Connections(vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::ConnectionsKeys;

        let _ibc = crate::ibc_node::ibc::storage::Channels(vec![1, 2, 3], vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::ChannelsKeys;

        let _ibc = crate::ibc_node::ibc::storage::ChannelsConnection(vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::NextSequenceSend(vec![1, 2, 3], vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::NextSequenceRecv(vec![1, 2, 3], vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::NextSequenceAck(vec![1, 2, 3], vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::Acknowledgements(
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
        );

        let _ibc = crate::ibc_node::ibc::storage::AcknowledgementsKeys;

        let _ibc = crate::ibc_node::ibc::storage::Clients(vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::ClientCounter;

        let _ibc = crate::ibc_node::ibc::storage::ConnectionCounter;

        let _ibc = crate::ibc_node::ibc::storage::ChannelCounter;

        let _ibc = crate::ibc_node::ibc::storage::ConnectionClient(vec![1, 2, 3]);

        let _ibc = crate::ibc_node::ibc::storage::PacketReceipt(
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
        );

        let _ibc = crate::ibc_node::ibc::storage::PacketCommitment(
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
        );

        let _ibc = crate::ibc_node::ibc::storage::PacketCommitmentKeys;

        let _ibc = crate::ibc_node::ibc::storage::SendPacketEvent(vec![1, 2, 3], vec![1, 2, 3], 1);

        let _ibc =
            crate::ibc_node::ibc::storage::WriteAckPacketEvent(vec![1, 2, 3], vec![1, 2, 3], 1);

        let _ibc = crate::ibc_node::ibc::storage::LatestHeight;

        let _ibc = crate::ibc_node::ibc::storage::OldHeight;
    }
}
