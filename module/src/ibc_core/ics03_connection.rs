use crate::{ChannelRpc, ConnectionRpc, OctopusxtClient};
use ibc::core::{
    ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
    ics04_channel::channel::IdentifiedChannelEnd,
    ics24_host::identifier::{ChannelId, ConnectionId, PortId},
};

use async_trait::async_trait;
use core::str::FromStr;
use ibc::core::ics23_commitment::merkle::MerkleProof;

use ibc_relayer::chain::requests::{
    IncludeProof, QueryChannelRequest, QueryConnectionChannelsRequest, QueryConnectionRequest,
    QueryConnectionsRequest, QueryHeight,
};
use tendermint_proto::Protobuf;

#[async_trait]
impl ConnectionRpc for OctopusxtClient {
    type Error = anyhow::Error;

    async fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc: [get_connection_end]");

        let QueryConnectionRequest {
            connection_id,
            height,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .connections(connection_id.as_bytes(), Some(block_hash))
            .await?;

        if data.is_empty() {
            return Err(anyhow::anyhow!(
                "get_connection_end is empty! by connection_identifier = ({})",
                connection_id
            ));
        }

        let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((connection_end, None)),
        }
    }

    async fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Self::Error> {
        tracing::info!("in call_ibc: [get_connections]");

        let QueryConnectionsRequest {
            pagination: _pagination,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
            .await?;

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
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Self::Error> {
        tracing::info!("in call_ibc: [get_connection_channels]");

        let QueryConnectionChannelsRequest {
            connection_id,
            pagination: _pagination,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
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

            let query_channel_request = QueryChannelRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                height: QueryHeight::Latest,
            };

            // get channel_end
            let (channel_end, _) = self
                .query_channel(query_channel_request, IncludeProof::No)
                .await?;

            result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
        }

        Ok(result)
    }
}
