use sp_core::hexdisplay::HexDisplay;
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use std::str::FromStr;
use structopt::StructOpt;
use subxt::{ClientBuilder, PairSigner};


#[derive(Debug, StructOpt)]
pub enum Account {
    #[structopt(name = "substrate-account")]
    /// basic substrate account
    SubstrateAccount(SubstrateAccount),
}

impl Account {
    pub fn run(&self) {
        match self {
            Account::SubstrateAccount(value) => {
                let ret = value.run();
            }
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct SubstrateAccount {}

impl SubstrateAccount {
    pub fn run(&self) {
        println!("substrate account!");
    }
}
