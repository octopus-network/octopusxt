use octopusxt::ibc_node;
use structopt::StructOpt;
use subxt::ClientBuilder;

#[derive(Debug, StructOpt)]
pub struct Sudo {
    /// websocket_url
    pub websocket_url: Option<String>,
}

impl Sudo {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api = ClientBuilder::new()
            .set_url(
                self.websocket_url
                    .as_ref()
                    .unwrap_or(&"ws://localhost:9944".to_string()),
            )
            .build::<ibc_node::DefaultConfig>()
            .await?
            .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: sp_core::H256 = block_header.hash();

        let account_id32 = api.storage().sudo().key(Some(block_hash)).await?;

        println!("Account id32 = {:?}", account_id32);

        Ok(())
    }
}
