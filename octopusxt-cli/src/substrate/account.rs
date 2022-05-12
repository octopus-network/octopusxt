use octopusxt::ibc_node;
use sp_keyring::AccountKeyring;
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
        let dest: sp_runtime::AccountId32 = AccountKeyring::Bob.to_account_id().into();
        println!("😂😂😂😂😂😂😂😂😂😂😂😂 account id = {:?}", dest);

        let dest: &[u8; 32] = dest.as_ref();
        let hex_dest = hex::encode(dest);
        println!("HH hex_dest = {}", hex_dest);

        let account = sp_core::H256::from_str(&hex_dest).unwrap();
        println!("sudo key h256 format = {:?}", account);

        let account = sp_runtime::AccountId32::from_str(&hex_dest).unwrap();
        println!("account  = {:?}", account);
    }
}
