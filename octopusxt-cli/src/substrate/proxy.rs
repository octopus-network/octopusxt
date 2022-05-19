use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Proxy {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Proxy {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Proxy::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Proxy::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Proxy::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
