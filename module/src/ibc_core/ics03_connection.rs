use crate::{ChannelRpc, ConnectionRpc, OctopusxtClient};
use ibc::core::{
    ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
    ics04_channel::channel::IdentifiedChannelEnd,
    ics24_host::identifier::{ChannelId, ConnectionId, PortId},
};

use anyhow::Result;
use async_trait::async_trait;
use core::str::FromStr;
use sp_core::H256;
use tendermint_proto::Protobuf;

#[async_trait]
impl ConnectionRpc for OctopusxtClient {
    async fn get_connection_end(
        &self,
        connection_identifier: ConnectionId,
    ) -> Result<ConnectionEnd> {
        tracing::info!("in call_ibc: [get_connection_end]");

        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .connections(connection_identifier.as_bytes(), Some(block_hash))
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

    async fn get_connections(&self) -> Result<Vec<IdentifiedConnectionEnd>> {
        tracing::info!("in call_ibc: [get_connections]");

        let api = self.to_runtime_api();

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
            return Err(anyhow::anyhow!(
                "get_connections: get empty connection_keys"
            ));
        }

        for key in connection_keys {
            // get connections value
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

            let connnection_end = ConnectionEnd::decode_vec(connection_end).unwrap();

            result.push(IdentifiedConnectionEnd::new(connection_id, connnection_end));
        }

        Ok(result)
    }

    async fn get_connection_channels(
        &self,
        connection_id: ConnectionId,
    ) -> Result<Vec<IdentifiedChannelEnd>> {
        tracing::info!("in call_ibc: [get_connection_channels]");

        let api = self.to_runtime_api();

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
            return Err(anyhow::anyhow!(
                "get_connection_channels is empty! by connection_id = ({})",
                connection_id
            ));
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
            let channel_end = self
                .get_channel_end(port_id.clone(), channel_id.clone())
                .await?;

            result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
        }

        Ok(result)
    }
}
