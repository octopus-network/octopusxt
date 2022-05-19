use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Utility {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Utility {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Utility::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Utility::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Utility::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
