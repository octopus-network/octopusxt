use octopusxt::ibc_node;
use subxt::BlockNumber;
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

    // Get a block hash, returns hash of latest block by default
    let block_hash = api.client.rpc().block_hash(None).await?;

    println!("the latest block hash : {:?}", block_hash);

    // Get the block number 56 hash
    let block_hash = api
        .client
        .rpc()
        .block_hash(Some(BlockNumber::from(56)))
        .await?;

    println!("block number 56 is hash : {:?}", block_hash);

    Ok(())
}
