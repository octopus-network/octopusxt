use std::str::FromStr;

use codec::Encode;
use octopusxt::ibc_node;
use octopusxt::ibc_rpc::get_storage_key;
use subxt::ClientBuilder;

/// this is example for usage query block hash
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    println!("😂😂😂😂😂😂😂😂😂😂😂😂😂 Example1: Read proof RPC 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    // example 1: read_proof rpc
    let block_weight = ibc_node::system::storage::BlockWeight;
    let block_weight_key = get_storage_key(&block_weight);

    let block_weight_proof = api
        .client
        .rpc()
        .read_proof(vec![block_weight_key], None)
        .await?;

    println!("Get system blockWeight proof = {:?}", block_weight_proof);

    println!("😂😂😂😂😂😂😂😂😂😂😂😂😂Example2: Get Storage RPC 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");

    // example 2: storage rpc

    // getStorage(key: StorageKey, at?: BlockHash): StorageData
    // interface: api.rpc.state.getStorage
    // jsonrpc: state_getStorage
    // summary: Retrieves the storage for a key
    //
    // this exmaple use getStorage api, can get storage_data by a StorageKey
    //
    let sudo_key = ibc_node::sudo::storage::Key;
    let sudo_key = get_storage_key(&sudo_key);

    // get sudo key storage data
    // Fetch a storage Key: storage function
    //
    let storage_data = api.client.rpc().storage(&sudo_key, None).await?.unwrap();

    // encode sudo key storage data to hex format
    let key_data = hex::encode(storage_data.0);
    println!("sudo key data hex format = {}", key_data);

    // serde to sudo key to H256
    let account = sp_core::H256::from_str(&key_data).unwrap();
    println!("sudo key h256 format = {:?}", account);

    // serde to sudo key to AccountId32
    let account_id = sp_runtime::AccountId32::from_str(&key_data).unwrap();

    println!("sudo key AccountId32 format = {}", account_id);

    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3: storage_keys_paged 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");

    // getKeysPaged(key: StorageKey, count: u32, startKey?: StorageKey, at?: BlockHash): Vec<StorageKey>
    // interface: api.rpc.state.getKeysPaged
    // jsonrpc: state_getKeysPaged
    // summary: Returns the keys with prefix with pagination support.

    // Returns the keys with prefix with pagination support.
    // Up to `count` keys will be returned.
    // If `start_key` is passed, return next keys in storage in lexicographic order.
    // pub async fn storage_keys_paged(
    //     &self,
    //     prefix: Option<StorageKeyPrefix>,
    //     count: u32,
    //     start_key: Option<StorageKey>,
    //     hash: Option<T::Hash>,
    // ) -> Result<Vec<StorageKey>, Error>;

    // todo

    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3: query_storage RPC 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");

    let sudo_key = ibc_node::sudo::storage::Key;
    let sudo_key = get_storage_key(&sudo_key);

    // get sudo key storage data
    // Fetch a storage Key: storage function
    //

    let from_block_hash = sp_core::H256::from_str(
        "0x856f7e105cafe285d361c9aae72f7763bfb222a7d65b93c8833899b352a37e8b",
    )
    .unwrap();
    println!("from_block_hash = {}", from_block_hash);

    let storage_data = api
        .client
        .rpc()
        .query_storage(vec![sudo_key], from_block_hash, None)
        .await?;

    println!("query_storage data = {:?}", storage_data);

    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
    println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3:  😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");

    // state_queryStorageAt
    // chain_getBlockHash
    // state_getMetadata
    // system_properties
    // chain_getHeader
    // chain_getBlockHash
    // chain_getFinalizedHead
    // chain_getBlock
    // state_getReadProof
    // state_getRuntimeVersion
    // subscribe_events
    // subscribe_finalized_events
    // subscribe_blocks
    // subscribe_finalized_blocks
    //subscribe_beefy_justifications
    // submit_extrinsic
    // watch_extrinsic
    // submit_and_watch_extrinsic
    // insert_key
    // rotate_keys
    // has_session_keys
    // has_key
    //

    Ok(())
}
