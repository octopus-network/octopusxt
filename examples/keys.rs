use octopusxt::ibc_node;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, PairSigner};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest: sp_runtime::AccountId32 = AccountKeyring::Bob.to_account_id().into();
    println!("😂😂😂😂😂😂😂😂😂😂😂😂 account id = {:?}", dest);

    let dest : &[u8; 32] = dest.as_ref();
    let hex_dest = hex::encode(dest);
    println!("HH hex_dest = {}", hex_dest);

    let account = sp_core::H256::from_str(&hex_dest).unwrap();
    println!("sudo key h256 format = {:?}", account);

    let account = sp_runtime::AccountId32::from_str(&hex_dest).unwrap();
    println!("account  = {:?}", account);

    Ok(())
}