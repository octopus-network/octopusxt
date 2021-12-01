use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, EventSubscription, PairSigner};

mod codegen;
pub use codegen::astar::*;

#[subxt::subxt(
    runtime_metadata_path = "metadata_file/metadata.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod ibc_node {}

use ibc_node::runtime_types::frame_support::PalletId;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pallet_id = PalletId([1u8; 8]);
    let _ = <PalletId as Clone>::clone(&pallet_id);
    Ok(())
}
