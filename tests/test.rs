
use crate::ibc_node;
use octopusxt::*;
use subxt::ClientBuilder;
use ibc::Height;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics24_host::identifier::{ChannelId, ClientId, PortId};
use core::str::FromStr;
use subxt::BlockNumber;


// test API get_block_header
// use `cargo test -- --captuer` can print content
#[tokio::test]
async fn test_get_block_header() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await?;
    let block_number = Some(BlockNumber::from(3));
    let header = get_header_by_block_number(client, block_number).await?;

    println!("convert header = {:?}", header);

    Ok(())
}

#[tokio::test]
async fn test_get_client_consensus() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await?;

    let result = get_client_consensus(
        &ClientId::new(ClientType::Grandpa, 0).unwrap(),
        Height::new(0, 320),
        client,
    )
    .await?;

    println!("result = {:?}", result);

    Ok(())
}

#[tokio::test]
async fn test_get_key() -> Result<(), Box<dyn std::error::Error>> {
    use subxt::StorageEntry;
    let ibc = crate::ibc_node::ibc::storage::ClientStatesKeys;
    let result = ibc.key();

    Ok(())
}

#[tokio::test]
async fn test_get_mmr_leaf_and_mmr_proof() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await?;

    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let block_number = 22;

    let block_hash: sp_core::H256 = api
        .client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await?
        .unwrap();

    println!("block_hash = {:?}", block_hash);

    let result =
        get_mmr_leaf_and_mmr_proof((block_number - 1) as u64, Some(block_hash), client).await?;

    println!("result = {:?}", result);

    Ok(())
}

#[tokio::test]
async fn test_get_packet_commitment() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await?;

    let client_id = PortId::from_str("transfer").unwrap();
    let channel_id = ChannelId::from_str("channel-0").unwrap();

    let result = get_packet_commitment(&client_id, &channel_id, 1, client)
        .await
        .unwrap();
    println!("packet_commitment = {:?}", result);

    Ok(())
}
// add unit test for get storage key
#[test]
fn test_get_storage_key() {
    let _ibc = crate::ibc_node::ibc::storage::ClientStates(vec![1, 2, 3]);
    let ibc = crate::ibc_node::ibc::storage::ClientStatesKeys;
    let _ibc = crate::ibc_node::ibc::storage::ConsensusStates(vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::Connections(vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::ConnectionsKeys;
    let _ibc = crate::ibc_node::ibc::storage::Channels(vec![1, 2, 3], vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::ChannelsKeys;
    let _ibc = crate::ibc_node::ibc::storage::ChannelsConnection(vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::NextSequenceSend(vec![1, 2, 3], vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::NextSequenceRecv(vec![1, 2, 3], vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::NextSequenceAck(vec![1, 2, 3], vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::Acknowledgements(vec![1, 2, 3], vec![1, 2, 3], 1);
    let _ibc = crate::ibc_node::ibc::storage::AcknowledgementsKeys;
    let _ibc = crate::ibc_node::ibc::storage::Clients(vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::ClientCounter;
    let _ibc = crate::ibc_node::ibc::storage::ConnectionCounter;
    let _ibc = crate::ibc_node::ibc::storage::ChannelCounter;
    let _ibc = crate::ibc_node::ibc::storage::ConnectionClient(vec![1, 2, 3]);
    let _ibc = crate::ibc_node::ibc::storage::PacketReceipt(vec![1, 2, 3], vec![1, 2, 3], 1);
    let _ibc = crate::ibc_node::ibc::storage::PacketCommitment(vec![1, 2, 3], vec![1, 2, 3], 1);
    let _ibc = crate::ibc_node::ibc::storage::PacketCommitmentKeys;
    let _ibc = crate::ibc_node::ibc::storage::SendPacketEvent(vec![1, 2, 3], vec![1, 2, 3], 1);
    let _ibc = crate::ibc_node::ibc::storage::WriteAckPacketEvent(vec![1, 2, 3], vec![1, 2, 3], 1);
    let _ibc = crate::ibc_node::ibc::storage::LatestHeight;
    let _ibc = crate::ibc_node::ibc::storage::OldHeight;
}

#[tokio::test]
async fn test_get_latest_height() {
    let client = ClientBuilder::new()
        .set_url("ws://localhost:9944")
        .build::<ibc_node::DefaultConfig>()
        .await
        .unwrap();

    let height = get_latest_height(client).await.unwrap();
    println!("height = {:?}", height);
}
