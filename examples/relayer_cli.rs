use std::str::FromStr;

use codec::{Decode, Encode};
use ibc::applications::ics20_fungible_token_transfer::msgs::denom_trace::{
    parse_hex_hash, DenomTrace,
};
use octopusxt::ibc_node;
use octopusxt::ibc_rpc::get_storage_key;
use structopt::StructOpt;
use subxt::ClientBuilder;
use tendermint_proto::Protobuf;

#[derive(Debug, StructOpt)]
pub struct CliDenomTrace {
    // websocket_url
    pub websocket_url: Option<String>,
    /// port id
    pub port_id: Option<String>,
    /// channel id
    pub channel_id: Option<String>,
    /// denom
    pub denom: Option<String>,
}

impl CliDenomTrace {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.port_id.is_none() || self.channel_id.is_none() || self.denom.is_none() {
            println!("port id or channel id or denom is None!");
        }

        let port_id = self.port_id.as_ref().unwrap();
        let channel_id = self.channel_id.as_ref().unwrap();
        let base_denom = self.denom.as_ref().unwrap();
        let path = format!("{}/{}", port_id, channel_id);
        let ibc_denom_trace = DenomTrace::new(path, base_denom.clone());

        println!("ibc hash = {:?}", ibc_denom_trace.hash());
        let ibc_denom = ibc_denom_trace.ibc_denom().unwrap();
        println!("ibc_denom = {:?}", ibc_denom);
        let split_result = ibc_denom.split('/').collect::<Vec<&str>>();

        let denom_hash_str = split_result[1];

        let result = parse_hex_hash(denom_hash_str).unwrap();
        println!("prase hex hash = {:?}", result);

        let ret = self.get_chain_denom_trace().await?;

        for item in ret.iter() {
            println!("ibc_hash = {:?}, ibc_denom = {:?}", item.0, item.1);
        }

        println!("**********************************");

        Ok(())
    }

    async fn get_chain_denom_trace(
        &self,
    ) -> Result<Vec<(String, DenomTrace)>, Box<dyn std::error::Error>> {
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

        let mut data = api
            .storage()
            .ibc()
            .denomination_iter(Some(block_hash))
            .await?;

        let mut denom_hash_and_denom_trace = vec![];
        while let Ok(Some(value)) = data.next().await {
            let denom_trace_hash = &value.0 .0[..];
            println!("denom_trace_hash **1 = {:?}", denom_trace_hash);

            let denom_hex =
                String::from_utf8(subtle_encoding::hex::encode_upper(denom_trace_hash)).unwrap();
            println!("denom hex = {}", denom_hex);

            let denom_trace = DenomTrace::decode(&*value.1).unwrap();
            println!("denom trate = {:?}", denom_trace);
            println!("denom_trace_hash **2 = {:?}", denom_trace.hash());

            let ibc_denom = denom_trace.ibc_denom().unwrap();
            println!("ibc_denom: {}", ibc_denom);

            assert_eq!(ibc_denom, "ibc/".to_string() + &denom_hex);

            denom_hash_and_denom_trace.push((denom_hex, denom_trace));
        }

        Ok(denom_hash_and_denom_trace)
    }
}

#[derive(Debug, StructOpt)]
pub struct Balance {
    // websocket_url
    pub websocket_url: Option<String>,
}

impl Balance {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("substrate balance module start");

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

        let mut data = api
            .storage()
            .balances()
            .account_iter(Some(block_hash))
            .await?;

        while let Ok(Some(value)) = data.next().await {
            println!("storage key = {:?}", value.0);
            println!("account free = {:?}", value.1.free);
            println!("account reserved = {:?}", value.1.reserved);
            println!("account misc_frozen = {:?}", value.1.misc_frozen);
            println!("account fee_frozen = {:?}", value.1.fee_frozen);
        }

        println!("substrate balance module end");
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Sudo {
    /// websocket_url
    pub websocket_url: Option<String>,
}

impl Sudo {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
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

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "denom-trace")]
    /// Contruct Denom Trace will display ibc_denom
    DenomTrace(CliDenomTrace),

    #[structopt(name = "balance")]
    /// substrate balance module
    Balance(Balance),

    #[structopt(name = "sudo")]
    // substrate sudo module
    Sudo(Sudo),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "classify")]
pub struct CliArguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opt = CliArguments::from_args();

    match opt.command {
        Command::DenomTrace(value) => {
            let ret = value.run().await?;
        }
        Command::Balance(value) => {
            let ret = value.run().await?;
        }
        Command::Sudo(value) => {
            let ret = value.run().await?;
        }
    }

    Ok(())
}
