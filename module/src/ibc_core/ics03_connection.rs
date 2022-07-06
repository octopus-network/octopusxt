use crate::{ChannelRpc, ConnHandshakeProof, ConnectionRpc, OctopusxtClient, QueryHeight};
use ibc::core::{
    ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
    ics04_channel::channel::IdentifiedChannelEnd,
    ics24_host::identifier::{ChannelId, ConnectionId, PortId},
};

use crate::primitive::IdentifiedConnection;
use async_trait::async_trait;
use core::str::FromStr;
use ibc::core::ics24_host::identifier::ClientId;
use tendermint_proto::Protobuf;

#[async_trait]
impl ConnectionRpc for OctopusxtClient {
    type Error = anyhow::Error;

    async fn query_connection_end(
        &self,
        connection_identifier: ConnectionId,
        height: QueryHeight,
    ) -> Result<ConnectionEnd, Self::Error> {
        tracing::info!("in call_ibc: [get_connection_end]");

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

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

    async fn query_connections(
        &self,
        height: QueryHeight,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Self::Error> {
        tracing::info!("in call_ibc: [get_connections]");

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let mut ret = vec![];

        // TODO
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

            let connection_end = ConnectionEnd::decode_vec(connection_end).unwrap();

            result.push(IdentifiedConnectionEnd::new(connection_id, connection_end));
        }

        Ok(result)
    }

    async fn query_connection_channels(
        &self,
        connection_id: ConnectionId,
        height: QueryHeight,
    ) -> Result<Vec<IdentifiedChannelEnd>, Self::Error> {
        tracing::info!("in call_ibc: [get_connection_channels]");

        let api = self.to_runtime_api();

        let block_hash = self
            .query_block_hash_by_query_height(height.clone())
            .await?;

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
                .query_channel_end(port_id.clone(), channel_id.clone(), height.clone())
                .await?;

            result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
        }

        Ok(result)
    }

    fn query_connection_using_client(
        &self,
        _height: QueryHeight,
        _client_id: ClientId,
    ) -> Result<Vec<IdentifiedConnection>, Self::Error> {
        todo!()
    }

    fn generate_conn_handshake_proof(
        &self,
        _height: QueryHeight,
        _client_id: ClientId,
        _conn_id: ConnectionId,
    ) -> Result<ConnHandshakeProof, Self::Error> {
        todo!()
    }
}
