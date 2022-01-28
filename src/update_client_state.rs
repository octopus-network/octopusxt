use beefy_light_client::{beefy_ecdsa_to_ethereum, commitment, mmr, validator_set, Error};
use jsonrpsee::types::to_json_value;
use sp_core::hexdisplay::HexDisplay;

use crate::call_ibc::{get_block_header_by_block_number, get_mmr_leaf_and_mmr_proof};
use crate::ibc_node::{DefaultConfig, RuntimeApi};
use codec::{Decode, Encode};
use ibc::ics02_client::client_type::ClientType;
use ibc::ics10_grandpa::help;
use ibc::ics24_host::identifier::ClientId;
use sp_keyring::AccountKeyring;
use subxt::sp_core::Public;

use beefy_merkle_tree::{merkle_proof, merkle_root, verify_proof, Keccak256};

use beefy_merkle_tree::Hash;
use subxt::{BeefySubscription, BlockNumber, Client, PairSigner};

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
    log::info!("in call_ibc: [update_client_state]");
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let api = client.to_runtime_api::<RuntimeApi<DefaultConfig>>();
    // let client_state_bytes = <commitment::SignedCommitment as codec::Encode>::encode(&client_state);

    let encode_mmr_root = <help::MmrRoot as Encode>::encode(&mmr_root);
    let encode_client_id = client_id.as_bytes().to_vec();

    let result = api
        .tx()
        .ibc()
        .update_client_state(encode_client_id, encode_mmr_root)
        .sign_and_submit(&signer)
        .await?;

    log::info!("update client state result: {:?}", result);

    Ok(result)
}

/// update client state by cli for single.
pub async fn update_clien_state(
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
    let payload = format!("{}", HexDisplay::from(&payload));
    println!("signed commitment payload : {:?}", payload);

    // get block header by block number
    let block_header = get_block_header_by_block_number(src_client.clone(), block_number)
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
        signed_commitment: help::SignedCommitment::from(signed_commmitment.clone()),
        validator_merkle_proofs: validator_merkle_proofs,
        mmr_leaf: mmr_proof.mmr_leaf,
        mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
    };
    println!("build mmr_root = {:?}", mmr_root);

    // TODO: get client id from target chain,maybe query client id by rpc?
    // mock client id
    let client_id = ClientId::new(ClientType::Grandpa, 0).unwrap();

    // send mmr root to substrate-ibc
    let result = send_update_state_request(target_client.clone(), client_id, mmr_root)
        .await
        .unwrap();

    println!("update client state result: {:?}", result);

    Ok(())
}

/// update client state service.
pub async fn update_clien_state_service(
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
        let raw = sub.next().await.unwrap();
        // let target_raw = raw.clone();
        let signed_commmitment: commitment::SignedCommitment =
            <commitment::SignedCommitment as codec::Decode>::decode(&mut &raw.0[..]).unwrap();
        // let signed_commmitment = mmr::SignedCommitment::decode(&mut &raw.0[..]).unwrap();

        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commmitment.commitment;
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
        let block_header = get_block_header_by_block_number(src_client.clone(), block_number)
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
            signed_commitment: help::SignedCommitment::from(signed_commmitment.clone()),
            validator_merkle_proofs: validator_merkle_proofs,
            mmr_leaf: mmr_proof.mmr_leaf,
            mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
        };

        println!("build mmr_root = {:?}", mmr_root);

        // TODO: get client id from target chain
        // mock client id
        let client_id = ClientId::new(ClientType::Grandpa, 0).unwrap();

        // send mmr root to substrate-ibc
        let result = send_update_state_request(target_client.clone(), client_id, mmr_root)
            .await
            .unwrap();

        println!("update client state result: {:?}", result);
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{get_clients, ibc_node, subscribe_beefy};
    use beefy_light_client::{Error, LightClient};
    use beefy_merkle_tree::{merkle_proof, merkle_root, verify_proof, Keccak256, MerkleProof};
    use hex_literal::hex;

    use ibc::ics02_client::client_state::AnyClientState;
    use ibc::ics02_client::height::Height;
    use ibc::ics10_grandpa::client_state::ClientState;
    use ibc::ics10_grandpa::help::{BlockHeader, Commitment, ValidatorSet};
    use ibc::ics24_host::identifier::ChainId;

    use subxt::sp_core::hexdisplay::HexDisplay;
    use subxt::ClientBuilder;
    use tendermint_proto::Protobuf;

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
        let block_header = get_block_header_by_block_number(client.clone(), block_number)
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
    async fn mock_verify_and_update_statful() -> Result<(), Box<dyn std::error::Error>> {
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
            let block_header = get_block_header_by_block_number(client.clone(), block_number)
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
        let src_client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let target_client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        update_clien_state(src_client, target_client).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_update_client_state_service() -> Result<(), Box<dyn std::error::Error>> {
        let chain_a = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let chain_b = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        update_clien_state_service(chain_a, chain_b).await?;
        //TODO:利用两个tokio线程实现双向订阅

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
    async fn test_get_client_data() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();
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
            // let client_state = ClientState::decode_vec(&*client_state).unwrap();
            let client_state = match any_client_state {
                AnyClientState::Grandpa(value) => value,
                _ => unimplemented!(),
            };

            println!("client_id :  {:?}", client_id);
            println!("client_state : {:?}", client_state);
        }

        //-----------------------------------------------
        // get client type by client id
        //-----------------------------------------------

        // get client_state Keys
        let client_states_keys: Vec<Vec<u8>> = api
            .storage()
            .ibc()
            .client_states_keys(Some(block_hash))
            .await?;
        // assert!(!client_states_keys.is_empty());

        // vector key-value
        let mut ret = vec![];
        //  every item get client type value
        for key in client_states_keys {
            // get client type value
            let clients_store = ibc_node::ibc::storage::Clients(key.clone());
            // let client_type = api.client.rpc().storage(&entry.key().into(), None);
            let client_type = api
                .client
                .storage()
                .fetch_or_default(&clients_store, Some(block_hash))
                .await
                .unwrap();

            // assert!(!client_type.is_empty());
            ret.push((key.clone(), client_type));
        }

        for (client_id, client_type) in ret.iter() {
            let client_id_str = String::from_utf8(client_id.clone()).unwrap();
            let client_id = ClientId::from_str(client_id_str.as_str()).unwrap();

            let client_type_str = String::from_utf8(client_type.clone()).unwrap();
            let client_type = ClientType::from_str(&client_type_str).unwrap();

            println!("client_id :  {:?}", client_id);
            println!("client_state : {:?}", client_type);
        }

        Ok(())
    }
}
