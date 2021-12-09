use sp_keyring::AccountKeyring;
use subxt::{Client, ClientBuilder, EventSubscription, PairSigner};

// mod codegen;
// pub use codegen::astar::*;

use codec::{Decode, Encode};
use prost_types::Any;
use core::str::FromStr;

// #[derive(Eq, PartialEq, Encode, Decode)]
// pub struct ParachainId(u32);

#[derive(Encode, Decode)]
pub enum ClientType {
    Tendermint,
    Grandpa,
}

impl ClientType {
    pub fn to_ibc_client_type(self) -> ibc::ics02_client::client_type::ClientType {
        match self {
            ClientType::Tendermint => ibc::ics02_client::client_type::ClientType::Tendermint,
            ClientType::Grandpa => ibc::ics02_client::client_type::ClientType::Grandpa,
            _ => unreachable!(),
        }
    }
}

#[derive(Encode, Decode)]
pub struct MessageQueueChain(pub subxt::sp_core::H256);

#[subxt::subxt(runtime_metadata_path = "metadata_file/metadata.scale")]
pub mod ibc_node {
    // #[subxt(substitute_type = "polkadot_parachain::primitives::Id")]
    // use crate::ParachainId;

    // #[subxt(substitute_type = "polkadot_core_primitives::InboundHrmpMessage")]
    // use crate::ibc_node::runtime_types::polkadot_core_primitives::InboundHrmpMessage;

    #[subxt(substitute_type = "cumulus_pallet_parachain_system::MessageQueueChain")]
    use crate::MessageQueueChain;

}

const _: () = {
    use ibc_node::runtime_types::polkadot_parachain::primitives::Id;

    impl PartialEq for Id {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Id {

    }

    impl PartialOrd for Id {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Id {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
};

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Height {
    pub fn to_ibc_height(self) -> ibc::Height {
        ibc::Height {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Packet {
    pub fn to_ibc_packet(self) -> ibc::ics04_channel::packet::Packet {
        ibc::ics04_channel::packet::Packet {
            sequence: self.sequence.to_ibc_sequence(),
            source_port: self.source_port.to_ibc_port_id(),
            source_channel: self.source_channel.to_ibc_channel_id(),
            destination_port: self.destination_port.to_ibc_port_id(),
            destination_channel: self.destination_channel.to_ibc_channel_id(),
            data: self.data,
            timeout_height: self.timeout_height.to_ibc_height(),
            timeout_timestamp: self.timeout_timestamp.to_ibc_timestamp(),
        }
    }

}

impl  ibc_node::runtime_types::pallet_ibc::event::primitive::ConnectionId {
    pub fn to_ibc_connection_id(self) -> ibc::ics24_host::identifier::ConnectionId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::ConnectionId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ChannelId {
    pub fn to_ibc_channel_id(self) -> ibc::ics24_host::identifier::ChannelId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::ChannelId (value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::PortId {
    pub fn to_ibc_port_id(self) -> ibc::ics24_host::identifier::PortId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::PortId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ClientId {
    pub fn to_ibc_client_id(self) -> ibc::ics24_host::identifier::ClientId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::ClientId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Sequence {
    pub fn to_ibc_sequence(self) -> ibc::ics04_channel::packet::Sequence {
        ibc::ics04_channel::packet::Sequence(self.0)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Timestamp {
    pub fn to_ibc_timestamp(self) -> ibc::timestamp::Timestamp {
        let value = String::from_utf8(self.time).unwrap();
        let timestamp = ibc::timestamp::Timestamp::from_str(&value).unwrap();
        timestamp
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ClientType {
    pub fn to_ibc_client_type(self) -> ibc::ics02_client::client_type::ClientType {
        match self {
            ibc_node::runtime_types::pallet_ibc::event::primitive::ClientType::Tendermint => ibc::ics02_client::client_type::ClientType::Tendermint,
            ibc_node::runtime_types::pallet_ibc::event::primitive::ClientType::Grandpa => ibc::ics02_client::client_type::ClientType::Grandpa,
            _ => unreachable!(),
        }
    }
}

impl From<Any> for ibc_node::runtime_types::pallet_ibc::Any {
    fn from(value : Any) -> Self {
        ibc_node::runtime_types::pallet_ibc::Any {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        }
    }
}

impl Copy for ibc_node::runtime_types::pallet_ibc::event::primitive::Height {}

impl Clone for ibc_node::runtime_types::pallet_ibc::event::primitive::Height {
    fn clone(&self) -> Self {
        Self {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}

use ibc::ics02_client::client_state::AnyClientState;
use ibc::ics04_channel::channel::ChannelEnd;
use tendermint_proto::Protobuf;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api
        .client
        .rpc()
        .subscribe_finalized_blocks()
        .await?;
    let block_header = block.next().await.unwrap().unwrap();
    let block_hash = block_header.hash();

    let mut iter = api.storage().system().account_iter(None).await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }

    let data : Vec<u8> = api
        .storage()
        .ibc()
        .client_states(
            "07-tendermint-0".as_bytes().to_vec(),
            Some(block_hash),
        )
        .await?;

    println!("in substrate [get_client_state]: client_state: {:?}",data);
    let client_state = AnyClientState::decode_vec(&*data).unwrap();
    println!("in substrate [get_client_state]: any_client_state : {:?}", client_state);

    let data = api
        .storage()
        .ibc()
        .channels(
            "transfer1234".as_bytes().to_vec(),
            "channel-1234".as_bytes().to_vec(),
            Some(block_hash)
        )
        .await?;

    let channel_end = ChannelEnd::decode_vec(&*data).unwrap();
    println!(
        "In substrate: [get_channelend] >> channel_end: {:?}",
        channel_end
    );

    let sub = api.client.rpc().subscribe_events().await.unwrap();
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<ibc_node::DefaultConfig>::new(sub, decoder);

    while let Some(raw_event) = sub.next().await {
        if let Err(err) = raw_event {
            println!("In substrate_mointor: [run_loop] >> raw_event error: {:?}", err);
            continue;
        }
        let raw_event = raw_event.unwrap();
        println!("in substrate_mointor: [run_loop] >> raw_event : {:?}", raw_event);
    }

    Ok(())
}
