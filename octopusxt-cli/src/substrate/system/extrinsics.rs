use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Extrinsics {
    #[structopt(name = "fill-block")]
    FillBlock(FillBlock),
    #[structopt(name = "remark")]
    Remark(Remark),
    #[structopt(name = "set-heap-pages")]
    SetHeapPages(SetHeapPages),
    #[structopt(name = "set-code")]
    SetCode(SetCode),
    #[structopt(name = "set-code-without-checks")]
    SetCodeWithoutChecks(SetCodeWithoutChecks),
    #[structopt(name = "set-storage")]
    SetStorage(SetStorage),
    #[structopt(name = "kill-storage")]
    KillStorage(KillStorage),
    #[structopt(name = "kill-prefix")]
    KillPrefix(KillPrefix),
    #[structopt(name = "remark-with-event")]
    RemarkWithEvent(RemarkWithEvent),
}

impl Extrinsics {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Extrinsics::FillBlock(fill_block) => {}
            Extrinsics::Remark(remark) => {}
            Extrinsics::SetHeapPages(set_heap_pages) => {}
            Extrinsics::SetCode(set_code) => {}
            Extrinsics::SetCodeWithoutChecks(set_code_without_checks) => {}
            Extrinsics::SetStorage(set_storage) => {}
            Extrinsics::KillStorage(kill_storage) => {}
            Extrinsics::KillPrefix(kill_prefix) => {}
            Extrinsics::RemarkWithEvent(remark_with_event) => {}
        }
        Ok(())
    }
}
#[derive(Debug, StructOpt)]
pub struct FillBlock {}

#[derive(Debug, StructOpt)]
pub struct Remark {}

#[derive(Debug, StructOpt)]
pub struct SetHeapPages {}

#[derive(Debug, StructOpt)]
pub struct SetCode {}

#[derive(Debug, StructOpt)]
pub struct SetCodeWithoutChecks {}

#[derive(Debug, StructOpt)]
pub struct SetStorage {}

#[derive(Debug, StructOpt)]
pub struct KillStorage {}

#[derive(Debug, StructOpt)]
pub struct KillPrefix {}

#[derive(Debug, StructOpt)]
pub struct RemarkWithEvent {}
