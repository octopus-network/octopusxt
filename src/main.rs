use octopusxt::ibc_node;
use std::{str::FromStr, collections::HashMap};
use beefy_light_client::header::Digest;
use beefy_light_client::Hash;
use subxt::{ClientBuilder, EventSubscription, sp_arithmetic::traits::Signed};
use subxt::BlockNumber;
use subxt::sp_core::Public;
use octopusxt::call_ibc::{convert_substrate_header_to_ibc_header, get_block_header};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();
    let block_hash = block_header.hash();



    let api = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await?;


    let ibc = crate::ibc_node::ibc::storage::ClientStatesKeys;
    fun_name(&api, &ibc).await?;

    let ibc = crate::ibc_node::ibc::storage::ClientStatesKeys;
    let storage_key = api.storage().fetch(&ibc, None).await?;
    // println!("connection storage_key = {:?}", storage_key);
    // println!("connection storage_key = {:?}", ibc);
    // let ibc = crate::ibc_node::ibc::storage::Channels(vec![1,2,3], vec![1,2,3]);
    // let storage_key = api.storage().fetch(&ibc, None).await?;

    use serde::{Deserialize, Serialize};
    use sp_core::{storage::StorageKey, Bytes};
    use jsonrpsee::types::to_json_value;
    // let params = &[to_json_value(vec![StorageKey(vec![1,2,3])]).unwrap(), to_json_value(block_hash).unwrap()];
    let params = &[to_json_value(storage_key).unwrap(), to_json_value(block_hash).unwrap()];

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadProof_ {
        pub at: String,
        pub proof: Vec<Bytes>,
    }
    let storage_proof: ReadProof_ = api
        .rpc()
        .client
        .request("state_getReadProof", params)
        .await.unwrap();
    println!(
        "In Substrate: [generate_storage_proof] >> storage_proof : {:?}",
        storage_proof
    );


    Ok(())
}

use subxt::storage::StorageEntry;
async fn fun_name<F: StorageEntry> 
        (api: &subxt::Client<ibc_node::DefaultConfig>, ibc: &F) 
    -> Result<(), Box<dyn std::error::Error>> 
        where 
            <F as StorageEntry>::Value: std::fmt::Debug
{
    let storage_key = api.storage().fetch(ibc, None).await.unwrap().unwrap();
    println!("client storage_key = {:?}", storage_key);
    Ok(())
}
