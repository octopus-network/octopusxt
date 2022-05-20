use generator_metadata::{substrate, MyConfig};
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub enum Constants {
    #[structopt(name = "existential-deposit")]
    ExistentialDeposit {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "max-locks")]
    MaxLocks {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "max-reserves")]
    MaxReserves {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
}

impl Constants {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Constants::ExistentialDeposit { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().balances().existential_deposit();

                println!("existential_deposit = {:#?}", result);
            }
            Constants::MaxLocks { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().balances().max_locks();

                println!("max_locks = {:#?}", result);
            }
            Constants::MaxReserves { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().balances().max_reserves();

                println!("max_reserves = {:#?}", result);
            }
        }
        Ok(())
    }
}
