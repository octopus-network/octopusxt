use generator_metadata::{substrate, MyConfig};
use structopt::StructOpt;
use subxt::sp_core::crypto::AccountId32;
use subxt::sp_core::H256;
use subxt::ClientBuilder;
use subxt::SubstrateExtrinsicParams;

#[derive(Debug, StructOpt)]
pub enum Storage {
    #[structopt(name = "account")]
    Account {
        account_id_32: AccountId32,
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "account-iter")]
    AccountIter {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "extrinsic-count")]
    ExtrinsicCount {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "block-weight")]
    BlockWeight {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "all-extrinsics-len")]
    AllExtrinsicsLen {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "block-hash")]
    BlockHash {
        id: u32,
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "block-hash-iter")]
    BlockHashIter {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "extrinsic-data")]
    ExtrinsicData {
        id: u32,
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "extrinsic-data-iter")]
    ExtrinsicDataIter {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "number")]
    Number {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "parent-hash")]
    ParentHash {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "digest")]
    Digest {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "events")]
    Events {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "event-count")]
    EventCount {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "event-topics")]
    EventTopics {
        hash_value: H256,
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "event-topics-iter")]
    EventTopicsIter {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "last-runtime-upgrade")]
    LastRutimeUpgrade {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "upgraded-to-u32-ref-count")]
    UpgradedToU32RefCount {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "upgraded-to-trip-ref-count")]
    UpgradedToTripleRefCount {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
    #[structopt(name = "execution-phase")]
    ExecutionPhase {
        hash: Option<H256>,
        #[structopt(default_value = "ws://localhost:9944")]
        websocket_url: String,
    },
}

impl Storage {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Storage::Account {
                account_id_32,
                hash,
                websocket_url,
            } => {
                // TODO
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .account(account_id_32, hash.clone())
                    .await?;

                println!("account = {:#?}", result);
            }
            Storage::AccountIter {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().account_iter(hash.clone()).await?;

                // println!("account_iter = {:?}", result);
            }
            Storage::ExtrinsicCount {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().extrinsic_count(hash.clone()).await?;

                println!("extrinsic_count = {:#?}", result);
            }
            Storage::BlockWeight {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().block_weight(hash.clone()).await?;

                println!("block_weight = {:#?}", result);
            }
            Storage::AllExtrinsicsLen {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .all_extrinsics_len(hash.clone())
                    .await?;

                println!("all_extrinsics_len = {:#?}", result);
            }
            Storage::BlockHash {
                id,
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().block_hash(id, hash.clone()).await?;

                println!("block_hash = {:#?}", result);
            }
            Storage::BlockHashIter {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().block_hash_iter(hash.clone()).await?;

                // println!("block_hash_iter = {:?}", result);
            }
            Storage::ExtrinsicData {
                id,
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .extrinsic_data(id, hash.clone())
                    .await?;

                println!("extrinsic_data = {:#?}", result);
            }
            Storage::ExtrinsicDataIter {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .extrinsic_data_iter(hash.clone())
                    .await?;

                // println!("extrinsic_data_iter = {:?}", result);
            }
            Storage::Number {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().number(hash.clone()).await?;

                println!("number = {:#?}", result);
            }
            Storage::ParentHash {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().parent_hash(hash.clone()).await?;

                println!("parent_hash = {:#?}", result);
            }
            Storage::Digest {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().digest(hash.clone()).await?;

                println!("digest = {:#?}", result);
            }
            Storage::Events {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().events(hash.clone()).await?;

                println!("events = {:#?}", result);
            }
            Storage::EventCount {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().event_count(hash.clone()).await?;

                println!("event_count = {:#?}", result);
            }
            Storage::EventTopics {
                hash_value,
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .event_topics(hash_value, hash.clone())
                    .await?;

                println!("event_topics = {:#?}", result);
            }
            Storage::EventTopicsIter {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .event_topics_iter(hash.clone())
                    .await?;

                // println!("event_topics_iter = {:?}", result);
            }
            Storage::LastRutimeUpgrade {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .last_runtime_upgrade(hash.clone())
                    .await?;

                println!("last_runtime_upgrade = {:#?}", result);
            }
            Storage::UpgradedToU32RefCount {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .upgraded_to_u32_ref_count(hash.clone())
                    .await?;

                println!("upgraded_to_u32_ref_count = {:#?}", result);
            }
            Storage::UpgradedToTripleRefCount {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api
                    .storage()
                    .system()
                    .upgraded_to_triple_ref_count(hash.clone())
                    .await?;

                println!("upgraded_to_triple_ref_count = {:#?}", result);
            }
            Storage::ExecutionPhase {
                hash,
                websocket_url,
            } => {
                // set client
                let api = ClientBuilder::new()
                    .set_url(websocket_url)
                    .build()
                    .await?
                    .to_runtime_api::<substrate::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

                let result = api.storage().system().execution_phase(hash.clone()).await?;

                println!("execution_phase = {:#?}", result);
            }
        }
        Ok(())
    }
}
