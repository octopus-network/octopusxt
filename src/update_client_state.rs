use beefy_light_client::{beefy_ecdsa_to_ethereum, commitment, mmr};

use crate::call_ibc::{get_block_header_by_block_number, get_mmr_leaf_and_mmr_proof};
use crate::ibc_node::{DefaultConfig, RuntimeApi};
use codec::{Decode, Encode};
use ibc::ics10_grandpa::help;
use sp_keyring::AccountKeyring;
use subxt::sp_core::Public;

use beefy_merkle_tree::{merkle_proof, merkle_root, verify_proof, Keccak256};

use subxt::{BeefySubscription, BlockNumber, Client, PairSigner};

#[derive(Clone, Debug, Default)]
pub struct MmrProof {
    mmr_leaf: help::MmrLeaf,
    mmr_leaf_proof: help::MmrLeafProof,
}

/// build merkle proof for validator
pub async fn build_validator_merkle_proof(
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

        // // then
        // assert!(verify_proof::<Keccak256, _, _>(
        //     &proof.root,
        //     proof.proof.clone(),
        //     eth_addresss.len(),
        //     proof.leaf_index,
        //     &proof.leaf.clone()
        // ));

        // // finally
        // assert!(verify_proof::<Keccak256, _, _>(
        //     &eth_addresss_merkle_root,
        //     proof.proof.clone(),
        //     eth_addresss.len(),
        //     proof.leaf_index,
        //     &proof.leaf.clone()
        // ));
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
    // generae mmr leaf and mmr leaf proof
    // Note: target_height = signed_commitment.commitment.block_number-1
    let target_height = block_number - 1;

    let (mmr_leaf, mmr_leaf_proof) = get_mmr_leaf_and_mmr_proof(src_client, target_height as u64)
        .await
        .unwrap();
    let mmr_leaf = mmr::MmrLeaf::decode(&mut &mmr_leaf[..]).unwrap();
    let mmr_leaf_proof = mmr::MmrLeafProof::decode(&mut &mmr_leaf_proof[..]).unwrap();
    println!("get mmr_leaf : {:?}", mmr_leaf);
    println!("get mmr_leaf_proof : {:?}", mmr_leaf_proof);

    let mmr_proof = MmrProof {
        mmr_leaf: help::MmrLeaf::from(mmr_leaf),
        mmr_leaf_proof: help::MmrLeafProof::from(mmr_leaf_proof),
    };
    println!("get mmr proof = {:?}", mmr_proof);
    Ok(mmr_proof)
}

/// Update client state
pub async fn send_and_update_client_state(
    client: Client<DefaultConfig>,
    // signer: &PairSigner<DefaultConfig, Pair>,
    mmr_root: help::MmrRoot,
) -> Result<subxt::sp_core::H256, Box<dyn std::error::Error>> {
    log::info!("in call_ibc: [update_client_state]");
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let api = client.to_runtime_api::<RuntimeApi<DefaultConfig>>();
    // let client_state_bytes = <commitment::SignedCommitment as codec::Encode>::encode(&client_state);
    let mmr_root_bytes = <help::MmrRoot as Encode>::encode(&mmr_root);

    let result = api
        .tx()
        .ibc()
        .update_client_state(mmr_root_bytes)
        .sign_and_submit(&signer)
        .await?;

    log::info!("update client state result: {:?}", result);

    Ok(result)
}

/// Start update client state service.
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
        let payload = format!(
            "0x{}",
            subxt::sp_core::hexdisplay::HexDisplay::from(&payload)
        );
        println!("signed commitment payload : {:?}", payload);

        let signatures: Vec<String> = signed_commmitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| {
                format!(
                    "0x{}",
                    subxt::sp_core::hexdisplay::HexDisplay::from(&signature.unwrap().0)
                )
            })
            .collect();
        println!("signature :  {:?}", signatures);

        // get block header by block number
        let block_header = get_block_header_by_block_number(src_client.clone(), block_number)
            .await
            .unwrap();
        println!("header = {:?}", block_header);

        // build validator proof
        let validator_merkle_proofs =
            build_validator_merkle_proof(src_client.clone(), block_number)
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

        println!("mmr_root = {:#?}", mmr_root);

        // send mmr root to substrate-ibc
        let result = send_and_update_client_state(target_client.clone(), mmr_root).await;
        println!("update client state result: {:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ibc_node, subscribe_beefy};
    use beefy_merkle_tree::{merkle_proof, merkle_root, verify_proof, Keccak256, MerkleProof};
    use hex_literal::hex;
    use ibc::ics10_grandpa::help::Signature;
    use subxt::ClientBuilder;

    #[tokio::test]
    async fn test_build_validator_proof() -> Result<(), Box<dyn std::error::Error>> {
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

        let validator_proofs = build_validator_merkle_proof(client, block_number).await?;
        // println!(
        //     "all of the validator_merkle_proof is  {:?}",
        //     validator_proofs
        // );

        // verify validator proof
        for idx in 0..validator_proofs.len() {
            let proof: beefy_light_client::ValidatorMerkleProof =
                validator_proofs[idx].clone().into();
            // println!("ValidatorMerkleProof = {:?}", proof);

            assert!(verify_proof::<Keccak256, _, _>(
                &validators_merkle_root,
                proof.proof,
                validator_proofs.len(),
                proof.leaf_index,
                &proof.leaf
            ));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_build_mmr_proof() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        let block_number = 100;

        let proof = build_mmr_proof(client.clone(), block_number).await?;
        let beefy_next_authority_set = proof.mmr_leaf.beefy_next_authority_set;

        println!(
            "get beefy_next_authority_set id: {:?}",
            beefy_next_authority_set.id
        );
        println!(
            "get beefy_next_authority_set len: {:?}",
            beefy_next_authority_set.len
        );

        println!(
            "get beefy_next_authority_set root: {:?}",
            hex::encode(beefy_next_authority_set.root)
        );

        let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

        let block_hash = api
            .client
            .rpc()
            .block_hash(Some(BlockNumber::from(block_number)))
            .await?;
        println!("block_hash : {:?}", block_hash);

        // let validator_set_id = api.storage().beefy().validator_set_id(block_hash).await?;
        // println!("validator_set_id : {:?}", validator_set_id);

        // let next_authorities = api.storage().beefy().next_authorities(block_hash).await?;
        // println!("next_authorities: {:?}", next_authorities.len());

        let beefy_next_authorities = api
            .storage()
            .mmr_leaf()
            .beefy_next_authorities(block_hash)
            .await?;
        println!("beefy_next_authorities id: {:?}", beefy_next_authorities.id);
        println!(
            "beefy_next_authorities len: {:?}",
            beefy_next_authorities.len
        );
        println!(
            "beefy_next_authorities root: {:?}",
            hex::encode(beefy_next_authorities.root)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_update_state() -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;

        // get signed commitment
        let raw_signed_commitment = subscribe_beefy(client.clone()).await.unwrap();
        let signed_commmitment: commitment::SignedCommitment =
            <commitment::SignedCommitment as codec::Decode>::decode(
                &mut &raw_signed_commitment.clone().0[..],
            )
            .unwrap();

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

        let signatures: Vec<String> = signed_commmitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| {
                format!(
                    "0x{}",
                    subxt::sp_core::hexdisplay::HexDisplay::from(&signature.unwrap().0)
                )
            })
            .collect();
        println!("signature :  {:?}", signatures);

        // get block header by block number
        let block_header = get_block_header_by_block_number(client.clone(), block_number)
            .await
            .unwrap();
        println!("header = {:?}", block_header);

        // build validator proof
        let validator_merkle_proofs = build_validator_merkle_proof(client.clone(), block_number)
            .await
            .unwrap();

        // build mmr proof
        let mmr_proof = build_mmr_proof(client.clone(), block_number).await.unwrap();

        // build mmr root
        let mmr_root = help::MmrRoot {
            block_header: block_header,
            signed_commitment: help::SignedCommitment::from(signed_commmitment.clone()),
            validator_merkle_proofs: validator_merkle_proofs,
            mmr_leaf: mmr_proof.mmr_leaf,
            mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
        };
        println!("mmr_root = {:?}", mmr_root);
        // encode the mmr root
        let mmr_root_bytes = <help::MmrRoot as Encode>::encode(&mmr_root);

        // -------------------------------------------------
        // mock update state
        // decode the mmr root
        let receive_mmr_root_bytes = mmr_root_bytes.clone();
        let receive_mmr_root = help::MmrRoot::decode(&mut &receive_mmr_root_bytes[..]).unwrap();
        let block_header = receive_mmr_root.block_header;
        let signed_commitment: commitment::SignedCommitment =
            receive_mmr_root.signed_commitment.into();
        let validator_proofs = receive_mmr_root.validator_merkle_proofs;
        let mmr_leaf: mmr::MmrLeaf = receive_mmr_root.mmr_leaf.into();
        let mmr_leaf_proof: mmr::MmrLeafProof = receive_mmr_root.mmr_leaf_proof.into();

        // create beefy light client
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
        println!("light client: {:?}", lc);

        // encode data
        let encoded_signed_commitment = commitment::SignedCommitment::encode(&signed_commitment);
        let encoded_mmr_leaf = mmr::MmrLeaf::encode(&mmr_leaf);
        let encoded_mmr_proof = mmr::MmrLeafProof::encode(&mmr_leaf_proof);

        // covert the help validator proofs to beefy_light_client::ValidatorMerkleProof
        let validator_proofs: Vec<beefy_light_client::ValidatorMerkleProof> = validator_proofs
            .into_iter()
            .map(|validator_proof| beefy_light_client::ValidatorMerkleProof::from(validator_proof))
            .collect();

        assert!(lc
            .update_state(
                &encoded_signed_commitment,
                &validator_proofs,
                &encoded_mmr_leaf,
                &encoded_mmr_proof,
            )
            .is_ok());
        println!("light client: {:?}", lc);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_service() -> Result<(), Box<dyn std::error::Error>> {
        let src_client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        let target_client = ClientBuilder::new()
            .set_url("ws://localhost:9944")
            .build::<ibc_node::DefaultConfig>()
            .await?;
        update_clien_state_service(src_client, target_client).await?;
        Ok(())
    }

    #[test]
    fn should_generate_empty_root() {
        // given
        let _ = env_logger::try_init();
        let data: Vec<[u8; 1]> = Default::default();

        // when
        let out = merkle_root::<Keccak256, _, _>(data);

        // then
        assert_eq!(
            hex::encode(&out),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn should_generate_single_root() {
        // given
        let _ = env_logger::try_init();
        let data = vec![hex!("E04CC55ebEE1cBCE552f250e85c57B70B2E2625b")];

        // when
        let out = merkle_root::<Keccak256, _, _>(data);

        // then
        assert_eq!(
            hex::encode(&out),
            "aeb47a269393297f4b0a3c9c9cfd00c7a4195255274cf39d83dabc2fcc9ff3d7"
        );
    }

    #[test]
    fn should_generate_root_pow_2() {
        // given
        let _ = env_logger::try_init();
        let data = vec![
            hex!("E04CC55ebEE1cBCE552f250e85c57B70B2E2625b"),
            hex!("25451A4de12dcCc2D166922fA938E900fCc4ED24"),
        ];

        // when
        let out = merkle_root::<Keccak256, _, _>(data);

        // then
        assert_eq!(
            hex::encode(&out),
            "697ea2a8fe5b03468548a7a413424a6292ab44a82a6f5cc594c3fa7dda7ce402"
        );
    }

    #[test]
    fn should_generate_root_complex() {
        let _ = env_logger::try_init();
        let test = |root, data| {
            assert_eq!(hex::encode(&merkle_root::<Keccak256, _, _>(data)), root);
        };

        test(
            "aff1208e69c9e8be9b584b07ebac4e48a1ee9d15ce3afe20b77a4d29e4175aa3",
            vec!["a", "b", "c"],
        );

        test(
            "b8912f7269068901f231a965adfefbc10f0eedcfa61852b103efd54dac7db3d7",
            vec!["a", "b", "a"],
        );

        test(
            "dc8e73fe6903148ff5079baecc043983625c23b39f31537e322cd0deee09fa9c",
            vec!["a", "b", "a", "b"],
        );

        test(
            "fb3b3be94be9e983ba5e094c9c51a7d96a4fa2e5d8e891df00ca89ba05bb1239",
            vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"],
        );
    }

    #[test]
    fn should_generate_and_verify_proof_simple() {
        // given
        let _ = env_logger::try_init();
        let data = vec!["a", "b", "c"];

        // when
        let proof0 = merkle_proof::<Keccak256, _, _>(data.clone(), 0);
        assert!(verify_proof::<Keccak256, _, _>(
            &proof0.root,
            proof0.proof.clone(),
            data.len(),
            proof0.leaf_index,
            &proof0.leaf,
        ));

        let proof1 = merkle_proof::<Keccak256, _, _>(data.clone(), 1);
        assert!(verify_proof::<Keccak256, _, _>(
            &proof1.root,
            proof1.proof,
            data.len(),
            proof1.leaf_index,
            &proof1.leaf,
        ));

        let proof2 = merkle_proof::<Keccak256, _, _>(data.clone(), 2);
        assert!(verify_proof::<Keccak256, _, _>(
            &proof2.root,
            proof2.proof,
            data.len(),
            proof2.leaf_index,
            &proof2.leaf
        ));

        // then
        assert_eq!(hex::encode(proof0.root), hex::encode(proof1.root));
        assert_eq!(hex::encode(proof2.root), hex::encode(proof1.root));

        assert!(!verify_proof::<Keccak256, _, _>(
            &hex!("fb3b3be94be9e983ba5e094c9c51a7d96a4fa2e5d8e891df00ca89ba05bb1239"),
            proof0.proof,
            data.len(),
            proof0.leaf_index,
            &proof0.leaf
        ));

        assert!(!verify_proof::<Keccak256, _, _>(
            &proof0.root,
            vec![],
            data.len(),
            proof0.leaf_index,
            &proof0.leaf
        ));
    }

    #[test]
    fn should_generate_and_verify_proof_complex() {
        // given
        let _ = env_logger::try_init();
        let data = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for l in 0..data.len() {
            // when
            let proof = merkle_proof::<Keccak256, _, _>(data.clone(), l);
            // then
            assert!(verify_proof::<Keccak256, _, _>(
                &proof.root,
                proof.proof,
                data.len(),
                proof.leaf_index,
                &proof.leaf
            ));
        }
    }

    #[test]
    fn should_generate_and_verify_proof_large() {
        // given
        let _ = env_logger::try_init();
        let mut data = vec![];
        for i in 1..16 {
            for c in 'a'..'z' {
                if c as usize % i != 0 {
                    data.push(c.to_string());
                }
            }

            for l in 0..data.len() {
                // when
                let proof = merkle_proof::<Keccak256, _, _>(data.clone(), l);
                // then
                assert!(verify_proof::<Keccak256, _, _>(
                    &proof.root,
                    proof.proof,
                    data.len(),
                    proof.leaf_index,
                    &proof.leaf
                ));
            }
        }
    }

    #[test]
    fn should_generate_and_verify_proof_large_tree() {
        // given
        let _ = env_logger::try_init();
        let mut data = vec![];
        for i in 0..6000 {
            data.push(format!("{}", i));
        }

        for l in (0..data.len()).step_by(13) {
            // when
            let proof = merkle_proof::<Keccak256, _, _>(data.clone(), l);
            // then
            assert!(verify_proof::<Keccak256, _, _>(
                &proof.root,
                proof.proof,
                data.len(),
                proof.leaf_index,
                &proof.leaf
            ));
        }
    }

    #[test]
    #[should_panic]
    fn should_panic_on_invalid_leaf_index() {
        let _ = env_logger::try_init();
        merkle_proof::<Keccak256, _, _>(vec!["a"], 5);
    }

    #[test]
    fn should_generate_and_verify_proof_on_test_data() {
        let addresses = vec![
            "0x9aF1Ca5941148eB6A3e9b9C741b69738292C533f",
            "0xDD6ca953fddA25c496165D9040F7F77f75B75002",
            "0x60e9C47B64Bc1C7C906E891255EaEC19123E7F42",
            "0xfa4859480Aa6D899858DE54334d2911E01C070df",
            "0x19B9b128470584F7209eEf65B69F3624549Abe6d",
            "0xC436aC1f261802C4494504A11fc2926C726cB83b",
            "0xc304C8C2c12522F78aD1E28dD86b9947D7744bd0",
            "0xDa0C2Cba6e832E55dE89cF4033affc90CC147352",
            "0xf850Fd22c96e3501Aad4CDCBf38E4AEC95622411",
            "0x684918D4387CEb5E7eda969042f036E226E50642",
            "0x963F0A1bFbb6813C0AC88FcDe6ceB96EA634A595",
            "0x39B38ad74b8bCc5CE564f7a27Ac19037A95B6099",
            "0xC2Dec7Fdd1fef3ee95aD88EC8F3Cd5bd4065f3C7",
            "0x9E311f05c2b6A43C2CCF16fB2209491BaBc2ec01",
            "0x927607C30eCE4Ef274e250d0bf414d4a210b16f0",
            "0x98882bcf85E1E2DFF780D0eB360678C1cf443266",
            "0xFBb50191cd0662049E7C4EE32830a4Cc9B353047",
            "0x963854fc2C358c48C3F9F0A598B9572c581B8DEF",
            "0xF9D7Bc222cF6e3e07bF66711e6f409E51aB75292",
            "0xF2E3fd32D063F8bBAcB9e6Ea8101C2edd899AFe6",
            "0x407a5b9047B76E8668570120A96d580589fd1325",
            "0xEAD9726FAFB900A07dAd24a43AE941d2eFDD6E97",
            "0x42f5C8D9384034A9030313B51125C32a526b6ee8",
            "0x158fD2529Bc4116570Eb7C80CC76FEf33ad5eD95",
            "0x0A436EE2E4dEF3383Cf4546d4278326Ccc82514E",
            "0x34229A215db8FeaC93Caf8B5B255e3c6eA51d855",
            "0xEb3B7CF8B1840242CB98A732BA464a17D00b5dDF",
            "0x2079692bf9ab2d6dc7D79BBDdEE71611E9aA3B72",
            "0x46e2A67e5d450e2Cf7317779f8274a2a630f3C9B",
            "0xA7Ece4A5390DAB18D08201aE18800375caD78aab",
            "0x15E1c0D24D62057Bf082Cb2253dA11Ef0d469570",
            "0xADDEF4C9b5687Eb1F7E55F2251916200A3598878",
            "0xe0B16Fb96F936035db2b5A68EB37D470fED2f013",
            "0x0c9A84993feaa779ae21E39F9793d09e6b69B62D",
            "0x3bc4D5148906F70F0A7D1e2756572655fd8b7B34",
            "0xFf4675C26903D5319795cbd3a44b109E7DDD9fDe",
            "0xCec4450569A8945C6D2Aba0045e4339030128a92",
            "0x85f0584B10950E421A32F471635b424063FD8405",
            "0xb38bEe7Bdc0bC43c096e206EFdFEad63869929E3",
            "0xc9609466274Fef19D0e58E1Ee3b321D5C141067E",
            "0xa08EA868cF75268E7401021E9f945BAe73872ecc",
            "0x67C9Cb1A29E964Fe87Ff669735cf7eb87f6868fE",
            "0x1B6BEF636aFcdd6085cD4455BbcC93796A12F6E2",
            "0x46B37b243E09540b55cF91C333188e7D5FD786dD",
            "0x8E719E272f62Fa97da93CF9C941F5e53AA09e44a",
            "0xa511B7E7DB9cb24AD5c89fBb6032C7a9c2EfA0a5",
            "0x4D11FDcAeD335d839132AD450B02af974A3A66f8",
            "0xB8cf790a5090E709B4619E1F335317114294E17E",
            "0x7f0f57eA064A83210Cafd3a536866ffD2C5eDCB3",
            "0xC03C848A4521356EF800e399D889e9c2A25D1f9E",
            "0xC6b03DF05cb686D933DD31fCa5A993bF823dc4FE",
            "0x58611696b6a8102cf95A32c25612E4cEF32b910F",
            "0x2ed4bC7197AEF13560F6771D930Bf907772DE3CE",
            "0x3C5E58f334306be029B0e47e119b8977B2639eb4",
            "0x288646a1a4FeeC560B349d210263c609aDF649a6",
            "0xb4F4981E0d027Dc2B3c86afA0D0fC03d317e83C0",
            "0xaAE4A87F8058feDA3971f9DEd639Ec9189aA2500",
            "0x355069DA35E598913d8736E5B8340527099960b8",
            "0x3cf5A0F274cd243C0A186d9fCBdADad089821B93",
            "0xca55155dCc4591538A8A0ca322a56EB0E4aD03C4",
            "0xE824D0268366ec5C4F23652b8eD70D552B1F2b8B",
            "0x84C3e9B25AE8a9b39FF5E331F9A597F2DCf27Ca9",
            "0xcA0018e278751De10d26539915d9c7E7503432FE",
            "0xf13077dE6191D6c1509ac7E088b8BE7Fe656c28b",
            "0x7a6bcA1ec9Db506e47ac6FD86D001c2aBc59C531",
            "0xeA7f9A2A9dd6Ba9bc93ca615C3Ddf26973146911",
            "0x8D0d8577e16F8731d4F8712BAbFa97aF4c453458",
            "0xB7a7855629dF104246997e9ACa0E6510df75d0ea",
            "0x5C1009BDC70b0C8Ab2e5a53931672ab448C17c89",
            "0x40B47D1AfefEF5eF41e0789F0285DE7b1C31631C",
            "0x5086933d549cEcEB20652CE00973703CF10Da373",
            "0xeb364f6FE356882F92ae9314fa96116Cf65F47d8",
            "0xdC4D31516A416cEf533C01a92D9a04bbdb85EE67",
            "0x9b36E086E5A274332AFd3D8509e12ca5F6af918d",
            "0xBC26394fF36e1673aE0608ce91A53B9768aD0D76",
            "0x81B5AB400be9e563fA476c100BE898C09966426c",
            "0x9d93C8ae5793054D28278A5DE6d4653EC79e90FE",
            "0x3B8E75804F71e121008991E3177fc942b6c28F50",
            "0xC6Eb5886eB43dD473f5BB4e21e56E08dA464D9B4",
            "0xfdf1277b71A73c813cD0e1a94B800f4B1Db66DBE",
            "0xc2ff2cCc98971556670e287Ff0CC39DA795231ad",
            "0x76b7E1473f0D0A87E9B4a14E2B179266802740f5",
            "0xA7Bc965660a6EF4687CCa4F69A97563163A3C2Ef",
            "0xB9C2b47888B9F8f7D03dC1de83F3F55E738CebD3",
            "0xEd400162E6Dd6bD2271728FFb04176bF770De94a",
            "0xE3E8331156700339142189B6E555DCb2c0962750",
            "0xbf62e342Bc7706a448EdD52AE871d9C4497A53b1",
            "0xb9d7A1A111eed75714a0AcD2dd467E872eE6B03D",
            "0x03942919DFD0383b8c574AB8A701d89fd4bfA69D",
            "0x0Ef4C92355D3c8c7050DFeb319790EFCcBE6fe9e",
            "0xA6895a3cf0C60212a73B3891948ACEcF1753f25E",
            "0x0Ed509239DB59ef3503ded3d31013C983d52803A",
            "0xc4CE8abD123BfAFc4deFf37c7D11DeCd5c350EE4",
            "0x4A4Bf59f7038eDcd8597004f35d7Ee24a7Bdd2d3",
            "0x5769E8e8A2656b5ed6b6e6fa2a2bFAeaf970BB87",
            "0xf9E15cCE181332F4F57386687c1776b66C377060",
            "0xc98f8d4843D56a46C21171900d3eE538Cc74dbb5",
            "0x3605965B47544Ce4302b988788B8195601AE4dEd",
            "0xe993BDfdcAac2e65018efeE0F69A12678031c71d",
            "0x274fDf8801385D3FAc954BCc1446Af45f5a8304c",
            "0xBFb3f476fcD6429F4a475bA23cEFdDdd85c6b964",
            "0x806cD16588Fe812ae740e931f95A289aFb4a4B50",
            "0xa89488CE3bD9C25C3aF797D1bbE6CA689De79d81",
            "0xd412f1AfAcf0Ebf3Cd324593A231Fc74CC488B12",
            "0xd1f715b2D7951d54bc31210BbD41852D9BF98Ed1",
            "0xf65aD707c344171F467b2ADba3d14f312219cE23",
            "0x2971a4b242e9566dEF7bcdB7347f5E484E11919B",
            "0x12b113D6827E07E7D426649fBd605f427da52314",
            "0x1c6CA45171CDb9856A6C9Dba9c5F1216913C1e97",
            "0x11cC6ee1d74963Db23294FCE1E3e0A0555779CeA",
            "0x8Aa1C721255CDC8F895E4E4c782D86726b068667",
            "0xA2cDC1f37510814485129aC6310b22dF04e9Bbf0",
            "0xCf531b71d388EB3f5889F1f78E0d77f6fb109767",
            "0xBe703e3545B2510979A0cb0C440C0Fba55c6dCB5",
            "0x30a35886F989db39c797D8C93880180Fdd71b0c8",
            "0x1071370D981F60c47A9Cd27ac0A61873a372cBB2",
            "0x3515d74A11e0Cb65F0F46cB70ecf91dD1712daaa",
            "0x50500a3c2b7b1229c6884505D00ac6Be29Aecd0C",
            "0x9A223c2a11D4FD3585103B21B161a2B771aDA3d1",
            "0xd7218df03AD0907e6c08E707B15d9BD14285e657",
            "0x76CfD72eF5f93D1a44aD1F80856797fBE060c70a",
            "0x44d093cB745944991EFF5cBa151AA6602d6f5420",
            "0x626516DfF43bf09A71eb6fd1510E124F96ED0Cde",
            "0x6530824632dfe099304E2DC5701cA99E6d031E08",
            "0x57e6c423d6a7607160d6379A0c335025A14DaFC0",
            "0x3966D4AD461Ef150E0B10163C81E79b9029E69c3",
            "0xF608aCfd0C286E23721a3c347b2b65039f6690F1",
            "0xbfB8FAac31A25646681936977837f7740fCd0072",
            "0xd80aa634a623a7ED1F069a1a3A28a173061705c7",
            "0x9122a77B36363e24e12E1E2D73F87b32926D3dF5",
            "0x62562f0d1cD31315bCCf176049B6279B2bfc39C2",
            "0x48aBF7A2a7119e5675059E27a7082ba7F38498b2",
            "0xb4596983AB9A9166b29517acD634415807569e5F",
            "0x52519D16E20BC8f5E96Da6d736963e85b2adA118",
            "0x7663893C3dC0850EfC5391f5E5887eD723e51B83",
            "0x5FF323a29bCC3B5b4B107e177EccEF4272959e61",
            "0xee6e499AdDf4364D75c05D50d9344e9daA5A9AdF",
            "0x1631b0BD31fF904aD67dD58994C6C2051CDe4E75",
            "0xbc208e9723D44B9811C428f6A55722a26204eEF2",
            "0xe76103a222Ee2C7Cf05B580858CEe625C4dc00E1",
            "0xC71Bb2DBC51760f4fc2D46D84464410760971B8a",
            "0xB4C18811e6BFe564D69E12c224FFc57351f7a7ff",
            "0xD11DB0F5b41061A887cB7eE9c8711438844C298A",
            "0xB931269934A3D4432c084bAAc3d0de8143199F4f",
            "0x070037cc85C761946ec43ea2b8A2d5729908A2a1",
            "0x2E34aa8C95Ffdbb37f14dCfBcA69291c55Ba48DE",
            "0x052D93e8d9220787c31d6D83f87eC7dB088E998f",
            "0x498dAC6C69b8b9ad645217050054840f1D91D029",
            "0xE4F7D60f9d84301e1fFFd01385a585F3A11F8E89",
            "0xEa637992f30eA06460732EDCBaCDa89355c2a107",
            "0x4960d8Da07c27CB6Be48a79B96dD70657c57a6bF",
            "0x7e471A003C8C9fdc8789Ded9C3dbe371d8aa0329",
            "0xd24265Cc10eecb9e8d355CCc0dE4b11C556E74D7",
            "0xDE59C8f7557Af779674f41CA2cA855d571018690",
            "0x2fA8A6b3b6226d8efC9d8f6EBDc73Ca33DDcA4d8",
            "0xe44102664c6c2024673Ff07DFe66E187Db77c65f",
            "0x94E3f4f90a5f7CBF2cc2623e66B8583248F01022",
            "0x0383EdBbc21D73DEd039E9C1Ff6bf56017b4CC40",
            "0x64C3E49898B88d1E0f0d02DA23E0c00A2Cd0cA99",
            "0xF4ccfB67b938d82B70bAb20975acFAe402E812E1",
            "0x4f9ee5829e9852E32E7BC154D02c91D8E203e074",
            "0xb006312eF9713463bB33D22De60444Ba95609f6B",
            "0x7Cbe76ef69B52110DDb2e3b441C04dDb11D63248",
            "0x70ADEEa65488F439392B869b1Df7241EF317e221",
            "0x64C0bf8AA36Ba590477585Bc0D2BDa7970769463",
            "0xA4cDc98593CE52d01Fe5Ca47CB3dA5320e0D7592",
            "0xc26B34D375533fFc4c5276282Fa5D660F3d8cbcB",
        ];
        let root = hex!("72b0acd7c302a84f1f6b6cefe0ba7194b7398afb440e1b44a9dbbe270394ca53");

        let data = addresses
            .into_iter()
            .map(|address| hex::decode(&address[2..]).unwrap())
            .collect::<Vec<_>>();

        for l in 0..data.len() {
            // when
            let proof = merkle_proof::<Keccak256, _, _>(data.clone(), l);
            assert_eq!(hex::encode(&proof.root), hex::encode(&root));
            assert_eq!(proof.leaf_index, l);
            assert_eq!(&proof.leaf, &data[l]);

            // then
            assert!(verify_proof::<Keccak256, _, _>(
                &proof.root,
                proof.proof,
                data.len(),
                proof.leaf_index,
                &proof.leaf
            ));
        }

        let proof = merkle_proof::<Keccak256, _, _>(data.clone(), data.len() - 1);

        assert_eq!(
            proof,
            MerkleProof {
                root,
                proof: vec![
                    hex!("340bcb1d49b2d82802ddbcf5b85043edb3427b65d09d7f758fbc76932ad2da2f"),
                    hex!("ba0580e5bd530bc93d61276df7969fb5b4ae8f1864b4a28c280249575198ff1f"),
                    hex!("d02609d2bbdb28aa25f58b85afec937d5a4c85d37925bce6d0cf802f9d76ba79"),
                    hex!("ae3f8991955ed884613b0a5f40295902eea0e0abe5858fc520b72959bc016d4e"),
                ],
                number_of_leaves: data.len(),
                leaf_index: data.len() - 1,
                leaf: hex!("c26B34D375533fFc4c5276282Fa5D660F3d8cbcB").to_vec(),
            }
        );
    }

    #[test]
    fn should_generate_and_verify_proof_on_signatures() {
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
}
