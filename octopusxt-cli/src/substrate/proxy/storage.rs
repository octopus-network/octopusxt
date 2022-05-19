use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Storage {}
impl Storage {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
