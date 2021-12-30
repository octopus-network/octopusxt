use octopusxt::ibc_node;
use subxt::{ClientBuilder, EventSubscription};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api
        .client
        .rpc()
        .subscribe_finalized_blocks()
        .await?;
    
    let block_header = block.next().await.unwrap().unwrap();
    let block_hash = block_header.hash();

    //example 1
    let mut iter = api.storage().system().account_iter(Some(block_hash)).await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }

    // example 2
    // let data = api
    //     .storage()
    //     .ibc()
    //     .channels(
    //         "transfer".as_bytes().to_vec(),
    //         "channel-0".as_bytes().to_vec(),
    //         Some(block_hash)
    //     )
    //     .await?;

    // let channel_end = ChannelEnd::decode_vec(&*data).unwrap();
    // println!(
    //     "In substrate: [get_channelend] >> channel_end: {:?}",
    //     channel_end
    // );
    
    // example 2, 2-ways
    let port_id = ibc::ics24_host::identifier::PortId::from_str("transfer").unwrap();
    let channel_id = ibc::ics24_host::identifier::ChannelId::from_str("channel-0").unwrap();

    let client = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await?;

    let ret = octopusxt::get_channel_end(&port_id, &channel_id, client).await?;
    println!("In substrate: [get_channelend] >> channel_end: {:?}", ret);

    // example 3
    let sub = api.client.rpc().subscribe_events().await.unwrap();
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<ibc_node::DefaultConfig>::new(sub, decoder);

    let mut counter = 0;
    while let Some(raw_event) = sub.next().await {
        if let Err(err) = raw_event {
            println!("In substrate_mointor: [run_loop] >> raw_event error: {:?}", err);
            continue;
        }
        if counter == 5 {
            break;
        }
        counter += 1;
        let raw_event = raw_event.unwrap();
        println!("in substrate_mointor: [run_loop] >> raw_event : {:?}", raw_event);
    }

    Ok(())
}
