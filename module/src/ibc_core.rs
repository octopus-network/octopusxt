use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::packet::{Packet, Sequence},
    ics24_host::identifier::{ChannelId, PortId},
};
use std::future::Future;
use subxt::{rpc::ClientT, BlockNumber, Client, SignedCommitment};

use anyhow::Result;
use async_trait::async_trait;
use codec::Output;
use ibc::core::ics02_client::client_consensus::AnyConsensusState;
use ibc::core::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::core::ics04_channel::packet::Receipt;
use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc::Height as ICSHeight;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::PacketState;
use jsonrpsee::rpc_params;
use sp_core::H256;

pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics26_router;

pub use crate::events::*;
pub use ics02_client::*;
pub use ics03_connection::*;
pub use ics04_channel::*;

#[async_trait]
pub trait ClientRpc {
    async fn get_client_state(&self, client_id: ClientId) -> Result<AnyClientState>;

    async fn get_client_consensus(
        &self,
        client_id: ClientId,
        height: ICSHeight,
    ) -> Result<AnyConsensusState>;

    async fn get_consensus_state_with_height(
        &self,
        client_id: ClientId,
    ) -> Result<Vec<(ICSHeight, AnyConsensusState)>>;

    async fn get_clients(&self) -> Result<Vec<IdentifiedAnyClientState>>;

    async fn get_client_connections(&self, client_id: ClientId) -> Result<Vec<ConnectionId>>;
}

#[async_trait]
pub trait ChannelRpc {
    /// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    ///
    /// # Usage example
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_channels;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_channels(client).await?;
    /// ```
    async fn get_channels(&self) -> Result<Vec<IdentifiedChannelEnd>>;

    /// get channelEnd according by port_identifier, channel_identifier and read Channles StorageMaps
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
    /// use octopusxt::MyConfig;
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_channel_end;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let result = get_channel_end(&port_id, &channel_id, client).await?;
    /// ```
    async fn get_channel_end(&self, port_id: PortId, channel_id: ChannelId) -> Result<ChannelEnd>;

    /// get packet receipt by port_id, channel_id and sequence
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::MyConfig;
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_packet_receipt;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_packet_receipt(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_packet_receipt(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Receipt>;

    /// get packet receipt by port_id, channel_id and sequence
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::MyConfig;
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_packet_receipt_vec;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_packet_receipt_vec(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_packet_receipt_vec(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>>;

    /// get  unreceipt packet
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::MyConfig;
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_unreceipt_packet;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let port_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = vec![Sequence::from(12),Sequence::from(13)];
    /// let result = get_unreceipt_packet(&port_id, &channel_id, sequence, client).await?;
    /// ```
    async fn get_unreceipt_packet(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequences: Vec<Sequence>,
    ) -> Result<Vec<u64>>;

    /// get get_commitment_packet_state
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use octopusxt::MyConfig;
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_commitment_packet_state;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_commitment_packet_state(client).await?;
    /// ```
    async fn get_commitment_packet_state(&self) -> Result<Vec<PacketState>>;

    /// get packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::MyConfig;
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_packet_commitment;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let port_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(23);
    /// let result = get_packet_commitment(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_packet_commitment(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>>;

    /// get packet acknowledgement by port_id, channel_id and sequence to verify if the packet has been received by the target chain
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::MyConfig;
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_packet_ack;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let port_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(12);
    /// let result = get_packet_ack(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_packet_ack(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>>;

    /// get packet receipt by port_id, channel_id and sequence
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_next_sequence_recv;
    /// use octopusxt::MyConfig;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let result = get_next_sequence_recv(&prot_id, &channel_id, client).await?;
    /// ```
    async fn get_next_sequence_recv(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Vec<u8>>;

    /// get get_commitment_packet_state
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_acknowledge_packet_state;
    /// use octopusxt::MyConfig;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_acknowledge_packet_state(client).await?;
    /// ```
    async fn get_acknowledge_packet_state(&self) -> Result<Vec<PacketState>>;
}

#[async_trait]
pub trait ConnectionRpc {}

#[async_trait]
pub trait PacketRpc {
    /// get send packet event by port_id, channel_id and sequence
    /// (port_id, channel_id, sequence), packet)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use subxt::MyConfig;
    /// use octopusxt::get_send_packet_event;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id =PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_send_packet_event(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_send_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Packet>;

    /// (port_id, channel_id, sequence), ackHash)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use subxt::MyConfig;
    /// use octopusxt::get_write_ack_packet_event;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_write_ack_packet_event(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_write_ack_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait Router {
    async fn deliver(&self, msg: Vec<Any>) -> Result<H256>;
}

#[async_trait]
pub trait OctopusxtRpc: ClientRpc + ChannelRpc + ConnectionRpc + PacketRpc {}

#[derive(Debug)]
pub struct OctopusxtClient {
    client: Client<MyConfig>,
}

#[async_trait]
impl ConnectionRpc for OctopusxtClient {}

#[async_trait]
impl OctopusxtRpc for OctopusxtClient {}

impl OctopusxtClient {
    pub fn new(client: Client<MyConfig>) -> Self {
        Self { client }
    }

    /// Subscribe beefy justifiactions
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::subscribe_beefy;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = subscribe_beefy(client).await?;
    /// ```
    pub async fn subscribe_beefy(&self) -> Result<SignedCommitment, Box<dyn std::error::Error>> {
        tracing::info!("In call_ibc: [subscribe_beefy_justifications]");

        let api = self.to_runtime_api();

        let mut sub = api.client.rpc().subscribe_beefy_justifications().await?;

        let raw = sub.next().await.unwrap().unwrap();

        Ok(raw)
    }

    /// get latest height used by subscribe_blocks
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_latest_height;
    ///
    /// let api = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_latest_height(api).await?;
    /// ```
    ///
    pub async fn get_latest_height(&self) -> Result<u64> {
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
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::{ClientBuilder, BlockNumber};
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_mmr_leaf_and_mmr_proof;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let block_number = Some(BlockNumber::from(12));
    /// let block_hash = None;
    /// let result = get_mmr_leaf_and_mmr_proof(block_number, block_hash, client).await?;
    /// ```
    pub async fn get_mmr_leaf_and_mmr_proof(
        &self,
        block_number: Option<BlockNumber>,
        block_hash: Option<H256>,
    ) -> Result<(String, Vec<u8>, Vec<u8>)> {
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
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_header_by_block_hash;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let block_hash = None;
    /// let result = get_header_by_block_hash(block_hash, client).await?;
    /// ```
    ///
    pub async fn get_header_by_block_hash(
        &self,
        block_hash: Option<H256>,
    ) -> Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
        let api = self.to_runtime_api();

        let header = api.client.rpc().header(block_hash).await?.unwrap();

        let header = crate::utils::convert_substrate_header_to_ibc_header(header);

        Ok(header.into())
    }

    /// get header by block number
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::{ClientBuilder, BlockNumber};
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_header_by_block_number;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let block_number = Some(BlockNumber::from(2));
    /// let result = get_header_by_block_number(block_number, client).await?;
    /// ```
    ///
    pub async fn get_header_by_block_number(
        &self,
        block_number: Option<BlockNumber>,
    ) -> Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
        let api = self.to_runtime_api();

        let block_hash = api.client.rpc().block_hash(block_number).await?;

        let header = api.client.rpc().header(block_hash).await?.unwrap();

        let header = crate::utils::convert_substrate_header_to_ibc_header(header);

        Ok(header.into())
    }
}
