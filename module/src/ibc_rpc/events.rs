use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use anyhow::Result;
use futures::StreamExt;
use ibc::events::IbcEvent;
use subxt::{Client, RawEventDetails};

/// Subscribe ibc events
/// Maybe in the future call ocw
///
/// # Usage example
///
/// ```rust
/// use subxt::ClientBuilder;
/// use octopusxt::MyConfig;
/// use octopusxt::subscribe_ibc_event;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let result = subscribe_ibc_event(client).await?;
/// ```
///
pub async fn subscribe_ibc_event(client: Client<MyConfig>) -> Result<Vec<IbcEvent>> {
    tracing::info!("In call_ibc: [subscribe_events]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    // Subscribe to any events that occur:
    let mut event_sub = api.events().subscribe().await?;

    let mut result_events = Vec::new();

    // Our subscription will see the events emitted as a result of this:
    while let Some(events) = event_sub.next().await {
        let events = events?;

        for event in events.iter_raw() {
            let event: RawEventDetails = event?;

            let raw_event = event.clone();

            let ibc_event = inner_process_ibc_event(raw_event);

            result_events.push(ibc_event);
        }
    }
    Ok(result_events)
}

/// convert substrate event to ibc event
pub fn from_substrate_event_to_ibc_event(raw_events: Vec<RawEventDetails>) -> Vec<IbcEvent> {
    fn inner_convert_event(raw_events: Vec<RawEventDetails>, module: &str) -> Vec<RawEventDetails> {
        raw_events
            .into_iter()
            .filter(|raw| raw.pallet == module)
            .collect::<Vec<_>>()
    }

    let ret = inner_convert_event(raw_events, "Ibc");

    ret.into_iter()
        .map(inner_process_ibc_event)
        .collect::<Vec<_>>()
}

pub fn inner_process_ibc_event(raw_event: RawEventDetails) -> IbcEvent {
    let variant = raw_event.variant;

    match variant.as_str() {
        "CreateClient" => {
            let event = <ibc_node::ibc::events::CreateClient as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

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
            let event = <ibc_node::ibc::events::ClientMisbehaviour as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

            let height = event.0;
            let client_id = event.1;
            let client_type = event.2;
            let consensus_height = event.3;

            use ibc::core::ics02_client::events::Attributes;
            IbcEvent::ClientMisbehaviour(ibc::core::ics02_client::events::ClientMisbehaviour::from(
                Attributes {
                    height: height.to_ibc_height(),
                    client_id: client_id.to_ibc_client_id(),
                    client_type: client_type.to_ibc_client_type(),
                    consensus_height: consensus_height.to_ibc_height(),
                },
            ))
        }
        "OpenInitConnection" => {
            let event = <ibc_node::ibc::events::OpenInitConnection as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

            let height = event.0;
            let connection_id = event.1.map(|val| val.to_ibc_connection_id());
            let client_id = event.2;
            let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
            let counterparty_client_id = event.4;

            use ibc::core::ics03_connection::events::Attributes;
            IbcEvent::OpenInitConnection(ibc::core::ics03_connection::events::OpenInit::from(
                Attributes {
                    height: height.to_ibc_height(),
                    connection_id,
                    client_id: client_id.to_ibc_client_id(),
                    counterparty_connection_id,
                    counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                },
            ))
        }
        "OpenTryConnection" => {
            let event = <ibc_node::ibc::events::OpenTryConnection as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

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
            let event = <ibc_node::ibc::events::OpenAckConnection as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

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
            let event = <ibc_node::ibc::events::OpenConfirmConnection as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

            let height = event.0;
            let connection_id = event.1.map(|val| val.to_ibc_connection_id());
            let client_id = event.2;
            let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
            let counterparty_client_id = event.4;

            use ibc::core::ics03_connection::events::Attributes;
            IbcEvent::OpenConfirmConnection(ibc::core::ics03_connection::events::OpenConfirm::from(
                Attributes {
                    height: height.to_ibc_height(),
                    connection_id,
                    client_id: client_id.to_ibc_client_id(),
                    counterparty_connection_id,
                    counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                },
            ))
        }

        "OpenInitChannel" => {
            let event = <ibc_node::ibc::events::OpenInitChannel as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

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
            let event = <ibc_node::ibc::events::OpenConfirmChannel as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

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
            let event = <ibc_node::ibc::events::CloseConfirmChannel as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

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

            let receive_packet = ibc::core::ics04_channel::events::ReceivePacket {
                height: event.0.to_ibc_height(),
                packet: event.1.to_ibc_packet(),
            };

            IbcEvent::ReceivePacket(receive_packet)
        }
        "WriteAcknowledgement" => {
            let event = <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

            let write_acknowledgement = ibc::core::ics04_channel::events::WriteAcknowledgement {
                height: event.0.to_ibc_height(),
                packet: event.1.to_ibc_packet(),
                ack: event.2,
            };

            IbcEvent::WriteAcknowledgement(write_acknowledgement)
        }
        "AcknowledgePacket" => {
            let event = <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

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

            let timeout_packet = ibc::core::ics04_channel::events::TimeoutPacket {
                height: event.0.to_ibc_height(),
                packet: event.1.to_ibc_packet(),
            };

            IbcEvent::TimeoutPacket(timeout_packet)
        }
        "TimeoutOnClosePacket" => {
            let event = <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

            let timeout_on_close_packet = ibc::core::ics04_channel::events::TimeoutOnClosePacket {
                height: event.0.to_ibc_height(),
                packet: event.1.to_ibc_packet(),
            };

            IbcEvent::TimeoutOnClosePacket(timeout_on_close_packet)
        }
        "Empty" => {
            let event =
                <ibc_node::ibc::events::Empty as codec::Decode>::decode(&mut &raw_event.data[..])
                    .unwrap();

            let data = String::from_utf8(event.0).unwrap();

            IbcEvent::Empty(data)
        }
        "ChainError" => {
            let event = <ibc_node::ibc::events::ChainError as codec::Decode>::decode(
                &mut &raw_event.data[..],
            )
            .unwrap();

            let data = String::from_utf8(event.0).unwrap();

            IbcEvent::Empty(data)
        }
        _ => unimplemented!(),
    }
}
