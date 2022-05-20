use generator_metadata::{substrate, MyConfig};
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub enum Constants {
    #[structopt(name = "uncle-generations")]
    UncleGenerations {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
}

impl Constants {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Constants::UncleGenerations { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().authorship().uncle_generations();

                println!("uncle_generations = {:#?}", result);
            }
        }
        Ok(())
    }
}
