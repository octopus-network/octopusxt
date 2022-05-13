#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use ibc::applications::ics20_fungible_token_transfer::msgs::denom_trace::{
    parse_hex_hash, DenomTrace,
};
use octopusxt::ibc_node;
use structopt::StructOpt;
use subxt::ClientBuilder;
use tendermint_proto::Protobuf;

pub mod ibc_cli;
pub mod substrate;

use ibc_cli::CliDenomTrace;
use ibc_cli::IbcModule;
use substrate::account::Account;
use substrate::balance::Balance;
use substrate::query::Query;
use substrate::sudo::Sudo;

#[derive(Debug, StructOpt)]
#[structopt(name = "octopusxt-cli", about = "A tools for octopus cli command")]
pub enum Command {
    #[structopt(name = "denom-trace")]
    /// Contruct Denom Trace will display ibc_denom
    DenomTrace(CliDenomTrace),

    #[structopt(name = "balance")]
    /// substrate balance module
    Balance(Balance),

    #[structopt(name = "sudo")]
    /// substrate sudo module
    Sudo(Sudo),

    #[structopt(name = "account")]
    /// basic account
    Account(Account),

    #[structopt(name = "ibc-transfer")]
    /// ibc protocol for corss chain transfer
    IbcModule(IbcModule),

    #[structopt(name = "query")]
    /// basic query
    Query(Query),
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
            let _ = value.run().await?;
        }
        Command::Balance(value) => {
            let _ = value.run().await?;
        }
        Command::Sudo(value) => {
            let _ = value.run().await?;
        }
        Command::IbcModule(value) => {
            let ret = value.run().await?;
        }
        Command::Account(value) => {
            let ret = value.run();
        }
        Command::Query(value) => {
            let ret = value.run().await?;
        }
    }

    Ok(())
}
