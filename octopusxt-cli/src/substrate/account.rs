use octopusxt::ibc_node;
use sp_core::{hexdisplay::HexDisplay, Pair};
use sp_keyring::AccountKeyring;
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use std::str::FromStr;
use structopt::StructOpt;
use subxt::{ClientBuilder, PairSigner};

/// Public key type for Runtime
pub type PublicFor<P> = <P as sp_core::Pair>::Public;

/// formats public key as accountId as hex
fn format_account_id<P: sp_core::Pair>(public_key: PublicFor<P>) -> String
where
    PublicFor<P>: Into<MultiSigner>,
{
    format!(
        "0x{}",
        HexDisplay::from(&public_key.into().into_account().as_ref())
    )
}

#[derive(Debug, StructOpt)]
pub enum Account {
    #[structopt(name = "substrate-account")]
    /// basic substrate account
    SubstrateAccount(SubstrateAccount),
}

impl Account {
    pub fn run(&self) {
        match self {
            Account::SubstrateAccount(value) => value.run(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct SubstrateAccount {}

impl SubstrateAccount {
    pub fn run(&self) {
        let account_id32: sp_runtime::AccountId32 = AccountKeyring::Bob.to_account_id();
        println!("😂😂😂😂😂😂😂😂😂😂😂😂 account id = {:?}", account_id32);

        let account_id32: &[u8; 32] = account_id32.as_ref();
        let hex_account_id32 = hex::encode(account_id32);
        println!("account id 32: hex_dest = {}", hex_account_id32);

        let account_id32_h256 = sp_core::H256::from_str(&hex_account_id32).unwrap();
        println!("account id32 h256 format = {:?}", account_id32_h256);

        let account = sp_runtime::AccountId32::from_str(&hex_account_id32).unwrap();
        println!("account id  = {:?}", account);

        let private_seed = "ecology agent adjust admit raw castle rather travel asthma good say field away vote timber miss demand mandate rib print sport vault warfare thrive".to_string();

        sc_cli::utils::print_from_uri::<sp_core::sr25519::Pair>(
            &private_seed,
            None,
            None,
            sc_cli::OutputType::Json,
        );

        // let temp = "0x3cea083c24de7949fd99c890f351abbeed730efae8f074aa6625293dbc9ef642".to_string();

        // let account_tmp = sp_core::H256::from_str(&temp).unwrap();
        // println!("account_tmp hex = {:?}",account_tmp);

        // let account = sp_runtime::AccountId32::from_str(&temp).unwrap();
        // println!("account id = {:?}", account);

        // let password = password.as_ref().map(|s| s.expose_secret().as_str());

        let (pair, seed) = sp_core::sr25519::Pair::from_phrase(&private_seed, None).unwrap();
        let public_key = pair.public();

        let account_id = format_account_id::<sp_core::sr25519::Pair>(public_key);

        println!("account_id = {:?}", account_id);
    }
}
