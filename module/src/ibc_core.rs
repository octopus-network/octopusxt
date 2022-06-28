use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::packet::{Packet, Sequence},
    ics24_host::identifier::{ChannelId, PortId},
};
use std::future::Future;
use subxt::{
    rpc::ClientT,
    storage::{StorageEntry, StorageKeyPrefix},
    BlockNumber, Client, SignedCommitment,
};

use anyhow::Result;
use beefy_merkle_tree::Hash;
use ibc_proto::google::protobuf::Any;
use jsonrpsee::rpc_params;
use sp_core::{storage::StorageKey, H256};

pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics26_router;

pub use crate::events::*;
pub use ics02_client::*;
pub use ics03_connection::*;
pub use ics04_channel::*;

pub trait ClientRpc {}

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

impl ClientRpc for OctopusxtClient {}

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
    /// use subxt::MyConfig;
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
///
pub async fn get_mmr_leaf_and_mmr_proof(
    block_number: Option<BlockNumber>,
    block_hash: Option<H256>,
    client: Client<MyConfig>,
) -> Result<(String, Vec<u8>, Vec<u8>)> {
    tracing::info!("in call_ibc [get_mmr_leaf_and_mmr_proof]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

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
    block_hash: Option<H256>,
    client: Client<MyConfig>,
) -> Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let header = api.client.rpc().header(block_hash).await?.unwrap();

    let header = convert_substrate_header_to_ibc_header(header);

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
    block_number: Option<BlockNumber>,
    client: Client<MyConfig>,
) -> Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let block_hash = api.client.rpc().block_hash(block_number).await?;

    let header = api.client.rpc().header(block_hash).await?.unwrap();

    let header = convert_substrate_header_to_ibc_header(header);

    Ok(header.into())
}

/// convert substrate Header to Ibc Header
fn convert_substrate_header_to_ibc_header(
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

fn convert_substrate_digest_to_beefy_light_client_digest(
    digest: sp_runtime::Digest,
) -> beefy_light_client::header::Digest {
    beefy_light_client::header::Digest {
        logs: digest
            .logs
            .into_iter()
            .map(convert_substrate_digest_item_to_beefy_light_client_digest_item)
            .collect(),
    }
}

fn convert_substrate_digest_item_to_beefy_light_client_digest_item(
    digest_item: sp_runtime::DigestItem,
) -> beefy_light_client::header::DigestItem {
    match digest_item {
        sp_runtime::DigestItem::PreRuntime(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::PreRuntime(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Consensus(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::Consensus(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Seal(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::Seal(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Other(value) => {
            beefy_light_client::header::DigestItem::Other(value)
        }
        sp_runtime::DigestItem::RuntimeEnvironmentUpdated => {
            beefy_light_client::header::DigestItem::RuntimeEnvironmentUpdated
        }
    }
}

/// # Usage example
///
/// ```rust
///  use octopusxt::ibc_node;
///  use octopusxt::get_storage_key;
///
///  let storage_entry = ibc_node::ibc::storage::ClientStates("10-grandpa-0".as_bytes().to_vec());
///  let storage_key = get_storage_key(&storage_entry);
///  println!("key = {:?}", storage_key);
/// ```
pub fn get_storage_key<F: StorageEntry>(store: &F) -> StorageKey {
    let prefix = StorageKeyPrefix::new::<F>();
    store.key().final_key(prefix)
}
