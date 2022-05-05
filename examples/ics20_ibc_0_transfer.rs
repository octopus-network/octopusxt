use codec::Encode;
use octopusxt::ibc_node;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, PairSigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("ws://localhost:8844")
        .build()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let signer = PairSigner::new(AccountKeyring::Ferdie.pair());
    
    let source_port = ibc::core::ics24_host::identifier::PortId::transfer();
    let vec_source_port = ibc::core::ics24_host::identifier::PortId::transfer().as_bytes().to_vec();

    let source_channel = ibc::core::ics24_host::identifier::ChannelId::new(0);
    let vec_source_channel = ibc::core::ics24_host::identifier::ChannelId::new(0).as_bytes().to_vec();

    // contruct prefix
    let prefix = format!("{}/{}",source_port, source_channel);
    
    let token = format!("{}/{}", prefix, "ATOM").as_bytes().to_vec();

    let amount = 100 * 1_000_000_000_000_000_000u128;

    // 对方链上的账户
    let receiver = AccountKeyring::Alice.to_account_id();

    let encode_receiver = sp_runtime::AccountId32::encode(&receiver);
    let hex_receiver = hex::encode(encode_receiver).as_bytes().to_vec();
    println!("transfer : hex  : {:?}", hex_receiver);

    let timeout_height = 9999;

    let timeout_timestamp = 9999;

    // this chain sender is Feridie 
    // the countray chain receiver is Alice 
    // send token is 
    let events = api
        .tx()
        .ibc()
        .transfer(
            vec_source_port,
            vec_source_channel,
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
