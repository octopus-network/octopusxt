use octopusxt::ibc_node;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, PairSigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // let signer = PairSigner::new(AccountKeyring::Alice.pair());
    // let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    // this is substrate balance transfer
    // let hash = api
    //     .tx()
    //     .balances()
    //     .transfer(dest, 123_456_789_012_345)
    //     .sign_and_submit_then_watch(&signer)
    //     .await?;

    // let result_event = hash.find_event_raw("Balances", "Transfer").unwrap();

    // println!("Balance transfer extrinsic submitted: {:#?}", result_event);

    // this is pallet ibc transfer
    
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let source_port = ibc::core::ics24_host::identifier::PortId::transfer().as_bytes().to_vec();  
    let source_channel = ibc::core::ics24_host::identifier::ChannelId::new(0).as_bytes().to_vec();

    let token = "sample".to_string().as_bytes().to_vec();
    
    let amount = 23;

    let receiver = format!("{}", AccountKeyring::Bob.to_account_id()).as_bytes().to_vec();
    
    let timeout_height = 90;
    
    let timeout_timestamp = 100;


    let events = api
        .tx()
        .ibc()
        .transfer(
            source_port,
            source_channel,
            token,
            amount,
            receiver,
            timeout_height,
            timeout_timestamp,
        )
        .sign_and_submit_then_watch(&signer)
        .await?;

    println!("Balance transfer extrinsic submitted: {:?}", events);

    Ok(())
}
