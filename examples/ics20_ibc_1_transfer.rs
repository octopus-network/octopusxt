use codec::Encode;
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
    let source_port = ibc::core::ics24_host::identifier::PortId::transfer()
        .as_bytes()
        .to_vec();
    let source_channel = format!("{}",ibc::core::ics24_host::identifier::ChannelId::new(0))
        .as_bytes()
        .to_vec();

    let token = "ATOM".to_string().as_bytes().to_vec();

    let amount = 100 * 1_000_000_000_000_000_000u128;

    // 对方链上的账户
    let receiver = AccountKeyring::Ferdie.to_account_id();

    let encode_receiver = sp_runtime::AccountId32::encode(&receiver);
    let hex_receiver = hex::encode(encode_receiver).as_bytes().to_vec();
    println!("transfer : hex  : {:?}", hex_receiver);

    let timeout_height = 9999;

    let timeout_timestamp = 9999;

    // this chain sender is alice, just is sign this tx signer
    // countray chain receiver is Ferdie
    // send token is this chain token atom 
    // result is countray chain receiver will receive amount and token is ATOM
    let events = api
        .tx()
        .ibc()
        .transfer(
            source_port,
            source_channel,
            token,
            amount,
            hex_receiver,
            timeout_height,
            timeout_timestamp,
        )
        .sign_and_submit_then_watch(&signer)
        .await?;

    println!("Balance transfer extrinsic submitted: {:?}", events);

    Ok(())
}
