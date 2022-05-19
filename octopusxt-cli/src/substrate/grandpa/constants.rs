use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Constants {}

impl Constants {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
