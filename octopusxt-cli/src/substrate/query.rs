use octopusxt::{
    ibc_node, ibc_core::get_storage_key, MyConfig, SubstrateNodeTemplateExtrinsicParams,
};
use sp_keyring::AccountKeyring;
use std::str::FromStr;
use structopt::StructOpt;
use subxt::{BlockNumber, ClientBuilder, PairSigner};

#[derive(Debug, StructOpt)]
pub enum Query {
    #[structopt(name = "substrate")]
    /// substrate query
    SubstrateQuery(SubstrateQuery),
}

impl Query {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Query::SubstrateQuery(value) => {
                value.run().await?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct BlockHash {
    /// websocket url, the default websocket url is ws://localhost:9944
    #[structopt(default_value = "ws://localhost:9944")]
    pub websocket_url: String,
    /// block number, query block hash by block number, is blocl number is None, query last block's block hash
    pub block_number: Option<u32>,
}

#[derive(Debug, StructOpt)]
pub struct Other {
    /// websocket url, the default websocket url is ws://localhost:9944
    #[structopt(default_value = "ws://localhost:9944")]
    pub websocket_url: String,
}

#[derive(Debug, StructOpt)]
pub enum SubstrateQuery {
    #[structopt(name = "block-hash")]
    /// query block hash
    BlockHash(BlockHash),

    #[structopt(name = "other")]
    /// query other
    Other(Other),
}

impl SubstrateQuery {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SubstrateQuery::BlockHash(value) => {
                println!("BlockHash = {:?}", value);

                let api =
                    ClientBuilder::new()
                        .set_url(value.websocket_url.clone())
                        .build()
                        .await?
                        .to_runtime_api::<ibc_node::RuntimeApi<
                            MyConfig,
                            SubstrateNodeTemplateExtrinsicParams<MyConfig>,
                        >>();

                let block_number = value.block_number.map(BlockNumber::from);

                // Get a block hash, returns hash of latest block by default
                let block_hash = api.client.rpc().block_hash(block_number).await?;

                if value.block_number.is_none() {
                    println!("the latest block hash : {:?}", block_hash);
                } else {
                    println!(
                        "the number of [{:?}] block hash: {:?}",
                        value.block_number, block_hash
                    );
                }
            }
            SubstrateQuery::Other(value) => {
                let api =
                    ClientBuilder::new()
                        .set_url(value.websocket_url.clone())
                        .build()
                        .await?
                        .to_runtime_api::<ibc_node::RuntimeApi<
                            MyConfig,
                            SubstrateNodeTemplateExtrinsicParams<MyConfig>,
                        >>();

                println!("😂😂😂😂😂😂😂😂😂😂😂😂😂 Example1: Read proof RPC 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");
                // example 1: read_proof rpc
                let block_weight = ibc_node::system::storage::BlockWeight;
                let block_weight_key = get_storage_key(&block_weight);

                let block_weight_proof = api
                    .client
                    .rpc()
                    .read_proof(vec![block_weight_key], None)
                    .await?;

                println!("Get system blockWeight proof = {:?}", block_weight_proof);

                println!("😂😂😂😂😂😂😂😂😂😂😂😂😂Example2: Get Storage RPC 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");

                // example 2: storage rpc

                // getStorage(key: StorageKey, at?: BlockHash): StorageData
                // interface: api.rpc.state.getStorage
                // jsonrpc: state_getStorage
                // summary: Retrieves the storage for a key
                //
                // this exmaple use getStorage api, can get storage_data by a StorageKey
                //
                let sudo_key = ibc_node::sudo::storage::Key;
                let sudo_key = get_storage_key(&sudo_key);

                // get sudo key storage data
                // Fetch a storage Key: storage function
                //
                let storage_data = api.client.rpc().storage(&sudo_key, None).await?.unwrap();

                // encode sudo key storage data to hex format
                let key_data = hex::encode(storage_data.0);
                println!("sudo key data hex format = {}", key_data);

                // serde to sudo key to H256
                let account = sp_core::H256::from_str(&key_data).unwrap();
                println!("sudo key h256 format = {:?}", account);

                // serde to sudo key to AccountId32
                let account_id = sp_runtime::AccountId32::from_str(&key_data).unwrap();

                println!("sudo key AccountId32 format = {}", account_id);

                println!("😂😂😂😂😂😂😂😂😂😂😂😂Example3: query_storage RPC 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");

                let sudo_key = ibc_node::sudo::storage::Key;
                let sudo_key = get_storage_key(&sudo_key);

                println!("😂😂😂😂😂😂😂😂😂😂😂😂Example Query Block 😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂");

                let from_block_hash = sp_core::H256::from_str(
                    "0x0d232c3a18e8dd0144386ce53e5524fd30cdef673b2460041752fb641ed4de1f",
                )
                .unwrap();

                println!("from_block_hash = {}", from_block_hash);

                let block = api.client.rpc().block(Some(from_block_hash)).await?;

                println!("block = {:?}", block);
            }
        }

        Ok(())
    }
}
