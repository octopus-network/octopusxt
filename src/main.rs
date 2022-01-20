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
    fun_name(&api, ibc).await?;

    let ibc = crate::ibc_node::ibc::storage::ClientStatesKeys;
    let storage_key = api.storage().fetch(&ibc, None).await?;
    println!("connection storage_key = {:?}", storage_key);

    Ok(())
}

use subxt::storage::StorageEntry;
async fn fun_name(api: &subxt::Client<ibc_node::DefaultConfig>, ibc: StorageEntry) 
    -> Result<(), Box<dyn std::error::Error>>
{
    let storage_key = api.storage().fetch(&ibc, None).await?;
    println!("client storage_key = {:?}", storage_key);
    Ok(())
}
