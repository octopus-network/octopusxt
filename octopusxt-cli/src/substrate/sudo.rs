use structopt::StructOpt;
use subxt::ClientBuilder;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Sudo {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Sudo {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Sudo::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Sudo::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Sudo::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
