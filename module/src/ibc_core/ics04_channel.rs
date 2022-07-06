use crate::ibc_core::{OctopusxtClient, PacketRpc};
use crate::ChannelRpc;
use ibc::core::{
    ics04_channel::{
        channel::{ChannelEnd, IdentifiedChannelEnd},
        packet::{Packet, Sequence},
    },
    ics24_host::identifier::{ChannelId, PortId},
};
use tendermint_proto::Protobuf;

// use crate::primitive::QueryChannelsResponse;
use async_trait::async_trait;

use core::str::FromStr;
use ibc::core::ics02_client::client_state::IdentifiedAnyClientState;
use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::Height as ICSHeight;
// use ibc::core::ics24_host::identifier::ConnectionId;

use ibc_relayer::chain::requests::{
    IncludeProof, QueryChannelClientStateRequest, QueryChannelRequest, QueryChannelsRequest,
    QueryHeight, QueryNextSequenceReceiveRequest, QueryPacketAcknowledgementRequest,
    QueryPacketAcknowledgementsRequest, QueryPacketCommitmentRequest,
    QueryPacketCommitmentsRequest, QueryPacketReceiptRequest, QueryUnreceivedAcksRequest,
    QueryUnreceivedPacketsRequest,
};

#[async_trait]
impl ChannelRpc for OctopusxtClient {
    type Error = anyhow::Error;

    async fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc: [get_channel_end]");

        let QueryChannelRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .channels(
                port_id.as_bytes(),
                channel_id.to_string().as_bytes(),
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

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((channel_end, None)),
        }
    }

    async fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Self::Error> {
        tracing::info!("in call_ibc: [get_channels]");

        let QueryChannelsRequest {
            pagination: _pagination,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
            .await?;

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

    fn query_channel_client_state(
        &self,
        _request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Self::Error> {
        todo!()
    }

    async fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc : [get_packet_receipt]");

        let QueryPacketReceiptRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let sequence = u64::from(sequence);

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .packet_receipt(
                port_id.as_bytes(),
                channel_id.to_string().as_bytes(),
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

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((data, None)),
        }
    }

    async fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc: [get_packet_commitment]");

        let QueryPacketCommitmentRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .packet_commitment(
                port_id.as_bytes(),
                channel_id.to_string().as_bytes(),
                &u64::from(sequence),
                Some(block_hash),
            )
            .await?;

        if data.is_empty() {
            return Err(anyhow::anyhow!(
            "get_packet_commitment is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id,
            channel_id,
            sequence
        ));
        }

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((data, None)),
        }
    }

    async fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Self::Error> {
        tracing::info!("in call_ibc: [get_commitment_packet_state]");
        let QueryPacketCommitmentsRequest {
            port_id,
            channel_id,
            pagination: _pagination,
        } = request;

        let api = self.to_runtime_api();

        let height = self.query_latest_height().await?;

        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
            .await?;

        let packet_commitments_keys: Vec<(Vec<u8>, Vec<u8>, u64)> = api
            .storage()
            .ibc()
            .packet_commitment_keys(Some(block_hash))
            .await?;

        let mut sequences = vec![];

        for (port_id_tmp, channel_id_tmp, sequence) in packet_commitments_keys.into_iter() {
            if port_id_tmp == port_id.as_bytes()
                && channel_id_tmp == channel_id.to_string().as_bytes()
            {
                sequences.push(Sequence::from(sequence));
            }
        }

        // TODO height revision number not is zero.
        Ok((sequences, ICSHeight::new(0, height)))
    }

    async fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc: [get_packet_ack]");

        let QueryPacketAcknowledgementRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let data: Vec<u8> = api
            .storage()
            .ibc()
            .acknowledgements(
                port_id.as_bytes(),
                channel_id.to_string().as_bytes(),
                &u64::from(sequence),
                Some(block_hash),
            )
            .await?;

        if data.is_empty() {
            return Err(anyhow::anyhow!(
                "get_packet_ack is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
                port_id,
                channel_id,
                sequence
            ));
        }

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((data, None)),
        }
    }

    async fn query_packet_acknowledgements(
        &self,
        _request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Self::Error> {
        todo!()
    }

    async fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Self::Error> {
        tracing::info!("in call_ibc: [get_next_sequence_recv]");

        let QueryNextSequenceReceiveRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

        let sequence: u64 = api
            .storage()
            .ibc()
            .next_sequence_recv(
                port_id.as_bytes(),
                channel_id.to_string().as_bytes(),
                Some(block_hash),
            )
            .await?;

        match include_proof {
            IncludeProof::Yes => todo!(),
            IncludeProof::No => Ok((Sequence::from(sequence), None)),
        }
    }

    async fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, Self::Error> {
        tracing::info!("in call_ibc: [get_receipt_packet]");

        let QueryUnreceivedPacketsRequest {
            port_id,
            channel_id,
            packet_commitment_sequences,
        } = request;

        let api = self.to_runtime_api();

        let block_hash = self
            .query_block_hash_by_query_height(QueryHeight::Latest)
            .await?;

        let mut result = Vec::new();

        let pair = packet_commitment_sequences.into_iter().map(|sequence| {
            (
                port_id.clone().as_bytes().to_vec(),
                channel_id.to_string().as_bytes().to_vec(),
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

    async fn query_unreceived_acknowledgements(
        &self,
        _request: QueryUnreceivedAcksRequest,
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
        height: QueryHeight,
    ) -> Result<Packet, Self::Error> {
        tracing::info!("in call_ibc: [get_send_packet_event]");

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

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
        height: QueryHeight,
    ) -> Result<Vec<u8>, Self::Error> {
        tracing::info!("in call_ibc: [get_write_ack_packet_event]");

        let api = self.to_runtime_api();

        let block_hash = self.query_block_hash_by_query_height(height).await?;

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
        _height: QueryHeight,
    ) -> Result<Vec<Packet>, Self::Error> {
        todo!()
    }
}
