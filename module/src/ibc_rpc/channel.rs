use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::{
        channel::{ChannelEnd, IdentifiedChannelEnd},
        packet::{Receipt, Sequence},
    },
    ics24_host::identifier::{ChannelId, PortId},
};
use subxt::Client;
use tendermint_proto::Protobuf;

use core::str::FromStr;
use ibc_proto::ibc::core::channel::v1::PacketState;

use anyhow::Result;
use codec::Decode;
use ibc::core::ics24_host::Path;
use ibc::core::ics24_host::path::{
    AcksPath, ChannelEndsPath, CommitmentsPath, ReceiptsPath, SeqRecvsPath,
};

use sp_core::H256;
use subxt::storage::StorageClient;

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
pub async fn get_channels(client: Client<MyConfig>) -> Result<Vec<IdentifiedChannelEnd>> {
    tracing::info!("in call_ibc: [get_channels]");
    println!("in call_ibc: [get_channels]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

    let mut iter = storage
        .iter::<ibc_node::ibc::storage::Channels>(Some(block_hash))
        .await?;
    println!("in call_ibc: [get_channels] >> iter");

    let mut result = vec![];

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        println!("in call_ibc: [get_channels] >> while");
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)?;
        let raw_path = String::from_utf8(raw_key)?;
        // decode key
        let path = Path::from_str(&raw_path).map_err(|_| anyhow::anyhow!("decode path error"))?;
        println!("[get_channels] >> Path: {:?}", path);
        match path {
            Path::ChannelEnds(channel_ends_path) => {
                let ChannelEndsPath(port_id, channel_id)= channel_ends_path;
                let channel_end = ChannelEnd::decode_vec(&*value).unwrap();

                result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
            },
            _=> unimplemented!(),
        }
        println!("  Value: {:?}", value);
    }

    Ok(result)
}

/// get channelEnd according by port_identifier, channel_identifier and read Channel StorageMaps
pub async fn get_channel_end(
    port_id: &PortId,
    channel_id: &ChannelId,
    client: Client<MyConfig>,
) -> Result<ChannelEnd> {
    tracing::info!("in call_ibc: [get_channel_end]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let channel_end_path = ChannelEndsPath(port_id.clone(), channel_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .channels(&channel_end_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(anyhow::anyhow!(
            "get_channel_end is empty by port_id = ({}), channel_id = ({})",
            port_id,
            channel_id
        ));
    }

    let channel_end = ChannelEnd::decode_vec(&*data).unwrap();

    Ok(channel_end)
}

/// get packet receipt by port_id, channel_id and sequence
pub async fn get_packet_receipt(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<MyConfig>,
) -> Result<Receipt> {
    tracing::info!("in call_ibc : [get_packet_receipt]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let packet_receipt_path = ReceiptsPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_receipt(&packet_receipt_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(anyhow::anyhow!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id,
            channel_id
        ));
    }

    let receipt = String::from_utf8(data)?;
    if receipt.eq("Ok") {
        Ok(Receipt::Ok)
    } else {
        Err(anyhow::anyhow!(
            "unrecognized packet receipt: {:?}",
            receipt
        ))
    }
}

/// get packet receipt by port_id, channel_id and sequence
pub async fn get_packet_receipt_vec(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<MyConfig>,
) -> Result<Vec<u8>> {
    tracing::info!("in call_ibc : [get_packet_receipt]");
    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let packet_receipt_path = ReceiptsPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_receipt(&packet_receipt_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(anyhow::anyhow!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id,
            channel_id
        ));
    }

    Ok(data)
}

/// get  unreceipt packet
pub async fn get_unreceipt_packet(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequences: Vec<Sequence>,
    client: Client<MyConfig>,
) -> Result<Vec<u64>> {
    tracing::info!("in call_ibc: [get_receipt_packet]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let mut result = Vec::new();

    let pair = sequences
        .into_iter()
        .map(|sequence| (port_id.clone(), channel_id.clone(), sequence.clone()));

    for (port_id, channel_id, sequence) in pair {
        let packet_receipt_path = ReceiptsPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence: sequence.clone(),
        }
        .to_string()
        .as_bytes()
        .to_vec();

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .packet_receipt(&packet_receipt_path, Some(block_hash))
            .await?;
        if data.is_empty() {
            result.push(u64::from(sequence));
        }
    }

    Ok(result)
}

/// get get_commitment_packet_state
pub async fn get_commitment_packet_state(client: Client<MyConfig>) -> Result<Vec<PacketState>> {
    tracing::info!("in call_ibc: [get_commitment_packet_state]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

    let mut iter = storage
        .iter::<ibc_node::ibc::storage::PacketCommitment>(Some(block_hash))
        .await?;
    println!("in call_ibc: [get_commitment_packet_state] >> iter");

    let mut result = vec![];

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        println!("in call_ibc: [get_commitment_packet_state] >> while");
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)?;
        let raw_path = String::from_utf8(raw_key)?;
        // decode key
        let path = Path::from_str(&raw_path).map_err(|_| anyhow::anyhow!("decode path error"))?;
        println!("[get_commitment_packet_state] >> Path: {:?}", path);
        println!("  Value: {:?}", value);
        match path {
            Path::Commitments(commitments) => {
                let CommitmentsPath { port_id, channel_id, sequence }= commitments;

                let packet_state = PacketState {
                    port_id: port_id.to_string(),
                    channel_id: channel_id.to_string(),
                    sequence: u64::from(sequence),
                    data: value,
                };
                result.push(packet_state);
            },
            _=> unimplemented!(),
        }

    }


    Ok(result)
}

/// get packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
pub async fn get_packet_commitment(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<MyConfig>,
) -> Result<Vec<u8>> {
    tracing::info!("in call_ibc: [get_packet_commitment]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let packet_commits_path = CommitmentsPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_commitment(&packet_commits_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        Err(anyhow::anyhow!(
            "get_packet_commitment is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id,
            channel_id,
            sequence
        ))
    } else {
        Ok(data)
    }
}

/// get packet acknowlegement by port_id, channel_id and sequence to verify if the packet has been received by the target chain
pub async fn get_packet_ack(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: Client<MyConfig>,
) -> Result<Vec<u8>> {
    tracing::info!("in call_ibc: [get_packet_ack]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let acks_path = AcksPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .acknowledgements(&acks_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        Err(anyhow::anyhow!(
            "get_packet_ack is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id,
            channel_id,
            sequence
        ))
    } else {
        Ok(data)
    }
}

/// get packet receipt by port_id, channel_id and sequence
pub async fn get_next_sequence_recv(
    port_id: &PortId,
    channel_id: &ChannelId,
    client: Client<MyConfig>,
) -> Result<Vec<u8>> {
    tracing::info!("in call_ibc: [get_next_sequence_recv]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let seq_recvs_path = SeqRecvsPath(port_id.clone(), channel_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    let sequence: u64 = api
        .storage()
        .ibc()
        .next_sequence_recv(&seq_recvs_path, Some(block_hash))
        .await?;

    let packet_commits_path = CommitmentsPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: Sequence::from(sequence),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_commitment(&packet_commits_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        Err(anyhow::anyhow!(
            "get_next_sequence_recv is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        ))
    } else {
        Ok(data)
    }
}

/// get get_commitment_packet_state
pub async fn get_acknowledge_packet_state(client: Client<MyConfig>) -> Result<Vec<PacketState>> {
    tracing::info!("in call_ibc: [get_acknowledge_packet_state]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // Obtain the storage client wrapper from the API.
    let storage: StorageClient<_> = api.client.storage();

    let mut iter = storage
        .iter::<ibc_node::ibc::storage::Acknowledgements>(Some(block_hash))
        .await?;
    println!("in call_ibc: [get_acknowledge_packet_state] >> iter");

    let mut result = vec![];

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        println!("in call_ibc: [get_acknowledge_packet_state] >> while");
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)?;
        let raw_path = String::from_utf8(raw_key)?;
        // decode key
        let path = Path::from_str(&raw_path).map_err(|_| anyhow::anyhow!("decode path error"))?;
        println!("[get_acknowledge_packet_state] >> Path: {:?}", path);
        println!("  Value: {:?}", value);
        match path {
            Path::Acks(acks_path) => {
                let AcksPath { port_id, channel_id, sequence }= acks_path;

                let packet_state = PacketState {
                    port_id: port_id.to_string(),
                    channel_id: channel_id.to_string(),
                    sequence: u64::from(sequence),
                    data: value,
                };
                result.push(packet_state);
            },
            _=> unimplemented!(),
        }

    }


    Ok(result)
}
