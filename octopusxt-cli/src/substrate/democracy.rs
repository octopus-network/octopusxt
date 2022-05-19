use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Democracy {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Democracy {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Democracy::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Democracy::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Democracy::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
