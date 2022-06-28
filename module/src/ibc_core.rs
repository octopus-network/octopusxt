use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::packet::{Packet, Sequence},
    ics24_host::identifier::{ChannelId, PortId},
};

use subxt::{rpc::ClientT, BlockNumber, Client, SignedCommitment};

use async_trait::async_trait;
use ibc::core::ics02_client::client_consensus::AnyConsensusState;
use ibc::core::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::core::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd};
use ibc::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::core::ics04_channel::packet::Receipt;
use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc::Height as ICSHeight;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::PacketState;
use jsonrpsee::rpc_params;
use sp_core::H256;

pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics26_router;

pub use crate::events::*;
pub use ics02_client::*;
pub use ics03_connection::*;
pub use ics04_channel::*;

#[async_trait]
pub trait ClientRpc {
    type Error;

    /// get client_state according by client_id, and read ClientStates StoraageMap
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use octopusxt::get_client_state;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let client_id = ClientId::default();
    /// let result = get_client_state(&client_id, client).await?;
    /// ```
    async fn get_client_state(&self, client_id: ClientId) -> Result<AnyClientState, Self::Error>;

    /// get appoint height consensus_state according by client_identifier and height
    /// and read ConsensusStates StoreageMap
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use octopusxt::get_client_consensus;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let client_id = ClientId::default();
    /// let height = ICSHeight::default();
    /// let result = get_client_consensus(&client_id, &height, client).await?;
    /// ```
    async fn get_client_consensus(
        &self,
        client_id: ClientId,
        height: ICSHeight,
    ) -> Result<AnyConsensusState, Self::Error>;

    /// get consensus state with height
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use octopusxt::get_consensus_state_with_height;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let client_id = ClientId::default();
    /// let result = get_consensus_state_with_height(&client_id, client).await?;
    /// ```
    async fn get_consensus_state_with_height(
        &self,
        client_id: ClientId,
    ) -> Result<Vec<(ICSHeight, AnyConsensusState)>, Self::Error>;

    /// get key-value pair (client_identifier, client_state) construct IdentifieredAnyClientstate
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_clients;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_clients(client).await?;
    /// ```
    async fn get_clients(&self) -> Result<Vec<IdentifiedAnyClientState>, Self::Error>;

    /// get connection_identifier vector according by client_identifier
    ///
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use octopusxt::get_client_connections;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let client_id = ClientId::default();
    /// let result = get_client_connections(&client_id, client).await?;
    /// ```
    async fn get_client_connections(
        &self,
        client_id: ClientId,
    ) -> Result<Vec<ConnectionId>, Self::Error>;
}

#[async_trait]
pub trait ChannelRpc {
    type Error;
    /// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    ///
    /// # Usage example
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::get_channels;
    /// use octopusxt::MyConfig;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_channels(client).await?;
    /// ```
    async fn get_channels(&self) -> Result<Vec<IdentifiedChannelEnd>, Self::Error>;

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
    async fn get_channel_end(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<ChannelEnd, Self::Error>;

    /// get packet receipt by port_id, channel_id and sequence
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
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
    async fn get_packet_receipt(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Receipt, Self::Error>;

    /// get packet receipt by port_id, channel_id and sequence
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
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
    async fn get_packet_receipt_vec(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error>;

    /// get  unreceipt packet
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
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
    async fn get_unreceipt_packet(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequences: Vec<Sequence>,
    ) -> Result<Vec<u64>, Self::Error>;

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
    async fn get_commitment_packet_state(&self) -> Result<Vec<PacketState>, Self::Error>;

    /// get packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
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
    async fn get_packet_commitment(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error>;

    /// get packet acknowledgement by port_id, channel_id and sequence to verify if the packet has been received by the target chain
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
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
    async fn get_packet_ack(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error>;

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
    async fn get_next_sequence_recv(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Vec<u8>, Self::Error>;

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
    async fn get_acknowledge_packet_state(&self) -> Result<Vec<PacketState>, Self::Error>;
}

#[async_trait]
pub trait ConnectionRpc {
    type Error;
    /// get connectionEnd according by connection_identifier and read Connections StorageMaps
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use octopusxt::get_connection_end;
    ///
    /// let api = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let conection_id = ConnectionId::default();
    /// let result = get_connection_end(&conection_id, api).await?;
    /// ```
    async fn get_connection_end(
        &self,
        connection_identifier: ConnectionId,
    ) -> Result<ConnectionEnd, Self::Error>;

    /// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_connections;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_connections(client).await?;
    /// ```
    async fn get_connections(&self) -> Result<Vec<IdentifiedConnectionEnd>, Self::Error>;

    ///  # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use octopusxt::get_connection_channels;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let connection = ConnectionId::default();
    /// let result = get_connection_channels(&connection, client).await?;
    /// ```
    async fn get_connection_channels(
        &self,
        connection_id: ConnectionId,
    ) -> Result<Vec<IdentifiedChannelEnd>, Self::Error>;
}

#[async_trait]
pub trait PacketRpc {
    type Error;
    /// get send packet event by port_id, channel_id and sequence
    /// (port_id, channel_id, sequence), packet)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{get_send_packet_event, MyConfig};
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id =PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_send_packet_event(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_send_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Packet, Self::Error>;

    /// (port_id, channel_id, sequence), ackHash)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{get_write_ack_packet_event, MyConfig};
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = get_write_ack_packet_event(&port_id, &channel_id, &sequence, client).await?;
    /// ```
    async fn get_write_ack_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error>;
}

#[async_trait]
pub trait Router {
    type Error;
    /// ibc protocol core function, ics26 deliver function
    /// this function will dispatch msg to process
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics26_routing::handler::deliver;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{MyConfig, OctopusxtClient, Router};
    /// use ibc_proto::google::protobuf::Any;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let msg = vec![Any::default()];
    /// let octopus_client = OctopusxtClient::new(client);
    /// let result = octopus_client.deliver(msg).await?;
    /// ```
    /// return block_hash, extrinsic_hash, and event
    async fn deliver(&self, msg: Vec<Any>) -> Result<H256, Self::Error>;
}

#[async_trait]
pub trait OctopusxtRpc: ClientRpc + ChannelRpc + ConnectionRpc + PacketRpc {}

#[derive(Debug)]
pub struct OctopusxtClient {
    client: Client<MyConfig>,
}

#[async_trait]
impl OctopusxtRpc for OctopusxtClient {}

impl OctopusxtClient {
    pub fn new(client: Client<MyConfig>) -> Self {
        Self { client }
    }

    /// Subscribe beefy justifiactions
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::subscribe_beefy;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = subscribe_beefy(client).await?;
    /// ```
    pub async fn subscribe_beefy(&self) -> anyhow::Result<SignedCommitment> {
        tracing::info!("In call_ibc: [subscribe_beefy_justifications]");

        let api = self.to_runtime_api();

        let mut sub = api.client.rpc().subscribe_beefy_justifications().await?;

        let raw = sub.next().await.unwrap().unwrap();

        Ok(raw)
    }

    /// get latest height used by subscribe_blocks
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_latest_height;
    ///
    /// let api = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let result = get_latest_height(api).await?;
    /// ```
    ///
    pub async fn get_latest_height(&self) -> anyhow::Result<u64> {
        tracing::info!("In call_ibc: [get_latest_height]");

        let api = self.to_runtime_api();

        let mut blocks = api.client.rpc().subscribe_finalized_blocks().await?;

        let height = match blocks.next().await {
            Some(Ok(header)) => header.number as u64,
            Some(Err(_)) => 0,
            None => 0,
        };

        Ok(height)
    }

    pub fn to_runtime_api(
        &self,
    ) -> ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>> {
        self.client.clone()
            .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>()
    }

    /// # get_mmr_leaf_and_mmr_proof
    ///
    /// This get_mmr_leaf_and_mmr_proof api generate form generateProof api
    /// generateProof(leafIndex: u64, at?: BlockHash): MmrLeafProof
    /// interface: api.rpc.mmr.generateProof
    /// json_rpc: mmr_generateProof
    /// summary: Generate MMR proof for given leaf index.
    ///
    /// Return value a tuple (mmr_leaf, mmr_proof)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::{ClientBuilder, BlockNumber};
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_mmr_leaf_and_mmr_proof;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let block_number = Some(BlockNumber::from(12));
    /// let block_hash = None;
    /// let result = get_mmr_leaf_and_mmr_proof(block_number, block_hash, client).await?;
    /// ```
    pub async fn get_mmr_leaf_and_mmr_proof(
        &self,
        block_number: Option<BlockNumber>,
        block_hash: Option<H256>,
    ) -> anyhow::Result<(String, Vec<u8>, Vec<u8>)> {
        tracing::info!("in call_ibc [get_mmr_leaf_and_mmr_proof]");

        let api = self.to_runtime_api();

        let params = rpc_params![block_number, block_hash];

        let generate_proof: pallet_mmr_rpc::LeafProof<String> = api
            .client
            .rpc()
            .client
            .request("mmr_generateProof", params)
            .await?;

        Ok((
            generate_proof.block_hash,
            generate_proof.leaf.0,
            generate_proof.proof.0,
        ))
    }

    /// get header by block hash
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_header_by_block_hash;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let block_hash = None;
    /// let result = get_header_by_block_hash(block_hash, client).await?;
    /// ```
    ///
    pub async fn get_header_by_block_hash(
        &self,
        block_hash: Option<H256>,
    ) -> anyhow::Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
        let api = self.to_runtime_api();

        let header = api.client.rpc().header(block_hash).await?.unwrap();

        let header = crate::utils::convert_substrate_header_to_ibc_header(header);

        Ok(header.into())
    }

    /// get header by block number
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::{ClientBuilder, BlockNumber};
    /// use octopusxt::MyConfig;
    /// use octopusxt::get_header_by_block_number;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let block_number = Some(BlockNumber::from(2));
    /// let result = get_header_by_block_number(block_number, client).await?;
    /// ```
    ///
    pub async fn get_header_by_block_number(
        &self,
        block_number: Option<BlockNumber>,
    ) -> anyhow::Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
        let api = self.to_runtime_api();

        let block_hash = api.client.rpc().block_hash(block_number).await?;

        let header = api.client.rpc().header(block_hash).await?.unwrap();

        let header = crate::utils::convert_substrate_header_to_ibc_header(header);

        Ok(header.into())
    }
}
