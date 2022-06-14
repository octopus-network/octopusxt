use generator_metadata::{substrate, MyConfig};
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub enum Constants {
    #[structopt(name = "epoch-duration")]
    EpochDuration {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "expected-block-time")]
    ExpectedBlockTime {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "max-authorities")]
    MaxAuthorities {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
}

impl Constants {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // match self {
        //     Constants::EpochDuration { websocket_url } => {
        //         // set client
        //         let api = ClientBuilder::new()
        //             .set_url(websocket_url)
        //             .build()
        //             .await?
        //             .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();
        //
        //         let result = api.constants().babe().epoch_duration();
        //
        //         println!("epoch_duration = {:#?}", result);
        //     }
        //     Constants::ExpectedBlockTime { websocket_url } => {
        //         // set client
        //         let api = ClientBuilder::new()
        //             .set_url(websocket_url)
        //             .build()
        //             .await?
        //             .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();
        //
        //         let result = api.constants().babe().expected_block_time();
        //
        //         println!("expected_block_time = {:#?}", result);
        //     }
        //     Constants::MaxAuthorities { websocket_url } => {
        //         // set client
        //         let api = ClientBuilder::new()
        //             .set_url(websocket_url)
        //             .build()
        //             .await?
        //             .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();
        //
        //         let result = api.constants().babe().max_authorities();
        //
        //         println!("max_authorities = {:#?}", result);
        //     }
        // }
        Ok(())
    }
}
