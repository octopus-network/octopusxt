use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::{
    core::{
        ics02_client::{
            client_consensus::AnyConsensusState,
            client_state::{AnyClientState, IdentifiedAnyClientState},
        },
        ics24_host::identifier::{ClientId, ConnectionId},
        ics24_host::path::ClientStatePath,
    },
    Height as ICSHeight,
};
use subxt::Client;
use tendermint_proto::Protobuf;

use anyhow::Result;
use codec::Decode;
use core::str::FromStr;
use ibc::core::ics24_host::path::{ClientConnectionsPath, ClientConsensusStatePath};
use ibc::core::ics24_host::Path;
use sp_core::H256;
use subxt::storage::StorageClient;

/// get client_state according by client_id, and read ClientStates StoraageMap
pub async fn get_client_state(
    client_id: &ClientId,
    client: Client<MyConfig>,
) -> Result<AnyClientState> {
    tracing::info!("in call_ibc : [get_client_state]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let client_state_path = ClientStatePath(client_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .client_states(&client_state_path, Some(block_hash))
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

/// get appoint height consensus_state according by client_identifier and height
/// and read ConsensusStates StorageMap
pub async fn get_client_consensus(
    client_id: &ClientId,
    height: &ICSHeight,
    client: Client<MyConfig>,
) -> Result<AnyConsensusState> {
    tracing::info!("in call_ibc: [get_client_consensus]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // search key
    let client_consensus_state_path = ClientConsensusStatePath {
        client_id: client_id.clone(),
        epoch: height.revision_number,
        height: height.revision_height,
    }
    .to_string()
    .as_bytes()
    .to_vec();

    let consensus_state: Vec<u8> = api
        .storage()
        .ibc()
        .consensus_states(&client_consensus_state_path, Some(block_hash))
        .await?;

    tracing::info!(
        "get_client_consensus is empty! by client_id = ({}), height = ({})",
        client_id,
        height
    );

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

/// get consensus state with height
pub async fn get_consensus_state_with_height(
    client_id: &ClientId,
    client: Client<MyConfig>,
) -> Result<Vec<(ICSHeight, AnyConsensusState)>> {
    tracing::info!("in call_ibc: [get_consensus_state_with_height]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

    // Read Store ConsensusStates
    let mut iter = storage
        .iter::<ibc_node::ibc::storage::ConsensusStates>(Some(block_hash))
        .await?;

    let mut result = vec![];

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)?;
        let client_state_path = String::from_utf8(raw_key)?;
        // decode key
        let path =
            Path::from_str(&client_state_path).map_err(|_| anyhow::anyhow!("decode path error"))?;
        // println!("[get_consensus_state_with_height] >> path: {:?}", path);
        match path {
            Path::ClientConsensusState(client_consensus_state) => {
                let ClientConsensusStatePath {
                    client_id: read_client_id,
                    epoch,
                    height,
                } = client_consensus_state;

                if read_client_id == client_id.clone() {
                    let height = ICSHeight::new(epoch, height);
                    let consensus_state = AnyConsensusState::decode_vec(&*value).unwrap();
                    // store key-value
                    result.push((height, consensus_state));
                }
            }
            _ => unimplemented!(),
        }
        // println!("[get_consensus_state_with_height]>>  Value: {:?}", value);
    }

    Ok(result)
}

/// get key-value pair (client_identifier, client_state) construct IdentifierAny Client state
pub async fn get_clients(client: Client<MyConfig>) -> Result<Vec<IdentifiedAnyClientState>> {
    tracing::info!("in call_ibc: [get_clients]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

    let mut iter = storage
        .iter::<ibc_node::ibc::storage::ClientStates>(Some(block_hash))
        .await?;

    let mut result = vec![];

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)?;
        let client_state_path = String::from_utf8(raw_key)?;
        // decode key
        let path =
            Path::from_str(&client_state_path).map_err(|_| anyhow::anyhow!("decode path error"))?;
        match path {
            Path::ClientState(ClientStatePath(client_id)) => {
                let client_state = AnyClientState::decode_vec(&*value).unwrap();

                result.push(IdentifiedAnyClientState::new(client_id, client_state));
            }
            _ => unimplemented!(),
        }
        println!("  Value: {:?}", value);
    }

    Ok(result)
}

/// get connection_identifier vector according by client_identifier
pub async fn get_client_connections(
    client_id: &ClientId,
    client: Client<MyConfig>,
) -> Result<Vec<ConnectionId>> {
    tracing::info!("in call_ibc: [get_client_connections]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let client_connection_paths = ClientConnectionsPath(client_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // client_id <-> connection_id
    let connection_id: Vec<u8> = api
        .storage()
        .ibc()
        .connection_client(&client_connection_paths, Some(block_hash))
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
