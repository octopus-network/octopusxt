use crate::{ibc_node, MyConfig, SubstrateNodeTemplateExtrinsicParams};
use ibc::core::{
    ics04_channel::packet::{Packet, Sequence},
    ics24_host::identifier::{ChannelId, PortId},
};

use subxt::{rpc::ClientT, BlockNumber, Client, SignedCommitment};

use async_trait::async_trait;
use beefy_merkle_tree::Hash;
use ibc::core::ics02_client::client_consensus::AnyConsensusState;
use ibc::core::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::core::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd};
use ibc::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::core::ics04_channel::packet::Receipt;
use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc::{Height as ICSHeight, Height};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::PacketState;
use jsonrpsee::rpc_params;
use serde::{Deserialize, Serialize};
use sp_core::H256;

pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics26_router;

pub use crate::events::*;
use crate::primitive::{
    IdentifiedClientState, IdentifiedConnection, QueryChannelsResponse, QueryClientStateResponse,
    QueryConsensusStateResponse, QueryDenomTraceResponse, QueryDenomTracesResponse,
};
pub use ics02_client::*;
pub use ics03_connection::*;
pub use ics04_channel::*;

pub enum QueryHeight {
    Latest,
    Specific(Height),
}

#[async_trait]
pub trait ClientRpc {
    type Error;

    /// Query a client state
    /// query client_state according by client_id, and read ClientStates StorageMap
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let client_id = ClientId::default();
    /// let result = octopusxt_client.query_client_state(client_id).await?;
    /// ```
    async fn query_client_state(&self, client_id: ClientId) -> Result<AnyClientState, Self::Error>;

    /// query appoint height consensus_state according by client_identifier and height
    /// and read ConsensusStates StorageMap
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let client_id = ClientId::default();
    /// let height = ICSHeight::default();
    /// let result = octopusxt_client.query_client_consensus_state(client_id, height).await?;
    /// ```
    async fn query_client_consensus_state(
        &self,
        client_id: ClientId,
        height: ICSHeight,
    ) -> Result<AnyConsensusState, Self::Error>;

    /// query consensus state with height
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let client_id = ClientId::default();
    /// let result = octopusxt_client.query_consensus_state_with_height(client_i).await?;
    /// ```
    async fn query_consensus_state_with_height(
        &self,
        client_id: ClientId,
    ) -> Result<Vec<(ICSHeight, AnyConsensusState)>, Self::Error>;

    /// query key-value pair (client_identifier, client_state) construct IdentifieredAnyClientstate
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.query_clients().await?;
    /// ```
    async fn query_clients(&self) -> Result<Vec<IdentifiedAnyClientState>, Self::Error>;

    /// get connection_identifier vector according by client_identifier
    ///
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let client_id = ClientId::default();
    /// let result = octopusxt_client.query_client_connections(client_id).await?;
    /// ```
    async fn query_client_connections(
        &self,
        client_id: ClientId,
    ) -> Result<Vec<ConnectionId>, Self::Error>;

    /// Query local chain consensus state
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use ibc::Height;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let height = Height::new(0, 12);
    /// let result = octopusxt_client.query_consensus_state(height).await?;
    /// ```
    fn query_consensus_state(
        &self,
        height: Height,
    ) -> Result<QueryConsensusStateResponse, Self::Error>;

    /// Query upgraded client state
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use ibc::Height;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let height = Height::new(0, 12);
    /// let result = octopusxt_client.query_upgraded_client(height).await?;
    /// ```
    fn query_upgraded_client(
        &self,
        height: Height,
    ) -> Result<QueryClientStateResponse, Self::Error>;

    /// Query upgraded consensus state for client
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use ibc::Height;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let height = Height::new(0, 12);
    /// let result = octopusxt_client.query_upgraded_cons_state(height).await?;
    /// ```
    fn query_upgraded_cons_state(
        &self,
        height: Height,
    ) -> Result<QueryConsensusStateResponse, Self::Error>;

    /// Query newly created clients in block
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ClientRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use ibc::Height;
    /// use sp_core::H256;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let hash = H256::default();
    /// let result = octopusxt_client.query_newly_created_clients(hash.into()).await?;
    /// ```
    fn query_newly_created_clients(
        &self,
        block_hash: Hash,
    ) -> Result<Vec<IdentifiedClientState>, Self::Error>;
}

#[async_trait]
pub trait ChannelRpc {
    type Error;

    /// Query all channel states
    /// query key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    ///
    /// # Usage example
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ChannelRpc, get_channels, OctopusxtClient};
    /// use octopusxt::MyConfig;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.query_channels().await?;
    /// ```
    async fn query_channels(&self) -> Result<Vec<IdentifiedChannelEnd>, Self::Error>;

    /// get channelEnd according by port_identifier, channel_identifier and read Channles StorageMaps
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    /// use subxt::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let result = octopusxt_client.query_channel_end(port_id, channel_id).await?;
    /// ```
    async fn query_channel_end(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<ChannelEnd, Self::Error>;

    // TODO
    // /// Query client state for channel and port id
    // #[method(name = "ibc_queryChannelClient")]
    // fn query_channel_client(
    //     &self,
    //     height: u32,
    //     channel_id: String,
    //     port_id: String,
    // ) -> Result<IdentifiedClientState>;

    // /// Query all channel states for associated connection
    // fn query_connection_channels(
    //     &self,
    //     height: Height,
    //     connection_id: ConnectionId,
    // ) -> Result<QueryChannelsResponse, Self::Error>;

    /// query packet receipt by port_id, channel_id and sequence
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    /// use subxt::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = octopusxt_client.query_packet_receipt(port_id, channel_id, sequence).await?;
    /// ```
    async fn query_packet_receipt(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Receipt, Self::Error>;

    /// query packet receipt by port_id, channel_id and sequence
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    /// use subxt::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = octopusxt_client.query_packet_receipt_vec(port_id, channel_id, sequence).await?;
    /// ```
    async fn query_packet_receipt_vec(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error>;

    /// query  unreceipt packet
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    /// use subxt::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let port_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = vec![Sequence::from(12),Sequence::from(13)];
    /// let result = octopusxt_client.query_unreceipt_packet(port_id, channel_id, sequence).await?;
    /// ```
    async fn query_unreceipt_packet(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequences: Vec<Sequence>,
    ) -> Result<Vec<Sequence>, Self::Error>;

    /// query get_commitment_packet_state
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    /// use subxt::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.query_commitment_packet_state().await?;
    /// ```
    async fn query_commitment_packet_state(&self) -> Result<Vec<PacketState>, Self::Error>;

    /// query packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    /// use subxt::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let port_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(23);
    /// let result = octopusxt_client.query_packet_commitment(port_id, channel_id, sequence).await?;
    /// ```
    async fn query_packet_commitment(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error>;

    /// query packet acknowledgement by port_id, channel_id and sequence to verify if the packet has been received by the target chain
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    /// use subxt::ClientBuilder;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let port_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(12);
    /// let result = octopusxt_client.query_packet_acknowledgements(port_id, channel_id, sequence).await?;
    /// ```
    async fn query_packet_acknowledgements(
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
    /// use octopusxt::{ChannelRpc, MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let result = octopusxt_client.query_next_sequence_recv(prot_id, channel_id).await?;
    /// ```
    async fn query_next_sequence_recv(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Vec<u8>, Self::Error>;

    /// query get_commitment_packet_state
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ChannelRpc, get_acknowledge_packet_state, OctopusxtClient};
    /// use octopusxt::MyConfig;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.query_acknowledge_packet_state().await?;
    /// ```
    async fn query_acknowledge_packet_state(&self) -> Result<Vec<PacketState>, Self::Error>;

    /// Query unreceived packet commitments
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
    /// use ibc::Height;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ChannelRpc, OctopusxtClient};
    /// use octopusxt::MyConfig;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let height = Height::zero();
    /// let channel_id= ChannelId::new(12);
    /// let port_id = PortId::default();
    /// let seqs = vec![Sequence::from(10)];
    /// let result = octopusxt_client.query_unreceived_packets(height, channel_id, port_id, seqs).await?;
    /// ```
    fn query_unreceived_packets(
        &self,
        height: Height,
        channel_id: ChannelId,
        port_id: PortId,
        seqs: Vec<Sequence>,
    ) -> Result<Vec<Sequence>, Self::Error>;

    /// Query the unreceived acknowledgements
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId};
    /// use ibc::Height;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ChannelRpc, OctopusxtClient};
    /// use octopusxt::MyConfig;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let height = Height::zero();
    /// let channel_id= ChannelId::new(12);
    /// let port_id = PortId::default();
    /// let seqs = vec![Sequence::from(10)];
    /// let result = octopusxt_client.query_unreceived_acknowledgements(height, channel_id, port_id, seqs).await?;
    /// ```
    fn query_unreceived_acknowledgements(
        &self,
        height: Height,
        channel_id: ChannelId,
        port_id: PortId,
        seqs: Vec<Sequence>,
    ) -> Result<Vec<Sequence>, Self::Error>;
}

#[async_trait]
pub trait ConnectionRpc {
    type Error;
    /// query connectionEnd according by connection_identifier and read Connections StorageMaps
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ConnectionRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let conection_id = ConnectionId::default();
    /// let result = octopusxt_client.query_connection_end(conection_id).await?;
    /// ```
    async fn query_connection_end(
        &self,
        connection_identifier: ConnectionId,
    ) -> Result<ConnectionEnd, Self::Error>;

    // TODO
    // /// Query a connection state
    // fn query_connection(
    //     &self,
    //     height: u32,
    //     connection_id: String,
    // ) -> Result<QueryConnectionResponse>;

    /// Query all connection states
    /// query key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ConnectionRpc, MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.query_connections().await?;
    /// ```
    async fn query_connections(&self) -> Result<Vec<IdentifiedConnectionEnd>, Self::Error>;

    /// # Query IdentifiedChannelEnd by connection_identifier
    ///
    /// ## Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ConnectionRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let connection = ConnectionId::default();
    /// let result = octopusxt_client.query_connection_channels(connection).await?;
    /// ```
    async fn query_connection_channels(
        &self,
        connection_id: ConnectionId,
    ) -> Result<Vec<IdentifiedChannelEnd>, Self::Error>;

    /// Query all connection states for associated client
    ///
    /// ## Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ConnectionRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use ibc::Height;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let height = Height::default();
    /// let client_id = ClientId::default();
    /// let result = octopusxt_client.query_connection_using_client(height, client_id).await?;
    /// ```
    fn query_connection_using_client(
        &self,
        height: Height,
        client_id: ClientId,
    ) -> Result<Vec<IdentifiedConnection>, Self::Error>;

    /// Generate proof for connection handshake
    ///
    /// ## Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::{ConnectionRpc, MyConfig, OctopusxtClient};
    /// use ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
    /// use ibc::Height;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let height = Height::default();
    /// let client_id = ClientId::default();
    /// let conn_id = ConnectionId::default();
    /// let result = octopusxt_client.generate_conn_handshake_proof(height, client_id, conn_id).await?;
    /// ```
    fn generate_conn_handshake_proof(
        &self,
        height: Height,
        client_id: ClientId,
        conn_id: ConnectionId,
    ) -> Result<ConnHandshakeProof, Self::Error>;
}

#[async_trait]
pub trait PacketRpc {
    type Error;
    /// Query send packet event by port_id, channel_id and sequence
    /// (port_id, channel_id, sequence), packet)
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{MyConfig, OctopusxtClient, PacketRpc};
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let prot_id =PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = octopusxt_client.query_send_packet_event(port_id, channel_id, sequence).await?;
    /// ```
    async fn query_send_packet_event(
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
    /// use octopusxt::{MyConfig, OctopusxtClient, PacketRpc};
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequence = Sequence::from(0);
    /// let result = octopusxt_client.query_write_ack_packet_event(port_id, channel_id, sequence).await?;
    /// ```
    async fn query_write_ack_packet_event(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<Vec<u8>, Self::Error>;

    /// Query packet data
    ///
    /// # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics04_channel::packet::Sequence;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{MyConfig, OctopusxtClient, PacketRpc};
    /// use ibc::core::ics24_host::identifier::{ChannelId, PortId, Sequence};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let prot_id = PortId::default();
    /// let channel_id = ChannelId::default();
    /// let sequences = vec![Sequence::from(0)];
    /// let result = octopusxt_client.query_packets(channel_id, prot_id, sequences).await?;
    /// ```
    async fn query_packets(
        &self,
        channel_id: ChannelId,
        port_id: PortId,
        seqs: Vec<Sequence>,
    ) -> Result<Vec<Packet>, Self::Error>;
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

pub trait Transfer {
    type Error;

    /// Query the denom trace for an ibc denom
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use ibc::core::ics26_routing::handler::deliver;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{MyConfig, OctopusxtClient, Router, Transfer};
    /// use ibc_proto::google::protobuf::Any;
    /// use sp_core::crypto::AccountId32;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopus_client = OctopusxtClient::new(client);
    /// let msg = AccountId32::default().to_string();
    /// let result = octopus_client.query_denom_trace(msg).await?;
    /// ```
    fn query_denom_trace(&self, denom: String) -> Result<QueryDenomTraceResponse, Self::Error>;

    /// Query the denom traces for ibc denoms matching offset
    ///
    /// ```rust
    /// use ibc::core::ics26_routing::handler::deliver;
    /// use ibc::Height;
    /// use subxt::ClientBuilder;
    /// use octopusxt::{MyConfig, OctopusxtClient, Router, Transfer};
    /// use ibc_proto::google::protobuf::Any;
    /// use sp_core::crypto::AccountId32;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopus_client = OctopusxtClient::new(client);
    /// let offset = "1".to_string();
    /// let limit = 23;
    /// let height = Height::default();
    /// let result = octopus_client.query_denom_traces(offset, limit, height).await?;
    /// ```
    fn query_denom_traces(
        &self,
        offset: String,
        limit: u64,
        height: Height,
    ) -> Result<QueryDenomTracesResponse, Self::Error>;
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
    /// use octopusxt::{MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.subscribe_beefy().await?;
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
    /// use octopusxt::{MyConfig, OctopusxtClient};
    ///
    /// let api = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.query_latest_height().await?;
    /// ```
    pub async fn query_latest_height(&self) -> anyhow::Result<u64> {
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

    /// return RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>> struct
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
    /// use octopusxt::{MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let block_number = Some(BlockNumber::from(12));
    /// let block_hash = None;
    /// let result = octopusxt_client.query_mmr_leaf_and_mmr_proof(block_number, block_hash).await?;
    /// ```
    pub async fn query_mmr_leaf_and_mmr_proof(
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
    /// use octopusxt::{MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let block_hash = None;
    /// let result = octopusxt_client.query_header_by_block_hash(block_hash).await?;
    /// ```
    pub async fn query_header_by_block_hash(
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
    /// use octopusxt::{MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let block_number = Some(BlockNumber::from(2));
    /// let result = octopusxt_client.query_header_by_block_number(block_number).await?;
    /// ```
    pub async fn query_header_by_block_number(
        &self,
        block_number: Option<BlockNumber>,
    ) -> anyhow::Result<ibc::clients::ics10_grandpa::help::BlockHeader> {
        let api = self.to_runtime_api();

        let block_hash = api.client.rpc().block_hash(block_number).await?;

        let header = api.client.rpc().header(block_hash).await?.unwrap();

        let header = crate::utils::convert_substrate_header_to_ibc_header(header);

        Ok(header.into())
    }

    /// query block hash by block number
    ///
    /// ```rust
    /// use subxt::{ClientBuilder, BlockNumber};
    /// use octopusxt::{MyConfig, OctopusxtClient};
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let octopusxt_client = OctopusxtClient::new(client);
    /// let result = octopusxt_client.query_block_hash_by_block_number(Some(BlockNumber::from(2))).await?;
    /// ```
    pub async fn query_block_hash_by_block_number(
        &self,
        block_number: Option<BlockNumber>,
    ) -> anyhow::Result<H256> {
        let api = self.to_runtime_api();

        let block_hash: H256 = api
            .client
            .rpc()
            .block_hash(block_number)
            .await?
            .ok_or(anyhow::anyhow!("query block hash error"))?;

        Ok(block_hash)
    }

    /// Generate proof for given key
    pub fn query_proof(&self, _height: u32, _keys: Vec<Vec<u8>>) -> anyhow::Result<Proof> {
        todo!()
    }

    /// Query balance of an address on chain, addr should be a valid hexadecimal or SS58 string,
    /// representing the account id.
    pub fn query_balance_with_address(
        &self,
        _addr: String,
    ) -> anyhow::Result<ibc_proto::cosmos::base::v1beta1::Coin> {
        todo!()
    }
}

/// Proof for a set of keys
#[derive(Serialize, Deserialize)]
pub struct Proof {
    /// Trie proof
    pub proof: Vec<u8>,
    /// Height at which proof was recovered
    pub height: ibc_proto::ibc::core::client::v1::Height,
}

/// Connection handshake proof
#[derive(Serialize, Deserialize)]
pub struct ConnHandshakeProof {
    /// Protobuf encoded client state
    pub client_state: IdentifiedClientState,
    /// Trie proof for connection state, client state and consensus state
    pub proof: Vec<u8>,
    /// Proof height
    pub height: ibc_proto::ibc::core::client::v1::Height,
}
