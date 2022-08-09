use crate::ibc_core::Router;
use crate::{ibc_node, OctopusxtClient};
use async_trait::async_trait;
use ibc_proto::google::protobuf::Any;
use sp_core::H256;
use sp_keyring::AccountKeyring;
use subxt::PairSigner;

#[async_trait]
impl Router for OctopusxtClient {
    type Error = anyhow::Error;

    async fn deliver(&self, msg: Vec<Any>) -> Result<H256, Self::Error> {
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
