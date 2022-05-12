use octopusxt::ibc_node;
use structopt::StructOpt;
use subxt::sp_core::Public;
use subxt::ClientBuilder;

#[derive(Debug, StructOpt)]
pub enum Beefy {}

impl Beefy {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // let map_key = match storage_entry {
        //     StorageEntryKey::Map(map_key) => map_key,
        //     StorageEntryKey::Plain => todo!()
        // };
        // let storage_key = map_key.iter().map( |val| StorageKey(val.value.clone())).collect::<Vec<StorageKey>>();
        // println!("client storage_key = {:?}", storage_key);

        // let params = &[to_json_value(storage_key).unwrap(), to_json_value(block_hash).unwrap()];
        //
        // #[derive(Debug, PartialEq, Serialize, Deserialize)]
        // #[serde(rename_all = "camelCase")]
        // pub struct ReadProof_ {
        //     pub at: String,
        //     pub proof: Vec<Bytes>,
        // }
        // let storage_proof: ReadProof_ = api
        //     .rpc()
        //     .client
        //     .request("state_getReadProof", params)
        //     .await.unwrap();
        //
        // println!(
        //     "In Substrate: [generate_storage_proof] >> storage_proof : {:?}",
        //     storage_proof
        // );

        // example 2, 2-ways
        // let port_id = ibc::ics24_host::identifier::PortId::from_str("transfer").unwrap();
        // let channel_id = ibc::ics24_host::identifier::ChannelId::from_str("channel-0").unwrap();

        // let client = ClientBuilder::new()
        //     .set_url("ws://localhost:9944")
        //     .build::<ibc_node::DefaultConfig>()
        //     .await?;

        // let ret = octopusxt::get_channel_end(&port_id, &channel_id, client).await?;
        // println!("In main: [get_channelend] >> channel_end: {:?}", ret);

        // example 3
        // let sub = api.client.rpc().subscribe_events().await.unwrap();
        // let decoder = api.client.events_decoder();
        // let mut sub = EventSubscription::<ibc_node::DefaultConfig>::new(sub, decoder);

        // let mut counter = 0;
        // while let Some(raw_event) = sub.next().await {
        //     if let Err(err) = raw_event {
        //         println!(
        //             "In main: [run_loop] >> raw_event error: {:?}",
        //             err
        //         );
        //         continue;
        //     }
        //     if counter == 5 {
        //         break;
        //     }
        //     counter += 1;
        //     let raw_event = raw_event.unwrap();
        //     println!(
        //         "in main: [run_loop] >> raw_event : {:?}",
        //         raw_event
        //     );
        // }

        // example mmr
        // let root_hash = api.storage().mmr().root_hash(Some(block_hash)).await?;
        // println!("block_hash : {:?}", block_hash);

        // let number_of_leaves = api.storage().mmr().number_of_leaves(Some(block_hash)).await?;
        // println!("number_of_leaves : {:?}", number_of_leaves);

        // let nodes = api.storage().mmr().nodes(1, Some(block_hash)).await?;
        // println!("node : {:?}", nodes);

        // let mut mmr = api.storage().mmr().nodes_iter(Some(block_hash)).await?;
        // let mut counter = 0;
        // while let Some(mmr_item) = mmr.next().await? {
        //     counter += 1;
        //     // println!("mmr: {}", hex::encode(mmr_item.0));
        // }
        // println!("counter : {}", counter);

        let api = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        let beefy_storage = ibc_node::beefy::storage::StorageApi::new(&api);
        let authorities = beefy_storage.authorities(None).await?;
        println!("authorities : {:?}", authorities);

        let api = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?
            .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;
        let block_header = block.next().await.unwrap().unwrap();
        let block_hash = block_header.hash();

        let authorities = api.storage().beefy().authorities(Some(block_hash)).await?;
        println!("authorities : {:?}", authorities);

        // println!("authorities length : {:?}", authorities.len());
        // for item in authorities.iter() {
        //     println!("authorities display: {}", item);
        //     println!("authorities debug: {:?}", item);
        //     println!("authorities raw vec: {:?}", item.to_raw_vec());
        //     let result =  format!("0x{}", subxt::sp_core::hexdisplay::HexDisplay::from(&  item.to_raw_vec()));
        //     println!("authorities name = {}", result);
        // }

        let result: Vec<String> = authorities
            .into_iter()
            .map(|val| {
                format!(
                    "0x{}",
                    subxt::sp_core::hexdisplay::HexDisplay::from(&val.to_raw_vec())
                )
            })
            .collect();
        println!("result = {:?}", result);

        // let validator_set_id = api.storage().beefy().validator_set_id(Some(block_hash)).await?;
        // println!("validator_set_id : {:?}", validator_set_id);

        // let next_authorities = api.storage().beefy().next_authorities(Some(block_hash)).await?;
        // println!("next_authorities: {:?}", next_authorities.len());

        // pub struct BeefyNextAuthoritySet<MerkleRoot> {
        //     /// Id of the next set.
        //     ///
        //     /// Id is required to correlate BEEFY signed commitments with the validator set.
        //     /// Light Client can easily verify that the commitment witness it is getting is
        //     /// produced by the latest validator set.
        //     pub id: crate::ValidatorSetId,
        //     /// Number of validators in the set.
        //     ///
        //     /// Some BEEFY Light Clients may use an interactive protocol to verify only subset
        //     /// of signatures. We put set length here, so that these clients can verify the minimal
        //     /// number of required signatures.
        //     pub len: u32,
        //     /// Merkle Root Hash build from BEEFY AuthorityIds.
        //     ///
        //     /// This is used by Light Clients to confirm that the commitments are signed by the correct
        //     /// validator set. Light Clients using interactive protocol, might verify only subset of
        //     /// signatures, hence don't require the full list here (will receive inclusion proofs).
        //     pub root: MerkleRoot,
        // }
        // let result = api.storage().mmr_leaf().beefy_next_authorities(Some(block_hash)).await?;
        // println!("beefy_next_authorities id: {:?}", result.id);
        // println!("beefy_next_authorities len: {:?}", result.len);
        // println!("beefy_next_authorities root: {:?}", hex::encode(result.root));

        // let finalized_head = api.client.rpc().finalized_head().await?;
        // println!("finalized_head : {:?}", finalized_head);

        // let block = api.client.rpc().block(Some(block_hash.unwrap())).await?;
        // println!("block : {:?}", block);

        Ok(())
    }
}
