use generator_metadata::{substrate, MyConfig};
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub enum Constants {
    #[structopt(name = "block-weights")]
    BlockWeights {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "block-length")]
    BlockLength {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "block-hash-count")]
    BlockHashCount {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "db-weight")]
    DbWeight {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "version")]
    Version {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "ss58-prefix")]
    Ss58Prefix {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
}

impl Constants {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Constants::BlockWeights { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().system().block_weights();

                println!("block_weights = {:#?}", result);
            }
            Constants::BlockLength { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().system().block_length();

                println!("block_length = {:#?}", result);
            }
            Constants::BlockHashCount { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().system().block_hash_count();

                println!("block_hash_count = {:#?}", result);
            }
            Constants::DbWeight { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().system().db_weight();

                println!("db_weight = {:#?}", result);
            }
            Constants::Version { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().system().version();

                println!("version = {:#?}", result);
            }
            Constants::Ss58Prefix { websocket_url } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.constants().system().ss58_prefix();

                println!("ss58_prefix = {:#?}", result);
            }
        }
        Ok(())
    }
}
