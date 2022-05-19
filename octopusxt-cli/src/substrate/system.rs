use structopt::StructOpt;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum System {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl System {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            System::Extrinsics(extrinsics) => {
                println!("extrinsics");
            }
            System::Storage(storage) => {
                println!("storage");
            }
            System::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
