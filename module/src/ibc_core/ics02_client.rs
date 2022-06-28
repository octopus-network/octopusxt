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
use beefy_merkle_tree::Hash;
use sp_core::H256;
use crate::primitive::{IdentifiedClientState, QueryClientStateResponse, QueryConsensusStateResponse};

#[async_trait]
impl ClientRpc for OctopusxtClient {
    type Error = anyhow::Error;

    async fn query_client_state(&self, client_id: ClientId) -> Result<AnyClientState, Self::Error> {
        tracing::info!("in call_ibc : [get_client_state]");

        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

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

        let client_state = AnyClientState::decode_vec(&*data).unwrap();

        Ok(client_state)
    }

    async fn query_client_consensus_state(
        &self,
        client_id: ClientId,
        height: ICSHeight,
    ) -> Result<AnyConsensusState, Self::Error> {
        tracing::info!("in call_ibc: [get_client_consensus]");

        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

        let data: Vec<(Vec<u8>, Vec<u8>)> = api
            .storage()
            .ibc()
            .consensus_states(client_id.as_bytes(), Some(block_hash))
            .await?;

        if data.is_empty() {
            return Err(anyhow::anyhow!(
                "get_client_consensus is empty! by client_id = ({}), height = ({})",
                client_id,
                height
            ));
        }

        // get the height consensus_state
        let mut consensus_state = vec![];
        for item in data.iter() {
            if item.0 == height.encode_vec().unwrap() {
                consensus_state = item.1.clone();
            }
        }

        let consensus_state = if consensus_state.is_empty() {
            // TODO
            AnyConsensusState::Grandpa(
                ibc::clients::ics10_grandpa::consensus_state::ConsensusState::default(),
            )
        } else {
            AnyConsensusState::decode_vec(&*consensus_state).unwrap()
        };

        Ok(consensus_state)
    }

    async fn query_consensus_state_with_height(
        &self,
        client_id: ClientId,
    ) -> Result<Vec<(ICSHeight, AnyConsensusState)>, Self::Error> {
        tracing::info!("in call_ibc: [get_consensus_state_with_height]");

        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

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
            result.push((height, consensus_state));
        }

        Ok(result)
    }

    async fn query_clients(&self) -> Result<Vec<IdentifiedAnyClientState>, Self::Error> {
        tracing::info!("in call_ibc: [get_clients]");

        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

        // vector key-value
        let mut ret = vec![];

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
        client_id: ClientId,
    ) -> Result<Vec<ConnectionId>, Self::Error> {
        tracing::info!("in call_ibc: [get_client_connections]");

        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

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

    fn query_consensus_state(&self, _height: ICSHeight) -> Result<QueryConsensusStateResponse, Self::Error> {
        todo!()
    }

    fn query_upgraded_client(&self, _height: ICSHeight) -> Result<QueryClientStateResponse, Self::Error> {
        todo!()
    }

    fn query_upgraded_cons_state(&self, _height: ICSHeight) -> Result<QueryConsensusStateResponse, Self::Error> {
        todo!()
    }

    fn query_newly_created_clients(&self, _block_hash: Hash) -> Result<Vec<IdentifiedClientState>, Self::Error> {
        todo!()
    }
}
