use crate::ibc_core::Router;
use crate::{ibc_node, OctopusxtClient};
use anyhow::Result;
use ibc_proto::google::protobuf::Any;
use sp_core::H256;
use sp_keyring::AccountKeyring;
use std::future::Future;
use subxt::PairSigner;
use async_trait::async_trait;

#[async_trait]
impl Router for OctopusxtClient {
    /// ibc protocol core function, ics26 deliver function
    /// this function will dispatch msg to process
    ///
    ///  # Usage example
    ///
    /// ```rust
    /// use subxt::ClientBuilder;
    /// use octopusxt::MyConfig;
    /// use ibc_proto::google::protobuf::Any;
    /// use octopusxt::deliver;
    ///
    /// let client = ClientBuilder::new().set_url("ws://localhost:9944").build::<MyConfig>().await?;
    /// let msg = vec![Any::default()];
    /// let result = deliver(msg, client).await?;
    /// ```
    /// return block_hash, extrinsic_hash, and event
    async fn deliver(&self, msg: Vec<Any>) -> Result<H256> {
        tracing::info!("in call_ibc: [deliver]");

        let msg: Vec<ibc_node::runtime_types::pallet_ibc::Any> = msg
            .into_iter()
            .map(|value| ibc_node::runtime_types::pallet_ibc::Any {
                type_url: value.type_url.as_bytes().to_vec(),
                value: value.value,
            })
            .collect();

        let signer = PairSigner::new(AccountKeyring::Bob.pair());

        let api = self.to_runtime_api();

        let result = api
            .tx()
            .ibc()
            .deliver(msg, 0)?
            .sign_and_submit_default(&signer)
            .await?;

        Ok(result)
    }
}
