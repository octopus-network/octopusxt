// use octopusxt::ibc_node;
use structopt::StructOpt;
use subxt::ClientBuilder;

#[derive(Debug, StructOpt)]
pub struct Sudo {
    /// websocket_url
    #[structopt(default_value = "ws://localhost:9944")]
    pub websocket_url: String,
}

impl Sudo {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("sudo");

        Ok(())
    }
}
