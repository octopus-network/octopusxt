use crate::ibc_core::{OctopusxtClient, PacketRpc};
use crate::ChannelRpc;
use ibc::core::{
    ics04_channel::{
        channel::{ChannelEnd, IdentifiedChannelEnd},
        packet::{Packet, Receipt, Sequence},
    },
    ics24_host::identifier::{ChannelId, PortId},
};
use tendermint_proto::Protobuf;

use crate::primitive::QueryChannelsResponse;
use async_trait::async_trait;
use codec::Decode;
use core::str::FromStr;
use ibc::core::ics24_host::identifier::ConnectionId;
use ibc::Height;
use ibc_proto::ibc::core::channel::v1::PacketState;
use sp_core::H256;

#[async_trait]
impl ChannelRpc for OctopusxtClient {
    type Error = anyhow::Error;

    async fn query_channels(&self) -> Result<Vec<IdentifiedChannelEnd>, Self::Error> {
        tracing::info!("in call_ibc: [get_channels]");

        let api = self.to_runtime_api();

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

    async fn query_channel_end(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<ChannelEnd, Self::Error> {
        tracing::info!("in call_ibc: [get_channel_end]");

        let api = self.to_runtime_api();

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

    async fn query_packet_receipt(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Receipt, Self::Error> {
        tracing::info!("in call_ibc : [get_packet_receipt]");

        let packet_receipt_vec = self
            .query_packet_receipt_vec(port_id, channel_id, sequence)
            .await?;
        let data = String::decode(&mut packet_receipt_vec.as_slice()).unwrap();
        if data.eq("Ok") {
            Ok(Receipt::Ok)
        } else {
            Err(anyhow::anyhow!("unrecognized packet receipt: {:?}", data))
        }
    }

    async fn query_packet_receipt_vec(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error> {
        tracing::info!("in call_ibc : [get_packet_receipt]");

        let api = self.to_runtime_api();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

        let sequence = u64::from(sequence);

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

    async fn query_unreceipt_packet(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequences: Vec<Sequence>,
    ) -> Result<Vec<Sequence>, Self::Error> {
        tracing::info!("in call_ibc: [get_receipt_packet]");

        let api = self.to_runtime_api();
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
                result.push(Sequence::from(sequence));
            }
        }

        Ok(result)
    }

    async fn query_commitment_packet_state(&self) -> Result<Vec<PacketState>, Self::Error> {
        tracing::info!("in call_ibc: [get_commitment_packet_state]");

        let api = self.to_runtime_api();

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

    async fn query_packet_commitment(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error> {
        tracing::info!("in call_ibc: [get_packet_commitment]");

        let api = self.to_runtime_api();

        // let result = async move {
        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .packet_commitment(
                port_id.as_bytes(),
                format!("{}", channel_id).as_bytes(),
                &u64::from(sequence),
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

    async fn query_packet_acknowledgements(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error> {
        tracing::info!("in call_ibc: [get_packet_ack]");

        let api = self.to_runtime_api();

        // let result = async move {
        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();

        let block_hash: H256 = block_header.hash();

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .acknowledgements(
                port_id.as_bytes(),
                format!("{}", channel_id).as_bytes(),
                &u64::from(sequence),
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

    async fn query_next_sequence_recv(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Vec<u8>, Self::Error> {
        tracing::info!("in call_ibc: [get_next_sequence_recv]");

        let api = self.to_runtime_api();

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

    async fn query_acknowledge_packet_state(&self) -> Result<Vec<PacketState>, Self::Error> {
        tracing::info!("in call_ibc: [get_acknowledge_packet_state]");

        let api = self.to_runtime_api();

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

    // fn query_connection_channels(
    //     &self,
    //     _height: Height,
    //     _connection_id: ConnectionId,
    // ) -> Result<QueryChannelsResponse, Self::Error> {
    //     todo!()
    // }

    fn query_unreceived_packets(
        &self,
        _height: Height,
        _channel_id: ChannelId,
        _port_id: PortId,
        _seqs: Vec<Sequence>,
    ) -> Result<Vec<Sequence>, Self::Error> {
        todo!()
    }

    fn query_unreceived_acknowledgements(
        &self,
        _height: Height,
        _channel_id: ChannelId,
        _port_id: PortId,
        _seqs: Vec<Sequence>,
    ) -> Result<Vec<Sequence>, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl PacketRpc for OctopusxtClient {
    type Error = anyhow::Error;

    async fn query_send_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Packet, Self::Error> {
        tracing::info!("in call_ibc: [get_send_packet_event]");

        let api = self.to_runtime_api();

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
    }

    async fn query_write_ack_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error> {
        tracing::info!("in call_ibc: [get_write_ack_packet_event]");

        let api = self.to_runtime_api();

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
    }

    async fn query_packets(
        &self,
        _channel_id: ChannelId,
        _port_id: PortId,
        _seqs: Vec<Sequence>,
    ) -> Result<Vec<Packet>, Self::Error> {
        todo!()
    }
}
