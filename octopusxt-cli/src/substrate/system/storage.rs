use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Storage {
    Account(Account),
    AccountIter(AccountIter),
    ExtrinsicCount(ExtrinsicCount),
    BlockWeight(BlockWeight),
    AllExtrinsicsLen(AllExtrinsicsLen),
    BlockHash(BlockHash),
    BlockHashIter(BlockHashIter),
    ExtrinsicData(ExtrinsicsData),
    ExtrinsicDataIter(ExtrinsicDataIter),
    Number(Number),
    ParentHash(ParentHash),
    Digest(Digest),
    Events(Events),
    EventCount(EventCount),
    EventTopics(EventTopics),
    EventTopicsIter(EventTopicsIter),
    LastRutimeUpgrade(LastRuntimeUpgrade),
    UpgradedToU32RefCount(UpgradedToU32RefCount),
    UpgradedToTripleRefCount(UpgradedToTripleRefCount),
    ExecutionPhase(ExecutionPhase),
}

impl Storage {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Storage::Account(account) => {}
            Storage::AccountIter(acounter) => {}
            Storage::ExtrinsicCount(extrinsic_count) => {}
            Storage::BlockWeight(block_weight) => {}
            Storage::AllExtrinsicsLen(all_extrinsics_len) => {}
            Storage::BlockHash(block_hash) => {}
            Storage::BlockHashIter(block_hash_iter) => {}
            Storage::ExtrinsicData(extrinsic_data) => {}
            Storage::ExtrinsicDataIter(extrinsic_data_iter) => {}
            Storage::Number(number) => {}
            Storage::ParentHash(parent_hash) => {}
            Storage::Digest(digest) => {}
            Storage::Events(events) => {}
            Storage::EventCount(event_count) => {}
            Storage::EventTopics(event_topics) => {}
            Storage::EventTopicsIter(event_topics_iter) => {}
            Storage::LastRutimeUpgrade(last_runtime_upfgrade) => {}
            Storage::UpgradedToU32RefCount(upgrade_to_u32_ref_count) => {}
            Storage::UpgradedToTripleRefCount(upgrade_to_triple_ref_count) => {}
            Storage::ExecutionPhase(execution_phase) => {}
        }
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Account {}

#[derive(Debug, StructOpt)]
pub struct AccountIter {}

#[derive(Debug, StructOpt)]
pub struct ExtrinsicCount {}

#[derive(Debug, StructOpt)]
pub struct BlockWeight {}

#[derive(Debug, StructOpt)]
pub struct AllExtrinsicsLen {}

#[derive(Debug, StructOpt)]
pub struct BlockHash {}

#[derive(Debug, StructOpt)]
pub struct BlockHashIter {}

#[derive(Debug, StructOpt)]
pub struct ExtrinsicsData {}

#[derive(Debug, StructOpt)]
pub struct ExtrinsicDataIter {}

#[derive(Debug, StructOpt)]
pub struct Number {}

#[derive(Debug, StructOpt)]
pub struct ParentHash {}

#[derive(Debug, StructOpt)]
pub struct Digest {}

#[derive(Debug, StructOpt)]
pub struct Events {}

#[derive(Debug, StructOpt)]
pub struct EventCount {}

#[derive(Debug, StructOpt)]
pub struct EventTopics {}

#[derive(Debug, StructOpt)]
pub struct EventTopicsIter {}

#[derive(Debug, StructOpt)]
pub struct LastRuntimeUpgrade {}

#[derive(Debug, StructOpt)]
pub struct UpgradedToU32RefCount {}

#[derive(Debug, StructOpt)]
pub struct UpgradedToTripleRefCount {}

#[derive(Debug, StructOpt)]
pub struct ExecutionPhase {}
