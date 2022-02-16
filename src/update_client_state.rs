use beefy_light_client::{beefy_ecdsa_to_ethereum, commitment, Error};
use ibc::ics02_client::client_state::AnyClientState;
use sp_core::hexdisplay::HexDisplay;
use std::str::FromStr;
use tendermint_proto::Protobuf;

use crate::call_ibc::{get_header_by_block_number, get_mmr_leaf_and_mmr_proof};
use crate::ibc_node::{DefaultConfig, RuntimeApi};
use codec::Encode;
use ibc::ics02_client::client_type::ClientType;
use ibc::ics10_grandpa::help;
use ibc::ics24_host::identifier::ClientId;
use sp_keyring::AccountKeyring;
use subxt::sp_core::Public;

use beefy_merkle_tree::{merkle_proof, verify_proof, Keccak256};

use beefy_merkle_tree::Hash;
use subxt::{BeefySubscription, BlockNumber, Client, PairSigner};

/// mmr proof struct
#[derive(Clone, Debug, Default)]
pub struct MmrProof {
    mmr_leaf: Vec<u8>,
    mmr_leaf_proof: Vec<u8>,
}

/// build merkle proof for validator
pub async fn build_validator_proof(
    src_client: Client<DefaultConfig>,
    block_number: u32,
) -> Result<Vec<help::ValidatorMerkleProof>, Box<dyn std::error::Error>> {
    let api = src_client.to_runtime_api::<RuntimeApi<DefaultConfig>>();

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
        .map(|authority| {
            format!(
                "{}",
                subxt::sp_core::hexdisplay::HexDisplay::from(&authority.to_raw_vec())
            )
        })
        .collect();
    println!("get authorities strs : {:?}", authority_strs);

    // Convert BEEFY secp256k1 public keys into Ethereum addresses
    let validators: Vec<Vec<u8>> = authority_strs
        .into_iter()
        .map(|authority| {
            hex::decode(&authority)
                .map(|compressed_key| beefy_ecdsa_to_ethereum(&compressed_key))
                .unwrap_or_default()
        })
        .collect();
    println!("get validators : {:?}", validators);

    // let eth_addresss_merkle_root = merkle_root::<Keccak256, _, _>(eth_addresss.clone());
    // println!(
    //     "eth_addresss_merkle_root = {}",
    //     hex::encode(&eth_addresss_merkle_root)
    // );

    let mut validator_merkle_proofs: Vec<help::ValidatorMerkleProof> = Vec::new();
    for l in 0..validators.len() {
        // when
        let proof = merkle_proof::<Keccak256, _, _>(validators.clone(), l);
        println!("get validator proof root = {}", hex::encode(&proof.root));
        // assert_eq!(
        //     hex::encode(&proof.root),
        //     hex::encode(&eth_addresss_merkle_root)
        // );

        println!("get validator proof  = {:?}", proof.proof);
        println!(
            "get validator number_of_leaves = {}",
            proof.number_of_leaves
        );

        println!("get validator leaf_index = {}", proof.leaf_index);
        // assert_eq!(proof.leaf_index, l);

        println!("get validator leaf = {}", hex::encode(&proof.leaf));
        // assert_eq!(&proof.leaf, &eth_addresss[l]);

        let validator_merkle_proof =
            help::ValidatorMerkleProof::from(beefy_light_client::ValidatorMerkleProof {
                proof: proof.proof,
                number_of_leaves: proof.number_of_leaves,
                leaf_index: proof.leaf_index,
                leaf: proof.leaf,
            });

        println!("get validator merkle proof = {:?}", validator_merkle_proof);
        validator_merkle_proofs.push(validator_merkle_proof);
    }

    println!(
        "all of the validator_merkle_proof is : {:?}",
        validator_merkle_proofs
    );

    Ok(validator_merkle_proofs)
}

/// build mmr proof
pub async fn build_mmr_proof(
    src_client: Client<DefaultConfig>,
    block_number: u32,
) -> Result<MmrProof, Box<dyn std::error::Error>> {
    let api = src_client
        .clone()
        .to_runtime_api::<RuntimeApi<DefaultConfig>>();
    //get block hash by block_number
    let block_hash: sp_core::H256 = api
        .client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await?
        .unwrap();
    println!(
        "block number : {} -> block hash : {:?}",
        block_number, block_hash
    );

    //get mmr leaf and proof
    // Note: target_height = signed_commitment.commitment.block_number-1
    let target_height = (block_number - 1) as u64;
    let (block_hash, mmr_leaf, mmr_leaf_proof) =
        get_mmr_leaf_and_mmr_proof(target_height, Some(block_hash), src_client.clone()).await?;
    println!("generate_proof block hash : {:?}", block_hash);

    let mmr_proof = MmrProof {
        mmr_leaf: mmr_leaf,
        mmr_leaf_proof: mmr_leaf_proof,
    };
    println!("get mmr proof = {:?}", mmr_proof);

    Ok(mmr_proof)
}

/// send Update client state request
pub async fn send_update_state_request(
    client: Client<DefaultConfig>,
    client_id: ClientId,
    mmr_root: help::MmrRoot,
) -> Result<subxt::sp_core::H256, Box<dyn std::error::Error>> {
    tracing::info!("in call_ibc: [update_client_state]");
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let api = client.to_runtime_api::<RuntimeApi<DefaultConfig>>();
    // let client_state_bytes = <commitment::SignedCommitment as codec::Encode>::encode(&client_state);

    let encode_client_id = client_id.as_bytes().to_vec();
    let encode_mmr_root = <help::MmrRoot as Encode>::encode(&mmr_root);
    println!("encode mmr root is {:?}", encode_mmr_root);

    // // test
    // let received_mmr_root = encode_mmr_root.clone();
    // let decode_received_mmr_root = help::MmrRoot::decode(&mut &received_mmr_root[..]).unwrap();
    // println!("decode mmr root is {:?}", decode_received_mmr_root);

    let result = api
        .tx()
        .ibc()
        .update_client_state(encode_client_id, encode_mmr_root)
        .sign_and_submit(&signer)
        .await?;

    tracing::info!("update client state result: {:?}", result);

    Ok(result)
}

/// update client state by cli for single.
pub async fn update_client_state(
    src_client: Client<DefaultConfig>,
    target_client: Client<DefaultConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::init();

    // subscribe beefy justification for src chain
    let api_a = src_client
        .clone()
        .to_runtime_api::<RuntimeApi<DefaultConfig>>();
    let sub = api_a.client.rpc().subscribe_beefy_justifications().await?;
    let mut sub = BeefySubscription::new(sub);

    let raw_signed_commitment = sub.next().await.unwrap().0;
    // decode signed commitment
    let signed_commmitment: commitment::SignedCommitment =
        <commitment::SignedCommitment as codec::Decode>::decode(
            &mut &raw_signed_commitment.clone()[..],
        )
        .unwrap();

    // get commitment
    let commitment::Commitment {
        payload,
        block_number,
        validator_set_id,
    } = signed_commmitment.clone().commitment;
    println!("signed commitment block_number : {}", block_number);
    println!("signed commitment validator_set_id : {}", validator_set_id);
    let payload = format!("{}", HexDisplay::from(&payload));
    println!("signed commitment payload : {:?}", payload);

    // get block header by block number
    let block_header =
        get_header_by_block_number(src_client.clone(), Some(BlockNumber::from(block_number)))
            .await
            .unwrap();
    println!("header = {:?}", block_header);

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
        block_header,
        signed_commitment: help::SignedCommitment::from(signed_commmitment),
        validator_merkle_proofs: validator_merkle_proofs,
        mmr_leaf: mmr_proof.mmr_leaf,
        mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
    };
    println!("build mmr_root = {:?}", mmr_root);

    // get client id from target chain
    // mock client id
    // let client_id = ClientId::new(ClientType::Grandpa, 0).unwrap();
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
    src_client: Client<DefaultConfig>,
    target_client: Client<DefaultConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::init();

    // subscribe beefy justification for src chain
    let api_a = src_client
        .clone()
        .to_runtime_api::<RuntimeApi<DefaultConfig>>();
    let sub = api_a.client.rpc().subscribe_beefy_justifications().await?;
    let mut sub = BeefySubscription::new(sub);

    // msg loop for handle the beefy SignedCommitment
    loop {
        let raw = sub.next().await.unwrap().0;
        // let target_raw = raw.clone();
        let signed_commmitment: commitment::SignedCommitment =
            <commitment::SignedCommitment as codec::Decode>::decode(&mut &raw[..]).unwrap();
        // let signed_commmitment = mmr::SignedCommitment::decode(&mut &raw.0[..]).unwrap();

        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commmitment.clone().commitment;
        println!("signed commitment block_number : {}", block_number);
        println!("signed commitment validator_set_id : {}", validator_set_id);
        let payload = format!("{}", HexDisplay::from(&payload));
        println!("signed commitment payload : {:?}", payload);

        let signatures: Vec<String> = signed_commmitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| format!("{}", HexDisplay::from(&signature.unwrap().0)))
            .collect();
        println!("signature :  {:?}", signatures);

        // get block header by block number
        let block_header =
            get_header_by_block_number(src_client.clone(), Some(BlockNumber::from(block_number)))
                .await
                .unwrap();
        println!("header = {:?}", block_header);

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
            block_header: block_header,
            signed_commitment: help::SignedCommitment::from(signed_commmitment),
            validator_merkle_proofs: validator_merkle_proofs,
            mmr_leaf: mmr_proof.mmr_leaf,
            mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
        };

        println!("build mmr_root = {:?}", mmr_root);

        // get client id from target chain
        // mock client id
        // let client_id = ClientId::new(ClientType::Grandpa, 0).unwrap();
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
) -> Result<(), Error> {
    let msg =
        libsecp256k1::Message::parse_slice(&commitment_hash[..]).or(Err(Error::InvalidMessage))?;
    println!("verify_commitment_signatures:commiment msg is {:?}", msg);

    for signature in signatures
        .into_iter()
        .skip(start_position)
        .take(interations)
    {
        if let Some(signature) = signature {
            let sig = libsecp256k1::Signature::parse_standard_slice(&signature.0[..64])
                .or(Err(Error::InvalidSignature))?;
            println!("verify_commitment_signatures:signature is {:?}", sig);

            let recovery_id = libsecp256k1::RecoveryId::parse(signature.0[64])
                .or(Err(Error::InvalidRecoveryId))?;
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
                        &validator_set_root,
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
        }
    }

    Ok(())
}

/// get client ids store in chain
async fn get_client_ids(
    client: Client<DefaultConfig>,
    expect_client_type: ClientType,
) -> Result<Vec<ClientId>, Box<dyn std::error::Error>> {
    let api = client.to_runtime_api::<RuntimeApi<DefaultConfig>>();

    // get client_state Keys
    let client_states_keys: Vec<Vec<u8>> = api
        .storage()
        .ibc()
        // .client_states_keys(Some(block_hash))
        .client_states_keys(None)
        .await?;
    // assert!(!client_states_keys.is_empty());

    let mut client_ids = vec![];
    for key in client_states_keys {
        // get client_state value
        let client_states_value: Vec<u8> =
            api.storage().ibc().client_states(key.clone(), None).await?;
        // assert!(!client_states_value.is_empty());
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
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{get_client_state, get_clients, ibc_node, subscribe_beefy};
    use beefy_light_client;
    use beefy_merkle_tree::{merkle_proof, merkle_root, verify_proof, Keccak256};
    use hex_literal::hex;

    use ibc::ics02_client::client_state::AnyClientState;
    use ibc::ics02_client::height::Height;
    use ibc::ics10_grandpa::client_state::ClientState;
    use ibc::ics10_grandpa::help::{BlockHeader, Commitment};
    use ibc::ics24_host::identifier::ChainId;

    use chrono::Local;
    // use ibc::ics24_host::Path::ClientType;
    use ibc::ics02_client::client_type::ClientType;
    use subxt::sp_core::hexdisplay::HexDisplay;
    use subxt::ClientBuilder;
    use tendermint_proto::Protobuf;
    use tokio::{self, task, time};

    #[tokio::test]
    async fn test_subscribe_beefy_justification() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        // subscribe beefy justification
        let signed_commitment_raw = subscribe_beefy(client.clone()).await.unwrap().0 .0;
        println!(
            "signed_commitment = {:?}",
            HexDisplay::from(&signed_commitment_raw)
        );
        // decode signed_commitment
        let signed_commitment =
            commitment::SignedCommitment::decode(&mut &signed_commitment_raw.clone()[..]).unwrap();
        println!("signed_commitment = {:?}", signed_commitment);

        let commitment = signed_commitment.commitment;
        println!("commitment : {:?}", commitment);

        Ok(())
    }

    #[test]
    fn test_verify_validator_proofs() {
        let signatures = vec![
            "020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1", // Alice
            "0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27", // Bob
            "0389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb", // Charlie
            "03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c", // Dave
            "031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa", // Eve
        ];

        // covert to eth address
        let eth_addresss: Vec<Vec<u8>> = signatures
            .into_iter()
            .map(|signature| {
                hex::decode(&signature)
                    .map(|compressed_key| beefy_ecdsa_to_ethereum(&compressed_key))
                    .unwrap_or_default()
            })
            .collect();
        println!("eth_addresss = {:#?}", eth_addresss);

        let eth_addresss_merkle_root = merkle_root::<Keccak256, _, _>(eth_addresss.clone());
        println!(
            "eth_addresss_merkle_root = {}",
            hex::encode(&eth_addresss_merkle_root)
        );

        for l in 0..eth_addresss.len() {
            // when
            let proof = merkle_proof::<Keccak256, _, _>(eth_addresss.clone(), l);
            println!("generate address proof root = {}", hex::encode(&proof.root));
            assert_eq!(
                hex::encode(&proof.root),
                hex::encode(&eth_addresss_merkle_root)
            );
            assert_eq!(proof.leaf_index, l);
            assert_eq!(&proof.leaf, &eth_addresss[l]);

            // then
            assert!(verify_proof::<Keccak256, _, _>(
                &proof.root,
                proof.proof.clone(),
                eth_addresss.len(),
                proof.leaf_index,
                &proof.leaf
            ));

            // finally
            assert!(verify_proof::<Keccak256, _, _>(
                &eth_addresss_merkle_root,
                proof.proof.clone(),
                eth_addresss.len(),
                proof.leaf_index,
                &proof.leaf
            ));
        }
    }

    #[tokio::test]
    async fn test_build_and_verify_signature() -> Result<(), Box<dyn std::error::Error>> {
        // let signatures = vec![
        //     "020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1", // Alice
        //     "0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27", // Bob
        //     "0389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb", // Charlie
        //     "03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c", // Dave
        //     "031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa", // Eve
        // ];
        let signatures = vec![
            "020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1", // Alice
        ];
        println!("test signatures strs = {:?}", signatures);

        // covert to eth address
        let validators: Vec<Vec<u8>> = signatures
            .into_iter()
            .map(|signature| {
                hex::decode(&signature)
                    .map(|compressed_key| beefy_ecdsa_to_ethereum(&compressed_key))
                    .unwrap_or_default()
            })
            .collect();
        println!("test validators = {:?}", validators);

        // build validators merkle root
        let validators_merkle_root = merkle_root::<Keccak256, _, _>(validators.clone());

        println!(
            "test validators_merkle_root = {}",
            hex::encode(&validators_merkle_root)
        );

        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        let block_number = 100;

        let validator_proofs = build_validator_proof(client.clone(), block_number).await?;

        // verify validator proof
        for idx in 0..validator_proofs.len() {
            let proof: beefy_light_client::ValidatorMerkleProof =
                validator_proofs[idx].clone().into();
            // verify validator proof
            assert!(verify_proof::<Keccak256, _, _>(
                &validators_merkle_root,
                proof.proof,
                validator_proofs.len(),
                proof.leaf_index,
                &proof.leaf
            ));
        }

        // -----------------------------------
        // mock verify signed commiment on chain
        // -----------------------------------
        // init beefy light client
        // let public_keys = vec![
        //     "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        //     "0x0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27".to_string(), // Bob
        //     "0x0389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb".to_string(), // Charlie
        //     "0x03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c".to_string(), // Dave
        //     "0x031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa".to_string(), // Eve
        // ];
        println!("-------------------- mock verify signed commiment on chain --------------------");

        let public_keys = vec![
            "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        ];
        let light_client = beefy_light_client::new(public_keys);
        println!("init beefy light client: {:?}", light_client);
        println!(
            "beefy light client validators set root = {}",
            hex::encode(&light_client.validator_set.root)
        );

        // subscribe beefy justification
        let signed_commitment = subscribe_beefy(client.clone()).await.unwrap().0;
        // decode
        let signed_commitment =
            commitment::SignedCommitment::decode(&mut &signed_commitment[..]).unwrap();

        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commitment.commitment;
        println!("signed commitment block_number : {}", block_number);
        println!("signed commitment validator_set_id : {}", validator_set_id);
        let payload = format!("{}", HexDisplay::from(&payload));
        println!("signed commitment payload : {:?}", payload);

        let signatures: Vec<String> = signed_commitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| format!("{}", HexDisplay::from(&signature.unwrap().0)))
            .collect();
        println!("signature :  {:?}", signatures);

        let validator_proofs = build_validator_proof(client.clone(), block_number).await?;

        // covert the grandpa validator proofs to beefy_light_client::ValidatorMerkleProof
        let validator_proofs: Vec<beefy_light_client::ValidatorMerkleProof> = validator_proofs
            .clone()
            .into_iter()
            .map(|validator_proof| validator_proof.into())
            .collect();

        for idx in 0..validator_proofs.len() {
            let proof = validator_proofs[idx].clone();
            assert!(verify_proof::<Keccak256, _, _>(
                &validators_merkle_root,
                proof.proof,
                validator_proofs.len(),
                proof.leaf_index,
                &proof.leaf
            ));
        }

        if let Some(latest_commitment) = &light_client.latest_commitment {
            if signed_commitment.commitment <= *latest_commitment {
                println!("Commitment Already Updated! ");
                // return Err(Error::CommitmentAlreadyUpdated);
                return Ok(());
            }
        }

        let signatures_count = signed_commitment
            .signatures
            .iter()
            .filter(|&sig| sig.is_some())
            .count();
        if signatures_count < (light_client.validator_set.len / 2) as usize {
            println!(
                "InvalidNumberOfSignatures! expected: {}",
                (light_client.validator_set.len / 2) as usize
            );
            return Ok(());
        }

        let commitment::SignedCommitment {
            commitment,
            signatures,
        } = signed_commitment;
        let commitment_hash = commitment.hash();

        let result = verify_commitment_signatures(
            &commitment_hash,
            &signatures,
            &light_client.validator_set.root,
            &validator_proofs,
            0,
            signatures.len(),
        );
        match result {
            Ok(_) => println!("verify_validator_signature sucesse! "),
            Err(e) => println!("verify_validator_signature failure! : {:?}", e),
        }
        println!("verify_commitment_signatures passed !",);

        Ok(())
    }

    #[tokio::test]
    async fn verify_leaf_proof_works_1() -> Result<(), Box<dyn std::error::Error>> {
        //static data test1
        let root_hash = hex!("aa0b510cee4270257f6362a353262253de422f069826b5af4398377a4eee03f7");
        let leaf = hex!("c5010058000000e5ac4bf69913974aeb79779c77d6e22d40575a63d4bca9044b501b12916a6090010000000000000005000000304803fa5a91d9852caafe04b4b867a4ed27a07a5bee3d1507b4b187a68777a20000000000000000000000000000000000000000000000000000000000000000");
        let leaf: Vec<u8> = Decode::decode(&mut &leaf[..]).unwrap();
        let mmr_leaf: mmr::MmrLeaf = Decode::decode(&mut &*leaf).unwrap();
        println!("decode the mmr leaf1  = {:?}", mmr_leaf);
        let mmr_leaf_hash = Keccak256::hash(&leaf[..]);
        println!(
            "the mmr leaf hash1 = {:?}",
            format!("{}", HexDisplay::from(&mmr_leaf_hash))
        );

        let proof = hex!("580000000000000059000000000000000c638bedc14bfdb5cfb8eb7313f311859820948868afbaa340de2a467f4eec130cd789e49d14c7068ec08e0b5680c5e01b372d28802acaeba7b63a5e1482d5147c0e395b48e5a134164c4dac0b30fc8bfd56756329824e6c70c7325769c92c1ff8");
        let mmr_proof = mmr::MmrLeafProof::decode(&mut &proof[..]).unwrap();
        println!("decode the mmr leaf proof1 = {:?}", mmr_proof);

        assert_eq!(
            mmr::verify_leaf_proof(root_hash, mmr_leaf_hash, mmr_proof),
            Ok(true)
        );

        Ok(())
    }

    #[tokio::test]
    async fn verify_leaf_proof_works_2() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        // subscribe beefy justification
        let signed_commitment_raw = subscribe_beefy(client.clone()).await.unwrap().0 .0;
        println!(
            "signed_commitment = {:?}",
            HexDisplay::from(&signed_commitment_raw)
        );
        // decode signed_commitment
        let signed_commitment =
            commitment::SignedCommitment::decode(&mut &signed_commitment_raw.clone()[..]).unwrap();
        println!("signed_commitment = {:?}", signed_commitment);

        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commitment.commitment;

        // get mmr root
        let mmr_root = payload;
        println!(
            "root_hash(signed commitment payload) : {:?}
signed commitment block_number : {}
signed commitment validator_set_id : {}",
            format!("{}", HexDisplay::from(&mmr_root)),
            block_number,
            validator_set_id
        );

        let api = client
            .clone()
            .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

        //get block hash by block_number
        let block_hash: sp_core::H256 = api
            .client
            .rpc()
            .block_hash(Some(BlockNumber::from(block_number)))
            .await?
            .unwrap();
        println!(
            "block number : {} -> block hash : {:?}",
            block_number, block_hash
        );

        //get mmr leaf and proof
        // Note: target height=block_number - 1
        let target_height = (block_number - 8) as u64;
        let (block_hash, mmr_leaf, mmr_leaf_proof) =
            get_mmr_leaf_and_mmr_proof(target_height, Some(block_hash), client.clone()).await?;
        println!("generate_proof block hash : {:?}", block_hash);

        // mmr leaf proof
        println!(
            "generated the mmr leaf proof = {:?}",
            format!("{}", HexDisplay::from(&mmr_leaf_proof))
        );
        let decode_mmr_proof = mmr::MmrLeafProof::decode(&mut &mmr_leaf_proof[..]).unwrap();
        println!("decode the mmr leaf proof = {:?}", decode_mmr_proof);

        // mmr leaf
        println!(
            "generated the mmr leaf  = {:?}",
            format!("{}", HexDisplay::from(&mmr_leaf))
        );

        let mmr_leaf_1: mmr::MmrLeaf = mmr::MmrLeaf::decode(&mut &mmr_leaf.clone()[..]).unwrap();
        println!("decode the mmr leaf  = {:?}", mmr_leaf_1);

        let mmr_leaf: Vec<u8> = Decode::decode(&mut &mmr_leaf[..]).unwrap();
        println!(
            "decode the mmr leaf vec<u8> = {:?}",
            format!("{}", HexDisplay::from(&mmr_leaf))
        );

        let mmr_leaf_hash = Keccak256::hash(&mmr_leaf[..]);
        println!(
            "the mmr leaf hash = {:?}",
            format!("{}", HexDisplay::from(&mmr_leaf_hash))
        );

        let mmr_leaf_2: mmr::MmrLeaf = Decode::decode(&mut &*mmr_leaf).unwrap();
        println!("decode the mmr leaf  = {:?}", mmr_leaf_2);
        println!("parent_number  = {}", mmr_leaf_2.parent_number_and_hash.0);
        println!(
            "parent_hash  = {:?}",
            HexDisplay::from(&mmr_leaf_2.parent_number_and_hash.1)
        );

        // verify leaf proof
        // assert_eq!(
        //     mmr::verify_leaf_proof(mmr_root, mmr_leaf_hash, decode_mmr_proof),
        //     Ok(true)
        // );
        let result = mmr::verify_leaf_proof(mmr_root, mmr_leaf_hash, decode_mmr_proof);

        match result {
            Ok(b) => {
                if !b {
                    println!("mmr::verify_leaf_proof failure:InvalidMmrLeafProof! ");
                } else {
                    println!("mmr::verify_leaf_proof succees! ");
                }
            }

            Err(e) => println!("mr::verify_leaf_proof error! : {:?}", e),
        }

        Ok(())
    }

    #[tokio::test]
    async fn verify_leaf_proof_works_3() -> Result<(), Box<dyn std::error::Error>> {
        // init beefy light client
        // let public_keys = vec![
        //     "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        //     "0x0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27".to_string(), // Bob
        //     "0x0389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb".to_string(), // Charlie
        //     "0x03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c".to_string(), // Dave
        //     "0x031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa".to_string(), // Eve
        // ];
        let public_keys = vec![
            "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        ];
        let mut light_client = beefy_light_client::new(public_keys);
        println!("init beefy light client: {:?}", light_client);
        println!(
            "beefy light client validators set root = {}",
            hex::encode(&light_client.validator_set.root)
        );

        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        // subscribe beefy justification
        let signed_commitment_raw = subscribe_beefy(client.clone()).await.unwrap().0;

        let signed_commitment =
            commitment::SignedCommitment::decode(&mut &signed_commitment_raw.clone()[..]).unwrap();
        // let signed_commitment = commitment::SignedCommitment::decode(&mut &signed_commitment[..])
        //     .map_err(|_| Error::InvalidSignedCommitment)?;

        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commitment.commitment;
        println!("signed commitment block_number : {}", block_number);
        println!("signed commitment validator_set_id : {}", validator_set_id);
        let payload = format!("{}", HexDisplay::from(&payload));
        println!("signed commitment payload : {:?}", payload);

        let signatures: Vec<String> = signed_commitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| format!("{}", HexDisplay::from(&signature.unwrap().0)))
            .collect();
        println!("signature :  {:?}", signatures);

        // mock encode and decode
        let signed_commitment = help::SignedCommitment::from(signed_commitment);
        let signed_commitment_bytes = help::SignedCommitment::encode(&signed_commitment);
        let signed_commitment =
            help::SignedCommitment::decode(&mut &signed_commitment_bytes[..]).unwrap();
        // covert to beefy signed_commitment
        let signed_commitment: commitment::SignedCommitment = signed_commitment.into();

        let validator_proofs = build_validator_proof(client.clone(), block_number).await?;
        // covert the grandpa validator proofs to beefy_light_client::ValidatorMerkleProof
        let validator_proofs: Vec<beefy_light_client::ValidatorMerkleProof> = validator_proofs
            .clone()
            .into_iter()
            .map(|validator_proof| validator_proof.into())
            .collect();

        if let Some(latest_commitment) = &light_client.latest_commitment {
            if signed_commitment.commitment <= *latest_commitment {
                println!("Commitment already Updated! ");
                // return Err(Error::CommitmentAlreadyUpdated);
                return Ok(());
            }
        }

        let signatures_count = signed_commitment
            .signatures
            .iter()
            .filter(|&sig| sig.is_some())
            .count();
        if signatures_count < (light_client.validator_set.len / 2) as usize {
            println!(
                "InvalidNumberOfSignatures! expected: {}",
                (light_client.validator_set.len / 2) as usize
            );

            return Ok(());
        }

        let commitment::SignedCommitment {
            commitment,
            signatures,
        } = signed_commitment;
        let commitment_hash = commitment.hash();

        // verify commitment signatures
        assert!(verify_commitment_signatures(
            &commitment_hash,
            &signatures,
            &light_client.validator_set.root,
            &validator_proofs.clone(),
            0,
            signatures.len(),
        )
        .is_ok());

        println!("verify_commitment_signatures passed ! ");

        //get mmr proof
        let proof = build_mmr_proof(client.clone(), block_number).await?;
        let MmrProof {
            mmr_leaf,
            mmr_leaf_proof,
        } = proof.clone();

        // mock verify mmr proof
        let mmr_leaf_proof = mmr::MmrLeafProof::decode(&mut &mmr_leaf_proof[..]).unwrap();
        println!("decode mmr_leaf_proof : {:?}", mmr_leaf_proof);

        let mmr_leaf: Vec<u8> = Decode::decode(&mut &mmr_leaf[..]).unwrap();
        println!("decode mmr_leaf : {:?}", hex::encode(&mmr_leaf.clone()));
        let mmr_leaf_hash = Keccak256::hash(&mmr_leaf[..]);
        let mmr_leaf: mmr::MmrLeaf = Decode::decode(&mut &*mmr_leaf).unwrap();
        println!("decode mmr_leaf : {:?}", mmr_leaf);

        // assert!(mmr::verify_leaf_proof(commitment.payload, mmr_leaf_hash, mmr_leaf_proof).is_ok());

        let result = mmr::verify_leaf_proof(commitment.payload, mmr_leaf_hash, mmr_leaf_proof);

        match result {
            Ok(b) => {
                if !b {
                    println!("mmr::verify_leaf_proof failure! ");
                } else {
                    println!("mmr::verify_leaf_proof succees! ");
                    // update the latest commitment, including mmr_root
                    light_client.latest_commitment = Some(commitment);

                    // update validator_set
                    if mmr_leaf.beefy_next_authority_set.id > light_client.validator_set.id {
                        println!(
                            "mmr_leaf.beefy_next_authority_set.id = {}",
                            mmr_leaf.beefy_next_authority_set.id
                        );
                        println!(
                            "light_client.validator_set.id= {}",
                            light_client.validator_set.id
                        );
                        light_client.validator_set = mmr_leaf.beefy_next_authority_set;
                    }
                    println!("the updated beefy light client is : {:?}", light_client);
                    println!(
                        "beefy light client validators set root = {}",
                        hex::encode(&light_client.validator_set.root)
                    );
                }
            }

            Err(e) => println!("mr::verify_leaf_proof error! : {:?}", e),
        }

        // -------------------------------------------------------
        // verify mmr proof use beefy_light_client::update_state()
        // -------------------------------------------------------
        let public_keys2 = vec![
            "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        ];
        let mut light_client2 = beefy_light_client::new(public_keys2);
        println!("init beefy light client 2: {:?}", light_client2);
        println!(
            "beefy light client validators set root = {}",
            hex::encode(&light_client2.validator_set.root)
        );
        let signed_commitment_raw2 = subscribe_beefy(client.clone()).await.unwrap().0 .0;

        let signed_commitment2 =
            commitment::SignedCommitment::decode(&mut &signed_commitment_raw2.clone()[..]).unwrap();

        println!(
            "signed commitment block_number : {}",
            signed_commitment2.commitment.block_number
        );
        println!(
            "signed commitment validator_set_id : {}",
            signed_commitment2.commitment.validator_set_id
        );
        let payload = format!(
            "{}",
            HexDisplay::from(&signed_commitment2.commitment.payload)
        );
        println!("signed commitment payload : {:?}", payload);

        let validator_proofs2 =
            build_validator_proof(client.clone(), signed_commitment2.commitment.block_number)
                .await?;
        // covert the grandpa validator proofs to beefy_light_client::ValidatorMerkleProof
        let validator_proofs2: Vec<beefy_light_client::ValidatorMerkleProof> = validator_proofs2
            .clone()
            .into_iter()
            .map(|validator_proof| validator_proof.into())
            .collect();

        let proof2 =
            build_mmr_proof(client.clone(), signed_commitment2.commitment.block_number).await?;

        // let signed_commitment2_bytes = commitment::SignedCommitment::encode(&signed_commitment2);

        let result2 = light_client2.update_state(
            &signed_commitment_raw2,
            &validator_proofs2,
            &proof2.mmr_leaf,
            &proof2.mmr_leaf_proof,
        );
        match result2 {
            Ok(_) => println!(
                "update the beefy light client sucesse! : {:?}",
                light_client2
            ),
            Err(e) => println!("update the beefy light client failure! : {:?}", e),
        }

        Ok(())
    }

    #[tokio::test]
    async fn mock_verify_and_update_stateless() -> Result<(), Box<dyn std::error::Error>> {
        // subscribe beefy justification for src chain
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let api_a = client.clone().to_runtime_api::<RuntimeApi<DefaultConfig>>();
        let sub = api_a.client.rpc().subscribe_beefy_justifications().await?;
        let mut sub = BeefySubscription::new(sub);

        let raw_signed_commitment = sub.next().await.unwrap();
        // decode signed commitment
        let signed_commmitment: commitment::SignedCommitment =
            <commitment::SignedCommitment as codec::Decode>::decode(
                &mut &raw_signed_commitment.clone().0[..],
            )
            .unwrap();

        // get commitment
        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commmitment.commitment;
        println!("signed commitment block_number : {}", block_number);
        println!("signed commitment validator_set_id : {}", validator_set_id);
        let payload = format!(
            "0x{}",
            subxt::sp_core::hexdisplay::HexDisplay::from(&payload)
        );
        println!("signed commitment payload : {:?}", payload);

        // get signatures
        let signatures: Vec<String> = signed_commmitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| format!("{}", HexDisplay::from(&signature.unwrap().0)))
            .collect();
        println!("signature :  {:?}", signatures);

        // get block header by block number
        let block_header =
            get_header_by_block_number(client.clone(), Some(BlockNumber::from(block_number)))
                .await
                .unwrap();
        println!("header = {:?}", block_header);

        // build validator proof
        let validator_merkle_proofs = build_validator_proof(client.clone(), block_number)
            .await
            .unwrap();

        // build mmr proof
        let mmr_proof = build_mmr_proof(client.clone(), block_number).await.unwrap();

        // build mmr root
        let mmr_root = help::MmrRoot {
            block_header,
            signed_commitment: help::SignedCommitment::from(signed_commmitment.clone()),
            validator_merkle_proofs: validator_merkle_proofs,
            mmr_leaf: mmr_proof.mmr_leaf,
            mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
        };
        println!("build mmr_root = {:?}", mmr_root);
        // encode the mmr root
        let mmr_root_bytes = <help::MmrRoot as Encode>::encode(&mmr_root);

        // -------------------------------------------------
        // mock send and update client state
        // -------------------------------------------------

        // init beefy light client
        // let public_keys = vec![
        //     "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        //     "0x0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27".to_string(), // Bob
        //     "0x0389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb".to_string(), // Charlie
        //     "0x03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c".to_string(), // Dave
        //     "0x031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa".to_string(), // Eve
        // ];
        let public_keys = vec![
            "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        ];
        let mut lc = beefy_light_client::new(public_keys);
        println!("init beefy light client: {:?}", lc);

        // init client state
        let epoch_number = 10;
        let chain_id = ChainId::new("10-grandpa-0".to_string(), epoch_number);
        let mut client_state = ClientState {
            chain_id: chain_id.clone(),
            block_number: u32::default(),
            frozen_height: Height::default(),
            block_header: BlockHeader::default(),
            // latest_commitment: lc.latest_commitment.unwrap().into(),
            latest_commitment: Commitment::default(),
            validator_set: lc.validator_set.clone().into(),
        };
        println!("init client_state: {:?}", client_state);
        // println!("before update,the beefy light client is : {:?}", lc);

        // check exist the target client id or chain id?
        assert!(client_state.chain_id.as_str() == chain_id.as_str());

        // decode the mmr root
        let receive_mmr_root_bytes = mmr_root_bytes.clone();
        let receive_mmr_root = help::MmrRoot::decode(&mut &receive_mmr_root_bytes[..]).unwrap();
        println!("receive mmr root is {:?}", receive_mmr_root);

        let signed_commitment =
            commitment::SignedCommitment::from(receive_mmr_root.signed_commitment);

        let block_header = receive_mmr_root.block_header;

        let validator_proofs = receive_mmr_root.validator_merkle_proofs;
        let mmr_leaf = receive_mmr_root.mmr_leaf;
        let mmr_leaf_proof = receive_mmr_root.mmr_leaf_proof;

        // covert the grandpa validator proofs to beefy_light_client::ValidatorMerkleProof
        let validator_proofs: Vec<beefy_light_client::ValidatorMerkleProof> = validator_proofs
            .into_iter()
            .map(|validator_proof| validator_proof.into())
            .collect();

        // encode signed_commitment
        let encoded_signed_commitment = commitment::SignedCommitment::encode(&signed_commitment);

        let result = lc.update_state(
            &encoded_signed_commitment,
            &validator_proofs,
            &mmr_leaf,
            &mmr_leaf_proof,
        );
        match result {
            Ok(_) => {
                println!("update the beefy light client sucesse! ");
                println!("after update,the beefy light client is : {:?}", lc);

                // update client_state by lc state
                let latest_commitment = lc.latest_commitment.unwrap();
                // let latest_commitment = signed_commitment.commitment;
                client_state.block_number = latest_commitment.block_number;
                client_state.latest_commitment = help::Commitment::from(latest_commitment);

                // update validator_set
                client_state.validator_set = help::ValidatorSet::from(lc.validator_set.clone());
                client_state.block_header = block_header;

                // mock: save to chain
                //ClientKeeper::store_client_state(client_state)
                println!("the updated client state is : {:?}", client_state);
            }
            Err(e) => {
                println!("update the beefy light client failure! : {:?}", e);
            }
        }

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn mock_verify_and_update_stateful() -> Result<(), Box<dyn std::error::Error>> {
        // init beefy light client
        // let public_keys = vec![
        //     "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        //     "0x0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27".to_string(), // Bob
        //     "0x0389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb".to_string(), // Charlie
        //     "0x03bc9d0ca094bd5b8b3225d7651eac5d18c1c04bf8ae8f8b263eebca4e1410ed0c".to_string(), // Dave
        //     "0x031d10105e323c4afce225208f71a6441ee327a65b9e646e772500c74d31f669aa".to_string(), // Eve
        // ];
        let public_keys = vec![
            "0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1".to_string(), // Alice
        ];
        let lc = beefy_light_client::new(public_keys);
        println!("init beefy light client: {:?}", lc);

        // init client state
        let epoch_number = 10;
        let chain_id = ChainId::new("10-grandpa-0".to_string(), epoch_number);
        let mut client_state = ClientState {
            chain_id: chain_id.clone(),
            block_number: u32::default(),
            frozen_height: Height::default(),
            block_header: BlockHeader::default(),
            // latest_commitment: lc.latest_commitment.unwrap().into(),
            latest_commitment: Commitment::default(),
            validator_set: lc.validator_set.into(),
        };
        println!("init client_state: {:?}", client_state);

        // subscribe beefy justification for src chain
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let api_a = client.clone().to_runtime_api::<RuntimeApi<DefaultConfig>>();
        let sub = api_a.client.rpc().subscribe_beefy_justifications().await?;
        let mut sub = BeefySubscription::new(sub);

        // msg loop for handle the beefy SignedCommitment
        loop {
            let raw_signed_commitment = sub.next().await.unwrap();
            // decode signed commitment
            let signed_commmitment: commitment::SignedCommitment =
                <commitment::SignedCommitment as codec::Decode>::decode(
                    &mut &raw_signed_commitment.clone().0[..],
                )
                .unwrap();

            // get commitment
            let commitment::Commitment {
                payload,
                block_number,
                validator_set_id,
            } = signed_commmitment.commitment;
            println!("signed commitment block_number : {}", block_number);
            println!("signed commitment validator_set_id : {}", validator_set_id);
            let payload = format!(
                "0x{}",
                subxt::sp_core::hexdisplay::HexDisplay::from(&payload)
            );
            println!("signed commitment payload : {:?}", payload);

            // get signatures
            let signatures: Vec<String> = signed_commmitment
                .signatures
                .clone()
                .into_iter()
                .map(|signature| format!("{}", HexDisplay::from(&signature.unwrap().0)))
                .collect();
            println!("signature :  {:?}", signatures);

            // get block header by block number
            let block_header =
                get_header_by_block_number(client.clone(), Some(BlockNumber::from(block_number)))
                    .await
                    .unwrap();
            println!("header = {:?}", block_header);

            // build validator proof
            let validator_merkle_proofs = build_validator_proof(client.clone(), block_number)
                .await
                .unwrap();

            // build mmr proof
            let mmr_proof = build_mmr_proof(client.clone(), block_number).await.unwrap();

            // build mmr root
            let mmr_root = help::MmrRoot {
                block_header,
                signed_commitment: help::SignedCommitment::from(signed_commmitment.clone()),
                validator_merkle_proofs: validator_merkle_proofs,
                mmr_leaf: mmr_proof.mmr_leaf,
                mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
            };
            println!("build mmr_root = {:?}", mmr_root);
            // encode the mmr root
            let mmr_root_bytes = <help::MmrRoot as Encode>::encode(&mmr_root);

            // -------------------------------------------------
            // mock send and update client state
            // -------------------------------------------------
            println!("------------------------------------------------ ");
            println!("begine to mock the update client state on chain ");
            println!("------------------------------------------------ ");

            // decode the mmr root
            let receive_mmr_root_bytes = mmr_root_bytes.clone();
            let receive_mmr_root = help::MmrRoot::decode(&mut &receive_mmr_root_bytes[..]).unwrap();
            println!("receive mmr root is {:?}", receive_mmr_root);

            //mock: get latest client_client from chain by client_id
            // let client_state = ClientReader(client_id)

            // mock:check exist the target client id or chain id?
            // if client_state.chain_id.as_str() != chain_id.as_str() {
            //     println!("client is not existing!");
            //     // 实际情况，这个地方触发错误，直接返回
            //     continue;
            // }
            assert!(client_state.chain_id.as_str() == chain_id.as_str());

            // 检查高度，如果接收到的高度小于等于链上已存在高度，就无需更新了，直接返回
            let signed_commitment =
                commitment::SignedCommitment::from(receive_mmr_root.signed_commitment);
            let rev_block_number = signed_commitment.clone().commitment.block_number;
            if rev_block_number <= client_state.latest_commitment.block_number {
                println!("receive mmr root block number({}) less than client_state.latest_commitment.block_number({})",
                rev_block_number,client_state.latest_commitment.block_number
            );
                // 链上直接返回，这里继续循环即可
                continue;
            }

            // rebuild beefy light client by client_state
            let mut rebuild_light_client = beefy_light_client::LightClient {
                latest_commitment: Some(client_state.latest_commitment.clone().into()),
                validator_set: client_state.validator_set.clone().into(),
                in_process_state: None,
            };

            println!(
                "recover beefy_light_client from client_state store in chain \n {:?}",
                rebuild_light_client
            );

            let validator_proofs = receive_mmr_root.validator_merkle_proofs;
            // covert the grandpa validator proofs to beefy_light_client::ValidatorMerkleProof
            let validator_proofs: Vec<beefy_light_client::ValidatorMerkleProof> = validator_proofs
                .into_iter()
                .map(|validator_proof| validator_proof.into())
                .collect();

            // encode signed_commitment
            let encoded_signed_commitment =
                commitment::SignedCommitment::encode(&signed_commitment);

            let mmr_leaf = receive_mmr_root.mmr_leaf;
            let mmr_leaf_proof = receive_mmr_root.mmr_leaf_proof;

            let result = rebuild_light_client.update_state(
                &encoded_signed_commitment,
                &validator_proofs,
                &mmr_leaf,
                &mmr_leaf_proof,
            );
            match result {
                Ok(_) => {
                    println!("update the beefy light client sucesse! ");
                    println!(
                        "after update,the beefy light client is : {:?}",
                        rebuild_light_client
                    );

                    // update client_client by lc state
                    let latest_commitment = rebuild_light_client.latest_commitment.unwrap();
                    client_state.block_number = latest_commitment.block_number;
                    client_state.latest_commitment = help::Commitment::from(latest_commitment);

                    // update validator_set
                    client_state.validator_set =
                        help::ValidatorSet::from(rebuild_light_client.validator_set.clone());

                    // update block header
                    client_state.block_header = receive_mmr_root.block_header;

                    // mock: save to chain
                    //ClientKeeper::store_client_state(client_state)
                    println!("the updated client state is : {:?}", client_state);
                }
                Err(e) => {
                    println!("update the beefy light client failure! : {:?}", e);
                }
            }

            println!("------------------------------------------------ ");
            println!("end mock !");
            println!("------------------------------------------------ ");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_update_client_state() -> Result<(), Box<dyn std::error::Error>> {
        let chain_a = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let chain_b = ClientBuilder::new()
            .set_url("ws://localhost:8844")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        update_client_state(chain_a.clone(), chain_b.clone()).await?;
        update_client_state(chain_b.clone(), chain_a.clone()).await?;

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_update_client_state_service() -> Result<(), Box<dyn std::error::Error>> {
        //use two tokio task to update client state each other for chian a and chain b
        let update_task1 = task::spawn(async {
            let chain_a = ClientBuilder::new()
                .set_url("ws://localhost:9944")
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();
            let chain_b = ClientBuilder::new()
                .set_url("ws://localhost:8844")
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();
            let _ret = update_client_state_service(chain_a, chain_b).await;
        });

        let update_task2 = task::spawn(async {
            let chain_a = ClientBuilder::new()
                .set_url("ws://localhost:9944")
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();
            let chain_b = ClientBuilder::new()
                .set_url("ws://localhost:8844")
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();
            let _ret = update_client_state_service(chain_b, chain_a).await;
        });

        update_task1.await?;
        update_task2.await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_clients() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let clients = get_clients(client).await.unwrap();
        println!("{:?}", clients);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_client_state() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:8844")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let client_state =
            get_client_state(&ClientId::new(ClientType::Grandpa, 0).unwrap(), client)
                .await
                .unwrap();

        println!("{:?}", client_state);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_client_type() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let api = client
            .clone()
            .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();
        let mut block = api.client.rpc().subscribe_finalized_blocks().await?;

        let block_header = block.next().await.unwrap().unwrap();
        let block_hash = block_header.hash();

        // vector key-value
        let mut ret = vec![];
        // get client_state Keys
        let client_states_keys: Vec<Vec<u8>> = api
            .storage()
            .ibc()
            .client_states_keys(Some(block_hash))
            .await?;
        // assert!(!client_states_keys.is_empty());

        // enumate every item get client_state value
        for key in client_states_keys {
            // get client_state value
            let client_states_value: Vec<u8> = api
                .storage()
                .ibc()
                .client_states(key.clone(), Some(block_hash))
                .await?;
            // assert!(!client_states_value.is_empty());
            // store key-value
            ret.push((key.clone(), client_states_value));
        }

        for (client_id, client_state) in ret.iter() {
            let client_id_str = String::from_utf8(client_id.clone()).unwrap();
            let client_id = ClientId::from_str(client_id_str.as_str()).unwrap();

            let any_client_state = AnyClientState::decode_vec(&*client_state).unwrap();
            let client_type = any_client_state.client_type();
            // let client_state = ClientState::decode_vec(&*client_state).unwrap();
            let client_state = match any_client_state {
                AnyClientState::Grandpa(value) => value,
                _ => unimplemented!(),
            };

            println!("client_id :  {:?}", client_id);
            println!("client_type :  {:?}", client_type);
            println!("client_state : {:?}", client_state);
        }

        // get client ids by client type
        let client_ids = get_client_ids(client, ClientType::Grandpa).await.unwrap();
        println!("client ids :  {:?}", client_ids);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_tokio_task() -> Result<(), Box<dyn std::error::Error>> {
        fn now() -> String {
            Local::now().format("%F %T").to_string()
        }

        async fn task1() {
            loop {
                println!("doing task1: {}", now());
                time::sleep(time::Duration::from_secs(2)).await;
                println!("task1 done: {}", now());
            }
        }

        async fn task2() {
            loop {
                println!("doing task2: {}", now());
                time::sleep(time::Duration::from_secs(1)).await;
                println!("task2 done: {}", now());
            }
        }

        let task1 = task::spawn(async {
            task1().await;
        });

        let task2 = task::spawn(async {
            task2().await;
        });

        task1.await?;
        task2.await?;

        Ok(())
    }

    #[test]
    fn test_mmr_root_codec() {
        let ecode_mmr_root = vec![
            128, 137, 94, 158, 21, 227, 31, 196, 155, 125, 200, 219, 109, 230, 157, 141, 158, 242,
            66, 166, 214, 237, 83, 164, 149, 22, 189, 198, 166, 166, 36, 143, 126, 229, 47, 128,
            180, 112, 7, 70, 66, 79, 66, 139, 27, 61, 16, 50, 23, 218, 119, 203, 43, 79, 143, 73,
            24, 77, 91, 202, 3, 59, 64, 183, 185, 74, 95, 134, 128, 248, 28, 233, 27, 211, 42, 91,
            18, 212, 162, 34, 199, 81, 201, 94, 106, 199, 94, 162, 226, 146, 149, 227, 79, 60, 85,
            99, 244, 141, 163, 100, 172, 12, 1, 66, 65, 66, 69, 52, 2, 0, 0, 0, 0, 178, 241, 170,
            32, 0, 0, 0, 0, 2, 66, 69, 69, 70, 132, 3, 164, 44, 63, 247, 20, 42, 25, 203, 74, 214,
            168, 102, 56, 93, 8, 126, 2, 17, 82, 153, 94, 53, 167, 32, 163, 9, 220, 164, 64, 95,
            187, 12, 3, 66, 65, 66, 69, 1, 1, 38, 248, 87, 199, 3, 117, 26, 156, 145, 2, 20, 79,
            129, 207, 244, 163, 185, 212, 24, 131, 97, 64, 231, 107, 7, 16, 136, 115, 154, 31, 183,
            95, 124, 16, 164, 169, 253, 94, 165, 168, 227, 9, 12, 161, 22, 9, 29, 234, 164, 164,
            29, 91, 161, 205, 124, 30, 120, 76, 21, 110, 209, 228, 186, 132, 1, 249, 11, 0, 0, 128,
            164, 44, 63, 247, 20, 42, 25, 203, 74, 214, 168, 102, 56, 93, 8, 126, 2, 17, 82, 153,
            94, 53, 167, 32, 163, 9, 220, 164, 64, 95, 187, 12, 0, 0, 0, 0, 0, 0, 0, 0, 4, 5, 1,
            201, 155, 178, 232, 50, 15, 230, 111, 107, 26, 149, 12, 239, 111, 181, 207, 212, 201,
            203, 191, 169, 25, 125, 245, 90, 234, 216, 200, 247, 129, 137, 183, 120, 5, 50, 130,
            130, 245, 198, 12, 21, 160, 166, 212, 175, 250, 126, 253, 225, 79, 126, 10, 190, 203,
            224, 18, 143, 5, 57, 136, 149, 13, 207, 210, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 80, 224,
            76, 197, 94, 190, 225, 203, 206, 85, 47, 37, 14, 133, 197, 123, 112, 178, 226, 98, 91,
            205, 1, 197, 1, 0, 248, 11, 0, 0, 137, 94, 158, 21, 227, 31, 196, 155, 125, 200, 219,
            109, 230, 157, 141, 158, 242, 66, 166, 214, 237, 83, 164, 149, 22, 189, 198, 166, 166,
            36, 143, 126, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 174, 180, 122, 38, 147, 147, 41, 127,
            75, 10, 60, 156, 156, 253, 0, 199, 164, 25, 82, 85, 39, 76, 243, 157, 131, 218, 188,
            47, 204, 159, 243, 215, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 4, 248, 11, 0, 0, 0, 0, 0, 0, 249, 11, 0, 0, 0, 0,
            0, 0, 32, 44, 196, 20, 81, 144, 115, 65, 140, 207, 100, 73, 95, 246, 160, 5, 126, 190,
            47, 125, 211, 70, 154, 209, 11, 187, 3, 154, 253, 204, 98, 56, 249, 119, 116, 48, 180,
            238, 93, 136, 3, 121, 143, 95, 143, 75, 238, 238, 51, 169, 155, 32, 117, 12, 190, 152,
            193, 151, 91, 153, 213, 141, 198, 181, 192, 198, 155, 137, 180, 221, 37, 48, 237, 190,
            67, 221, 64, 12, 64, 209, 98, 202, 110, 34, 15, 76, 52, 229, 130, 94, 175, 4, 191, 102,
            111, 137, 33, 105, 28, 246, 231, 6, 228, 147, 79, 239, 2, 19, 135, 255, 169, 219, 156,
            183, 117, 1, 23, 189, 42, 186, 114, 56, 71, 236, 19, 39, 252, 183, 5, 208, 188, 83, 8,
            86, 82, 35, 99, 44, 204, 160, 72, 179, 176, 244, 51, 189, 171, 24, 191, 204, 115, 65,
            198, 121, 110, 217, 243, 84, 246, 12, 88, 212, 108, 60, 243, 79, 239, 98, 127, 117, 96,
            152, 247, 57, 118, 173, 160, 204, 239, 123, 138, 135, 84, 79, 177, 109, 111, 96, 69,
            249, 46, 247, 209, 191, 94, 191, 20, 36, 105, 150, 24, 103, 222, 239, 40, 214, 162, 43,
            219, 195, 46, 75, 11, 91, 205, 55, 144, 178, 39, 156, 50, 220, 89, 114, 235, 225, 201,
            81, 230, 141, 78, 8, 198, 228, 26, 177, 200, 213, 81, 70, 120, 111, 222, 191, 22, 151,
            36, 141, 176, 37, 234, 145, 70, 240, 203, 94, 51,
        ];
        let decode_received_mmr_root = help::MmrRoot::decode(&mut &ecode_mmr_root[..]).unwrap();
        println!("decode mmr root is {:?}", decode_received_mmr_root);
    }
}
