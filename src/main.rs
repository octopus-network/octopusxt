use sp_keyring::AccountKeyring;
use subxt::{Client, ClientBuilder, EventSubscription, PairSigner};

mod codegen;
pub use codegen::astar::*;

#[subxt::subxt(
    runtime_metadata_path = "metadata_file/metadata.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod ibc_node {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut iter = api.storage().system().account_iter(None).await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }
    Ok(())
}
