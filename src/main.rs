use octopusxt::ibc_node;
use sp_core::storage::StorageKey;
use subxt::{ClientBuilder, StorageEntryKey};
use octopusxt::call_ibc::get_storage_key;
use subxt::storage::StorageEntry;

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


    let storage_entry = ibc_node::ibc::storage::ClientStates("10-grandpa-0".as_bytes().to_vec()).key();
    // let storage_key = get_storage_key(&storage_entry);
    // println!("key = {:?}", storage_key);

    let map_key = match storage_entry {
        StorageEntryKey::Map(map_key) => map_key,
        StorageEntryKey::Plain => todo!()
    };
    let storage_key = map_key.iter().map( |val| StorageKey(val.value.clone())).collect::<Vec<StorageKey>>();
    println!("client storage_key = {:?}", storage_key);


    // let params = &[to_json_value(storage_key).unwrap(), to_json_value(block_hash).unwrap()];
    //
    // #[derive(Debug, PartialEq, Serialize, Deserialize)]
    // #[serde(rename_all = "camelCase")]
    // pub struct ReadProof_ {
    //     pub at: String,
    //     pub proof: Vec<Bytes>,
    // }
    // let storage_proof: ReadProof_ = api
    //     .rpc()
    //     .client
    //     .request("state_getReadProof", params)
    //     .await.unwrap();
    //
    // println!(
    //     "In Substrate: [generate_storage_proof] >> storage_proof : {:?}",
    //     storage_proof
    // );


    Ok(())
}


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
