use prost_types::Any;
use std::str::FromStr;

use crate::ibc_node::runtime_types::pallet_ibc::{
    event::primitive::{
        ChannelId as OctopusxtChannelId, ClientId as OctopusxtClientId,
        ClientType as OctopusxtClientType, ConnectionId as OctopusxtConnectionId,
        Height as OctopusxtHeight, Packet as OctopusxtPacket, PortId as OctopusxtPortId,
        Sequence as OctopusxtSequence, Timestamp as OctopusxtTimestamp,
    },
    Any as OctopusxtAny,
};

use ibc::{
    core::{
        ics02_client::client_type::ClientType as IbcClientType,
        ics04_channel::packet::{Packet as IbcPacket, Sequence as IbcSequence},
        ics24_host::identifier::{
            ChannelId as IbcChannelId, ClientId as IbcClientId, ConnectionId as IbcConnectionId,
            PortId as IbcPortId,
        },
    },
    timestamp::Timestamp as IbcTimestamp,
    Height as IbcHeight,
};
use ibc::core::ics04_channel::timeout::TimeoutHeight;

impl From<OctopusxtClientType> for IbcClientType {
    fn from(octopus_client_type: OctopusxtClientType) -> Self {
        match octopus_client_type {
            OctopusxtClientType::Tendermint => Self::Tendermint,
            OctopusxtClientType::Grandpa => Self::Grandpa,
        }
    }
}

impl From<OctopusxtHeight> for IbcHeight {
    fn from(height: OctopusxtHeight) -> Self {
        if height.revision_number == 0 {
            return IbcHeight::new(1, height.revision_height).unwrap();
        }

        IbcHeight::new(height.revision_number, height.revision_height).unwrap()
    }
}

impl From<OctopusxtPacket> for IbcPacket {
    fn from(octopus_packet: OctopusxtPacket) -> Self {
        Self {
            sequence: octopus_packet.sequence.into(),
            source_port: octopus_packet.source_port.into(),
            source_channel: octopus_packet.source_channel.into(),
            destination_port: octopus_packet.destination_port.into(),
            destination_channel: octopus_packet.destination_channel.into(),
            data: octopus_packet.data,
            timeout_height: TimeoutHeight::At(octopus_packet.timeout_height.into()),
            timeout_timestamp: octopus_packet.timeout_timestamp.into(),
        }
    }
}

impl From<OctopusxtConnectionId> for IbcConnectionId {
    fn from(octopus_connection_id: OctopusxtConnectionId) -> Self {
        let value = String::from_utf8(octopus_connection_id.0).unwrap();
        Self(value)
    }
}

impl From<OctopusxtChannelId> for IbcChannelId {
    fn from(octopus_channel_id: OctopusxtChannelId) -> Self {
        let value = String::from_utf8(octopus_channel_id.0).unwrap();
        Self::from_str(&value).unwrap()
    }
}

impl From<OctopusxtPortId> for IbcPortId {
    fn from(octopus_port_id: OctopusxtPortId) -> Self {
        let value = String::from_utf8(octopus_port_id.0).unwrap();
        Self(value)
    }
}

impl From<OctopusxtClientId> for IbcClientId {
    fn from(octopus_client_id: OctopusxtClientId) -> Self {
        let value = String::from_utf8(octopus_client_id.0).unwrap();
        Self(value)
    }
}

impl From<OctopusxtSequence> for IbcSequence {
    fn from(octopus_sequence: OctopusxtSequence) -> Self {
        Self::from(octopus_sequence.0)
    }
}

impl From<OctopusxtTimestamp> for IbcTimestamp {
    fn from(octopus_timestamp: OctopusxtTimestamp) -> Self {
        let value = String::from_utf8(octopus_timestamp.time).unwrap();
        Self::from_str(&value).unwrap()
    }
}

impl From<Any> for OctopusxtAny {
    fn from(value: Any) -> Self {
        Self {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        }
    }
}

impl Copy for OctopusxtHeight {}

impl Clone for OctopusxtHeight {
    fn clone(&self) -> Self {
        Self {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}
