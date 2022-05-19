use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum TechnicalCommittee {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl TechnicalCommittee {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TechnicalCommittee::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            TechnicalCommittee::Storage(storage) => {
                let ret = storage.run().await?;
            }
            TechnicalCommittee::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
