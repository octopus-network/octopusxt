use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
    ics04_channel::channel::IdentifiedChannelEnd,
    ics24_host::identifier::ConnectionId,
};
use subxt::Client;
use tendermint_proto::Protobuf;

use crate::channel::get_channel_end;
use anyhow::Result;
use core::str::FromStr;
use ibc::core::ics24_host::path::{ChannelEndsPath, ConnectionsPath};
use ibc::core::ics24_host::Path;
use sp_core::H256;

/// get connectionEnd according by connection_identifier and read Connections StorageMaps
pub async fn get_connection_end(
    connection_identifier: &ConnectionId,
    client: Client<MyConfig>,
) -> Result<ConnectionEnd> {
    tracing::info!("in call_ibc: [get_connection_end]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let connections_path = ConnectionsPath(connection_identifier.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .connections(&connections_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(anyhow::anyhow!(
            "get_connection_end is empty! by connection_identifier = ({})",
            connection_identifier
        ));
    }

    let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

    Ok(connection_end)
}

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
pub async fn get_connections(client: Client<MyConfig>) -> Result<Vec<IdentifiedConnectionEnd>> {
    tracing::info!("in call_ibc: [get_connctions]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let mut result = vec![];

    // get connection_keys
    let connection_keys: Vec<Vec<u8>> = api
        .storage()
        .ibc()
        .connections_keys(Some(block_hash))
        .await?;

    if connection_keys.is_empty() {
        return Err(anyhow::anyhow!(
            "get_connections: get empty connection_keys"
        ));
    }

    for key in connection_keys {
        let connection_id_str = String::from_utf8(key.clone()).unwrap();
        let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

        // read connection path key
        let connections_path = ConnectionsPath(connection_id.clone())
            .to_string()
            .as_bytes()
            .to_vec();

        // get connections value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .connections(&connections_path, Some(block_hash))
            .await?;

        // store key-value
        let connection_end = ConnectionEnd::decode_vec(&*value).unwrap();

        result.push(IdentifiedConnectionEnd::new(connection_id, connection_end));
    }

    Ok(result)
}

pub async fn get_connection_channels(
    connection_id: &ConnectionId,
    client: Client<MyConfig>,
) -> Result<Vec<IdentifiedChannelEnd>> {
    tracing::info!("in call_ibc: [get_connection_channels]");

    let api = client
        .clone()
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let connections_path = ConnectionsPath(connection_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // ConnectionsPath(connection_id) <-> Vec<ChannelEndsPath(port_id, channel_id)>
    let connections_paths: Vec<Vec<u8>> = api
        .storage()
        .ibc()
        .channels_connection(&connections_path, Some(block_hash))
        .await?;

    if connections_paths.is_empty() {
        return Err(anyhow::anyhow!(
            "get_connection_channels is empty! by connection_id = ({})",
            connection_id
        ));
    }

    let mut result = vec![];

    for connections_path in connections_paths.into_iter() {
        let raw_path = String::from_utf8(connections_path)?;
        // decode key
        let path = Path::from_str(&raw_path).map_err(|_| anyhow::anyhow!("decode path error"))?;

        match path {
            Path::ChannelEnds(channel_ends_path) => {
                let ChannelEndsPath(port_id, channel_id) = channel_ends_path;

                // get channel_end
                let channel_end = get_channel_end(&port_id, &channel_id, client.clone()).await?;

                result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
            }
            _ => unimplemented!(),
        }
    }

    Ok(result)
}
