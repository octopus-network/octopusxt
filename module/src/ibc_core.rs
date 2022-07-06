use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::packet::{Packet, Sequence},
    ics24_host::identifier::{ChannelId, PortId},
};

use subxt::{rpc::ClientT, BlockNumber, Client, SignedCommitment};

use async_trait::async_trait;

use ibc::core::ics02_client::client_consensus::{AnyConsensusState, AnyConsensusStateWithHeight};
use ibc::core::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::core::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd};
use ibc::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};

use ibc::clients::ics10_grandpa::client_state::ClientState;
use ibc::clients::ics10_grandpa::consensus_state::ConsensusState;
use ibc::clients::ics10_grandpa::header::Header;
use ibc::core::ics03_connection::version::Version;
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc::core::ics24_host::identifier::ConnectionId;
use ibc::timestamp::Timestamp;
use ibc::{Height as ICSHeight, Height};
use ibc_proto::google::protobuf::Any;

use jsonrpsee::rpc_params;
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics26_router;

pub use crate::events::*;
use crate::primitive::IdentifiedClientState;
use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::events::IbcEvent;
use ibc_proto::cosmos::base::abci::v1beta1::TxResponse;
use ibc_relayer::chain::requests::{
    IncludeProof, QueryBlockRequest, QueryHeight, QueryPacketAcknowledgementRequest,
    QueryPacketCommitmentRequest, QueryPacketReceiptRequest, QueryTxRequest,
};
use ibc_relayer::chain::requests::{
    QueryChannelClientStateRequest, QueryChannelRequest, QueryChannelsRequest,
    QueryClientConnectionsRequest, QueryClientStateRequest, QueryClientStatesRequest,
    QueryConnectionChannelsRequest, QueryConnectionRequest, QueryConnectionsRequest,
    QueryConsensusStateRequest, QueryConsensusStatesRequest, QueryHostConsensusStateRequest,
    QueryNextSequenceReceiveRequest, QueryPacketAcknowledgementsRequest,
    QueryPacketCommitmentsRequest, QueryUnreceivedAcksRequest, QueryUnreceivedPacketsRequest,
    QueryUpgradedClientStateRequest, QueryUpgradedConsensusStateRequest,
};
use ibc_relayer::chain::tracking::TrackedMsgs;
pub use ics02_client::*;
pub use ics03_connection::*;
pub use ics04_channel::*;

#[async_trait]
pub trait ClientRpc {
    type Error;

    /// Query a client state
    /// query client_state according by client_id, and read ClientStates StorageMap
    /// Performs a query to retrieve the state of the specified light client. A
    /// proof can optionally be returned along with the result.
    async fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), Self::Error>;

    /// query key-value pair (client_identifier, client_state) construct IdentifieredAnyClientstate
    /// Performs a query to retrieve the state of all clients that a chain hosts.
    async fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Self::Error>;

    /// get connection_identifier vector according by client_identifier
    /// Performs a query to retrieve the identifiers of all connections.
    async fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Self::Error>;

    /// Query local chain consensus state
    async fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Self::Error>;

    /// Performs a query to retrieve all the consensus states that the specified
    /// light client stores.
    async fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Self::Error>;

    async fn query_upgraded_client_state(
        &self,
        request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Self::Error>;

    async fn query_upgraded_consensus_state(
        &self,
        request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Self::Error>;
}

#[async_trait]
pub trait ChannelRpc {
    type Error;

    /// get channelEnd according by port_identifier, channel_identifier and read Channles StorageMaps
    /// Performs a query to retrieve the channel associated with a given channel
    /// identifier. A proof can optionally be returned along with the result.
    async fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Self::Error>;

    /// Query all channel states
    /// query key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    /// Performs a query to retrieve all the channels of a chain.
    async fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Self::Error>;

    /// Performs a query to retrieve the client state for the channel associated
    /// with a given channel identifier.
    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Self::Error>;

    /// query packet receipt by port_id, channel_id and sequence
    /// Performs a query to retrieve a given packet receipt, stored on the chain at path
    /// `path::CommitmentsPath`. A proof can optionally be returned along with the result.
    async fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Self::Error>;

    /// query packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
    /// Performs a query to retrieve a stored packet commitment hash, stored on
    /// the chain at path `path::CommitmentsPath`. A proof can optionally be
    /// returned along with the result.
    async fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Self::Error>;

    /// Performs a query to retrieve all the packet commitments hashes
    /// associated with a channel. Returns the corresponding packet sequence
    /// numbers and the height at which they were retrieved.
    async fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Self::Error>;

    /// query packet acknowledgement by port_id, channel_id and sequence to verify if the packet has been received by the target chain
    /// Performs a query to retrieve a stored packet acknowledgement hash,
    /// stored on the chain at path `path::AcksPath`. A proof can optionally be
    /// returned along with the result.
    async fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Self::Error>;

    /// Performs a query to retrieve all the packet acknowledgements associated
    /// with a channel. Returns the corresponding packet sequence numbers and
    /// the height at which they were retrieved.
    async fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Self::Error>;

    /// get packet receipt by port_id, channel_id and sequence
    /// Performs a query to retrieve `nextSequenceRecv` stored at path
    /// `path::SeqRecvsPath` as defined in ICS-4. A proof can optionally be
    /// returned along with the result.
    async fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Self::Error>;

    /// Query unreceived packet commitments
    /// Performs a query about which IBC packets in the specified list has not
    /// been received. Returns the sequence numbers of the packets that were not
    /// received.
    ///
    /// For example, given a request with the sequence numbers `[5,6,7,8]`, a
    /// response of `[7,8]` would indicate that packets 5 & 6 were received,
    /// while packets 7, 8 were not.
    async fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, Self::Error>;

    /// Query the unreceived acknowledgements
    /// Performs a query about which IBC packets in the specified list has not
    /// been acknowledged. Returns the sequence numbers of the packets that were not
    /// acknowledged.
    ///
    /// For example, given a request with the sequence numbers `[5,6,7,8]`, a
    /// response of `[7,8]` would indicate that packets 5 & 6 were acknowledged,
    /// while packets 7, 8 were not.
    async fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<Sequence>, Self::Error>;
}

#[async_trait]
pub trait ConnectionRpc {
    type Error;

    /// query connectionEnd according by connection_identifier and read Connections StorageMaps
    /// Performs a query to retrieve the connection associated with a given
    /// connection identifier. A proof can optionally be returned along with the
    /// result.
    async fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Self::Error>;

    /// Query all connection states
    /// query key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    /// Performs a query to retrieve the identifiers of all connections.
    async fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Self::Error>;

    /// # Query IdentifiedChannelEnd by connection_identifier
    /// Performs a query to retrieve all channels associated with a connection.
    async fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Self::Error>;
}

#[async_trait]
pub trait PacketRpc {
    type Error;
    /// Query send packet event by port_id, channel_id and sequence
    /// (port_id, channel_id, sequence), packet)
    async fn query_send_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
        height: QueryHeight,
    ) -> Result<Packet, Self::Error>;

    /// (port_id, channel_id, sequence), ackHash)
    async fn query_write_ack_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
        height: QueryHeight,
    ) -> Result<Vec<u8>, Self::Error>;

    /// Query packet data
    async fn query_packets(
        &self,
        channel_id: ChannelId,
        port_id: PortId,
        seqs: Vec<Sequence>,
        height: QueryHeight,
    ) -> Result<Vec<Packet>, Self::Error>;
}

#[async_trait]
pub trait Router {
    type Error;
    /// ibc protocol core function, ics26 deliver function
    /// this function will dispatch msg to process
    /// return block_hash, extrinsic_hash, and event
    async fn deliver(&self, msg: Vec<Any>) -> Result<H256, Self::Error>;
}

#[async_trait]
pub trait OctopusxtRpc: ClientRpc + ChannelRpc + ConnectionRpc + PacketRpc {}

#[derive(Debug)]
pub struct OctopusxtClient {
    client: Client<MyConfig>,
}

#[async_trait]
impl OctopusxtRpc for OctopusxtClient {}

impl OctopusxtClient {
    pub fn new(client: Client<MyConfig>) -> Self {
        Self { client }
    }

    /// Subscribe beefy justifiactions
    pub async fn subscribe_beefy(&self) -> anyhow::Result<SignedCommitment> {
        tracing::info!("In call_ibc: [subscribe_beefy_justifications]");

        let api = self.to_runtime_api();

        let mut sub = api.client.rpc().subscribe_beefy_justifications().await?;

        let raw = sub.next().await.unwrap().unwrap();

        Ok(raw)
    }

    /// get latest height used by subscribe_blocks
    pub async fn query_latest_height(&self) -> anyhow::Result<u64> {
        tracing::info!("In call_ibc: [get_latest_height]");

        let api = self.to_runtime_api();

        let mut blocks = api.client.rpc().subscribe_finalized_blocks().await?;

        let height = match blocks.next().await {
            Some(Ok(header)) => header.number as u64,
            Some(Err(_)) => 0,
            None => 0,
        };

        Ok(height)
    }

    pub async fn query_height(&self, height: QueryHeight) -> anyhow::Result<Height> {
        let h = match height {
            QueryHeight::Latest => {
                let h = self.query_latest_height().await?;
                Height::new(0, h)
            }
            QueryHeight::Specific(h) => h,
        };

        Ok(h)
    }

    /// Query latest block hash
    pub async fn query_latest_block_hash(&self) -> anyhow::Result<H256> {
        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block
            .next()
            .await
            .ok_or(anyhow::anyhow!("query_block_header error"))??;

        let block_hash: H256 = block_header.hash();

        Ok(block_hash)
    }

    pub async fn query_block_hash_by_query_height(
        &self,
        height: QueryHeight,
    ) -> anyhow::Result<H256> {
        let block_hash = match height {
            QueryHeight::Latest => {
                let block_hash = self.query_latest_block_hash().await?;
                block_hash
            }
            QueryHeight::Specific(h) => {
                let block_hash = self
                    .query_block_hash_by_block_number(Some(BlockNumber::from(h.revision_height)))
                    .await?;
                block_hash
            }
        };

        Ok(block_hash)
    }

    /// return RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>> struct
    pub fn to_runtime_api(
        &self,
    ) -> ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>> {
        self.client.clone()
            .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>()
    }

    /// # get_mmr_leaf_and_mmr_proof
    ///
    /// This get_mmr_leaf_and_mmr_proof api generate form generateProof api
    /// generateProof(leafIndex: u64, at?: BlockHash): MmrLeafProof
    /// interface: api.rpc.mmr.generateProof
    /// json_rpc: mmr_generateProof
    /// summary: Generate MMR proof for given leaf index.
    ///
    /// Return value a tuple (mmr_leaf, mmr_proof)
    pub async fn query_mmr_leaf_and_mmr_proof(
        &self,
        block_number: Option<BlockNumber>,
        block_hash: Option<H256>,
    ) -> anyhow::Result<(String, Vec<u8>, Vec<u8>)> {
        tracing::info!("in call_ibc [get_mmr_leaf_and_mmr_proof]");

        let api = self.to_runtime_api();

        let params = rpc_params![block_number, block_hash];

        let generate_proof: pallet_mmr_rpc::LeafProof<String> = api
            .client
            .rpc()
            .client
            .request("mmr_generateProof", params)
            .await?;

        Ok((
            generate_proof.block_hash,
            generate_proof.leaf.0,
            generate_proof.proof.0,
        ))
    }

    /// get header by block hash
    pub async fn query_header_by_block_hash(
        &self,
        block_hash: Option<H256>,
    ) -> anyhow::Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
        let api = self.to_runtime_api();

        let header = api.client.rpc().header(block_hash).await?.unwrap();

        let header = crate::utils::convert_substrate_header_to_ibc_header(header);

        Ok(header.into())
    }

    /// get header by block number
    pub async fn query_header_by_block_number(
        &self,
        block_number: Option<BlockNumber>,
    ) -> anyhow::Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
        let api = self.to_runtime_api();

        let block_hash = api.client.rpc().block_hash(block_number).await?;

        let header = api.client.rpc().header(block_hash).await?.unwrap();

        let header = crate::utils::convert_substrate_header_to_ibc_header(header);

        Ok(header.into())
    }

    /// query block hash by block number
    pub async fn query_block_hash_by_block_number(
        &self,
        block_number: Option<BlockNumber>,
    ) -> anyhow::Result<H256> {
        let api = self.to_runtime_api();

        let block_hash: H256 = api
            .client
            .rpc()
            .block_hash(block_number)
            .await?
            .ok_or(anyhow::anyhow!("query block hash error"))?;

        Ok(block_hash)
    }

    /// Generate proof for given key
    pub fn query_proof(&self, _height: u32, _keys: Vec<Vec<u8>>) -> anyhow::Result<Proof> {
        todo!()
    }

    /// Sends one or more transactions with `msgs` to chain and
    /// synchronously wait for it to be committed.
    fn send_messages_and_wait_commit(
        &mut self,
        _tracked_msgs: TrackedMsgs,
    ) -> anyhow::Result<Vec<IbcEvent>> {
        todo!()
    }

    // TODO(davirain) TxResponse
    /// Sends one or more transactions with `msgs` to chain.
    /// Non-blocking alternative to `send_messages_and_wait_commit` interface.
    fn send_messages_and_wait_check_tx(
        &mut self,
        _tracked_msgs: TrackedMsgs,
    ) -> anyhow::Result<Vec<TxResponse>> {
        todo!()
    }

    /// Query balance of an address on chain, addr should be a valid hexadecimal or SS58 string,
    /// representing the account id.
    /// Query the balance of the given account for the denom used to pay tx fees.
    /// If no account is given, behavior must be specified, e.g. retrieve it from configuration file.
    fn query_balance(&self, _key_name: Option<String>) -> anyhow::Result<Balance> {
        todo!()
    }

    /// Query the denomination trace given a trace hash.
    fn query_denom_trace(&self, _hash: String) -> anyhow::Result<DenomTrace> {
        todo!()
    }

    fn query_commitment_prefix(&self) -> anyhow::Result<CommitmentPrefix> {
        todo!()
    }

    fn query_compatible_versions(&self) -> anyhow::Result<Vec<Version>> {
        todo!()
    }

    /// Query the latest height and timestamp the application is at
    fn query_application_status(&self) -> anyhow::Result<ChainStatus> {
        todo!()
    }

    fn query_txs(&self, _request: QueryTxRequest) -> anyhow::Result<Vec<IbcEvent>> {
        todo!()
    }

    fn query_blocks(
        &self,
        _request: QueryBlockRequest,
    ) -> anyhow::Result<(Vec<IbcEvent>, Vec<IbcEvent>)> {
        todo!()
    }

    fn query_host_consensus_state(
        &self,
        _request: QueryHostConsensusStateRequest,
    ) -> anyhow::Result<ConsensusState> {
        todo!()
    }

    // TODO (davirian) client settings
    fn build_client_state(
        &self,
        _height: ICSHeight,
        // settings: ClientSettings,
    ) -> anyhow::Result<ClientState> {
        todo!()
    }

    // TODO(Davirain) lightblock
    fn build_consensus_state(
        &self,
        // light_block: Self::LightBlock,
    ) -> anyhow::Result<ConsensusState> {
        todo!()
    }

    // TODO(davirian) lightblock
    /// Fetch, and verify the header at `target_height`, assuming we trust the
    /// header at `trusted_height` with the given `client_state`.
    ///
    /// Returns all the supporting headers that were need to verify the target
    /// header, for use when building a `ClientUpdate` message.
    fn build_header(
        &self,
        _trusted_height: ICSHeight,
        _target_height: ICSHeight,
        _client_state: &AnyClientState,
        // light_client: &mut Self::LightClient,
    ) -> anyhow::Result<(Header, Vec<Header>)> {
        todo!()
    }
}

/// The result of the application status query.
#[derive(Clone, Debug)]
pub struct ChainStatus {
    pub height: ICSHeight,
    pub timestamp: Timestamp,
}

/// The balance for a specific denom
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Balance {
    /// The amount of coins in the account, as a string to allow for large amounts
    pub amount: String,
    /// The denomination for that coin
    pub denom: String,
}

/// The denom trace
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DenomTrace {
    /// The chain of port/channel identifiers used for tracing the source of the coin.
    pub path: String,
    /// The base denomination for that coin
    pub base_denom: String,
}

/// Proof for a set of keys
#[derive(Serialize, Deserialize)]
pub struct Proof {
    /// Trie proof
    pub proof: Vec<u8>,
    /// Height at which proof was recovered
    pub height: ibc_proto::ibc::core::client::v1::Height,
}

/// Connection handshake proof
#[derive(Serialize, Deserialize)]
pub struct ConnHandshakeProof {
    /// Protobuf encoded client state
    pub client_state: IdentifiedClientState,
    /// Trie proof for connection state, client state and consensus state
    pub proof: Vec<u8>,
    /// Proof height
    pub height: ibc_proto::ibc::core::client::v1::Height,
}
