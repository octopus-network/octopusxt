use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Assets {}

impl Assets {
    pub fn run(&self) {
        println!("assets");
    }
}
