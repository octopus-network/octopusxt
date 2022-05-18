#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


use structopt::StructOpt;
use subxt::ClientBuilder;


pub mod substrate;

use substrate::account::Account;
use substrate::balance::Balance;
use substrate::query::Query;
use substrate::sudo::Sudo;
use substrate::timestamp::TimeStamp;

#[derive(Debug, StructOpt)]
#[structopt(name = "octopusxt-cli", about = "A tools for octopus cli command")]
pub enum Command {

    #[structopt(name = "balance")]
    /// substrate balance module
    Balance(Balance),

    #[structopt(name = "sudo")]
    /// substrate sudo module
    Sudo(Sudo),

    #[structopt(name = "account")]
    /// basic account
    Account(Account),

    #[structopt(name = "query")]
    /// basic query
    Query(Query),

    #[structopt(name = "timestamp")]
    /// substrate timestamp module
    TimeStamp(TimeStamp),
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
        Command::Balance(value) => {
            let _ = value.run().await?;
        }
        Command::Sudo(value) => {
            let _ = value.run().await?;
        }
      
        Command::Account(value) => {
            let ret = value.run();
        }
        Command::Query(value) => {
            let ret = value.run().await?;
        }
        Command::TimeStamp(value) => {
            let ret = value.run().await?;
        }
    }

    Ok(())
}
