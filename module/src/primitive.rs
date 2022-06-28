use prost_types::Any;
use std::str::FromStr;
use codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

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
        Self {
            revision_number: height.revision_number,
            revision_height: height.revision_height,
        }
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
            timeout_height: octopus_packet.timeout_height.into(),
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


#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct IdentifiedChannel {
    pub channel_id: Vec<u8>,
    pub port_id: Vec<u8>,
    /// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
    pub channel_end: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct IdentifiedClientState {
    pub client_id: Vec<u8>,
    /// Protobuf encoded `AnyClientState`
    pub client_state: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct IdentifiedConnection {
    pub connection_id: Vec<u8>,
    /// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
    pub connection_end: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryClientStateResponse {
    /// Protobuf encoded `AnyClientState`
    pub client_state: Vec<u8>,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryClientStatesResponse {
    pub client_states: Vec<Vec<u8>>,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryConsensusStateResponse {
    pub consensus_state: Vec<u8>,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryConnectionResponse {
    /// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
    pub connection: Vec<u8>,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryChannelResponse {
    /// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
    pub channel: Vec<u8>,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryChannelsResponse {
    pub channels: Vec<IdentifiedChannel>,
    pub height: u64,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryConnectionsResponse {
    pub connections: Vec<IdentifiedConnection>,
    pub height: u64,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryNextSequenceReceiveResponse {
    pub sequence: u64,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketCommitmentResponse {
    pub commitment: Vec<u8>,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct PacketState {
    pub port_id: Vec<u8>,
    pub channel_id: Vec<u8>,
    pub sequence: u64,
    pub data: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketCommitmentsResponse {
    pub commitments: Vec<PacketState>,
    pub height: u64,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketAcknowledgementResponse {
    pub ack: Vec<u8>,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketAcknowledgementsResponse {
    pub acks: Vec<PacketState>,
    pub height: u64,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketReceiptResponse {
    pub receipt: bool,
    pub height: u64,
    pub trie_key: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryDenomTraceResponse {
    pub trace: Vec<u8>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryDenomTracesResponse {
    pub trace: Vec<Vec<u8>>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct ConnectionHandshake {
    pub client_state: Vec<u8>,
    pub trie_keys: Vec<Vec<u8>>,
    pub height: u64,
}

