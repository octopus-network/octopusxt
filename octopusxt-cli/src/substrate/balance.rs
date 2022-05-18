use std::str::FromStr;
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::PairSigner;

#[derive(Debug, StructOpt)]
pub enum Balance {
    /// balance transfer api
    #[structopt(name = "transfer")]
    Transfer(Transfer),
}

#[derive(Debug, StructOpt)]
pub struct Transfer {
    /// websocket_url
    #[structopt(default_value = "ws://localhost:9944")]
    pub websocket_url: String,
    /// account sender, now advice account is alice, bob, dave, eve, ferdie, one, two,
    pub sender: Option<String>,
    /// account receiver, now advice account is alice, bob, dave, eve, ferdie, one, two,
    pub receiver: Option<String>,
    /// sender want to send amout
    pub amount: Option<u128>,
}

impl Balance {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Balance::Transfer(value) => {
                println!("balance transfer");
            }
        }

        Ok(())
    }
}
