use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::packet::{Packet, Sequence},
    ics24_host::identifier::{ChannelId, PortId},
};
use std::future::Future;
use subxt::{rpc::ClientT, BlockNumber, Client, SignedCommitment};

use anyhow::Result;
use codec::Output;
use ibc::core::ics02_client::client_consensus::AnyConsensusState;
use ibc::core::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc::Height as ICSHeight;
use ibc_proto::google::protobuf::Any;
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

pub trait ClientRpc {
    fn get_client_state(
        &self,
        client_id: ClientId,
    ) -> Box<dyn Future<Output = Result<AnyClientState>>>;

    fn get_client_consensus(
        &self,
        client_id: ClientId,
        height: ICSHeight,
    ) -> Box<dyn Future<Output = Result<AnyConsensusState>>>;

    fn get_consensus_state_with_height(
        &self,
        client_id: ClientId,
    ) -> Box<dyn Future<Output = Result<Vec<(ICSHeight, AnyConsensusState)>>>>;

    fn get_clients(&self) -> Box<dyn Future<Output = Result<Vec<IdentifiedAnyClientState>>>>;

    fn get_client_connections(
        &self,
        client_id: ClientId,
    ) -> Box<dyn Future<Output = Result<Vec<ConnectionId>>>>;
}

pub trait ChannelRpc {}

pub trait ConnectionRpc {}

pub trait PacketRpc {
    fn get_send_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Box<dyn Future<Output = Result<Packet>>>;

    fn get_write_ack_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Box<dyn Future<Output = Result<Vec<u8>>>>;
}

pub trait Router {
    fn deliver(&self, msg: Vec<Any>) -> Box<dyn Future<Output = Result<H256>>>;
}

pub trait OctopusxtRpc: ClientRpc + ChannelRpc + ConnectionRpc + PacketRpc {}

#[derive(Debug)]
pub struct OctopusxtClient {
    client: Client<MyConfig>,
}

impl ChannelRpc for OctopusxtClient {}

impl ConnectionRpc for OctopusxtClient {}

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