#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 4)]
        Balances(balances::Event),
        #[codec(index = 6)]
        OctopusAppchain(octopus_appchain::Event),
        #[codec(index = 7)]
        OctopusLpos(octopus_lpos::Event),
        #[codec(index = 8)]
        OctopusUpwardMessages(octopus_upward_messages::Event),
        #[codec(index = 9)]
        OctopusAssets(octopus_assets::Event),
        #[codec(index = 10)]
        Session(session::Event),
        #[codec(index = 11)]
        Grandpa(grandpa::Event),
        #[codec(index = 12)]
        ImOnline(im_online::Event),
        #[codec(index = 17)]
        Uniques(uniques::Event),
        #[codec(index = 18)]
        Sudo(sudo::Event),
        #[codec(index = 19)]
        Ibc(ibc::Event),
        #[codec(index = 20)]
        TemplateModule(template_module::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    FillBlock,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Remark, DispatchError, root_mod::Event>
                {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetHeapPages,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetCode, DispatchError, root_mod::Event>
                {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetCodeWithoutChecks,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetStorage,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    KillStorage,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    KillPrefix,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    RemarkWithEvent,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::node_template_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BlockHash<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::BasicError,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::node_template_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: &::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EventTopics<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("BlockWeights")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("BlockLength")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("BlockHashCount")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("DbWeight")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("Version")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("SS58Prefix")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod babe {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PlanConfigChange {
                pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            impl ::subxt::Call for PlanConfigChange {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ReportEquivocation,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ReportEquivocationUnsigned,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    PlanConfigChange,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = PlanConfigChange { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction<'_> {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    [::core::primitive::u8; 32usize],
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = GenesisSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_epoch_config_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::BasicError,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::BasicError,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn epoch_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Babe")?;
                    let constant = pallet.constant("EpochDuration")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn expected_block_time(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Babe")?;
                    let constant = pallet.constant("ExpectedBlockTime")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Babe")?;
                    let constant = pallet.constant("MaxAuthorities")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Set, DispatchError, root_mod::Event>
                {
                    let call = Set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Timestamp")?;
                    let constant = pallet.constant("MinimumPeriod")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod authorship {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for SetUncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetUncles,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetUncles { new_uncles };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Uncles;
            impl ::subxt::StorageEntry for Uncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Uncles";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Author";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetUncles;
            impl ::subxt::StorageEntry for DidSetUncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "DidSetUncles";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn uncle_generations(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Authorship")?;
                    let constant = pallet.constant("UncleGenerations")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetBalance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferKeepAlive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceUnreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
                {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetBalance,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetBalance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceTransfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceTransfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    TransferKeepAlive,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    TransferAll,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceUnreserve,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReserveRepatriated {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Slashed {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Balances")?;
                    let constant = pallet.constant("ExistentialDeposit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Balances")?;
                    let constant = pallet.constant("MaxLocks")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Balances")?;
                    let constant = pallet.constant("MaxReserves")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::BasicError,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("TransactionPayment")?;
                    let constant = pallet.constant("TransactionByteFee")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("TransactionPayment")?;
                    let constant = pallet.constant("OperationalFeeMultiplier")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn weight_to_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_support::weights::WeightToFeeCoefficient<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("TransactionPayment")?;
                    let constant = pallet.constant("WeightToFee")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod octopus_appchain {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SubmitObservations {
                pub payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                    runtime_types::sp_runtime::MultiSigner,
                    ::core::primitive::u32,
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub signature: runtime_types::sp_runtime::MultiSignature,
            }
            impl ::subxt::Call for SubmitObservations {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "submit_observations";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceSetIsActivated {
                pub is_activated: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceSetIsActivated {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_is_activated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ForceSetNextSetId {
                pub next_set_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceSetNextSetId {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_next_set_id";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceSetPlannedValidators {
                pub validators: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::Call for ForceSetPlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_planned_validators";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Lock {
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Lock {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "lock";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MintAsset {
                pub asset_id: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for MintAsset {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "mint_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BurnAsset {
                pub asset_id: ::core::primitive::u32,
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for BurnAsset {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "burn_asset";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_observations(
                    &self,
                    payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    signature: runtime_types::sp_runtime::MultiSignature,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SubmitObservations,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitObservations { payload, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_is_activated(
                    &self,
                    is_activated: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceSetIsActivated,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceSetIsActivated { is_activated };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_next_set_id(
                    &self,
                    next_set_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceSetNextSetId,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceSetNextSetId { next_set_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_planned_validators(
                    &self,
                    validators: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceSetPlannedValidators,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceSetPlannedValidators { validators };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn lock(
                    &self,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Lock, DispatchError, root_mod::Event>
                {
                    let call = Lock {
                        receiver_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    sender_id: ::std::vec::Vec<::core::primitive::u8>,
                    receiver: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    MintAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = MintAsset {
                        asset_id,
                        sender_id,
                        receiver,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    BurnAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = BurnAsset {
                        asset_id,
                        receiver_id,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_appchain::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewPlannedValidators(
                pub ::core::primitive::u32,
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            );
            impl ::subxt::Event for NewPlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NewPlannedValidators";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Locked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u128,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Locked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Locked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Unlocked(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unlocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnlockFailed(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UnlockFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "UnlockFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AssetMinted(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetMinted {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMinted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AssetBurned(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetBurned {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetBurned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AssetMintFailed(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetMintFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMintFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AssetIdGetFailed(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetIdGetFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetIdGetFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AnchorContract;
            impl ::subxt::StorageEntry for AnchorContract {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "AnchorContract";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssetIdByName<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for AssetIdByName<'_> {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "AssetIdByName";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct IsActivated;
            impl ::subxt::StorageEntry for IsActivated {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "IsActivated";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextSetId;
            impl ::subxt::StorageEntry for NextSetId {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NextSetId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PlannedValidators;
            impl ::subxt::StorageEntry for PlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "PlannedValidators";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextNotificationId;
            impl ::subxt::StorageEntry for NextNotificationId {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NextNotificationId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Observations<'a>(
                pub &'a runtime_types::pallet_octopus_appchain::ObservationType,
                pub &'a ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Observations<'_> {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "Observations";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_octopus_appchain::Observation<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct Observing<'a>(
                pub  &'a runtime_types::pallet_octopus_appchain::Observation<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::StorageEntry for Observing<'_> {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "Observing";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PalletAccount;
            impl ::subxt::StorageEntry for PalletAccount {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "PalletAccount";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NotificationHistory<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for NotificationHistory<'_> {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NotificationHistory";
                type Value = runtime_types::pallet_octopus_appchain::NotificationResult;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn anchor_contract(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = AnchorContract;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_id_by_name(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AssetIdByName(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_id_by_name_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIdByName<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn is_activated(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = IsActivated;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = NextSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn planned_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = PlannedValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_notification_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = NextNotificationId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observations(
                    &self,
                    _0: &runtime_types::pallet_octopus_appchain::ObservationType,
                    _1: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_octopus_appchain::Observation<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Observations(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observations_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Observations<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn observing(
                    &self,
                    _0: &runtime_types::pallet_octopus_appchain::Observation<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Observing(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observing_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Observing<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn pallet_account(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = PalletAccount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn notification_history(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_octopus_appchain::NotificationResult,
                    ::subxt::BasicError,
                > {
                    let entry = NotificationHistory(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn notification_history_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NotificationHistory<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn grace_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAppchain")?;
                    let constant = pallet.constant("GracePeriod")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAppchain")?;
                    let constant = pallet.constant("UnsignedPriority")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn request_event_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAppchain")?;
                    let constant = pallet.constant("RequestEventLimit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod octopus_lpos {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHistoryDepth {
                const PALLET: &'static str = "OctopusLpos";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ForceSetEraPayout {
                pub era_payout: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceSetEraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const FUNCTION: &'static str = "force_set_era_payout";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetHistoryDepth,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetHistoryDepth {
                        new_history_depth,
                        era_items_deleted,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_era_payout(
                    &self,
                    era_payout: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceSetEraPayout,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceSetEraPayout { era_payout };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_lpos::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct PlanNewEra(pub ::core::primitive::u32);
            impl ::subxt::Event for PlanNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEra";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PlanNewEraFailed;
            impl ::subxt::Event for PlanNewEraFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEraFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TriggerNewEra;
            impl ::subxt::Event for TriggerNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "TriggerNewEra";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EraPayout(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayout";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct EraPayoutFailed(pub ::core::primitive::u32);
            impl ::subxt::Event for EraPayoutFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayoutFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::pallet_octopus_lpos::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex<'_> {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasStakers<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers<'_> {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasStakers";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasValidatorReward<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward<'_> {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasRewardPoints<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints<'_> {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::pallet_octopus_lpos::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasTotalStake<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake<'_> {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "BondedEras";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EraPayout;
            impl ::subxt::StorageEntry for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "EraPayout";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_octopus_lpos::ActiveEraInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakers<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_octopus_lpos::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasRewardPoints<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasTotalStake<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn era_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = EraPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sessions_per_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusLpos")?;
                    let constant = pallet.constant("SessionsPerEra")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn blocks_per_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusLpos")?;
                    let constant = pallet.constant("BlocksPerEra")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn bonding_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusLpos")?;
                    let constant = pallet.constant("BondingDuration")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod octopus_upward_messages {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_upward_messages::pallet::Event;
        pub mod events {
            use super::runtime_types;
        }
        pub mod storage {
            use super::runtime_types;
            pub struct MessageQueue;
            impl ::subxt::StorageEntry for MessageQueue {
                const PALLET: &'static str = "OctopusUpwardMessages";
                const STORAGE: &'static str = "MessageQueue";
                type Value =
                    ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nonce;
            impl ::subxt::StorageEntry for Nonce {
                const PALLET: &'static str = "OctopusUpwardMessages";
                const STORAGE: &'static str = "Nonce";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn message_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>,
                    ::subxt::BasicError,
                > {
                    let entry = MessageQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nonce(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = Nonce;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn upward_messages_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusUpwardMessages")?;
                    let constant = pallet.constant("UpwardMessagesLimit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod octopus_assets {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Create {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceCreate {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub is_sufficient: ::core::primitive::bool,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceCreate {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Destroy {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub witness: runtime_types::pallet_assets::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Mint {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub beneficiary:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Mint {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Burn {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Burn {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferKeepAlive {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Freeze {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Thaw {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct FreezeAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for FreezeAsset {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "freeze_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ThawAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ThawAsset {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "thaw_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for TransferOwnership {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetTeam {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetTeam {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceSetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceSetMetadata {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "force_set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceClearMetadata {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "force_clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
                pub is_sufficient: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceAssetStatus {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ApproveTransfer {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for CancelApproval {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceCancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for ForceCancelApproval {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "force_cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferApproved {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub destination:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferApproved {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "transfer_approved";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn create(
                    &self,
                    id: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Create, DispatchError, root_mod::Event>
                {
                    let call = Create {
                        id,
                        admin,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    is_sufficient: ::core::primitive::bool,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceCreate,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceCreate {
                        id,
                        owner,
                        is_sufficient,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    id: ::core::primitive::u32,
                    witness: runtime_types::pallet_assets::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Destroy, DispatchError, root_mod::Event>
                {
                    let call = Destroy { id, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Mint, DispatchError, root_mod::Event>
                {
                    let call = Mint {
                        id,
                        beneficiary,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Burn, DispatchError, root_mod::Event>
                {
                    let call = Burn { id, who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
                {
                    let call = Transfer { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    TransferKeepAlive,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferKeepAlive { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceTransfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceTransfer {
                        id,
                        source,
                        dest,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Freeze, DispatchError, root_mod::Event>
                {
                    let call = Freeze { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Thaw, DispatchError, root_mod::Event>
                {
                    let call = Thaw { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    FreezeAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = FreezeAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ThawAsset,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ThawAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    TransferOwnership,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferOwnership { id, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    id: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetTeam, DispatchError, root_mod::Event>
                {
                    let call = SetTeam {
                        id,
                        issuer,
                        admin,
                        freezer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ClearMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceSetMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceSetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceClearMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                    is_sufficient: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceAssetStatus,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceAssetStatus {
                        id,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        min_balance,
                        is_sufficient,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ApproveTransfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ApproveTransfer {
                        id,
                        delegate,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    CancelApproval,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = CancelApproval { id, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceCancelApproval,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceCancelApproval {
                        id,
                        owner,
                        delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_approved(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    destination: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    TransferApproved,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferApproved {
                        id,
                        owner,
                        destination,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Created {
                pub asset_id: ::core::primitive::u32,
                pub creator: ::subxt::sp_core::crypto::AccountId32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Issued {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub total_supply: ::core::primitive::u128,
            }
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transferred {
                pub asset_id: ::core::primitive::u32,
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Burned {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TeamChanged {
                pub asset_id: ::core::primitive::u32,
                pub issuer: ::subxt::sp_core::crypto::AccountId32,
                pub admin: ::subxt::sp_core::crypto::AccountId32,
                pub freezer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OwnerChanged {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Frozen {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Thawed {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetFrozen {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetFrozen {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetThawed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetThawed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Destroyed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceCreated {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MetadataSet {
                pub asset_id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct MetadataCleared {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApprovedTransfer {
                pub asset_id: ::core::primitive::u32,
                pub source: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApprovalCancelled {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferredApproved {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub destination: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for TransferredApproved {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetStatusChanged {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Asset<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset<'_> {
                const PALLET: &'static str = "OctopusAssets";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_assets::types::AssetDetails<
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "OctopusAssets";
                const STORAGE: &'static str = "Account";
                type Value =
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u128, ()>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Approvals<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Approvals<'_> {
                const PALLET: &'static str = "OctopusAssets";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types::pallet_assets::types::Approval<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Metadata<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for Metadata<'_> {
                const PALLET: &'static str = "OctopusAssets";
                const STORAGE: &'static str = "Metadata";
                type Value = runtime_types::pallet_assets::types::AssetMetadata<
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn asset(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Asset(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u128, ()>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    _2: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Approvals(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approvals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Approvals<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn metadata(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Metadata(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn metadata_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Metadata<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn asset_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAssets")?;
                    let constant = pallet.constant("AssetDeposit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAssets")?;
                    let constant = pallet.constant("MetadataDepositBase")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAssets")?;
                    let constant = pallet.constant("MetadataDepositPerByte")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn approval_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAssets")?;
                    let constant = pallet.constant("ApprovalDeposit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn string_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("OctopusAssets")?;
                    let constant = pallet.constant("StringLimit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetKeys {
                pub keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PurgeKeys;
            impl ::subxt::Call for PurgeKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_keys(
                    &self,
                    keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetKeys, DispatchError, root_mod::Event>
                {
                    let call = SetKeys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    PurgeKeys,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = PurgeKeys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::node_template_runtime::opaque::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys<'_> {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value = runtime_types::node_template_runtime::opaque::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner<'a>(
                pub &'a runtime_types::sp_core::crypto::KeyTypeId,
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for KeyOwner<'_> {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &(&self.0, &self.1),
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextKeys<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: &runtime_types::sp_core::crypto::KeyTypeId,
                    _1: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, KeyOwner<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ReportEquivocation,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ReportEquivocationUnsigned,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    NoteStalled,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = NoteStalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession<'a>(pub &'a ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession<'_> {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Grandpa")?;
                    let constant = pallet.constant("MaxAuthorities")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod im_online {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            impl ::subxt::Call for Heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    Heartbeat,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Heartbeat {
                        heartbeat,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct HeartbeatReceived {
                pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            }
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AllGood;
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SomeOffline {
                pub offline: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for ReceivedHeartbeats<'_> {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
                    runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct AuthoredBlocks<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks<'_> {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthoredBlocks<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("ImOnline")?;
                    let constant = pallet.constant("UnsignedPriority")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod historical {
        use super::root_mod;
        use super::runtime_types;
    }
    pub mod mmr {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RootHash;
            impl ::subxt::StorageEntry for RootHash {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "RootHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NumberOfLeaves;
            impl ::subxt::StorageEntry for NumberOfLeaves {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "NumberOfLeaves";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nodes<'a>(pub &'a ::core::primitive::u64);
            impl ::subxt::StorageEntry for Nodes<'_> {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "Nodes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn root_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = RootHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn number_of_leaves(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = NumberOfLeaves;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nodes(
                    &self,
                    _0: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = Nodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nodes<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod beefy {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "Authorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSetId;
            impl ::subxt::StorageEntry for ValidatorSetId {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "ValidatorSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = ValidatorSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod mmr_leaf {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct BeefyNextAuthorities;
            impl ::subxt::StorageEntry for BeefyNextAuthorities {
                const PALLET: &'static str = "MmrLeaf";
                const STORAGE: &'static str = "BeefyNextAuthorities";
                type Value = runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn beefy_next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = BeefyNextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod uniques {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Create {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceCreate {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub free_holding: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceCreate {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Destroy {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub witness: runtime_types::pallet_uniques::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Mint {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Mint {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Burn {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub check_owner: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for Burn {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Redeposit {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub instances: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for Redeposit {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "redeposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Freeze {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Thaw {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct FreezeClass {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for FreezeClass {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "freeze_class";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ThawClass {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for ThawClass {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "thaw_class";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for TransferOwnership {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetTeam {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetTeam {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for ApproveTransfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CancelApproval {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub maybe_check_delegate: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for CancelApproval {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub free_holding: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceAssetStatus {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetAttribute {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for SetAttribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_attribute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ClearAttribute {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for ClearAttribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_attribute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for SetClassMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_class_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ClearClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearClassMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_class_metadata";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn create(
                    &self,
                    class: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Create, DispatchError, root_mod::Event>
                {
                    let call = Create { class, admin };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    free_holding: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceCreate,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceCreate {
                        class,
                        owner,
                        free_holding,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    class: ::core::primitive::u32,
                    witness: runtime_types::pallet_uniques::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Destroy, DispatchError, root_mod::Event>
                {
                    let call = Destroy { class, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Mint, DispatchError, root_mod::Event>
                {
                    let call = Mint {
                        class,
                        instance,
                        owner,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    check_owner: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Burn, DispatchError, root_mod::Event>
                {
                    let call = Burn {
                        class,
                        instance,
                        check_owner,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
                {
                    let call = Transfer {
                        class,
                        instance,
                        dest,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn redeposit(
                    &self,
                    class: ::core::primitive::u32,
                    instances: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    Redeposit,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Redeposit { class, instances };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Freeze, DispatchError, root_mod::Event>
                {
                    let call = Freeze { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Thaw, DispatchError, root_mod::Event>
                {
                    let call = Thaw { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_class(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    FreezeClass,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = FreezeClass { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_class(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ThawClass,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ThawClass { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    TransferOwnership,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferOwnership { class, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    class: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetTeam, DispatchError, root_mod::Event>
                {
                    let call = SetTeam {
                        class,
                        issuer,
                        admin,
                        freezer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ApproveTransfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ApproveTransfer {
                        class,
                        instance,
                        delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    maybe_check_delegate: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    CancelApproval,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = CancelApproval {
                        class,
                        instance,
                        maybe_check_delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    free_holding: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceAssetStatus,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceAssetStatus {
                        class,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        free_holding,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_attribute(
                    &self,
                    class: ::core::primitive::u32,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetAttribute,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetAttribute {
                        class,
                        maybe_instance,
                        key,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_attribute(
                    &self,
                    class: ::core::primitive::u32,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ClearAttribute,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ClearAttribute {
                        class,
                        maybe_instance,
                        key,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetMetadata {
                        class,
                        instance,
                        data,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ClearMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ClearMetadata { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_class_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetClassMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetClassMetadata {
                        class,
                        data,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_class_metadata(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ClearClassMetadata,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ClearClassMetadata { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_uniques::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Created {
                pub class: ::core::primitive::u32,
                pub creator: ::subxt::sp_core::crypto::AccountId32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ForceCreated {
                pub class: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Destroyed {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Issued {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transferred {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Burned {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Frozen {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Thawed {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ClassFrozen {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for ClassFrozen {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassFrozen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ClassThawed {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for ClassThawed {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassThawed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OwnerChanged {
                pub class: ::core::primitive::u32,
                pub new_owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TeamChanged {
                pub class: ::core::primitive::u32,
                pub issuer: ::subxt::sp_core::crypto::AccountId32,
                pub admin: ::subxt::sp_core::crypto::AccountId32,
                pub freezer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApprovedTransfer {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ApprovalCancelled {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetStatusChanged {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "AssetStatusChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ClassMetadataSet {
                pub class: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for ClassMetadataSet {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassMetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ClassMetadataCleared {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for ClassMetadataCleared {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassMetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MetadataSet {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MetadataCleared {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Redeposited {
                pub class: ::core::primitive::u32,
                pub successful_instances: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Event for Redeposited {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Redeposited";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AttributeSet {
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Event for AttributeSet {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "AttributeSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AttributeCleared {
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Event for AttributeCleared {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "AttributeCleared";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Class<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for Class<'_> {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Class";
                type Value = runtime_types::pallet_uniques::types::ClassDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a ::core::primitive::u32,
                pub &'a ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Account";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Asset<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Asset<'_> {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_uniques::types::InstanceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ClassMetadataOf<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ClassMetadataOf<'_> {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "ClassMetadataOf";
                type Value =
                    runtime_types::pallet_uniques::types::ClassMetadata<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct InstanceMetadataOf<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for InstanceMetadataOf<'_> {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "InstanceMetadataOf";
                type Value =
                    runtime_types::pallet_uniques::types::InstanceMetadata<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Attribute<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::core::option::Option<::core::primitive::u32>,
                pub  &'a runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            );
            impl ::subxt::StorageEntry for Attribute<'_> {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Attribute";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn class(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Class(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn class_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Class<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    _1: &::core::primitive::u32,
                    _2: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = Account(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Asset(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn class_metadata_of(
                    &self,
                    _0: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ClassMetadataOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn class_metadata_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClassMetadataOf<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn instance_metadata_of(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = InstanceMetadataOf(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn instance_metadata_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InstanceMetadataOf<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn attribute(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::core::option::Option<::core::primitive::u32>,
                    _2: &runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Attribute(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn attribute_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Attribute<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn class_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("ClassDeposit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn instance_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("InstanceDeposit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("MetadataDepositBase")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn attribute_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("AttributeDepositBase")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("DepositPerByte")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn string_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("StringLimit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn key_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("KeyLimit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
                pub fn value_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let pallet = self.client.metadata().pallet("Uniques")?;
                    let constant = pallet.constant("ValueLimit")?;
                    let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                    Ok(value)
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SudoAs {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Sudo, DispatchError, root_mod::Event>
                {
                    let call = Sudo {
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SudoUncheckedWeight,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SudoUncheckedWeight {
                        call: ::std::boxed::Box::new(call),
                        weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetKey, DispatchError, root_mod::Event>
                {
                    let call = SetKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SudoAs, DispatchError, root_mod::Event>
                {
                    let call = SudoAs {
                        who,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KeyChanged {
                pub new_sudoer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod ibc {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Deliver {
                pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                pub tmp: ::core::primitive::u8,
            }
            impl ::subxt::Call for Deliver {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "deliver";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UpdateClientState {
                pub client_id: ::std::vec::Vec<::core::primitive::u8>,
                pub mmr_root: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for UpdateClientState {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "update_client_state";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer {
                pub source_port: ::std::vec::Vec<::core::primitive::u8>,
                pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
                pub token: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
                pub receiver: ::std::vec::Vec<::core::primitive::u8>,
                pub timeout_height: ::core::primitive::u64,
                pub timeout_timestamp: ::core::primitive::u64,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "transfer";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn deliver(
                    &self,
                    messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                    tmp: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Deliver, DispatchError, root_mod::Event>
                {
                    let call = Deliver { messages, tmp };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_client_state(
                    &self,
                    client_id: ::std::vec::Vec<::core::primitive::u8>,
                    mmr_root: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    UpdateClientState,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = UpdateClientState {
                        client_id,
                        mmr_root,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    source_port: ::std::vec::Vec<::core::primitive::u8>,
                    source_channel: ::std::vec::Vec<::core::primitive::u8>,
                    token: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                    receiver: ::std::vec::Vec<::core::primitive::u8>,
                    timeout_height: ::core::primitive::u64,
                    timeout_timestamp: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
                {
                    let call = Transfer {
                        source_port,
                        source_channel,
                        token,
                        amount,
                        receiver,
                        timeout_height,
                        timeout_timestamp,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_ibc::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewBlock(pub runtime_types::pallet_ibc::event::primitive::Height);
            impl ::subxt::Event for NewBlock {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "NewBlock";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CreateClient(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for CreateClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CreateClient";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UpdateClient(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for UpdateClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpdateClient";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UpdateClientState(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientState,
            );
            impl ::subxt::Event for UpdateClientState {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpdateClientState";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UpgradeClient(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for UpgradeClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpgradeClient";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ClientMisbehaviour(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub runtime_types::pallet_ibc::event::primitive::ClientType,
                pub runtime_types::pallet_ibc::event::primitive::Height,
            );
            impl ::subxt::Event for ClientMisbehaviour {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ClientMisbehaviour";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenInitConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenInitConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenInitConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenTryConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenTryConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenTryConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenAckConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenAckConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenAckConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenConfirmConnection(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
                pub  ::core::option::Option<
                    runtime_types::pallet_ibc::event::primitive::ConnectionId,
                >,
                pub runtime_types::pallet_ibc::event::primitive::ClientId,
            );
            impl ::subxt::Event for OpenConfirmConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenConfirmConnection";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenInitChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenInitChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenInitChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenTryChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenTryChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenTryChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenAckChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenAckChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenAckChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenConfirmChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for OpenConfirmChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenConfirmChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CloseInitChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for CloseInitChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CloseInitChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CloseConfirmChannel(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
                pub runtime_types::pallet_ibc::event::primitive::ConnectionId,
                pub runtime_types::pallet_ibc::event::primitive::PortId,
                pub ::core::option::Option<runtime_types::pallet_ibc::event::primitive::ChannelId>,
            );
            impl ::subxt::Event for CloseConfirmChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CloseConfirmChannel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SendPacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for SendPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "SendPacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReceivePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for ReceivePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ReceivePacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct WriteAcknowledgement(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for WriteAcknowledgement {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "WriteAcknowledgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AcknowledgePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for AcknowledgePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "AcknowledgePacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TimeoutPacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for TimeoutPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutPacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TimeoutOnClosePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for TimeoutOnClosePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutOnClosePacket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Empty(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::Event for Empty {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "Empty";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ChainError(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::Event for ChainError {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ChainError";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EscrowToken(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for EscrowToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "EscrowToken";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BurnToken(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BurnToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "BurnToken";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnEscrowToken(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UnEscrowToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UnEscrowToken";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MintToken(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for MintToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "MintToken";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ClientStates<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for ClientStates<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientStates";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ClientStatesKeys;
            impl ::subxt::StorageEntry for ClientStatesKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientStatesKeys";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ClientProcessedTimes<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for ClientProcessedTimes<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientProcessedTimes";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ClientProcessedHeights<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for ClientProcessedHeights<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientProcessedHeights";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ConsensusStates<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for ConsensusStates<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConsensusStates";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Connections<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for Connections<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Connections";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ConnectionsKeys;
            impl ::subxt::StorageEntry for ConnectionsKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConnectionsKeys";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Channels<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for Channels<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Channels";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ChannelsKeys;
            impl ::subxt::StorageEntry for ChannelsKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ChannelsKeys";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChannelsConnection<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for ChannelsConnection<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ChannelsConnection";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct NextSequenceSend<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for NextSequenceSend<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceSend";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NextSequenceRecv<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for NextSequenceRecv<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceRecv";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct NextSequenceAck<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for NextSequenceAck<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "NextSequenceAck";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Acknowledgements<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
                pub &'a ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for Acknowledgements<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Acknowledgements";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct AcknowledgementsKeys;
            impl ::subxt::StorageEntry for AcknowledgementsKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "AcknowledgementsKeys";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::core::primitive::u64,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Clients<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for Clients<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Clients";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ClientCounter;
            impl ::subxt::StorageEntry for ClientCounter {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ClientCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ConnectionCounter;
            impl ::subxt::StorageEntry for ConnectionCounter {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConnectionCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChannelCounter;
            impl ::subxt::StorageEntry for ChannelCounter {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ChannelCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ConnectionClient<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for ConnectionClient<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConnectionClient";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct PacketReceipt<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
                pub &'a ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for PacketReceipt<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "PacketReceipt";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct PacketCommitment<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
                pub &'a ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for PacketCommitment<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "PacketCommitment";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct PacketCommitmentKeys;
            impl ::subxt::StorageEntry for PacketCommitmentKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "PacketCommitmentKeys";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::core::primitive::u64,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SendPacketEvent<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
                pub &'a ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for SendPacketEvent<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "SendPacketEvent";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct WriteAckPacketEvent<'a>(
                pub &'a [::core::primitive::u8],
                pub &'a [::core::primitive::u8],
                pub &'a ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for WriteAckPacketEvent<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "WriteAckPacketEvent";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct LatestHeight;
            impl ::subxt::StorageEntry for LatestHeight {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "LatestHeight";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OldHeight;
            impl ::subxt::StorageEntry for OldHeight {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "OldHeight";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Denomination<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for Denomination<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Denomination";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct EscrowAddresses<'a>(
                pub &'a runtime_types::pallet_ibc::event::primitive::PortId,
                pub &'a runtime_types::pallet_ibc::event::primitive::ChannelId,
            );
            impl ::subxt::StorageEntry for EscrowAddresses<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "EscrowAddresses";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct AssetIdByName<'a>(pub &'a [::core::primitive::u8]);
            impl ::subxt::StorageEntry for AssetIdByName<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "AssetIdByName";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn client_states(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ClientStates(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn client_states_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClientStates<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn client_states_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = ClientStatesKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn client_processed_times(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ClientProcessedTimes(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn client_processed_times_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClientProcessedTimes<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn client_processed_heights(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ClientProcessedHeights(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn client_processed_heights_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClientProcessedHeights<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn consensus_states(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = ConsensusStates(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn consensus_states_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConsensusStates<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn connections(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = Connections(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connections_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Connections<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn connections_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = ConnectionsKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channels(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = Channels(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channels_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Channels<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn channels_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = ChannelsKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channels_connection(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = ChannelsConnection(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channels_connection_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ChannelsConnection<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_sequence_send(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = NextSequenceSend(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_sequence_send_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextSequenceSend<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_sequence_recv(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = NextSequenceRecv(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_sequence_recv_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextSequenceRecv<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_sequence_ack(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = NextSequenceAck(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_sequence_ack_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextSequenceAck<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn acknowledgements(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = Acknowledgements(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn acknowledgements_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Acknowledgements<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn acknowledgements_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = AcknowledgementsKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn clients(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = Clients(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn clients_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Clients<'a>>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn client_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = ClientCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connection_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = ConnectionCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn channel_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = ChannelCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connection_client(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ConnectionClient(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn connection_client_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConnectionClient<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn packet_receipt(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = PacketReceipt(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn packet_receipt_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PacketReceipt<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn packet_commitment(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = PacketCommitment(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn packet_commitment_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PacketCommitment<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn packet_commitment_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = PacketCommitmentKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn send_packet_event(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = SendPacketEvent(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn send_packet_event_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SendPacketEvent<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn write_ack_packet_event(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = WriteAckPacketEvent(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn write_ack_packet_event_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, WriteAckPacketEvent<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn latest_height(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = LatestHeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn old_height(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = OldHeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn denomination(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = Denomination(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn denomination_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Denomination<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn escrow_addresses(
                    &self,
                    _0: &runtime_types::pallet_ibc::event::primitive::PortId,
                    _1: &runtime_types::pallet_ibc::event::primitive::ChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = EscrowAddresses(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn escrow_addresses_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EscrowAddresses<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset_id_by_name(
                    &self,
                    _0: &[::core::primitive::u8],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AssetIdByName(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_id_by_name_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIdByName<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod template_module {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct DoSomething {
                pub something: ::core::primitive::u32,
            }
            impl ::subxt::Call for DoSomething {
                const PALLET: &'static str = "TemplateModule";
                const FUNCTION: &'static str = "do_something";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CauseError;
            impl ::subxt::Call for CauseError {
                const PALLET: &'static str = "TemplateModule";
                const FUNCTION: &'static str = "cause_error";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn do_something(
                    &self,
                    something: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    DoSomething,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = DoSomething { something };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cause_error(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    CauseError,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = CauseError {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_template::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SomethingStored(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for SomethingStored {
                const PALLET: &'static str = "TemplateModule";
                const EVENT: &'static str = "SomethingStored";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Something;
            impl ::subxt::StorageEntry for Something {
                const PALLET: &'static str = "TemplateModule";
                const STORAGE: &'static str = "Something";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn something(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = Something;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod beefy_primitives {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
            pub mod mmr {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BeefyNextAuthoritySet<_0> {
                    pub id: ::core::primitive::u64,
                    pub len: ::core::primitive::u32,
                    pub root: _0,
                }
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct WrapperOpaque<_0>(
                        #[codec(compact)] pub ::core::primitive::u32,
                        pub _0,
                    );
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod node_template_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct SessionKeys {
                    pub babe: runtime_types::sp_consensus_babe::app::Public,
                    pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                    pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    pub beefy: runtime_types::beefy_primitives::crypto::Public,
                    pub octopus: runtime_types::pallet_octopus_appchain::crypto::Public,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 6)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Call),
                #[codec(index = 7)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Call),
                #[codec(index = 8)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Call),
                #[codec(index = 9)]
                OctopusAssets(runtime_types::pallet_assets::pallet::Call),
                #[codec(index = 10)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 11)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 12)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 15)]
                Beefy(runtime_types::pallet_beefy::pallet::Call),
                #[codec(index = 17)]
                Uniques(runtime_types::pallet_uniques::pallet::Call),
                #[codec(index = 18)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 19)]
                Ibc(runtime_types::pallet_ibc::pallet::Call),
                #[codec(index = 20)]
                TemplateModule(runtime_types::pallet_template::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 6)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Event),
                #[codec(index = 7)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Event),
                #[codec(index = 8)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Event),
                #[codec(index = 9)]
                OctopusAssets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 10)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 11)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 12)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 17)]
                Uniques(runtime_types::pallet_uniques::pallet::Event),
                #[codec(index = 18)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 19)]
                Ibc(runtime_types::pallet_ibc::pallet::Event),
                #[codec(index = 20)]
                TemplateModule(runtime_types::pallet_template::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Runtime;
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        witness: runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 9)]
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 10)]
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 15)]
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 17)]
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 18)]
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 21)]
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 22)]
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        destination: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    BalanceZero,
                    #[codec(index = 2)]
                    NoPermission,
                    #[codec(index = 3)]
                    Unknown,
                    #[codec(index = 4)]
                    Frozen,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    BadWitness,
                    #[codec(index = 7)]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    NoProvider,
                    #[codec(index = 9)]
                    BadMetadata,
                    #[codec(index = 10)]
                    Unapproved,
                    #[codec(index = 11)]
                    WouldDie,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Created {
                        asset_id: ::core::primitive::u32,
                        creator: ::subxt::sp_core::crypto::AccountId32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    Issued {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        total_supply: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transferred {
                        asset_id: ::core::primitive::u32,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    Burned {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    TeamChanged {
                        asset_id: ::core::primitive::u32,
                        issuer: ::subxt::sp_core::crypto::AccountId32,
                        admin: ::subxt::sp_core::crypto::AccountId32,
                        freezer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    OwnerChanged {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    Frozen {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    Thawed {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    AssetFrozen { asset_id: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    AssetThawed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    Destroyed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    ForceCreated {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 12)]
                    MetadataSet {
                        asset_id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 13)]
                    MetadataCleared { asset_id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    ApprovedTransfer {
                        asset_id: ::core::primitive::u32,
                        source: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 15)]
                    ApprovalCancelled {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 16)]
                    TransferredApproved {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        destination: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 17)]
                    AssetStatusChanged { asset_id: ::core::primitive::u32 },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AssetBalance<_0, _1> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub sufficient: ::core::primitive::bool,
                    pub extra: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _0,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub accounts: ::core::primitive::u32,
                    #[codec(compact)]
                    pub sufficients: ::core::primitive::u32,
                    #[codec(compact)]
                    pub approvals: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    TooManyUncles,
                    #[codec(index = 3)]
                    GenesisUncle,
                    #[codec(index = 4)]
                    TooHighUncle,
                    #[codec(index = 5)]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    OldUncle,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_beefy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {}
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_ibc {
            use super::runtime_types;
            pub mod event {
                use super::runtime_types;
                pub mod primitive {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct ChannelId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct ClientId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct ClientState {
                        pub chain_id: ::std::vec::Vec<::core::primitive::u8>,
                        pub block_number: ::core::primitive::u32,
                        pub frozen_height: ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::Height,
                        >,
                        pub block_header: ::std::vec::Vec<::core::primitive::u8>,
                        pub latest_commitment: ::std::vec::Vec<::core::primitive::u8>,
                        pub validator_set: ::std::vec::Vec<::core::primitive::u8>,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum ClientType {
                        #[codec(index = 0)]
                        Tendermint,
                        #[codec(index = 1)]
                        Grandpa,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct ConnectionId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Height {
                        pub revision_number: ::core::primitive::u64,
                        pub revision_height: ::core::primitive::u64,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Packet {
                        pub sequence: runtime_types::pallet_ibc::event::primitive::Sequence,
                        pub source_port: runtime_types::pallet_ibc::event::primitive::PortId,
                        pub source_channel: runtime_types::pallet_ibc::event::primitive::ChannelId,
                        pub destination_port: runtime_types::pallet_ibc::event::primitive::PortId,
                        pub destination_channel:
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        pub data: ::std::vec::Vec<::core::primitive::u8>,
                        pub timeout_height: runtime_types::pallet_ibc::event::primitive::Height,
                        pub timeout_timestamp:
                            runtime_types::pallet_ibc::event::primitive::Timestamp,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct PortId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        :: subxt :: codec :: CompactAs,
                    )]
                    pub struct Sequence(pub ::core::primitive::u64);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Timestamp {
                        pub time: ::std::vec::Vec<::core::primitive::u8>,
                    }
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    deliver {
                        messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                        tmp: ::core::primitive::u8,
                    },
                    #[codec(index = 1)]
                    update_client_state {
                        client_id: ::std::vec::Vec<::core::primitive::u8>,
                        mmr_root: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    transfer {
                        source_port: ::std::vec::Vec<::core::primitive::u8>,
                        source_channel: ::std::vec::Vec<::core::primitive::u8>,
                        token: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        timeout_height: ::core::primitive::u64,
                        timeout_timestamp: ::core::primitive::u64,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    UpdateBeefyLightClientFailure,
                    #[codec(index = 1)]
                    ReceiveMmrRootBlockNumberLessThanClientStateLatestCommitmentBlockNumber,
                    #[codec(index = 2)]
                    ClientIdNotFound,
                    #[codec(index = 3)]
                    InvalidEncode,
                    #[codec(index = 4)]
                    InvalidDecode,
                    #[codec(index = 5)]
                    InvalidFromUtf8,
                    #[codec(index = 6)]
                    Ics26Error,
                    #[codec(index = 7)]
                    InvalidSigner,
                    #[codec(index = 8)]
                    EmptyChannelId,
                    #[codec(index = 9)]
                    Ics20Error,
                    #[codec(index = 10)]
                    InvalidPacket,
                    #[codec(index = 11)]
                    InvalidSignedCommitment,
                    #[codec(index = 12)]
                    InvalidIdentifier,
                    #[codec(index = 13)]
                    InvalidTimestamp,
                    #[codec(index = 14)]
                    EmptyLatestCommitment,
                    #[codec(index = 15)]
                    SendPacketError,
                    #[codec(index = 16)]
                    ReceivePacketError,
                    #[codec(index = 17)]
                    TimeoutPacketError,
                    #[codec(index = 18)]
                    AcknowledgePacketError,
                    #[codec(index = 19)]
                    OpenInitChannelError,
                    #[codec(index = 20)]
                    OpenTryChannelError,
                    #[codec(index = 21)]
                    OpenAckChannelError,
                    #[codec(index = 22)]
                    OpenConfirmChannelError,
                    #[codec(index = 23)]
                    CloseInitChannelError,
                    #[codec(index = 24)]
                    CloseConfirmChannelError,
                    #[codec(index = 25)]
                    AmountOverflow,
                    #[codec(index = 26)]
                    SerdeIBCFungibleTokenPacketDataError,
                    #[codec(index = 27)]
                    InvalidParse,
                    #[codec(index = 28)]
                    ParseDenomTraceError,
                    #[codec(index = 29)]
                    AcknowledgementResponseEmpty,
                    #[codec(index = 30)]
                    GetIbcDenomError,
                    #[codec(index = 31)]
                    InvalidValidation,
                    #[codec(index = 32)]
                    StorePacketResultError,
                    #[codec(index = 33)]
                    InvalidTokenId,
                    #[codec(index = 34)]
                    WrongAssetId,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewBlock(runtime_types::pallet_ibc::event::primitive::Height),
                    #[codec(index = 1)]
                    CreateClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 2)]
                    UpdateClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 3)]
                    UpdateClientState(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientState,
                    ),
                    #[codec(index = 4)]
                    UpgradeClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 5)]
                    ClientMisbehaviour(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 6)]
                    OpenInitConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 7)]
                    OpenTryConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 8)]
                    OpenAckConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 9)]
                    OpenConfirmConnection(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                    ),
                    #[codec(index = 10)]
                    OpenInitChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 11)]
                    OpenTryChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 12)]
                    OpenAckChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 13)]
                    OpenConfirmChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 14)]
                    CloseInitChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 15)]
                    CloseConfirmChannel(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                        runtime_types::pallet_ibc::event::primitive::ConnectionId,
                        runtime_types::pallet_ibc::event::primitive::PortId,
                        ::core::option::Option<
                            runtime_types::pallet_ibc::event::primitive::ChannelId,
                        >,
                    ),
                    #[codec(index = 16)]
                    SendPacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 17)]
                    ReceivePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 18)]
                    WriteAcknowledgement(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 19)]
                    AcknowledgePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 20)]
                    TimeoutPacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 21)]
                    TimeoutOnClosePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 22)]
                    Empty(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 23)]
                    ChainError(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 24)]
                    EscrowToken(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 25)]
                    BurnToken(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 26)]
                    UnEscrowToken(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 27)]
                    MintToken(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Any {
                pub type_url: ::std::vec::Vec<::core::primitive::u8>,
                pub value: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    HeartbeatReceived {
                        authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    },
                    #[codec(index = 1)]
                    AllGood,
                    #[codec(index = 2)]
                    SomeOffline {
                        offline: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BoundedOpaqueNetworkState {
                pub peer_id:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                pub external_addresses:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_octopus_appchain {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    submit_observations {
                        payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                            runtime_types::sp_runtime::MultiSigner,
                            ::core::primitive::u32,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        signature: runtime_types::sp_runtime::MultiSignature,
                    },
                    #[codec(index = 1)]
                    force_set_is_activated {
                        is_activated: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    force_set_next_set_id { next_set_id: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    force_set_planned_validators {
                        validators: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                    #[codec(index = 4)]
                    lock {
                        receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    mint_asset {
                        asset_id: ::core::primitive::u32,
                        sender_id: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    burn_asset {
                        asset_id: ::core::primitive::u32,
                        receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    WrongSetId,
                    #[codec(index = 1)]
                    InvalidNotificationId,
                    #[codec(index = 2)]
                    NotValidator,
                    #[codec(index = 3)]
                    AmountOverflow,
                    #[codec(index = 4)]
                    NextNotificationIdOverflow,
                    #[codec(index = 5)]
                    WrongAssetId,
                    #[codec(index = 6)]
                    InvalidActiveTotalStake,
                    #[codec(index = 7)]
                    NotActivated,
                    #[codec(index = 8)]
                    InvalidReceiverId,
                    #[codec(index = 9)]
                    InvalidTokenId,
                    #[codec(index = 10)]
                    NextSetIdOverflow,
                    #[codec(index = 11)]
                    ObservationsExceededLimit,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewPlannedValidators(
                        ::core::primitive::u32,
                        ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    ),
                    #[codec(index = 1)]
                    Locked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u128,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 2)]
                    Unlocked(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    UnlockFailed(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    AssetMinted(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 5)]
                    AssetBurned(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 6)]
                    AssetMintFailed(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 7)]
                    AssetIdGetFailed(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BurnEvent<_0> {
                pub index: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct LockAssetEvent<_0> {
                pub index: ::core::primitive::u32,
                pub token_id: ::std::vec::Vec<::core::primitive::u8>,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum NotificationResult {
                #[codec(index = 0)]
                Success,
                #[codec(index = 1)]
                UnlockFailed,
                #[codec(index = 2)]
                AssetMintFailed,
                #[codec(index = 3)]
                AssetGetFailed,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Observation<_0> {
                #[codec(index = 0)]
                UpdateValidatorSet(runtime_types::pallet_octopus_appchain::ValidatorSet<_0>),
                #[codec(index = 1)]
                LockAsset(runtime_types::pallet_octopus_appchain::LockAssetEvent<_0>),
                #[codec(index = 2)]
                Burn(runtime_types::pallet_octopus_appchain::BurnEvent<_0>),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum ObservationType {
                #[codec(index = 0)]
                UpdateValidatorSet,
                #[codec(index = 1)]
                Burn,
                #[codec(index = 2)]
                LockAsset,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ObservationsPayload<_0, _1, _2> {
                pub public: _0,
                pub block_number: _1,
                pub observations:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Observation<_2>>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Validator<_0> {
                pub validator_id_in_appchain: _0,
                pub total_stake: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ValidatorSet<_0> {
                pub set_id: ::core::primitive::u32,
                pub validators:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Validator<_0>>,
            }
        }
        pub mod pallet_octopus_lpos {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_history_depth {
                        #[codec(compact)]
                        new_history_depth: ::core::primitive::u32,
                        #[codec(compact)]
                        era_items_deleted: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    force_set_era_payout { era_payout: ::core::primitive::u128 },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    NotController,
                    #[codec(index = 1)]
                    NotStash,
                    #[codec(index = 2)]
                    AlreadyBonded,
                    #[codec(index = 3)]
                    AlreadyPaired,
                    #[codec(index = 4)]
                    EmptyTargets,
                    #[codec(index = 5)]
                    DuplicateIndex,
                    #[codec(index = 6)]
                    InvalidSlashIndex,
                    #[codec(index = 7)]
                    InsufficientBond,
                    #[codec(index = 8)]
                    NoMoreChunks,
                    #[codec(index = 9)]
                    NoUnlockChunk,
                    #[codec(index = 10)]
                    FundedTarget,
                    #[codec(index = 11)]
                    InvalidEraToReward,
                    #[codec(index = 12)]
                    InvalidNumberOfNominations,
                    #[codec(index = 13)]
                    NotSortedAndUnique,
                    #[codec(index = 14)]
                    AlreadyClaimed,
                    #[codec(index = 15)]
                    IncorrectHistoryDepth,
                    #[codec(index = 16)]
                    IncorrectSlashingSpans,
                    #[codec(index = 17)]
                    BadState,
                    #[codec(index = 18)]
                    TooManyTargets,
                    #[codec(index = 19)]
                    BadTarget,
                    #[codec(index = 20)]
                    CannotChillOther,
                    #[codec(index = 21)]
                    TooManyNominators,
                    #[codec(index = 22)]
                    TooManyValidators,
                    #[codec(index = 23)]
                    NoClaimedRewards,
                    #[codec(index = 24)]
                    AmountOverflow,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    PlanNewEra(::core::primitive::u32),
                    #[codec(index = 1)]
                    PlanNewEraFailed,
                    #[codec(index = 2)]
                    TriggerNewEra,
                    #[codec(index = 3)]
                    EraPayout(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ),
                    #[codec(index = 4)]
                    EraPayoutFailed(::core::primitive::u32),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub set_id: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::subxt::KeyedVec<_0, ::core::primitive::u32>,
            }
        }
        pub mod pallet_octopus_support {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum PayloadType {
                    #[codec(index = 0)]
                    Lock,
                    #[codec(index = 1)]
                    BurnAsset,
                    #[codec(index = 2)]
                    PlanNewEra,
                    #[codec(index = 3)]
                    EraPayout,
                }
            }
        }
        pub mod pallet_octopus_upward_messages {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {}
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    NonceOverflow,
                    #[codec(index = 1)]
                    QueueSizeLimitReached,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {}
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Message {
                pub nonce: ::core::primitive::u64,
                pub payload_type: runtime_types::pallet_octopus_support::types::PayloadType,
                pub payload: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_keys {
                        keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    purge_keys,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    KeyChanged {
                        new_sudoer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_template {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    cause_error,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    NoneValue,
                    #[codec(index = 1)]
                    StorageOverflow,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    SomethingStored(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_uniques {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 1)]
                    force_create {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        free_holding: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    destroy {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        witness: runtime_types::pallet_uniques::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    mint {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 4)]
                    burn {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                        check_owner: ::core::option::Option<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 5)]
                    transfer {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 6)]
                    redeposit {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        instances: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 7)]
                    freeze {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    thaw {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    freeze_class {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    thaw_class {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    transfer_ownership {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 12)]
                    set_team {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    approve_transfer {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    cancel_approval {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                        maybe_check_delegate: ::core::option::Option<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 15)]
                    force_asset_status {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        free_holding: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    set_attribute {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 17)]
                    clear_attribute {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 18)]
                    set_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    clear_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        #[codec(compact)]
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 20)]
                    set_class_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 21)]
                    clear_class_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    NoPermission,
                    #[codec(index = 1)]
                    Unknown,
                    #[codec(index = 2)]
                    AlreadyExists,
                    #[codec(index = 3)]
                    WrongOwner,
                    #[codec(index = 4)]
                    BadWitness,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    Frozen,
                    #[codec(index = 7)]
                    WrongDelegate,
                    #[codec(index = 8)]
                    NoDelegate,
                    #[codec(index = 9)]
                    Unapproved,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Created {
                        class: ::core::primitive::u32,
                        creator: ::subxt::sp_core::crypto::AccountId32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    ForceCreated {
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    Destroyed { class: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    Issued {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    Transferred {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    Burned {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    Frozen {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    Thawed {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    ClassFrozen { class: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    ClassThawed { class: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    OwnerChanged {
                        class: ::core::primitive::u32,
                        new_owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 11)]
                    TeamChanged {
                        class: ::core::primitive::u32,
                        issuer: ::subxt::sp_core::crypto::AccountId32,
                        admin: ::subxt::sp_core::crypto::AccountId32,
                        freezer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 12)]
                    ApprovedTransfer {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 13)]
                    ApprovalCancelled {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 14)]
                    AssetStatusChanged { class: ::core::primitive::u32 },
                    #[codec(index = 15)]
                    ClassMetadataSet {
                        class: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    ClassMetadataCleared { class: ::core::primitive::u32 },
                    #[codec(index = 17)]
                    MetadataSet {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 18)]
                    MetadataCleared {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    Redeposited {
                        class: ::core::primitive::u32,
                        successful_instances: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 20)]
                    AttributeSet {
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 21)]
                    AttributeCleared {
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ClassDetails<_0, _1> {
                    pub owner: _0,
                    pub issuer: _0,
                    pub admin: _0,
                    pub freezer: _0,
                    pub total_deposit: _1,
                    pub free_holding: ::core::primitive::bool,
                    pub instances: ::core::primitive::u32,
                    pub instance_metadatas: ::core::primitive::u32,
                    pub attributes: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ClassMetadata<_0> {
                    pub deposit: _0,
                    pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub instances: ::core::primitive::u32,
                    #[codec(compact)]
                    pub instance_metadatas: ::core::primitive::u32,
                    #[codec(compact)]
                    pub attributes: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct InstanceDetails<_0, _1> {
                    pub owner: _0,
                    pub approved: ::core::option::Option<_0>,
                    pub is_frozen: ::core::primitive::bool,
                    pub deposit: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct InstanceMetadata<_0> {
                    pub deposit: _0,
                    pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub is_frozen: ::core::primitive::bool,
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _2, _1, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlakeTwo256;
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
        fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
            if let &Self::Module { index, error } = self {
                Some(::subxt::ModuleErrorData {
                    pallet_index: index,
                    error: [error, 0, 0, 0],
                })
            } else {
                None
            }
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<T, X> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X> RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn constants(&'a self) -> ConstantsApi<'a, T> {
            ConstantsApi {
                client: &self.client,
            }
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
        pub fn events(&'a self) -> EventsApi<'a, T> {
            EventsApi {
                client: &self.client,
            }
        }
    }
    pub struct EventsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> EventsApi<'a, T> {
        pub async fn at(
            &self,
            block_hash: T::Hash,
        ) -> Result<::subxt::events::Events<'a, T, Event>, ::subxt::BasicError> {
            ::subxt::events::at::<T, Event>(self.client, block_hash).await
        }
        pub async fn subscribe(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<'a, ::subxt::events::EventSub<T::Header>, T, Event>,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe::<T, Event>(self.client).await
        }
        pub async fn subscribe_finalized(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<
                'a,
                ::subxt::events::FinalizedEventSub<'a, T::Header>,
                T,
                Event,
            >,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe_finalized::<T, Event>(self.client).await
        }
    }
    pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn system(&self) -> system::constants::ConstantsApi<'a, T> {
            system::constants::ConstantsApi::new(self.client)
        }
        pub fn babe(&self) -> babe::constants::ConstantsApi<'a, T> {
            babe::constants::ConstantsApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi<'a, T> {
            timestamp::constants::ConstantsApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi<'a, T> {
            authorship::constants::ConstantsApi::new(self.client)
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi<'a, T> {
            balances::constants::ConstantsApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi<'a, T> {
            transaction_payment::constants::ConstantsApi::new(self.client)
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::constants::ConstantsApi<'a, T> {
            octopus_appchain::constants::ConstantsApi::new(self.client)
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::constants::ConstantsApi<'a, T> {
            octopus_lpos::constants::ConstantsApi::new(self.client)
        }
        pub fn octopus_upward_messages(
            &self,
        ) -> octopus_upward_messages::constants::ConstantsApi<'a, T> {
            octopus_upward_messages::constants::ConstantsApi::new(self.client)
        }
        pub fn octopus_assets(&self) -> octopus_assets::constants::ConstantsApi<'a, T> {
            octopus_assets::constants::ConstantsApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi<'a, T> {
            grandpa::constants::ConstantsApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi<'a, T> {
            im_online::constants::ConstantsApi::new(self.client)
        }
        pub fn uniques(&self) -> uniques::constants::ConstantsApi<'a, T> {
            uniques::constants::ConstantsApi::new(self.client)
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::storage::StorageApi<'a, T> {
            octopus_appchain::storage::StorageApi::new(self.client)
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::storage::StorageApi<'a, T> {
            octopus_lpos::storage::StorageApi::new(self.client)
        }
        pub fn octopus_upward_messages(
            &self,
        ) -> octopus_upward_messages::storage::StorageApi<'a, T> {
            octopus_upward_messages::storage::StorageApi::new(self.client)
        }
        pub fn octopus_assets(&self) -> octopus_assets::storage::StorageApi<'a, T> {
            octopus_assets::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn mmr(&self) -> mmr::storage::StorageApi<'a, T> {
            mmr::storage::StorageApi::new(self.client)
        }
        pub fn beefy(&self) -> beefy::storage::StorageApi<'a, T> {
            beefy::storage::StorageApi::new(self.client)
        }
        pub fn mmr_leaf(&self) -> mmr_leaf::storage::StorageApi<'a, T> {
            mmr_leaf::storage::StorageApi::new(self.client)
        }
        pub fn uniques(&self) -> uniques::storage::StorageApi<'a, T> {
            uniques::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::storage::StorageApi<'a, T> {
            ibc::storage::StorageApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::storage::StorageApi<'a, T> {
            template_module::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<'a, T, X> TransactionApi<'a, T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T, X> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T, X> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::calls::TransactionApi<'a, T, X> {
            octopus_appchain::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::calls::TransactionApi<'a, T, X> {
            octopus_lpos::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_upward_messages(
            &self,
        ) -> octopus_upward_messages::calls::TransactionApi<'a, T, X> {
            octopus_upward_messages::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_assets(&self) -> octopus_assets::calls::TransactionApi<'a, T, X> {
            octopus_assets::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T, X> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T, X> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn beefy(&self) -> beefy::calls::TransactionApi<'a, T, X> {
            beefy::calls::TransactionApi::new(self.client)
        }
        pub fn uniques(&self) -> uniques::calls::TransactionApi<'a, T, X> {
            uniques::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::calls::TransactionApi<'a, T, X> {
            ibc::calls::TransactionApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::calls::TransactionApi<'a, T, X> {
            template_module::calls::TransactionApi::new(self.client)
        }
    }
}
