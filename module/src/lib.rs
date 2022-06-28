// mod codegen;
#![allow(clippy::too_many_arguments)]

use subxt::{Config, DefaultConfig};

pub mod events;
pub mod ibc_core;
pub mod primitive;
pub mod update_client_state;

pub use ibc_core::*;
pub use update_client_state::*;

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a substrate node template.
pub type SubstrateNodeTemplateExtrinsicParams<T> =
    subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;

/// A builder which leads to [`SubstrateNodeTemplateExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type SubstrateNodeTemplateExtrinsicParamsBuilder<T> =
    subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;

#[subxt::subxt(runtime_metadata_path = "metadata_file/metadata.scale")]
pub mod ibc_node {

    #[subxt(substitute_type = "beefy_primitives::crypto::Public")]
    use beefy_primitives::crypto::Public;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MyConfig;
impl Config for MyConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <DefaultConfig as Config>::BlockNumber;
    type Hash = <DefaultConfig as Config>::Hash;
    type Hashing = <DefaultConfig as Config>::Hashing;
    type AccountId = <DefaultConfig as Config>::AccountId;
    type Address = <DefaultConfig as Config>::Address;
    type Header = <DefaultConfig as Config>::Header;
    type Signature = <DefaultConfig as Config>::Signature;
    type Extrinsic = <DefaultConfig as Config>::Extrinsic;
}
