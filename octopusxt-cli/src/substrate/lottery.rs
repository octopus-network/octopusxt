use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Lottery {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Lottery {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Lottery::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Lottery::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Lottery::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
