#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use octopusxt::ibc_node;
use structopt::StructOpt;
use subxt::ClientBuilder;
use tendermint_proto::Protobuf;

pub mod ibc_cli;
pub mod substrate;

// use ibc_cli::CliDenomTrace;
// use ibc_cli::IbcModule;
use substrate::account::Account;
use substrate::balance::Balance;
use substrate::beefy::Beefy;
use substrate::query::Query;
use substrate::sudo::Sudo;
use substrate::timestamp::TimeStamp;

#[derive(Debug, StructOpt)]
#[structopt(name = "octopusxt-cli", about = "A tools for octopus cli command")]
pub enum Command {
    #[structopt(name = "denom-trace")]
    // Contruct Denom Trace will display ibc_denom
    // DenomTrace(CliDenomTrace),
    #[structopt(name = "balance")]
    /// substrate balance module
    Balance(Balance),

    #[structopt(name = "sudo")]
    /// substrate sudo module
    Sudo(Sudo),

    #[structopt(name = "account")]
    /// basic account
    Account(Account),

    // #[structopt(name = "ibc-transfer")]
    // ibc protocol for corss chain transfer
    // IbcModule(IbcModule),
    #[structopt(name = "query")]
    /// basic query
    Query(Query),

    #[structopt(name = "timestamp")]
    /// substrate timestamp module
    TimeStamp(TimeStamp),

    #[structopt(name = "beefy")]
    /// beefy timestamp module
    Beefy(Beefy),
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
        // Command::DenomTrace(value) => {
        //     value.run().await?;
        // }
        Command::Balance(value) => {
            value.run().await?;
        }
        Command::Sudo(value) => {
            value.run().await?;
        }
        // Command::IbcModule(value) => {
        //     value.run().await?;
        // }
        Command::Account(value) => {
            value.run();
        }
        Command::Query(value) => {
            value.run().await?;
        }
        Command::TimeStamp(value) => {
            value.run().await?;
        }
        Command::Beefy(value) => {
            value.run().await?;
        }
    }

    Ok(())
}
