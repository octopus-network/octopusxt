use crate::ibc_core::{OctopusxtClient, PacketRpc};
use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::{
        channel::{ChannelEnd, IdentifiedChannelEnd},
        packet::{Packet, Receipt, Sequence},
    },
    ics24_host::identifier::{ChannelId, PortId},
};
use subxt::Client;
use tendermint_proto::Protobuf;

use codec::Decode;
use core::str::FromStr;
use ibc_proto::ibc::core::channel::v1::PacketState;
use std::future::Future;

use anyhow::Result;
use sp_core::H256;

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
///
/// # Usage example
///
///
/// ```rust
/// use subxt::ClientBuilder;
/// use octopusxt::get_channels;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let result = get_channels(client).await?;
/// ```
///
pub async fn get_channels(client: Client<MyConfig>) -> Result<Vec<IdentifiedChannelEnd>> {
    tracing::info!("in call_ibc: [get_channels]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // vector key-value
    let mut ret = vec![];

    let channels_keys: Vec<(Vec<u8>, Vec<u8>)> =
        api.storage().ibc().channels_keys(Some(block_hash)).await?;

    if channels_keys.is_empty() {
        return Err(anyhow::anyhow!("get_channels: get empty channels_keys",));
    }

    for key in channels_keys {
        // get value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .channels(&key.0, &key.1, Some(block_hash))
            .await?;

        // store key-value
        ret.push((key.0.clone(), key.1.clone(), value));
    }
    let mut result = vec![];

    for (port_id, channel_id, channel_end) in ret.iter() {
        let port_id_str = String::from_utf8(port_id.clone()).unwrap();
        let port_id = PortId::from_str(port_id_str.as_str()).unwrap();

        let channel_id_str = String::from_utf8(channel_id.clone()).unwrap();
        let channel_id = ChannelId::from_str(channel_id_str.as_str()).unwrap();

        let channel_end = ChannelEnd::decode_vec(channel_end).unwrap();

        result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
    }

    Ok(result)
}

/// get channelEnd according by port_identifier, channel_identifier and read Channles StorageMaps
///
/// # Usage example
///
/// ```rust
/// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
/// use octopusxt::MyConfig;
/// use subxt::ClientBuilder;
/// use octopusxt::get_channel_end;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let prot_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let result = get_channel_end(&port_id, &channel_id, client).await?;
/// ```
///
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

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .channels(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            Some(block_hash),
        )
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
///
/// # Usage example
///
/// ```rust
/// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
/// use octopusxt::MyConfig;
/// use subxt::ClientBuilder;
/// use octopusxt::get_packet_receipt;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let prot_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(0);
/// let result = get_packet_receipt(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
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

    let sequence = u64::from(*sequence);

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_receipt(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            &sequence,
            Some(block_hash),
        )
        .await?;

    if data.is_empty() {
        return Err(anyhow::anyhow!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id,
            channel_id
        ));
    }

    let _data = String::decode(&mut data.as_slice()).unwrap();
    if _data.eq("Ok") {
        Ok(Receipt::Ok)
    } else {
        Err(anyhow::anyhow!("unrecognized packet receipt: {:?}", _data))
    }
}

/// get packet receipt by port_id, channel_id and sequence
/// # Usage example
///
/// ```rust
/// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
/// use octopusxt::MyConfig;
/// use subxt::ClientBuilder;
/// use octopusxt::get_packet_receipt_vec;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let prot_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(0);
/// let result = get_packet_receipt_vec(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
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

    let sequence = u64::from(*sequence);

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_receipt(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            &sequence,
            Some(block_hash),
        )
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
///  # Usage example
///
/// ```rust
/// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
/// use octopusxt::MyConfig;
/// use subxt::ClientBuilder;
/// use octopusxt::get_unreceipt_packet;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let port_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = vec![Sequence::from(12),Sequence::from(13)];
/// let result = get_unreceipt_packet(&port_id, &channel_id, sequence, client).await?;
/// ```
///
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

    let pair = sequences.into_iter().map(|sequence| {
        (
            port_id.clone().as_bytes().to_vec(),
            format!("{}", channel_id).as_bytes().to_vec(),
            u64::from(sequence),
        )
    });

    for (port_id, channel_id, sequence) in pair {
        let data: Vec<u8> = api
            .storage()
            .ibc()
            .packet_receipt(&port_id, &channel_id, &sequence, Some(block_hash))
            .await?;
        if data.is_empty() {
            result.push(sequence);
        }
    }

    Ok(result)
}

/// get get_commitment_packet_state
///
/// # Usage example
///
/// ```rust
/// use octopusxt::MyConfig;
/// use subxt::ClientBuilder;
/// use octopusxt::get_commitment_packet_state;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let result = get_commitment_packet_state(client).await?;
/// ```
///
pub async fn get_commitment_packet_state(client: Client<MyConfig>) -> Result<Vec<PacketState>> {
    tracing::info!("in call_ibc: [get_commitment_packet_state]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let mut ret = vec![];

    let packet_commitments_keys: Vec<(Vec<u8>, Vec<u8>, u64)> = api
        .storage()
        .ibc()
        .packet_commitment_keys(Some(block_hash))
        .await?;

    for key in packet_commitments_keys {
        // get value
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .packet_commitment(&key.0, &key.1, &key.2, Some(block_hash))
            .await?;

        // store key-value
        ret.push((key.0.clone(), key.1.clone(), key.2, value));
    }

    let mut result = vec![];

    for (port_id, channel_id, sequence, data) in ret.into_iter() {
        let port_id = String::from_utf8(port_id).unwrap();
        let channel_id = String::from_utf8(channel_id).unwrap();

        let packet_state = PacketState {
            port_id,
            channel_id,
            sequence,
            data,
        };
        result.push(packet_state);
    }

    Ok(result)
}

/// get packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
///
///  # Usage example
///
/// ```rust
/// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
/// use octopusxt::MyConfig;
/// use subxt::ClientBuilder;
/// use octopusxt::get_packet_commitment;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let port_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(23);
/// let result = get_packet_commitment(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
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

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_commitment(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            &u64::from(*sequence),
            Some(block_hash),
        )
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
///
///  # Usage example
///
/// ```rust
/// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
/// use octopusxt::MyConfig;
/// use subxt::ClientBuilder;
/// use octopusxt::get_packet_ack;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let port_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let sequence = Sequence::from(12);
/// let result = get_packet_ack(&port_id, &channel_id, &sequence, client).await?;
/// ```
///
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

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .acknowledgements(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            &u64::from(*sequence),
            Some(block_hash),
        )
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
///  # Usage example
///
/// ```rust
/// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
/// use subxt::ClientBuilder;
/// use octopusxt::get_next_sequence_recv;
/// use octopusxt::MyConfig;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let prot_id = PortId::default();
/// let channel_id = ChannelId::default();
/// let result = get_next_sequence_recv(&prot_id, &channel_id, client).await?;
/// ```
///
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

    let sequence: u64 = api
        .storage()
        .ibc()
        .next_sequence_recv(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            Some(block_hash),
        )
        .await?;

    let data: Vec<u8> = api
        .storage()
        .ibc()
        .packet_commitment(
            port_id.as_bytes(),
            format!("{}", channel_id).as_bytes(),
            &sequence,
            Some(block_hash),
        )
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
///
/// # Usage example
///
/// ```rust
/// use subxt::ClientBuilder;
/// use octopusxt::get_acknowledge_packet_state;
/// use octopusxt::MyConfig;
///
/// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
/// let result = get_acknowledge_packet_state(client).await?;
/// ```
///
pub async fn get_acknowledge_packet_state(client: Client<MyConfig>) -> Result<Vec<PacketState>> {
    tracing::info!("in call_ibc: [get_acknowledge_packet_state]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let mut ret = vec![];

    let acknowledgements_keys: Vec<(Vec<u8>, Vec<u8>, u64)> = api
        .storage()
        .ibc()
        .acknowledgements_keys(Some(block_hash))
        .await?;

    for key in acknowledgements_keys {
        let value: Vec<u8> = api
            .storage()
            .ibc()
            .acknowledgements(&key.0, &key.1, &key.2, Some(block_hash))
            .await?;

        ret.push((key.0.clone(), key.1.clone(), key.2, value));
    }

    let mut result = vec![];

    for (port_id, channel_id, sequence, data) in ret.into_iter() {
        let port_id = String::from_utf8(port_id).unwrap();
        let channel_id = String::from_utf8(channel_id).unwrap();

        let packet_state = PacketState {
            port_id,
            channel_id,
            sequence,
            data,
        };
        result.push(packet_state);
    }

    Ok(result)
}

impl PacketRpc for OctopusxtClient {
    /// get send packet event by port_id, channel_id and sequence
    /// (port_id, channel_id, sequence), packet)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use subxt::MyConfig;
    /// use octopusxt::get_send_packet_event;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id =PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_send_packet_event(&port_id, &channel_id, &sequence, client).await?;
    ///
    /// ```
    fn get_send_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Box<dyn Future<Output = Result<Packet>>> {
        tracing::info!("in call_ibc: [get_send_packet_event]");

        let api = self.client.clone()
            .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

        let result = async move {
            let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

            let block_header = block.next().await.unwrap().unwrap();

            let block_hash: H256 = block_header.hash();

            let data: Vec<u8> = api
                .storage()
                .ibc()
                .send_packet_event(
                    port_id.as_bytes(),
                    format!("{}", channel_id).as_bytes(),
                    &u64::from(sequence),
                    Some(block_hash),
                )
                .await?;

            if data.is_empty() {
                return Err(anyhow::anyhow!(
            "get_send_packet_event is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id,
            channel_id,
            sequence
        ));
            }

            let packet = Packet::decode_vec(&*data).unwrap();
            Ok(packet)
        };

        Box::new(result)
    }

    /// (port_id, channel_id, sequence), ackHash)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use subxt::MyConfig;
    /// use octopusxt::get_write_ack_packet_event;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_write_ack_packet_event(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    ///
    fn get_write_ack_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Box<dyn Future<Output = Result<Vec<u8>>>> {
        tracing::info!("in call_ibc: [get_write_ack_packet_event]");
        let api = self.client.clone()
            .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

        let result = async move {
            let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

            let block_header = block.next().await.unwrap().unwrap();

            let block_hash: H256 = block_header.hash();

            let data: Vec<u8> = api
                .storage()
                .ibc()
                .write_ack_packet_event(
                    port_id.as_bytes(),
                    format!("{}", channel_id).as_bytes(),
                    &u64::from(sequence),
                    Some(block_hash),
                )
                .await?;

            if data.is_empty() {
                return Err(anyhow::anyhow!(
            "get_write_ack_packet_event is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        ));
            }

            Ok(data)
        };

        Box::new(result)
    }
}
