use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Extrinsics {}
impl Extrinsics {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
