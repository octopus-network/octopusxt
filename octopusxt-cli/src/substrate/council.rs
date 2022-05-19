use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Council {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Council {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Council::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Council::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Council::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
