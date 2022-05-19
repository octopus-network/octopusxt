use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Multisig {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Multisig {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Multisig::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Multisig::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Multisig::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
