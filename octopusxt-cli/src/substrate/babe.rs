use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Babe {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Babe {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Babe::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Babe::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Babe::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
