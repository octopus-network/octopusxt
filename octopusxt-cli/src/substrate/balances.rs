use sp_keyring::AccountKeyring;
use std::str::FromStr;
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::PairSigner;

pub mod constants;
pub mod extrinsics;
pub mod storage;

use constants::Constants;
use extrinsics::Extrinsics;
use storage::Storage;

#[derive(Debug, StructOpt)]
pub enum Balances {
    #[structopt(name = "extrinsics")]
    Extrinsics(Extrinsics),
    #[structopt(name = "storage")]
    Storage(Storage),
    #[structopt(name = "constants")]
    Constants(Constants),
}

impl Balances {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Balances::Extrinsics(extrinsics) => {
                let ret = extrinsics.run().await?;
            }
            Balances::Storage(storage) => {
                let ret = storage.run().await?;
            }
            Balances::Constants(constants) => {
                let ret = constants.run().await?;
            }
        }
        Ok(())
    }
}
