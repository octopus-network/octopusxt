use std::str::FromStr;

use futures::StreamExt;
use generator_metadata::substrate_node_template;
use generator_metadata::MyConfig;
use sp_keyring::AccountKeyring;
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::PairSigner;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub enum Extrinsics {
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
impl Extrinsics {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Extrinsics::Transfer(value) => {
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
                    .to_runtime_api::<substrate_node_template::RuntimeApi<
                        MyConfig,
                        SubstrateExtrinsicParams<MyConfig>,
                    >>();

                let mut transfer_events = api
                    .events()
                    .subscribe()
                    .await?
                    .filter_events::<(substrate_node_template::balances::events::Transfer,)>();

                let hash = api
                    .tx()
                    .balances()
                    .transfer(receiver, amoumt)?
                    .sign_and_submit_default(&sender)
                    .await?;

                // Our subscription will see all of the transfer events emitted as a result of this:
                while let Some(transfer_event) = transfer_events.next().await {
                    println!("Balance transfer event: {transfer_event:?}");
                }
            }
        }

        Ok(())
    }
}
