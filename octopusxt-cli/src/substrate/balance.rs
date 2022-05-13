use std::str::FromStr;

use octopusxt::ibc_node;
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
                let (alice, ferdie) = ("alice".to_string(), "ferdie".to_string());
                let sender = value.sender.as_ref().unwrap_or(&alice);
                let receiver = value.receiver.as_ref().unwrap_or(&ferdie);
                let amoumt = value.amount.unwrap_or(1_000_000_000_000_000_000_000);

                println!("sender = {:?}", sender);
                println!("receiver = {:?}", receiver);
                println!("amount = {:?}", amoumt);

                let sender = PairSigner::new(AccountKeyring::from_str(sender).unwrap().pair());
                let receiver = AccountKeyring::from_str(receiver)
                    .unwrap()
                    .to_account_id()
                    .into();

                let api = ClientBuilder::new()
                    .set_url(value.websocket_url.clone())
                    .build()
                    .await?
                    .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

                let hash = api
                    .tx()
                    .balances()
                    .transfer(receiver, amoumt)
                    .sign_and_submit_then_watch(&sender)
                    .await?;

                let result_event = hash.find_event_raw("Balances", "Transfer").unwrap();

                println!("Balance transfer extrinsic submitted: {:?}", result_event);
            }
        }

        Ok(())
    }
}
