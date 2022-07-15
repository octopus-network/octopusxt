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

pub const REVISION_NUMBER: u64 = 8888;

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a substrate node template.
pub type SubstrateNodeTemplateExtrinsicParams<T> =
    subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;

/// A builder which leads to [`SubstrateNodeTemplateExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type SubstrateNodeTemplateExtrinsicParamsBuilder<T> =
    subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;

// use subxt::SubstrateExtrinsicParams;
#[derive(Debug, Encode, Decode)]
pub enum ClientType {
    Tendermint,
    Grandpa,
}

impl From<ClientType> for ibc::core::ics02_client::client_type::ClientType {
    fn from(client_type: ClientType) -> Self {
        match client_type {
            ClientType::Tendermint => Self::Tendermint,
            ClientType::Grandpa => Self::Grandpa,
        }
    }
}

#[derive(Encode, Decode)]
pub struct MessageQueueChain(pub sp_core::H256);

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

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::Height> for ibc::Height {
    fn from(height: ibc_node::runtime_types::pallet_ibc::event::primitive::Height) -> Self {
        ibc::Height::new(REVISION_NUMBER, height.revision_height).expect("REVISION_NUMBER is 8888")
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::Packet>
    for ibc::core::ics04_channel::packet::Packet
{
    fn from(packet: ibc_node::runtime_types::pallet_ibc::event::primitive::Packet) -> Self {
        Self {
            sequence: packet.sequence.into(),
            source_port: packet.source_port.into(),
            source_channel: packet.source_channel.into(),
            destination_port: packet.destination_port.into(),
            destination_channel: packet.destination_channel.into(),
            data: packet.data,
            timeout_height: ibc::core::ics04_channel::timeout::TimeoutHeight::At(
                packet.timeout_height.into(),
            ),
            timeout_timestamp: packet.timeout_timestamp.into(),
        }
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::ConnectionId>
    for ibc::core::ics24_host::identifier::ConnectionId
{
    fn from(
        connection_id: ibc_node::runtime_types::pallet_ibc::event::primitive::ConnectionId,
    ) -> Self {
        let value = String::from_utf8(connection_id.0).unwrap();
        Self(value)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::ChannelId>
    for ibc::core::ics24_host::identifier::ChannelId
{
    fn from(channel_id: ibc_node::runtime_types::pallet_ibc::event::primitive::ChannelId) -> Self {
        let value = String::from_utf8(channel_id.0).unwrap();
        Self::from_str(&value).unwrap()
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::PortId>
    for ibc::core::ics24_host::identifier::PortId
{
    fn from(port_id: ibc_node::runtime_types::pallet_ibc::event::primitive::PortId) -> Self {
        let value = String::from_utf8(port_id.0).unwrap();
        Self(value)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::ClientId>
    for ibc::core::ics24_host::identifier::ClientId
{
    fn from(client_id: ibc_node::runtime_types::pallet_ibc::event::primitive::ClientId) -> Self {
        let value = String::from_utf8(client_id.0).unwrap();
        Self(value)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::Sequence>
    for ibc::core::ics04_channel::packet::Sequence
{
    fn from(sequence: ibc_node::runtime_types::pallet_ibc::event::primitive::Sequence) -> Self {
        Self::from(sequence.0)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::event::primitive::Timestamp>
    for ibc::timestamp::Timestamp
{
    fn from(time_stamp: ibc_node::runtime_types::pallet_ibc::event::primitive::Timestamp) -> Self {
        let value = String::from_utf8(time_stamp.time).unwrap();
        Self::from_str(&value).unwrap()
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
