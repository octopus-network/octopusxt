use std::str::FromStr;

use futures::StreamExt;
use octopusxt::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::{ClientBuilder, PairSigner};

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

                let api =
                    ClientBuilder::new()
                        .set_url(value.websocket_url.clone())
                        .build()
                        .await?
                        .to_runtime_api::<ibc_node::RuntimeApi<
                            MyConfig,
                            SubstrateNodeTemplateExtrinsicParams<MyConfig>,
                        >>();

                let mut transfer_events = api
                    .events()
                    .subscribe()
                    .await?
                    .filter_events::<(ibc_node::balances::events::Transfer,)>();

                let hash = api
                    .tx()
                    .balances()
                    .transfer(receiver, amoumt)?
                    .sign_and_submit_default(&sender)
                    .await?;

                println!("balance transfer Hash : {:?}", hash);

                // Our subscription will see all of the transfer events emitted as a result of this:
                if let Some(transfer_event) = transfer_events.next().await {
                    println!("Balance transfer event: {transfer_event:?}");
                }
            }
        }

        Ok(())
    }
}
