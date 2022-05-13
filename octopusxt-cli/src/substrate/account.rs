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
        let account_id32: sp_runtime::AccountId32 = AccountKeyring::Bob.to_account_id().into();
        println!("😂😂😂😂😂😂😂😂😂😂😂😂 account id = {:?}", account_id32);

        let account_id32: &[u8; 32] = account_id32.as_ref();
        let hex_account_id32 = hex::encode(account_id32);
        println!("account id 32: hex_dest = {}", hex_account_id32);

        let account_id32_h256 = sp_core::H256::from_str(&hex_account_id32).unwrap();
        println!("account id32 h256 format = {:?}", account_id32_h256);

        let account = sp_runtime::AccountId32::from_str(&hex_account_id32).unwrap();
        println!("account id  = {:?}", account);
    }
}
