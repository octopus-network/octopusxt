use generator_metadata::{substrate, MyConfig};
use sp_keyring::AccountKeyring;
use std::ops::Sub;
use structopt::StructOpt;
use subxt::ClientBuilder;
use subxt::PairSigner;
use subxt::SubstrateExtrinsicParams;

use substrate::runtime_types::sp_arithmetic::per_things::Perbill;

#[derive(Debug, StructOpt)]
pub enum Extrinsics {
    #[structopt(name = "fill-block")]
    /// system call fill-block
    FillBlock {
        ratio: u32,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "remark")]
    /// system call remark TODO
    Remark {
        remark: Vec<u8>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "set-heap-pages")]
    /// system call set heap pages
    SetHeapPages {
        pages: u64,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "set-code")]
    /// system call set code TODO
    SetCode {
        code: Vec<u8>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "set-code-without-checks")]
    /// system call set code without checks TODO
    SetCodeWithoutChecks {
        code: Vec<u8>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "set-storage")]
    /// system call set storage TODO
    SetStorage {
        // todo
        // items: Vec<(Vec<u8>, Vec<u8>)>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "kill-storage")]
    /// system call kill storage TODO
    KillStorage {
        // todo
        // keys: Vec<Vec<u8>>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "kill-prefix")]
    /// system call kill prefix TODO
    KillPrefix {
        prefix: Vec<u8>,
        subkeys: u32,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "remark-with-event")]
    /// system call remark with event TODO
    RemarkWithEvent {
        remarks: Vec<u8>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
}

impl Extrinsics {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Extrinsics::FillBlock {
                ratio,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::fill_block{
                        ratio: Perbill(*ratio),
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_default(&signer)
                    .await?;

                println!("result hash = {:#?}", hash);
            }
            // todo
            Extrinsics::Remark {
                remark,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::remark{
                        remark: remark.clone(),
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("remark = {:?}", hash);
            }
            Extrinsics::SetHeapPages {
                pages,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::set_heap_pages{
                        pages: *pages,
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("result set_heap_pages = {:#?}", hash);
            }
            // todo
            Extrinsics::SetCode {
                code,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::set_code{
                        code: code.clone()
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("set_code = {:?}", hash);
            }
            // todo
            Extrinsics::SetCodeWithoutChecks {
                code,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::set_code_without_checks{
                        code: code.clone()
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("set_code_without_checks = {:?}", hash);
            }
            // todo
            Extrinsics::SetStorage {
                // items,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                // todo
                let items = vec![(vec![1, 2, 3], vec![1, 2, 3])];
                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::set_storage{
                        items
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("set_storage = {:?}", hash);
            }
            // todo
            Extrinsics::KillStorage {
                // keys,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                // todo
                let keys = vec![vec![1, 2, 3, 4, 5]];

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::kill_storage{
                        keys
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("set_storage = {:?}", hash);
            }
            // todo
            Extrinsics::KillPrefix {
                prefix,
                subkeys,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::kill_prefix{
                        prefix: prefix.clone(),
                        subkeys: *subkeys,
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("kill_prefix = {:?}", hash);
            }
            // todo
            Extrinsics::RemarkWithEvent {
                remarks,
                websocket_url,
            } => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let call =
                    generator_metadata::substrate::runtime_types::node_runtime::Call::System(generator_metadata::substrate::runtime_types::frame_system::pallet::Call::remark_with_event{
                        remark: remarks.clone(),
                    });

                let hash = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;

                println!("remark_with_event = {:?}", hash);
            }
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
