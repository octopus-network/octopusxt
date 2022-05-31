use octopusxt::ibc_node;
use octopusxt::MyConfig;
use sp_core::ByteArray;
use structopt::StructOpt;
use subxt::sp_core::Public;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub struct Beefy {
    #[structopt(default_value = "ws://localhost:9944")]
    /// websocket url
    pub websocket_url: String,
}

impl Beefy {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api = ClientBuilder::new()
            .set_url(self.websocket_url.clone())
            .build::<MyConfig>()
            .await?;

        let beefy_storage = ibc_node::beefy::storage::StorageApi::new(&api);
        let authorities = beefy_storage.authorities(None).await?;
        println!("authorities : {:?}", authorities);

        let api = ClientBuilder::new()
            .set_url(self.websocket_url.clone())
            .build()
            .await?
            .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;
        let block_header = block.next().await.unwrap().unwrap();
        let block_hash = block_header.hash();

        let authorities = api.storage().beefy().authorities(Some(block_hash)).await?;
        println!("authorities : {:?}", authorities);

        let result: Vec<String> = authorities
            .into_iter()
            .map(|val| {
                format!(
                    "0x{}",
                    subxt::sp_core::hexdisplay::HexDisplay::from(&val.to_raw_vec())
                )
            })
            .collect();
        println!("result = {:?}", result);

        Ok(())
    }
}
