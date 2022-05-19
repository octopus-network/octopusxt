use sp_keyring::AccountKeyring;
use std::str::FromStr;
use structopt::StructOpt;
use subxt::BlockNumber;
use subxt::{ClientBuilder, PairSigner};

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Timestamp {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Timestamp {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Timestamp::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Timestamp::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Timestamp::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
