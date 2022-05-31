use crate::{ibc_node, MyConfig};
use ibc::core::{
    ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
    ics04_channel::channel::IdentifiedChannelEnd,
    ics24_host::identifier::{ChannelId, ConnectionId, PortId},
};
use subxt::{Client, SubstrateExtrinsicParams};
use tendermint_proto::Protobuf;

use crate::channel::get_channel_end;
use core::str::FromStr;
use sp_core::H256;

/// get connectionEnd according by connection_identifier and read Connections StorageMaps
///
/// # Usage example
///
/// ```rust
/// let api = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let conection_id = ConnectionId::default();
/// let result = get_connection_end(&conection_id, api).await?;
/// ```
///
pub async fn get_connection_end(
    connection_identifier: &ConnectionId,
    client: Client<MyConfig>,
) -> Result<ConnectionEnd, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connection_end]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .connections(connection_identifier.as_bytes(), Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(Box::from(format!(
            "get_connection_end is empty! by connection_identifier = ({})",
            connection_identifier
        )));
    }

    let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

    Ok(connection_end)
}

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
///
/// # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let result = get_connections(client).await?;
/// ```
///
pub async fn get_connections(
    client: Client<MyConfig>,
) -> Result<Vec<IdentifiedConnectionEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connctions]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let mut ret = vec![];

    // get connection_keys
    let connection_keys: Vec<Vec<u8>> = api
        .storage()
        .ibc()
        .connections_keys(Some(block_hash))
        .await?;

    if connection_keys.is_empty() {
        return Err(Box::from(
            "get_connections: get empty connection_keys".to_string(),
        ));
    }

    for key in connection_keys {
        // get connectons value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .connections(&key, Some(block_hash))
            .await?;

        // store key-value
        ret.push((key.clone(), value.clone()));
    }

    let mut result = vec![];

    for (connection_id, connection_end) in ret.iter() {
        let connection_id_str = String::from_utf8(connection_id.clone()).unwrap();
        let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

        let connnection_end = ConnectionEnd::decode_vec(&*connection_end).unwrap();

        result.push(IdentifiedConnectionEnd::new(connection_id, connnection_end));
    }

    Ok(result)
}

///  # Usage example
///
/// ```rust
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let connection = ConnectionId::default();
/// let result = get_connection_channels(&connection, client).await?;
/// ```
///
pub async fn get_connection_channels(
    connection_id: &ConnectionId,
    client: Client<MyConfig>,
) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [get_connection_channels]");

    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // connection_id <-> Ve<(port_id, channel_id)>
    let channel_id_and_port_id: Vec<(Vec<u8>, Vec<u8>)> = api
        .storage()
        .ibc()
        .channels_connection(connection_id.as_bytes(), Some(block_hash))
        .await?;

    if channel_id_and_port_id.is_empty() {
        return Err(Box::from(format!(
            "get_connection_channels is empty! by connection_id = ({})",
            connection_id
        )));
    }

    let mut result = vec![];

    for (port_id, channel_id) in channel_id_and_port_id.iter() {
        // get port_id
        let port_id = String::from_utf8(port_id.clone()).unwrap();
        let port_id = PortId::from_str(port_id.as_str()).unwrap();

        // get channel_id
        let channel_id = String::from_utf8(channel_id.clone()).unwrap();
        let channel_id = ChannelId::from_str(channel_id.as_str()).unwrap();

        // get channel_end
        let channel_end = get_channel_end(&port_id, &channel_id, client.clone()).await?;

        result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
    }

    Ok(result)
}
