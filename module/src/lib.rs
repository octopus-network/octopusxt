// mod codegen;
#![allow(clippy::too_many_arguments)]

use subxt::{Config, DefaultConfig};

pub mod events;
pub mod ibc_core;
pub mod ics20_transfer;
pub mod primitive;
pub mod update_client_state;
pub mod utils;

pub use ibc_core::*;
pub use update_client_state::*;

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a substrate node template.
pub type SubstrateNodeTemplateExtrinsicParams<T> =
    subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;

/// A builder which leads to [`SubstrateNodeTemplateExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type SubstrateNodeTemplateExtrinsicParamsBuilder<T> =
    subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;

#[subxt::subxt(runtime_metadata_path = "metadata_file/metadata.scale")]
pub mod ibc_node {
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

// /// Query packet data
// #[method(name = "ibc_queryPackets")]
// fn query_packets(
//     &self,
//     channel_id: String,
//     port_id: String,
//     seqs: Vec<u64>,
// ) -> Result<Vec<Packet>>;
// /// Generate proof for given key
// #[method(name = "ibc_queryProof")]
// fn query_proof(&self, height: u32, keys: Vec<Vec<u8>>) -> Result<Proof>;
//
// /// Query latest height
// #[method(name = "ibc_queryLatestHeight")]
// fn query_latest_height(&self) -> Result<BlockNumber>;
//
// /// Query balance of an address on chain, addr should be a valid hexadecimal or SS58 string,
// /// representing the account id.
// #[method(name = "ibc_queryBalanceWithAddress")]
// fn query_balance_with_address(&self, addr: String) -> Result<Coin>;
//
// /// Query a client state
// #[method(name = "ibc_queryClientState")]
// fn query_client_state(
//     &self,
//     height: u32,
//     src_client_id: String,
// ) -> Result<QueryClientStateResponse>;
//
// /// Query localchain consensus state
// #[method(name = "ibc_queryConsensusState")]
// fn query_consensus_state(&self, height: u32) -> Result<QueryConsensusStateResponse>;
//
// /// Query client consensus state
// #[method(name = "ibc_queryClientConsensusState")]
// fn query_client_consensus_state(
//     &self,
//     client_id: String,
//     revision_height: u64,
//     revision_number: u64,
//     latest_consensus_state: bool,
// ) -> Result<QueryConsensusStateResponse>;
//
// /// Query upgraded client state
// #[method(name = "ibc_queryUpgradedClient")]
// fn query_upgraded_client(&self, height: u32) -> Result<QueryClientStateResponse>;
//
// /// Query upgraded consensus state for client
// #[method(name = "ibc_queryUpgradedConnectionState")]
// fn query_upgraded_cons_state(&self, height: u32) -> Result<QueryConsensusStateResponse>;
//
// /// Query all client states
// #[method(name = "ibc_queryClients")]
// fn query_clients(&self) -> Result<Vec<IdentifiedClientState>>;
//
// /// Query a connection state
// #[method(name = "ibc_queryConnection")]
// fn query_connection(
//     &self,
//     height: u32,
//     connection_id: String,
// ) -> Result<QueryConnectionResponse>;
//
// /// Query all connection states
// #[method(name = "ibc_queryConnections")]
// fn query_connections(&self) -> Result<QueryConnectionsResponse>;
//
// /// Query all connection states for associated client
// #[method(name = "ibc_queryConnectionUsingClient")]
// fn query_connection_using_client(
//     &self,
//     height: u32,
//     client_id: String,
// ) -> Result<Vec<IdentifiedConnection>>;
//
// /// Generate proof for connection handshake
// #[method(name = "ibc_generateConnectionHandshakeProof")]
// fn generate_conn_handshake_proof(
//     &self,
//     height: u32,
//     client_id: String,
//     conn_id: String,
// ) -> Result<ConnHandshakeProof>;
//
// /// Query a channel state
// #[method(name = "ibc_queryChannel")]
// fn query_channel(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
// ) -> Result<QueryChannelResponse>;
//
// /// Query client state for channel and port id
// #[method(name = "ibc_queryChannelClient")]
// fn query_channel_client(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
// ) -> Result<IdentifiedClientState>;
//
// /// Query all channel states for associated connection
// #[method(name = "ibc_queryConnectionChannels")]
// fn query_connection_channels(
//     &self,
//     height: u32,
//     connection_id: String,
// ) -> Result<QueryChannelsResponse>;
//
// /// Query all channel states
// #[method(name = "ibc_queryChannels")]
// fn query_channels(&self) -> Result<QueryChannelsResponse>;
//
// /// Query packet commitments
// #[method(name = "ibc_queryPacketCommitments")]
// fn query_packet_commitments(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
// ) -> Result<QueryPacketCommitmentsResponse>;
//
// /// Query packet acknowledgements
// #[method(name = "ibc_queryPacketAcknowledgements")]
// fn query_packet_acknowledgements(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
// ) -> Result<QueryPacketAcknowledgementsResponse>;
//
// /// Query unreceived packet commitments
// #[method(name = "ibc_queryUnreceivedPackets")]
// fn query_unreceived_packets(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
//     seqs: Vec<u64>,
// ) -> Result<Vec<u64>>;
//
// /// Query the unreceived acknowledgements
// #[method(name = "ibc_queryUnreceivedAcknowledgement")]
// fn query_unreceived_acknowledgements(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
//     seqs: Vec<u64>,
// ) -> Result<Vec<u64>>;
//
// /// Query next sequence to be received on channel
// #[method(name = "ibc_queryNextSeqRecv")]
// fn query_next_seq_recv(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
// ) -> Result<QueryNextSequenceReceiveResponse>;
//
// /// Query packet commitment
// #[method(name = "ibc_queryPacketCommitment")]
// fn query_packet_commitment(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
//     seq: u64,
// ) -> Result<QueryPacketCommitmentResponse>;
//
// /// Query packet acknowledgement
// #[method(name = "ibc_queryPacketAcknowledgement")]
// fn query_packet_acknowledgement(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
//     seq: u64,
// ) -> Result<QueryPacketAcknowledgementResponse>;
//
// /// Query packet receipt
// #[method(name = "ibc_queryPacketReceipt")]
// fn query_packet_receipt(
//     &self,
//     height: u32,
//     channel_id: String,
//     port_id: String,
//     seq: u64,
// ) -> Result<QueryPacketReceiptResponse>;
//
// /// Query the denom trace for an ibc denom
// #[method(name = "ibc_queryDenomTrace")]
// fn query_denom_trace(&self, denom: String) -> Result<QueryDenomTraceResponse>;
//
// /// Query the denom traces for ibc denoms matching offset
// #[method(name = "ibc_queryDenomTraces")]
// fn query_denom_traces(
//     &self,
//     offset: String,
//     limit: u64,
//     height: u32,
// ) -> Result<QueryDenomTracesResponse>;
//
// /// Query newly created clients in block
// #[method(name = "ibc_queryNewlyCreatedClients")]
// fn query_newly_created_clients(&self, block_hash: Hash) -> Result<Vec<IdentifiedClientState>>;
