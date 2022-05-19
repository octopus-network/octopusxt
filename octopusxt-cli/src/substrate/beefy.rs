use structopt::StructOpt;
use subxt::sp_core::Public;
use subxt::ClientBuilder;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Beefy {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Beefy {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Beefy::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Beefy::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Beefy::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
