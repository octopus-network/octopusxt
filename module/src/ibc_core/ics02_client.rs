use crate::{ClientRpc, OctopusxtClient};
use async_trait::async_trait;
use ibc::{
    core::{
        ics02_client::{
            client_consensus::AnyConsensusState,
            client_state::{AnyClientState, IdentifiedAnyClientState},
        },
        ics24_host::identifier::{ClientId, ConnectionId},
    },
    Height as ICSHeight,
};
use tendermint_proto::Protobuf;

use core::str::FromStr;
use ibc::core::ics02_client::client_consensus::AnyConsensusStateWithHeight;
use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc_relayer::chain::requests::{
    IncludeProof, QueryClientConnectionsRequest, QueryClientStateRequest, QueryClientStatesRequest,
    QueryConsensusStateRequest, QueryConsensusStatesRequest, QueryHeight,
    QueryUpgradedClientStateRequest, QueryUpgradedConsensusStateRequest,
};

#[async_trait]
impl ClientRpc for OctopusxtClient {
    type Error = anyhow::Error;

    async fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc : [get_client_state]");

        let QueryClientStateRequest { client_id, height } = request;

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .client_states(client_id.as_bytes(), Some(block_hash))
            .await?;

        if data.is_empty() {
            return Err(anyhow::anyhow!(
                "get_client_state is empty! by client_id = ({})",
                client_id
            ));
        }

        let any_client_state = AnyClientState::decode_vec(&*data).unwrap();

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((any_client_state, None)),
        }
    }

    async fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Self::Error> {
        tracing::info!("in call_ibc: [get_clients]");

        // TODO(davirain), this is not handle pagination
        let QueryClientStatesRequest {
            pagination: _pagination,
        } = request;

        let api = self.to_runtime_api();

        // TODO(davirain), this is query use QueryHeight::Latest not use pagination
        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
            .await?;

        // vector key-value
        let mut ret = vec![];

        // TODO, This future will be remove
        // get client_state Keys
        let client_states_keys: Vec<Vec<u8>> = api
            .storage()
            .ibc()
            .client_states_keys(Some(block_hash))
            .await?;
        if client_states_keys.is_empty() {
            return Err(anyhow::anyhow!("get_clients: get empty client_states_keys"));
        }

        // enumerate every item get client_state value
        for key in client_states_keys {
            // get client_state value
            let client_states_value: Vec<u8> = api
                .storage()
                .ibc()
                .client_states(&key, Some(block_hash))
                .await?;
            // store key-value
            ret.push((key.clone(), client_states_value));
        }

        let mut result = vec![];

        for (client_id, client_state) in ret.iter() {
            let client_id_str = String::from_utf8(client_id.clone()).unwrap();
            let client_id = ClientId::from_str(client_id_str.as_str()).unwrap();

            let client_state = AnyClientState::decode_vec(client_state).unwrap();

            result.push(IdentifiedAnyClientState::new(client_id, client_state));
        }

        Ok(result)
    }

    async fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Self::Error> {
        tracing::info!("in call_ibc: [get_client_connections]");

        let QueryClientConnectionsRequest { client_id } = request;

        let api = self.to_runtime_api();

        // this is use QueryHeight::Latest to query latest block hash
        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
            .await?;

        // TODO, this store Connection <-> Client seem have problem,
        // because this function return value want to Vec<ConnectionId>,
        // So, In substrate pallet ConnectionClient should be HashMap<Client, Vec<ConnectionId>>.
        // future should be modify.
        // client_id <-> connection_id
        let connection_id: Vec<u8> = api
            .storage()
            .ibc()
            .connection_client(client_id.as_bytes(), Some(block_hash))
            .await?;

        if connection_id.is_empty() {
            return Err(anyhow::anyhow!(
                "get_client_connections is empty! by client_id = ({})",
                client_id
            ));
        }

        let mut result = vec![];

        let connection_id_str = String::from_utf8(connection_id).unwrap();
        let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

        result.push(connection_id);

        Ok(result)
    }

    /// Query local chain consensus state
    async fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc: [get_client_consensus]");
        let QueryConsensusStateRequest {
            client_id,
            consensus_height,
            query_height,
        } = request;

        let block_hash = self.query_block_hash_by_query_height(query_height).await?;

        let api = self.to_runtime_api();

        let data: Vec<(Vec<u8>, Vec<u8>)> = api
            .storage()
            .ibc()
            .consensus_states(client_id.as_bytes(), Some(block_hash))
            .await?;

        if data.is_empty() {
            return Err(anyhow::anyhow!(
                "get_client_consensus is empty! by client_id = ({}), height = ({:?})",
                client_id,
                query_height
            ));
        }

        // get the height consensus_state
        let mut consensus_state = vec![];
        for item in data.iter() {
            if item.0 == consensus_height.encode_vec().unwrap() {
                consensus_state = item.1.clone();
            }
        }

        let any_onsensus_state = if consensus_state.is_empty() {
            // TODO
            AnyConsensusState::Grandpa(
                ibc::clients::ics10_grandpa::consensus_state::ConsensusState::default(),
            )
        } else {
            AnyConsensusState::decode_vec(&*consensus_state).unwrap()
        };

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((any_onsensus_state, None)),
        }
    }

    async fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Self::Error> {
        tracing::info!("in call_ibc: [get_consensus_state_with_height]");

        let QueryConsensusStatesRequest {
            client_id,
            pagination: _pagination,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
            .await?;

        // vector<height, consensus_state>
        let data: Vec<(Vec<u8>, Vec<u8>)> = api
            .storage()
            .ibc()
            .consensus_states(client_id.as_bytes(), Some(block_hash))
            .await?;

        if data.is_empty() {
            return Err(anyhow::anyhow!(
                "get_consensus_state_with_height is empty! by client_id = ({})",
                client_id
            ));
        }

        let mut result = vec![];
        for (height, consensus_state) in data.iter() {
            let height = ICSHeight::decode_vec(height).unwrap();
            let consensus_state = AnyConsensusState::decode_vec(consensus_state).unwrap();
            let value = AnyConsensusStateWithHeight {
                height,
                consensus_state,
            };
            result.push(value);
        }

        Ok(result)
    }

    async fn query_upgraded_client_state(
        &self,
        _request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Self::Error> {
        todo!()
    }

    async fn query_upgraded_consensus_state(
        &self,
        _request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Self::Error> {
        todo!()
    }
}
