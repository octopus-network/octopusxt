use octopusxt::ibc_node;
use octopusxt::MyConfig;
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub struct Sudo {
    /// websocket_url
    #[structopt(default_value = "ws://localhost:9944")]
    pub websocket_url: String,
}

impl Sudo {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api = ClientBuilder::new()
            .set_url(self.websocket_url.clone())
            .build()
            .await?
            .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: sp_core::H256 = block_header.hash();

        let account_id32 = api.storage().sudo().key(Some(block_hash)).await?;

        println!("Account id32 = {:?}", account_id32);

        Ok(())
    }
}
