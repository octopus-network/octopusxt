use codec::{Decode, Encode};
use ibc::applications::ics20_fungible_token_transfer::msgs::denom_trace::{
    parse_hex_hash, DenomTrace,
};
use octopusxt::ibc_node;
use sp_keyring::AccountKeyring;
use std::str::FromStr;
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::PairSigner;
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
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.port_id.is_none() || self.channel_id.is_none() || self.denom.is_none() {
            println!("port id or channel id or denom is None!");
        }

        let default_port_id = "transfer".to_string();
        let default_channel_id = "channel-0".to_string();
        let default_token = "ATOM".to_string();
        let port_id = self.port_id.as_ref().unwrap_or(&default_port_id);
        let channel_id = self.channel_id.as_ref().unwrap_or(&default_channel_id);
        let base_denom = self.denom.as_ref().unwrap_or(&default_token);
        let path = format!("{}/{}", port_id, channel_id);
        println!("port_id = {}", port_id);
        println!("channel_id = {}", channel_id);
        println!("base_denom = {}", base_denom);
        println!("path = {}", path);

        let ibc_denom_trace = DenomTrace::new(path, base_denom.clone());

        println!("ibc_denom_trace = {:?}", ibc_denom_trace);

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
pub struct IbcModule {
    // websocket_url
    pub websocket_url: Option<String>,
    /// account sender, now advice account is alice, bob, dave, eve, ferdie, one, two,
    pub sender: Option<String>,
    /// account receiver, now advice account is alice, bob, dave, eve, ferdie, one, two,
    pub receiver: Option<String>,
    /// send token denom, There have two format denom, ATOM, ibc/04C1A8B4EC211C89630916F8424F16DC9611148A5F300C122464CE8E996AABD0
    pub token: Option<String>,
    /// source port id, default port id is transfer
    pub port_id: Option<String>,
    /// source channel id, default channel id is channel-0
    pub channel_id: Option<String>,
    /// sender want to send amout
    pub amount: Option<u128>,
    /// timeout height
    pub timeout_height: Option<u64>,
    /// timeout timestamp
    pub timeout_timestamp: Option<u64>,
}

impl IbcModule {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        // set client
        let api = ClientBuilder::new()
            .set_url(
                self.websocket_url
                    .as_ref()
                    .unwrap_or(&"ws://localhost:9944".to_string()),
            )
            .build()
            .await?
            .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

        let (alice, ferdie) = ("alice".to_string(), "ferdie".to_string());
        let default_token = "ATOM".to_string();
        let default_port_id = "transfer".to_string();
        let default_channel_id = "channel-0".to_string();

        let sender = self.sender.as_ref().unwrap_or(&alice);
        let receiver = self.receiver.as_ref().unwrap_or(&ferdie);
        let token = self.token.as_ref().unwrap_or(&default_token);
        let port_id = self.port_id.as_ref().unwrap_or(&default_port_id);
        let channel_id = self.channel_id.as_ref().unwrap_or(&default_channel_id);
        let amount = self.amount.unwrap_or(1_000_000_000_000_000_000_000);
        let timeout_height = self.timeout_height.unwrap_or(9999);
        let timeout_timestamp = self.timeout_timestamp.unwrap_or(9999);

        println!("sender = {:?}", sender);
        println!("receiver = {:?}", receiver);
        println!("token = {:?}", token);
        println!("amount = {:?}", amount);
        println!("timeout height = {:?}", timeout_height);
        println!("timeout timeout = {:?}", timeout_timestamp);

        let sender = PairSigner::new(AccountKeyring::from_str(sender).unwrap().pair());
        let receiver = AccountKeyring::from_str(receiver)
            .unwrap()
            .to_account_id()
            .into();

        let encode_receiver = sp_runtime::AccountId32::encode(&receiver);
        let hex_receiver = hex::encode(encode_receiver).as_bytes().to_vec();
        println!("transfer : hex  : {:?}", hex_receiver);

        // this chain sender is alice, just is sign this tx signer
        // countray chain receiver is Ferdie
        // send token is this chain token atom
        // result is countray chain receiver will receive amount and token is ATOM
        let events = api
            .tx()
            .ibc()
            .transfer(
                port_id.as_bytes().to_vec(),
                channel_id.as_bytes().to_vec(),
                token.as_bytes().to_vec(),
                amount,
                hex_receiver,
                timeout_height,
                timeout_timestamp,
            )
            .sign_and_submit_then_watch(&sender)
            .await?;

        println!("Balance transfer extrinsic submitted: {:?}", events);

        Ok(())
    }
}
