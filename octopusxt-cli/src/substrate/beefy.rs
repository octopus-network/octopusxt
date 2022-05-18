use structopt::StructOpt;
use subxt::sp_core::Public;
use subxt::ClientBuilder;

#[derive(Debug, StructOpt)]
pub struct Beefy {
    #[structopt(default_value = "ws://localhost:9944")]
    /// websocket url
    pub websocket_url: String,
}

impl Beefy {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("beefy");

        Ok(())
    }
}
