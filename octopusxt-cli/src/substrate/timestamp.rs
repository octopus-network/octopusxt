use sp_keyring::AccountKeyring;
use std::str::FromStr;
use structopt::StructOpt;
use subxt::BlockNumber;
use subxt::{ClientBuilder, PairSigner};

#[derive(Debug, StructOpt)]
pub enum TimeStamp {
    #[structopt(name = "now")]
    /// Current time for the current block.
    Now,
    #[structopt(name = "did-update")]
    ///  Did the timestamp get updated in this block?
    DidUpdate,
}

impl TimeStamp {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TimeStamp::Now => {
                println!("now");
            }
            TimeStamp::DidUpdate => {
                println!("did update");
            }
        }

        Ok(())
    }
}
