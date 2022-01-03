use octopusxt::ibc_node;
use std::str::FromStr;
use subxt::{ClientBuilder, EventSubscription};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();
    let block_hash = block_header.hash();

    //example 1
    // let mut iter = api
    //     .storage()
    //     .system()
    //     .account_iter(Some(block_hash))
    //     .await?;

    // while let Some((key, account)) = iter.next().await? {
    //     println!("{}: {}", hex::encode(key), account.data.free);
    // }

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
    // let port_id = ibc::ics24_host::identifier::PortId::from_str("transfer").unwrap();
    // let channel_id = ibc::ics24_host::identifier::ChannelId::from_str("channel-0").unwrap();

    // let client = ClientBuilder::new()
    //     .set_url("ws://localhost:9944")
    //     .build::<ibc_node::DefaultConfig>()
    //     .await?;

    // let ret = octopusxt::get_channel_end(&port_id, &channel_id, client).await?;
    // println!("In main: [get_channelend] >> channel_end: {:?}", ret);

    // example 3
    // let sub = api.client.rpc().subscribe_events().await.unwrap();
    // let decoder = api.client.events_decoder();
    // let mut sub = EventSubscription::<ibc_node::DefaultConfig>::new(sub, decoder);

    // let mut counter = 0;
    // while let Some(raw_event) = sub.next().await {
    //     if let Err(err) = raw_event {
    //         println!(
    //             "In main: [run_loop] >> raw_event error: {:?}",
    //             err
    //         );
    //         continue;
    //     }
    //     if counter == 5 {
    //         break;
    //     }
    //     counter += 1;
    //     let raw_event = raw_event.unwrap();
    //     println!(
    //         "in main: [run_loop] >> raw_event : {:?}",
    //         raw_event
    //     );
    // }

    // mmr exeample 4 
    // 1
    let root_hash = api.storage().mmr().root_hash(Some(block_hash)).await?;
    println!("root_hash : {:?}", root_hash);

    // 2
    let number_of_leaves = api.storage().mmr().number_of_leaves(Some(block_hash)).await?;
    println!("number_of_leaves : {:?}", number_of_leaves);


    // 3
    let mut mmr = api.storage().mmr().nodes_iter(Some(block_hash)).await?;
    let mut counter = 0;
    while let Some(mmr_item) = mmr.next().await? {
        counter += 1;
        println!("mmr: {}", hex::encode(mmr_item.0));
    }
    println!("counter : {}", counter);

    // 4 
    let mmr_node = api.storage().mmr().nodes(1, Some(block_hash)).await?;
    println!("mmr_node : {:?}", mmr_node);
    println!("mmr_node_encode: {}", hex::encode(mmr_node.unwrap()));

    // beefy 
    let authorities = api.storage().beefy().authorities(Some(block_hash)).await?;
    // println!("authorities : {:?}", authorities);

    let validator_set_id = api.storage().beefy().validator_set_id(Some(block_hash)).await?;
    println!("validator_set_id : {:?}", validator_set_id);

    let next_authorities = api.storage().beefy().next_authorities(Some(block_hash)).await?;
    // println!("next_authorities : {:?}", next_authorities);

    // mmr_leaf
    let beefy_next_authorities = api.storage().mmr_leaf().beefy_next_authorities(Some(block_hash)).await?;
    // println!("beefy_next_authorities : {:?}", beefy_next_authorities);
    

    Ok(())
}
