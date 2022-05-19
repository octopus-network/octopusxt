use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum RandomnessCollectiveFlip {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl RandomnessCollectiveFlip {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            RandomnessCollectiveFlip::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            RandomnessCollectiveFlip::Storage(storage) => {
                let ret = storage.run().await?;
            }
            RandomnessCollectiveFlip::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
