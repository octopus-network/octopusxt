use sp_keyring::AccountKeyring;
use std::str::FromStr;
use structopt::StructOpt;
use subxt::BlockNumber;
use subxt::{ClientBuilder, PairSigner};

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
                let ret = value.run().await?;
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
                println!("block hash!");
            }
            SubstrateQuery::Other(value) => {
                println!("other!");
            }
        }

        Ok(())
    }
}
