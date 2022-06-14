use generator_metadata::{substrate, MyConfig};
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub enum Constants {
    OperationalFeeMultiplier {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    WeightToFee {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    LengthToFee {
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    }
}

impl Constants {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // match self {
        //     Constants::OperationalFeeMultiplier {
        //         websocket_url
        //     } => {
        //         // set client
        //         let api = ClientBuilder::new()
        //             .set_url(websocket_url)
        //             .build()
        //             .await?
        //             .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();
        //
        //         let result = api.constants().transaction_payment().operational_fee_multiplier();
        //
        //         println!("operational_fee_multiplier = {:#?}", result);
        //     },
        //     Constants::WeightToFee {
        //         websocket_url
        //     } => {
        //         // set client
        //         let api = ClientBuilder::new()
        //             .set_url(websocket_url)
        //             .build()
        //             .await?
        //             .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();
        //
        //         let result = api.constants().transaction_payment().weight_to_fee();
        //
        //         println!("weight_to_fee = {:#?}", result);
        //     }
        //     Constants::LengthToFee {
        //         websocket_url
        //     } => {
        //         // set client
        //         let api = ClientBuilder::new()
        //             .set_url(websocket_url)
        //             .build()
        //             .await?
        //             .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();
        //
        //         let result = api.constants().transaction_payment().length_to_fee();
        //
        //         println!("length_to_fee = {:#?}", result);
        //     }
        // }
        Ok(())
    }
}
