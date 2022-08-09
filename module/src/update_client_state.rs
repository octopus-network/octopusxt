use crate::ibc_node::RuntimeApi;
use crate::ibc_rpc::{get_header_by_block_number, get_mmr_leaf_and_mmr_proof};
use crate::{get_latest_height, MyConfig, SubstrateNodeTemplateExtrinsicParams};

use anyhow::Result;
use beefy_light_client::{
    beefy_ecdsa_to_ethereum,
    commitment::{self, known_payload_ids::MMR_ROOT_ID},
    Error,
};
use beefy_merkle_tree::{merkle_proof, verify_proof, Hash, Keccak256};
use codec::{Decode, Encode};
use ibc::{
    clients::ics10_grandpa::help,
    core::{
        ics02_client::{client_state::AnyClientState, client_type::ClientType},
        ics24_host::identifier::ClientId,
    },
};
use sp_core::{hexdisplay::HexDisplay, ByteArray};
use sp_keyring::AccountKeyring;
use subxt::sp_core::Public;

use beefy_merkle_tree::{merkle_proof, verify_proof, Keccak256};

use beefy_merkle_tree::Hash;
use subxt::{BeefySubscription, BlockNumber, Client, PairSigner, SignedCommitment};

use std::str::FromStr;
use tendermint_proto::Protobuf;


/// mmr proof struct
#[derive(Clone, Debug, Default)]
pub struct MmrProof {
    pub mmr_leaf: Vec<u8>,
    pub mmr_leaf_proof: Vec<u8>,
}

/// build merkle proof for validator
pub async fn build_validator_proof(
    src_client: Client<MyConfig>,
    block_number: u32,
) -> Result<Vec<help::ValidatorMerkleProof>> {
    let api = src_client
        .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    // get block hash
    let block_hash = api
        .client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await?;

    //get validator set(authorities)
    let authorities = api.storage().beefy().authorities(block_hash).await?;
    // println!("get authorities :  {:?}", authorities);

    // covert authorities to strings
    let authority_strs: Vec<String> = authorities
        .into_iter()
        .map(|authority| format!("{}", HexDisplay::from(&authority.to_raw_vec())))
        .collect();
    // println!("get authorities strs : {:?}", authority_strs);

    // Convert BEEFY secp256k1 public keys into Ethereum addresses
    let validators: Vec<Vec<u8>> = authority_strs
        .into_iter()
        .map(|authority| {
            hex::decode(&authority)
                .map(|compressed_key| beefy_ecdsa_to_ethereum(&compressed_key))
                .unwrap_or_default()
        })
        .collect();
    // println!("get validators : {:?}", validators);

    let mut validator_merkle_proofs: Vec<help::ValidatorMerkleProof> = Vec::new();
    for l in 0..validators.len() {
        // when
        let proof = merkle_proof::<Keccak256, _, _>(validators.clone(), l);

        println!("get validator proof root = {}", hex::encode(&proof.root));

        // println!("get validator proof  = {:?}", proof.proof);
        // println!(
        //     "get validator number_of_leaves = {}",
        //     proof.number_of_leaves
        // );

        // println!("get validator leaf_index = {}", proof.leaf_index);
        // assert_eq!(proof.leaf_index, l);

        // println!("get validator leaf = {}", hex::encode(&proof.leaf));
        // assert_eq!(&proof.leaf, &eth_addresss[l]);

        let validator_merkle_proof =
            help::ValidatorMerkleProof::from(beefy_light_client::ValidatorMerkleProof {
                proof: proof.proof,
                number_of_leaves: proof.number_of_leaves,
                leaf_index: proof.leaf_index,
                leaf: proof.leaf,
            });

        // println!("get validator merkle proof = {:?}", validator_merkle_proof);
        validator_merkle_proofs.push(validator_merkle_proof);
    }

    // println!(
    //     "all of the validator_merkle_proof is : {:?}",
    //     validator_merkle_proofs
    // );

    Ok(validator_merkle_proofs)
}

/// build mmr proof
pub async fn build_mmr_proof(src_client: Client<MyConfig>, block_number: u32) -> Result<MmrProof> {
    let api = src_client
        .clone()
        .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    // asset block block number < get laset height
    {
        let latest_height = get_latest_height(src_client.clone()).await?;
        println!("[build_mmr_proof] latest height = {:?}", latest_height);
        assert!(
            u64::from(block_number) <= latest_height,
            "block_number must less than or equal latest height"
        );
    }

    //get block hash by block_number
    let block_hash = api
        .client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await?;

    println!(
        "block number : {} -> block hash : {:?}",
        block_number, block_hash
    );

    //get mmr leaf and proof
    // Note: target_height = signed_commitment.commitment.block_number-1
    let target_height = BlockNumber::from(block_number - 1);
    let (block_hash, mmr_leaf, mmr_leaf_proof) = get_mmr_leaf_and_mmr_proof(
        Some(target_height),
        Some(block_hash.unwrap()),
        src_client.clone(),
    )
    .await?;
    println!("generate_proof block hash : {:?}", block_hash);

    let mmr_proof = MmrProof {
        mmr_leaf,
        mmr_leaf_proof,
    };
    // println!("get mmr proof = {:?}", mmr_proof);

    Ok(mmr_proof)
}

/// build mmr root
pub async fn build_mmr_root(
    src_client: Client<DefaultConfig>,
    raw_signed_commitment: SignedCommitment,
) -> Result<help::MmrRoot, Box<dyn std::error::Error>> {
    // decode signed commitment
    let signed_commmitment: commitment::SignedCommitment =
        <commitment::SignedCommitment as codec::Decode>::decode(
            &mut &raw_signed_commitment.0.clone()[..],
        )
        .unwrap();

    // get commitment
    let commitment::Commitment {
        payload,
        block_number,
        validator_set_id,
    } = signed_commmitment.clone().commitment;
    // println!("signed commitment block_number : {}", block_number);
    // println!("signed commitment validator_set_id : {}", validator_set_id);
    let payload = format!("{}", HexDisplay::from(&payload));
    // println!("signed commitment payload : {:?}", payload);

    // get block header by block number
    // let block_header =
    //     get_header_by_block_number(Some(BlockNumber::from(block_number)), src_client.clone())
    //         .await
    //         .unwrap();
    // println!("header = {:?}", block_header);

    // build validator proof
    let validator_merkle_proofs = build_validator_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr proof
    let mmr_proof = build_mmr_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr root
    let mmr_root = help::MmrRoot {
        signed_commitment: help::SignedCommitment::from(signed_commmitment),
        validator_merkle_proofs,
        mmr_leaf: mmr_proof.mmr_leaf,
        mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
    };
    // println!("build mmr_root = {:?}", mmr_root);
    Ok(mmr_root)
}
/// send Update client state request
pub async fn send_update_state_request(
    client: Client<MyConfig>,
    client_id: ClientId,
    mmr_root: help::MmrRoot,
) -> Result<subxt::sp_core::H256> {
    tracing::info!("in call_ibc: [update_client_state]");
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let api = client
        .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let encode_client_id = client_id.as_bytes().to_vec();
    let encode_mmr_root = help::MmrRoot::encode(&mmr_root);
    println!("encode mmr root is {:?}", encode_mmr_root);

    let result = api
        .tx()
        .ibc()
        .update_client_state(encode_client_id, encode_mmr_root)?
        .sign_and_submit_default(&signer)
        .await?;

    tracing::info!("update client state result: {:?}", result);

    Ok(result)
}

/// update client state by cli for single.
pub async fn update_client_state(
    src_client: Client<MyConfig>,
    target_client: Client<MyConfig>,
) -> Result<()> {
    // subscribe beefy justification for src chain
    let api_a = src_client
        .clone()
        .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();
    let mut sub = api_a.client.rpc().subscribe_beefy_justifications().await?;

    let raw_signed_commitment = sub.next().await.unwrap().unwrap().0;
    // decode signed commitment
    let signed_commitment =
        commitment::SignedCommitment::decode(&mut &raw_signed_commitment.clone()[..]).unwrap();

    // get commitment
    let commitment::Commitment {
        payload,
        block_number,
        validator_set_id,
    } = signed_commitment.clone().commitment;
    println!("signed commitment block_number : {}", block_number);
    println!("signed commitment validator_set_id : {}", validator_set_id);
    let payload = format!("{:?}", payload.get_raw(&MMR_ROOT_ID));
    println!("signed commitment payload : {:?}", payload);

    // get block header by block number
    // let block_header =
    //     get_header_by_block_number(Some(BlockNumber::from(block_number)), src_client.clone())
    //         .await
    //         .unwrap();
    // println!("header = {:?}", block_header);

    // build validator proof
    let validator_merkle_proofs = build_validator_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr proof
    let mmr_proof = build_mmr_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr root
    let mmr_root = help::MmrRoot {
        signed_commitment: help::SignedCommitment::from(signed_commmitment),
        validator_merkle_proofs,
        mmr_leaf: mmr_proof.mmr_leaf,
        mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
    };
    println!("build mmr_root = {:?}", mmr_root);

    // get client id from target chain
    let client_ids = get_client_ids(target_client.clone(), ClientType::Grandpa)
        .await
        .unwrap();
    for client_id in client_ids {
        let result =
            send_update_state_request(target_client.clone(), client_id, mmr_root.clone()).await;

        match result {
            Ok(r) => {
                println!("update client state success and result is : {:?}", r);
            }
            Err(e) => {
                println!("update client state client failure and error is : {:?}", e);
            }
        }
    }
    // send mmr root to substrate-ibc

    Ok(())
}

/// update client state service.
pub async fn update_client_state_service(
    src_client: Client<MyConfig>,
    target_client: Client<MyConfig>,
) -> Result<()> {
    // subscribe beefy justification for src chain
    let api_a = src_client
        .clone()
        .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    let mut sub = api_a.client.rpc().subscribe_beefy_justifications().await?;

    // msg loop for handle the beefy SignedCommitment
    loop {
        let raw = sub.next().await.unwrap().unwrap().0;
        // let target_raw = raw.clone();
        let signed_commitment = commitment::SignedCommitment::decode(&mut &raw[..]).unwrap();

        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commitment.clone().commitment;
        println!("signed commitment block_number : {}", block_number);
        println!("signed commitment validator_set_id : {}", validator_set_id);
        let payload = format!("{:?}", payload.get_raw(&MMR_ROOT_ID));
        println!("signed commitment payload : {:?}", payload);

        let signatures: Vec<String> = signed_commitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| format!("{}", HexDisplay::from(&signature.unwrap().0)))
            .collect();
        println!("signature :  {:?}", signatures);

        // get block header by block number
        // let block_header =
        //     get_header_by_block_number(Some(BlockNumber::from(block_number)), src_client.clone())
        //         .await
        //         .unwrap();
        // println!("header = {:?}", block_header);

        // build validator proof
        let validator_merkle_proofs = build_validator_proof(src_client.clone(), block_number)
            .await
            .unwrap();

        // build mmr proof
        let mmr_proof = build_mmr_proof(src_client.clone(), block_number)
            .await
            .unwrap();

        // build mmr root
        let mmr_root = help::MmrRoot {
            signed_commitment: help::SignedCommitment::from(signed_commmitment),
            validator_merkle_proofs,
            mmr_leaf: mmr_proof.mmr_leaf,
            mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
        };

        println!("build mmr_root = {:?}", mmr_root);

        // get client id from target chain
        let client_ids = get_client_ids(target_client.clone(), ClientType::Grandpa)
            .await
            .unwrap();
        for client_id in client_ids {
            let result =
                send_update_state_request(target_client.clone(), client_id, mmr_root.clone()).await;

            match result {
                Ok(r) => {
                    println!("update client state success and result is : {:?}", r);
                }
                Err(e) => {
                    println!("update client state client failure and error is : {:?}", e);
                }
            }
        }
    }
}

// verify commitment signatures,copy from beefy light client
// #[warn(unused_must_use)]
pub fn verify_commitment_signatures(
    commitment_hash: &Hash,
    signatures: &[Option<commitment::Signature>],
    validator_set_root: &Hash,
    validator_proofs: &[beefy_light_client::ValidatorMerkleProof],
    start_position: usize,
    interations: usize,
) -> core::result::Result<(), Error> {
    let msg =
        libsecp256k1::Message::parse_slice(&commitment_hash[..]).or(Err(Error::InvalidMessage))?;
    println!("verify_commitment_signatures:commiment msg is {:?}", msg);

    for signature in signatures
        .iter()
        .skip(start_position)
        .take(interations)
        .flatten()
    {
        // if let Some(signature) = signature {
        let sig = libsecp256k1::Signature::parse_standard_slice(&signature.0[..64])
            .or(Err(Error::InvalidSignature))?;
        println!("verify_commitment_signatures:signature is {:?}", sig);

        let recovery_id =
            libsecp256k1::RecoveryId::parse(signature.0[64]).or(Err(Error::InvalidRecoveryId))?;
        println!(
            "verify_commitment_signatures:recovery_id is {:?}",
            recovery_id
        );

        let validator = libsecp256k1::recover(&msg, &sig, &recovery_id)
            .or(Err(Error::WrongSignature))?
            .serialize()
            .to_vec();
        let validator_address = Keccak256::hash(&validator[1..])[12..].to_vec();
        println!(
            "verify_commitment_signatures:validator_address is {:?}",
            hex::encode(&validator_address)
        );

        let mut found = false;
        for proof in validator_proofs.iter() {
            if validator_address == *proof.leaf {
                println!(
                    "verify_commitment_signatures:proof.leaf is {:?}",
                    hex::encode(&proof.leaf)
                );
                found = true;
                if !verify_proof::<Keccak256, _, _>(
                    validator_set_root,
                    proof.proof.clone(),
                    proof.number_of_leaves,
                    proof.leaf_index,
                    &proof.leaf,
                ) {
                    return Err(Error::InvalidValidatorProof);
                }
                break;
            }
        }
        if !found {
            return Err(Error::ValidatorNotFound);
        }
        // }
    }

    Ok(())
}

/// get client ids store in chain
pub async fn get_client_ids(
    client: Client<MyConfig>,
    expect_client_type: ClientType,
) -> Result<Vec<ClientId>> {
    let api = client
        .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

    // get client_state Keys
    let client_states_keys: Vec<Vec<u8>> = api.storage().ibc().client_states_keys(None).await?;

    let mut client_ids = vec![];
    for key in client_states_keys {
        // get client_state value
        let client_states_value: Vec<u8> = api.storage().ibc().client_states(&key, None).await?;

        let any_client_state = AnyClientState::decode_vec(&*client_states_value).unwrap();
        let client_type = any_client_state.client_type();
        if client_type == expect_client_type {
            let client_id_str = String::from_utf8(key).unwrap();
            let client_id = ClientId::from_str(client_id_str.as_str()).unwrap();
            client_ids.push(client_id)
        }
    }

    println!("client ids :  {:?}", client_ids);
    Ok(client_ids)
}
