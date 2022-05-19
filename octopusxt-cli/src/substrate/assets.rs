use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Assets {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Assets {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Assets::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Assets::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Assets::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
