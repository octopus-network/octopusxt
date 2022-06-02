// mod codegen;
#![allow(clippy::too_many_arguments)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use codec::{Decode, Encode};
use core::str::FromStr;
use prost_types::Any;
use subxt::{Config, DefaultConfig};

pub mod ibc_rpc;
pub mod update_client_state;
pub use ibc_rpc::*;
pub use update_client_state::*;

#[derive(Debug, Encode, Decode)]
pub enum ClientType {
    Tendermint,
    Grandpa,
}

impl ClientType {
    pub fn to_ibc_client_type(self) -> ibc::core::ics02_client::client_type::ClientType {
        match self {
            ClientType::Tendermint => ibc::core::ics02_client::client_type::ClientType::Tendermint,
            ClientType::Grandpa => ibc::core::ics02_client::client_type::ClientType::Grandpa,
        }
    }
}

#[derive(Encode, Decode)]
pub struct MessageQueueChain(pub subxt::sp_core::H256);

#[subxt::subxt(runtime_metadata_path = "metadata_file/metadata.scale")]
pub mod ibc_node {
    #[subxt(substitute_type = "pallet_ibc::event::primitive::ClientType")]
    use crate::ClientType;

    #[subxt(substitute_type = "cumulus_pallet_parachain_system::MessageQueueChain")]
    use crate::MessageQueueChain;

    #[subxt(substitute_type = "beefy_primitives::crypto::Public")]
    use beefy_primitives::crypto::Public;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MyConfig;
impl Config for MyConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <DefaultConfig as Config>::BlockNumber;
    type Hash = <DefaultConfig as Config>::Hash;
    type Hashing = <DefaultConfig as Config>::Hashing;
    type AccountId = <DefaultConfig as Config>::AccountId;
    type Address = <DefaultConfig as Config>::Address;
    type Header = <DefaultConfig as Config>::Header;
    type Signature = <DefaultConfig as Config>::Signature;
    type Extrinsic = <DefaultConfig as Config>::Extrinsic;
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Height {
    pub fn to_ibc_height(self) -> ibc::Height {
        ibc::Height {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Packet {
    pub fn to_ibc_packet(self) -> ibc::core::ics04_channel::packet::Packet {
        ibc::core::ics04_channel::packet::Packet {
            sequence: self.sequence.to_ibc_sequence(),
            source_port: self.source_port.to_ibc_port_id(),
            source_channel: self.source_channel.to_ibc_channel_id(),
            destination_port: self.destination_port.to_ibc_port_id(),
            destination_channel: self.destination_channel.to_ibc_channel_id(),
            data: self.data,
            timeout_height: self.timeout_height.to_ibc_height(),
            timeout_timestamp: self.timeout_timestamp.to_ibc_timestamp(),
        }
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ConnectionId {
    pub fn to_ibc_connection_id(self) -> ibc::core::ics24_host::identifier::ConnectionId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::core::ics24_host::identifier::ConnectionId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ChannelId {
    pub fn to_ibc_channel_id(self) -> ibc::core::ics24_host::identifier::ChannelId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::core::ics24_host::identifier::ChannelId::from_str(&value).unwrap()
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::PortId {
    pub fn to_ibc_port_id(self) -> ibc::core::ics24_host::identifier::PortId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::core::ics24_host::identifier::PortId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ClientId {
    pub fn to_ibc_client_id(self) -> ibc::core::ics24_host::identifier::ClientId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::core::ics24_host::identifier::ClientId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Sequence {
    pub fn to_ibc_sequence(self) -> ibc::core::ics04_channel::packet::Sequence {
        ibc::core::ics04_channel::packet::Sequence::from(self.0)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Timestamp {
    pub fn to_ibc_timestamp(self) -> ibc::timestamp::Timestamp {
        let value = String::from_utf8(self.time).unwrap();
        ibc::timestamp::Timestamp::from_str(&value).unwrap()
    }
}

impl From<Any> for ibc_node::runtime_types::pallet_ibc::Any {
    fn from(value: Any) -> Self {
        ibc_node::runtime_types::pallet_ibc::Any {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        }
    }
}

impl Copy for ibc_node::runtime_types::pallet_ibc::event::primitive::Height {}

impl Clone for ibc_node::runtime_types::pallet_ibc::event::primitive::Height {
    fn clone(&self) -> Self {
        Self {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}
