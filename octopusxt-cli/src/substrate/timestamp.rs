use octopusxt::ibc_node;
use octopusxt::ibc_rpc::get_storage_key;
use sp_keyring::AccountKeyring;
use std::str::FromStr;
use structopt::StructOpt;
use subxt::BlockNumber;
use subxt::{ClientBuilder, PairSigner};

#[derive(Debug, StructOpt)]
pub enum TimeStamp {
    #[structopt(name = "now")]
    /// Current time for the current block.
    Now,
    #[structopt(name = "did-update")]
    ///  Did the timestamp get updated in this block?
    DidUpdate,
}

impl TimeStamp {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TimeStamp::Now => {
                let api = ClientBuilder::new()
                    .set_url("ws://localhost:9944")
                    .build::<ibc_node::DefaultConfig>()
                    .await?;

                let storage_api = ibc_node::timestamp::storage::StorageApi::new(&api);

                let time_stamp = storage_api.now(None).await?;

                println!("time_stamp = {:?}", time_stamp);
            }
            TimeStamp::DidUpdate => {
                let api = ClientBuilder::new()
                    .set_url("ws://localhost:9944")
                    .build::<ibc_node::DefaultConfig>()
                    .await?;

                let storage_api = ibc_node::timestamp::storage::StorageApi::new(&api);

                let result = storage_api.did_update(None).await?;

                println!("did_update = {:?}", result);
            }
        }

        Ok(())
    }
}
