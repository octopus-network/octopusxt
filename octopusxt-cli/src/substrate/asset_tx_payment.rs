use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum AssetTxPayment {}

impl  AssetTxPayment {
    pub fn run(&self) {
        println!("Asset tx payment!");
    }

}
