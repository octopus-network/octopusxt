use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use anyhow::Result;
use beefy_merkle_tree::Hash;
use codec::Decode;
use core::str::FromStr;
use ibc::core::ics04_channel::events::WriteAcknowledgement;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::core::ics24_host::Path;
use ibc::core::{
    ics04_channel::packet::{Packet, Sequence},
    ics24_host::identifier::{ChannelId, PortId},
};
use ibc_proto::google::protobuf::Any;
use jsonrpsee::rpc_params;
use sp_core::{storage::StorageKey, H256};
use sp_keyring::AccountKeyring;
use subxt::storage::StorageClient;
use subxt::{
    rpc::ClientT,
    storage::{StorageEntry, StorageKeyPrefix},
    BlockNumber, Client, PairSigner, SignedCommitment,
};
use tendermint_proto::Protobuf;

pub mod channel;
pub mod client;
pub mod connection;
pub mod events;

pub use channel::*;
pub use client::*;
pub use connection::*;
pub use events::*;

/// Subscribe beefy justifications
pub async fn subscribe_beefy(
    client: Client<MyConfig>,
) -> Result<SignedCommitment, Box<dyn std::error::Error>> {
    tracing::info!("In call_ibc: [subscribe_beefy_justifications]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut sub = api.client.rpc().subscribe_beefy_justifications().await?;

    let raw = sub.next().await.unwrap().unwrap();

    Ok(raw)
}

/// get latest height used by subscribe_blocks
pub async fn get_latest_height(client: Client<MyConfig>) -> Result<u64> {
    tracing::info!("In call_ibc: [get_latest_height]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut blocks = api.client.rpc().subscribe_finalized_blocks().await?;

    let height = match blocks.next().await {
        Some(Ok(header)) => header.number as u64,
        Some(Err(_)) => 0,
        None => 0,
    };

    Ok(height)
}

/// get send packet event by port_id, channel_id and sequence
/// (port_id, channel_id, sequence), packet)
pub async fn get_send_packet_event(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<MyConfig>,
) -> Result<Packet> {
    tracing::info!("in call_ibc: [get_send_packet_event]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .send_packet_event(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            &u64::from(*sequence),
            Some(block_hash),
        )
        .await?;

    println!("get_send_packet_event packet vec<u8> = {:?}", data);

    if data.is_empty() {
        println!("get_send_packet_event packet vec<u8> is empty!");
        return Err(anyhow::anyhow!(
            "get_send_packet_event is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id,
            channel_id,
            sequence
        ));
    }

    // serde to packet, just is decode to packet
    let packet = Packet::decode_vec(&mut &*data).unwrap();
    println!("get_send_packet_event packet is = {:?}", packet);

    Ok(packet)
}

/// (port_id, channel_id, sequence), ackHash)
pub async fn get_write_ack_packet_event(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<MyConfig>,
) -> Result<WriteAcknowledgement> {
    tracing::info!("in call_ibc: [get_write_ack_packet_event] --> port_id = {}, channel_id = {}, sequence = {}",
        port_id, channel_id, sequence);

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .write_ack_packet_event(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            &u64::from(*sequence),
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        eprintln!("get write acknowledgement is empty!");
        return Err(anyhow::anyhow!(
            "get_write_ack_packet_event is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        ));
    }

    // serde to packet, just is decode to packet

    let write_ack = WriteAcknowledgement::decode_vec(&mut &*data).unwrap();

    Ok(write_ack)
}

/// ibc protocol core function, ics26 deliver function
/// this function will dispatch msg to process
/// return block_hash, extrinsic_hash, and event
pub async fn deliver(msg: Vec<Any>, client: Client<MyConfig>) -> Result<H256> {
    tracing::info!("in call_ibc: [deliver]");

    let msg: Vec<ibc_node::runtime_types::pallet_ibc::Any> = msg
        .into_iter()
        .map(|value| ibc_node::runtime_types::pallet_ibc::Any {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        })
        .collect();

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let result = api
        .tx()
        .ibc()
        .deliver(msg)?
        .sign_and_submit_default(&signer)
        .await?;

    Ok(result)
}

// process ibc transfer
pub async fn raw_transfer(msg: Vec<Any>, client: Client<MyConfig>) -> Result<H256> {
    tracing::info!("in call_ibc: [deliver]");

    let msg: Vec<ibc_node::runtime_types::pallet_ibc::Any> = msg
        .into_iter()
        .map(|value| ibc_node::runtime_types::pallet_ibc::Any {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        })
        .collect();

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let result = api
        .tx()
        .ibc()
        .raw_transfer(msg)?
        .sign_and_submit_default(&signer)
        .await?;

    Ok(result)
}

pub async fn delete_send_packet_event(client: Client<MyConfig>) -> Result<H256> {
    tracing::info!("in call_ibc: [delete_send_packet_event]");

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let result = api
        .tx()
        .ibc()
        .delete_send_packet_event()?
        .sign_and_submit_default(&signer)
        .await?;

    Ok(result)
}

pub async fn delete_write_packet_event(client: Client<MyConfig>) -> Result<H256> {
    tracing::info!("in call_ibc: [delete_write_packet_event]");

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let result = api
        .tx()
        .ibc()
        .delete_ack_packet_event()?
        .sign_and_submit_default(&signer)
        .await?;

    Ok(result)
}

/// # get_mmr_leaf_and_mmr_proof
///
/// This get_mmr_leaf_and_mmr_proof api generate form generateProof api
/// generateProof(leafIndex: u64, at?: BlockHash): MmrLeafProof
/// interface: api.rpc.mmr.generateProof
/// json rpc: mmr_generateProof
/// summary: Generate MMR proof for given leaf index.
///
/// Return value a tuple (mmr_leaf, mmr_proof)
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
    header: sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>,
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

pub fn get_storage_key<F: StorageEntry>(store: &F) -> StorageKey {
    let prefix = StorageKeyPrefix::new::<F>();
    store.key().final_key(prefix)
}

pub async fn storage_iter<T, H: StorageEntry>(
    client: Client<MyConfig>,
    result: &mut Vec<T>,
    client_id: ClientId,
    mut callback: Box<dyn FnMut(Path, &mut Vec<T>, <H as StorageEntry>::Value, ClientId)>,
) -> Result<()> {
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

    // Read Store
    let mut iter = storage.iter::<H>(Some(block_hash)).await?;

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)?;
        let client_state_path = String::from_utf8(raw_key)?;
        // decode key
        let path =
            Path::from_str(&client_state_path).map_err(|_| anyhow::anyhow!("decode path error"))?;

        let _ret = callback(path, result, value, client_id.clone());
    }

    Ok(())
}
