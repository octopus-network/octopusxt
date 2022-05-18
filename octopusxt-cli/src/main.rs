#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::ops::Rem;
use structopt::StructOpt;
use subxt::ClientBuilder;

pub mod substrate;

pub use substrate::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "octopusxt-cli", about = "A tools for octopus cli command")]
pub enum Command {
    #[structopt(name = "account")]
    /// basic account
    Account(Account),

    #[structopt(name = "query")]
    /// basic query
    Query(Query),

    #[structopt(name = "asset-tx-payment")]
    /// asset-tx-payment module
    AssetTxPayment(AssetTxPayment),

    #[structopt(name = "assets")]
    /// assets module
    Assets(Assets),

    #[structopt(name = "authority-discovery")]
    /// authority discovery module
    AuthorityDiscovery(AuthorityDiscovery),

    #[structopt(name = "authorship")]
    /// authorship module
    Authorship(Authorship),

    #[structopt(name = "babe")]
    /// babe module
    Babe(Babe),

    #[structopt(name = "bags-list")]
    /// bags list module
    BagsList(BagsList),

    #[structopt(name = "balance")]
    /// balance module
    Balance(Balance),

    #[structopt(name = "beefy")]
    /// beefy module
    Beefy(Beefy),

    #[structopt(name = "bounties")]
    /// bounties module
    Bounties(Bounties),

    #[structopt(name = "child-bounties")]
    /// child bounties module
    ChildBounties(ChildBounties),

    #[structopt(name = "contracts")]
    /// contracts module
    Contracts(Contracts),

    #[structopt(name = "conviction-voting")]
    /// conviction voting module
    ConvictionVoting(ConvictionVoting),

    #[structopt(name = "council")]
    /// council module
    Council(Council),

    #[structopt(name = "democracy")]
    /// demcracy module
    Democracy(Democracy),

    #[structopt(name = "election-provider-multi-phase")]
    /// election provider multi phase module
    ElectionProviderMultiPhase(ElectionProviderMultiPhase),

    #[structopt(name = "elections")]
    /// elections module
    Elections(Elections),

    #[structopt(name = "glit")]
    /// glit module
    Glit(Glit),

    #[structopt(name = "grandpa")]
    /// grandpa module
    Grandpa(Grandpa),

    #[structopt(name = "historical")]
    /// historical module
    Historical(Historical),

    #[structopt(name = "identity")]
    /// identity module
    Identity(Identity),

    #[structopt(name = "im-online")]
    /// im online module
    ImOnline(ImOnline),

    #[structopt(name = "indices")]
    /// indices module
    Indices(Indices),

    #[structopt(name = "lottery")]
    /// lottery module
    Lottery(Lottery),

    #[structopt(name = "mmr")]
    /// mmr module
    Mmr(Mmr),

    #[structopt(name = "multisig")]
    /// multisig module
    Multisig(Multisig),

    #[structopt(name = "nomiation-pools")]
    /// nomiation pools module
    NomiationPools(NomiationPools),

    #[structopt(name = "offences")]
    /// pffences module
    Offences(Offences),

    #[structopt(name = "preimage")]
    /// preimage module
    Preimage(Preimage),

    #[structopt(name = "proxy")]
    /// proxy module
    Proxy(Proxy),

    #[structopt(name = "randomness-collective-flip")]
    /// randomness collective flip
    RandomnessCollectiveFlip(RandomnessCollectiveFlip),

    #[structopt(name = "recovery")]
    /// recovery module
    Recovery(Recovery),

    #[structopt(name = "referenda")]
    Referenda(Referenda),

    #[structopt(name = "remark")]
    /// remark module
    Remark(Remark),

    #[structopt(name = "scheduler")]
    /// scheduler module
    Scheduler(Scheduler),

    #[structopt(name = "session")]
    /// session module
    Session(Session),

    #[structopt(name = "society")]
    /// society module
    Society(Society),

    #[structopt(name = "staking")]
    /// staking module
    Staking(Staking),

    #[structopt(name = "state-trie-migration")]
    /// state trie migration  module
    StateTrieMigration(StateTrieMigration),

    #[structopt(name = "sudo")]
    /// sudo module
    Sudo(Sudo),

    #[structopt(name = "system")]
    /// system module
    System(System),

    #[structopt(name = "technical-committee")]
    /// technical committee module
    TechnicalCommittee(TechnicalCommittee),

    #[structopt(name = "technical-membership")]
    /// technical membership
    TechnicalMembership(TechnicalMembership),

    #[structopt(name = "timestamp")]
    /// substrate timestamp module
    TimeStamp(TimeStamp),

    #[structopt(name = "tips")]
    /// tips module
    Tips(Tips),

    #[structopt(name = "transaction-payment")]
    /// transaction payment module
    TransactionPayment(TransactionPayment),

    #[structopt(name = "transaction-storage")]
    /// transaction storage module
    TransactionStorage(TransactionStorage),

    #[structopt(name = "treasury")]
    /// treasury module
    Treasury(Treasury),

    #[structopt(name = "uniques")]
    /// uniques module
    Uniques(Uniques),

    #[structopt(name = "utility")]
    /// utility module
    Utility(Utility),

    #[structopt(name = "vesting")]
    /// vesting module
    Vesting(Vesting),

    #[structopt(name = "whitelist")]
    /// whitelist module
    Whitelist(Whitelist),
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
        Command::Account(value) => {
            let ret = value.run();
        }
        Command::AssetTxPayment(value) => {}
        Command::Assets(value) => {}
        Command::AuthorityDiscovery(value) => {}
        Command::Authorship(value) => {}
        Command::Babe(value) => {}
        Command::BagsList(value) => {}
        Command::Balance(value) => {
            let _ = value.run().await?;
        }
        Command::Beefy(value) => {}
        Command::Bounties(value) => {}
        Command::ChildBounties(value) => {}
        Command::Contracts(value) => {}
        Command::ConvictionVoting(value) => {}
        Command::Council(value) => {}
        Command::Democracy(value) => {}
        Command::ElectionProviderMultiPhase(value) => {}
        Command::Elections(value) => {}
        Command::Glit(value) => {}
        Command::Grandpa(value) => {}
        Command::Historical(value) => {}
        Command::Identity(value) => {}
        Command::ImOnline(value) => {}
        Command::Indices(value) => {}
        Command::Lottery(value) => {}
        Command::Mmr(value) => {}
        Command::Multisig(value) => {}
        Command::NomiationPools(value) => {}
        Command::Offences(value) => {}
        Command::Preimage(value) => {}
        Command::Proxy(value) => {}
        Command::Query(value) => {
            let ret = value.run().await?;
        }
        Command::RandomnessCollectiveFlip(value) => {}
        Command::Recovery(value) => {}
        Command::Referenda(value) => {}
        Command::Remark(value) => {}
        Command::Scheduler(value) => {}
        Command::Session(value) => {}
        Command::Society(value) => {}
        Command::Staking(value) => {}
        Command::StateTrieMigration(value) => {}
        Command::Sudo(value) => {
            let _ = value.run().await?;
        }
        Command::System(value) => {}
        Command::TechnicalCommittee(value) => {}
        Command::TechnicalMembership(value) => {}

        Command::TimeStamp(value) => {
            let ret = value.run().await?;
        }
        Command::Tips(value) => {}
        Command::TransactionPayment(value) => {}
        Command::TransactionStorage(value) => {}
        Command::Treasury(value) => {}
        Command::Uniques(value) => {}
        Command::Utility(value) => {}
        Command::Vesting(value) => {}
        Command::Whitelist(value) => {}
    }

    Ok(())
}
