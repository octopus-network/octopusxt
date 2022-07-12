#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
    pub static PALLETS: [&str; 22usize] = [
        "System",
        "RandomnessCollectiveFlip",
        "Timestamp",
        "Babe",
        "Authorship",
        "Balances",
        "TransactionPayment",
        "OctopusAppchain",
        "OctopusLpos",
        "OctopusUpwardMessages",
        "OctopusAssets",
        "OctopusUniques",
        "Session",
        "Grandpa",
        "ImOnline",
        "Historical",
        "Mmr",
        "Beefy",
        "MmrLeaf",
        "Sudo",
        "TemplateModule",
        "Ibc",
    ];
    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 5)]
        Balances(balances::Event),
        #[codec(index = 7)]
        OctopusAppchain(octopus_appchain::Event),
        #[codec(index = 8)]
        OctopusLpos(octopus_lpos::Event),
        #[codec(index = 9)]
        OctopusUpwardMessages(octopus_upward_messages::Event),
        #[codec(index = 10)]
        OctopusAssets(octopus_assets::Event),
        #[codec(index = 11)]
        OctopusUniques(octopus_uniques::Event),
        #[codec(index = 12)]
        Session(session::Event),
        #[codec(index = 13)]
        Grandpa(grandpa::Event),
        #[codec(index = 14)]
        ImOnline(im_online::Event),
        #[codec(index = 19)]
        Sudo(sudo::Event),
        #[codec(index = 20)]
        TemplateModule(template_module::Event),
        #[codec(index = 21)]
        Ibc(ibc::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        FillBlock,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<FillBlock>()?
                        == [
                            228u8, 117u8, 251u8, 95u8, 47u8, 56u8, 32u8, 177u8, 191u8, 72u8, 75u8,
                            23u8, 193u8, 175u8, 227u8, 218u8, 127u8, 94u8, 114u8, 110u8, 215u8,
                            61u8, 162u8, 102u8, 73u8, 89u8, 218u8, 148u8, 59u8, 73u8, 59u8, 149u8,
                        ]
                    {
                        let call = FillBlock { ratio };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Remark, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Remark>()?
                        == [
                            186u8, 79u8, 33u8, 199u8, 216u8, 115u8, 19u8, 146u8, 220u8, 174u8,
                            98u8, 61u8, 179u8, 230u8, 40u8, 70u8, 22u8, 251u8, 77u8, 62u8, 133u8,
                            80u8, 186u8, 70u8, 135u8, 172u8, 178u8, 241u8, 69u8, 106u8, 235u8,
                            140u8,
                        ]
                    {
                        let call = Remark { remark };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetHeapPages,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetHeapPages>()?
                        == [
                            77u8, 138u8, 122u8, 55u8, 179u8, 101u8, 60u8, 137u8, 173u8, 39u8, 28u8,
                            36u8, 237u8, 243u8, 232u8, 162u8, 76u8, 176u8, 135u8, 58u8, 60u8,
                            177u8, 105u8, 136u8, 94u8, 53u8, 26u8, 31u8, 41u8, 156u8, 228u8, 241u8,
                        ]
                    {
                        let call = SetHeapPages { pages };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                #[doc = "  expensive)."]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                #[doc = "expensive. We will treat this as a full block."]
                #[doc = "# </weight>"]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetCode,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetCode>()?
                        == [
                            35u8, 75u8, 103u8, 203u8, 91u8, 141u8, 77u8, 95u8, 37u8, 157u8, 107u8,
                            240u8, 54u8, 242u8, 245u8, 205u8, 104u8, 165u8, 177u8, 37u8, 86u8,
                            197u8, 28u8, 202u8, 121u8, 159u8, 18u8, 204u8, 237u8, 117u8, 141u8,
                            131u8,
                        ]
                    {
                        let call = SetCode { code };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                #[doc = "block. # </weight>"]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetCodeWithoutChecks,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetCodeWithoutChecks>()?
                        == [
                            150u8, 148u8, 119u8, 129u8, 77u8, 216u8, 135u8, 187u8, 127u8, 24u8,
                            238u8, 15u8, 227u8, 229u8, 191u8, 217u8, 106u8, 129u8, 149u8, 79u8,
                            154u8, 78u8, 53u8, 159u8, 89u8, 69u8, 103u8, 197u8, 93u8, 161u8, 134u8,
                            17u8,
                        ]
                    {
                        let call = SetCodeWithoutChecks { code };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetStorage,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetStorage>()?
                        == [
                            197u8, 12u8, 119u8, 205u8, 152u8, 103u8, 211u8, 170u8, 146u8, 253u8,
                            25u8, 56u8, 180u8, 146u8, 74u8, 75u8, 38u8, 108u8, 212u8, 154u8, 23u8,
                            22u8, 148u8, 175u8, 107u8, 186u8, 222u8, 13u8, 149u8, 132u8, 204u8,
                            217u8,
                        ]
                    {
                        let call = SetStorage { items };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        KillStorage,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<KillStorage>()?
                        == [
                            154u8, 115u8, 185u8, 20u8, 126u8, 90u8, 222u8, 131u8, 199u8, 57u8,
                            184u8, 226u8, 43u8, 245u8, 161u8, 176u8, 194u8, 123u8, 139u8, 97u8,
                            97u8, 94u8, 47u8, 64u8, 204u8, 96u8, 190u8, 94u8, 216u8, 237u8, 69u8,
                            51u8,
                        ]
                    {
                        let call = KillStorage { keys };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        KillPrefix,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<KillPrefix>()?
                        == [
                            214u8, 101u8, 191u8, 241u8, 1u8, 241u8, 144u8, 116u8, 246u8, 199u8,
                            159u8, 249u8, 155u8, 164u8, 220u8, 221u8, 75u8, 33u8, 204u8, 3u8,
                            255u8, 201u8, 187u8, 238u8, 181u8, 213u8, 41u8, 105u8, 234u8, 120u8,
                            202u8, 115u8,
                        ]
                    {
                        let call = KillPrefix { prefix, subkeys };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        RemarkWithEvent,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<RemarkWithEvent>()?
                        == [
                            171u8, 82u8, 75u8, 237u8, 69u8, 197u8, 223u8, 125u8, 123u8, 51u8,
                            241u8, 35u8, 202u8, 210u8, 227u8, 109u8, 1u8, 241u8, 255u8, 63u8, 33u8,
                            115u8, 156u8, 239u8, 97u8, 76u8, 193u8, 35u8, 74u8, 199u8, 43u8, 255u8,
                        ]
                    {
                        let call = RemarkWithEvent { remark };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub hash: ::subxt::sp_core::H256,
            }
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
                #[doc = " The full account information for a particular account ID."]
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8, 130u8,
                            23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8, 88u8, 71u8,
                            168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8, 195u8, 46u8,
                        ]
                    {
                        let entry = Account(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The full account information for a particular account ID."]
                pub async fn account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8, 130u8,
                            23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8, 88u8, 71u8,
                            168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8, 195u8, 46u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Total extrinsics count for the current block."]
                pub async fn extrinsic_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExtrinsicCount>()?
                        == [
                            223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
                            222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
                            41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
                        ]
                    {
                        let entry = ExtrinsicCount;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current weight for the block."]
                pub async fn block_weight(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<BlockWeight>()?
                        == [
                            2u8, 236u8, 190u8, 174u8, 244u8, 98u8, 194u8, 168u8, 89u8, 208u8, 7u8,
                            45u8, 175u8, 171u8, 177u8, 121u8, 215u8, 190u8, 184u8, 195u8, 49u8,
                            133u8, 44u8, 1u8, 181u8, 215u8, 89u8, 84u8, 255u8, 16u8, 57u8, 152u8,
                        ]
                    {
                        let entry = BlockWeight;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub async fn all_extrinsics_len(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<AllExtrinsicsLen>()?
                        == [
                            202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
                            254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
                            219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
                            134u8, 60u8,
                        ]
                    {
                        let entry = AllExtrinsicsLen;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Map of block numbers to block hashes."]
                pub async fn block_hash(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<BlockHash>()?
                        == [
                            24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8, 24u8,
                            236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8, 111u8,
                            141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8, 246u8,
                            120u8,
                        ]
                    {
                        let entry = BlockHash(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Map of block numbers to block hashes."]
                pub async fn block_hash_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BlockHash<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<BlockHash>()?
                        == [
                            24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8, 24u8,
                            236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8, 111u8,
                            141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8, 246u8,
                            120u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub async fn extrinsic_data(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExtrinsicData>()?
                        == [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ]
                    {
                        let entry = ExtrinsicData(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub async fn extrinsic_data_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExtrinsicData>()?
                        == [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub async fn number(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Number>()?
                        == [
                            228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
                            246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
                            36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
                            164u8, 191u8,
                        ]
                    {
                        let entry = Number;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Hash of the previous block."]
                pub async fn parent_hash(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ParentHash>()?
                        == [
                            194u8, 221u8, 147u8, 22u8, 68u8, 141u8, 32u8, 6u8, 202u8, 39u8, 164u8,
                            184u8, 69u8, 126u8, 190u8, 101u8, 215u8, 27u8, 127u8, 157u8, 200u8,
                            69u8, 170u8, 139u8, 232u8, 27u8, 254u8, 181u8, 183u8, 105u8, 111u8,
                            177u8,
                        ]
                    {
                        let entry = ParentHash;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub async fn digest(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Digest>()?
                        == [
                            10u8, 176u8, 13u8, 228u8, 226u8, 42u8, 210u8, 151u8, 107u8, 212u8,
                            136u8, 15u8, 38u8, 182u8, 225u8, 12u8, 250u8, 56u8, 193u8, 243u8,
                            219u8, 113u8, 95u8, 233u8, 21u8, 229u8, 125u8, 146u8, 92u8, 250u8,
                            32u8, 168u8,
                        ]
                    {
                        let entry = Digest;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: This storage item is explicitly unbounded since it is never intended to be read"]
                #[doc = " from within the runtime."]
                pub async fn events(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::node_template_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Events>()?
                        == [
                            196u8, 76u8, 9u8, 244u8, 10u8, 252u8, 170u8, 176u8, 79u8, 80u8, 193u8,
                            37u8, 19u8, 144u8, 173u8, 151u8, 242u8, 30u8, 142u8, 52u8, 9u8, 167u8,
                            247u8, 236u8, 53u8, 130u8, 173u8, 138u8, 61u8, 122u8, 5u8, 42u8,
                        ]
                    {
                        let entry = Events;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub async fn event_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<EventCount>()?
                        == [
                            236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
                            203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
                            161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
                            112u8,
                        ]
                    {
                        let entry = EventCount;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub async fn event_topics(
                    &self,
                    _0: &::subxt::sp_core::H256,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EventTopics>()?
                        == [
                            231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
                            140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8, 93u8,
                            178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8, 245u8, 179u8,
                        ]
                    {
                        let entry = EventTopics(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub async fn event_topics_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EventTopics<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EventTopics>()?
                        == [
                            231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
                            140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8, 93u8,
                            178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8, 245u8, 179u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub async fn last_runtime_upgrade(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<LastRuntimeUpgrade>()?
                        == [
                            219u8, 153u8, 158u8, 38u8, 45u8, 65u8, 151u8, 137u8, 53u8, 76u8, 11u8,
                            181u8, 218u8, 248u8, 125u8, 190u8, 100u8, 240u8, 173u8, 75u8, 179u8,
                            137u8, 198u8, 197u8, 248u8, 185u8, 118u8, 58u8, 42u8, 165u8, 125u8,
                            119u8,
                        ]
                    {
                        let entry = LastRuntimeUpgrade;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<UpgradedToU32RefCount>()?
                        == [
                            171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
                            178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
                            83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
                        ]
                    {
                        let entry = UpgradedToU32RefCount;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<UpgradedToTripleRefCount>()?
                        == [
                            90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
                            210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
                            155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
                            185u8,
                        ]
                    {
                        let entry = UpgradedToTripleRefCount;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The execution phase of the block."]
                pub async fn execution_phase(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExecutionPhase>()?
                        == [
                            174u8, 13u8, 230u8, 220u8, 239u8, 161u8, 172u8, 122u8, 188u8, 95u8,
                            141u8, 118u8, 91u8, 158u8, 111u8, 145u8, 243u8, 173u8, 226u8, 212u8,
                            187u8, 118u8, 94u8, 132u8, 221u8, 244u8, 61u8, 148u8, 217u8, 30u8,
                            238u8, 225u8,
                        ]
                    {
                        let entry = ExecutionPhase;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "BlockWeights")?
                        == [
                            49u8, 105u8, 160u8, 185u8, 41u8, 100u8, 26u8, 18u8, 154u8, 196u8,
                            108u8, 96u8, 36u8, 148u8, 28u8, 162u8, 92u8, 234u8, 89u8, 152u8, 149u8,
                            176u8, 186u8, 20u8, 217u8, 167u8, 59u8, 167u8, 106u8, 9u8, 205u8,
                            106u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("BlockWeights")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "BlockLength")?
                        == [
                            120u8, 249u8, 182u8, 103u8, 246u8, 214u8, 149u8, 44u8, 42u8, 64u8, 2u8,
                            56u8, 157u8, 184u8, 43u8, 195u8, 214u8, 251u8, 207u8, 207u8, 249u8,
                            105u8, 203u8, 108u8, 179u8, 93u8, 93u8, 246u8, 40u8, 175u8, 160u8,
                            114u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("BlockLength")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "BlockHashCount")?
                        == [
                            123u8, 126u8, 182u8, 103u8, 71u8, 187u8, 233u8, 8u8, 47u8, 226u8,
                            159u8, 139u8, 0u8, 59u8, 190u8, 135u8, 189u8, 77u8, 190u8, 81u8, 39u8,
                            198u8, 224u8, 219u8, 70u8, 143u8, 6u8, 132u8, 196u8, 61u8, 117u8,
                            194u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("BlockHashCount")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().constant_hash("System", "DbWeight")?
                        == [
                            203u8, 8u8, 106u8, 152u8, 74u8, 132u8, 2u8, 132u8, 244u8, 106u8, 147u8,
                            12u8, 93u8, 80u8, 61u8, 158u8, 172u8, 178u8, 228u8, 125u8, 213u8,
                            102u8, 75u8, 210u8, 64u8, 185u8, 204u8, 84u8, 10u8, 164u8, 204u8, 62u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("DbWeight")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().constant_hash("System", "Version")?
                        == [
                            4u8, 196u8, 159u8, 160u8, 39u8, 68u8, 64u8, 128u8, 30u8, 31u8, 154u8,
                            92u8, 244u8, 227u8, 163u8, 224u8, 64u8, 138u8, 32u8, 61u8, 235u8,
                            207u8, 6u8, 209u8, 10u8, 186u8, 223u8, 81u8, 36u8, 116u8, 23u8, 169u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("Version")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The designated SS85 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "SS58Prefix")?
                        == [
                            197u8, 217u8, 49u8, 68u8, 82u8, 238u8, 120u8, 50u8, 91u8, 58u8, 6u8,
                            156u8, 40u8, 1u8, 241u8, 213u8, 141u8, 74u8, 83u8, 115u8, 117u8, 41u8,
                            119u8, 50u8, 140u8, 136u8, 163u8, 185u8, 34u8, 190u8, 60u8, 97u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("SS58Prefix")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
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
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub async fn random_material(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<RandomMaterial>()?
                        == [
                            60u8, 176u8, 119u8, 155u8, 161u8, 136u8, 144u8, 88u8, 26u8, 57u8,
                            142u8, 34u8, 5u8, 37u8, 115u8, 11u8, 90u8, 222u8, 147u8, 194u8, 82u8,
                            194u8, 70u8, 227u8, 175u8, 198u8, 235u8, 24u8, 7u8, 87u8, 203u8, 182u8,
                        ]
                    {
                        let entry = RandomMaterial;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                #[doc = "# </weight>"]
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Set, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Set>()?
                        == [
                            191u8, 73u8, 102u8, 150u8, 65u8, 157u8, 172u8, 194u8, 7u8, 72u8, 1u8,
                            35u8, 54u8, 99u8, 245u8, 139u8, 40u8, 136u8, 245u8, 53u8, 167u8, 100u8,
                            143u8, 244u8, 160u8, 5u8, 18u8, 130u8, 77u8, 160u8, 227u8, 51u8,
                        ]
                    {
                        let call = Set { now };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Current time for the current block."]
                pub async fn now(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Now>()?
                        == [
                            148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
                            221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
                            185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
                        ]
                    {
                        let entry = Now;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub async fn did_update(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<DidUpdate>()?
                        == [
                            70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
                            13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
                            27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
                        ]
                    {
                        let entry = DidUpdate;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Timestamp", "MinimumPeriod")?
                        == [
                            141u8, 242u8, 40u8, 24u8, 83u8, 43u8, 33u8, 194u8, 156u8, 149u8, 219u8,
                            61u8, 10u8, 123u8, 120u8, 247u8, 228u8, 22u8, 25u8, 24u8, 214u8, 188u8,
                            54u8, 135u8, 240u8, 162u8, 41u8, 216u8, 3u8, 58u8, 238u8, 39u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Timestamp")?;
                        let constant = pallet.constant("MinimumPeriod")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "Report authority equivocation/misbehavior. This method will verify"]
                #[doc = "the equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence will"]
                #[doc = "be reported."]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocation,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ReportEquivocation>()?
                        == [
                            123u8, 212u8, 216u8, 77u8, 79u8, 132u8, 201u8, 155u8, 166u8, 230u8,
                            50u8, 89u8, 98u8, 68u8, 56u8, 213u8, 206u8, 245u8, 91u8, 104u8, 89u8,
                            189u8, 57u8, 38u8, 127u8, 22u8, 47u8, 206u8, 142u8, 202u8, 106u8,
                            154u8,
                        ]
                    {
                        let call = ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Report authority equivocation/misbehavior. This method will verify"]
                #[doc = "the equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence will"]
                #[doc = "be reported."]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocationUnsigned,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .call_hash::<ReportEquivocationUnsigned>()?
                        == [
                            32u8, 163u8, 168u8, 251u8, 251u8, 9u8, 1u8, 195u8, 173u8, 32u8, 235u8,
                            125u8, 141u8, 201u8, 130u8, 207u8, 239u8, 76u8, 150u8, 99u8, 74u8,
                            193u8, 60u8, 165u8, 93u8, 49u8, 95u8, 224u8, 217u8, 243u8, 117u8,
                            173u8,
                        ]
                    {
                        let call = ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"]
                #[doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."]
                #[doc = "Multiple calls to this method will replace any existing planned config change that had"]
                #[doc = "not been enacted yet."]
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        PlanConfigChange,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<PlanConfigChange>()?
                        == [
                            215u8, 121u8, 90u8, 87u8, 178u8, 247u8, 114u8, 53u8, 174u8, 28u8, 20u8,
                            33u8, 139u8, 216u8, 13u8, 187u8, 74u8, 198u8, 38u8, 28u8, 175u8, 13u8,
                            73u8, 132u8, 103u8, 78u8, 217u8, 207u8, 113u8, 169u8, 42u8, 103u8,
                        ]
                    {
                        let call = PlanConfigChange { config };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Current epoch index."]
                pub async fn epoch_index(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<EpochIndex>()?
                        == [
                            51u8, 27u8, 91u8, 156u8, 118u8, 99u8, 46u8, 219u8, 190u8, 147u8, 205u8,
                            23u8, 106u8, 169u8, 121u8, 218u8, 208u8, 235u8, 135u8, 127u8, 243u8,
                            41u8, 55u8, 243u8, 235u8, 122u8, 57u8, 86u8, 37u8, 90u8, 208u8, 71u8,
                        ]
                    {
                        let entry = EpochIndex;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Current epoch authorities."]
                pub async fn authorities(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Authorities>()?
                        == [
                            39u8, 102u8, 251u8, 125u8, 230u8, 247u8, 174u8, 255u8, 2u8, 81u8, 86u8,
                            69u8, 182u8, 92u8, 191u8, 163u8, 66u8, 181u8, 247u8, 9u8, 57u8, 154u8,
                            239u8, 34u8, 25u8, 139u8, 119u8, 4u8, 131u8, 124u8, 135u8, 240u8,
                        ]
                    {
                        let entry = Authorities;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The slot at which the first epoch actually started. This is 0"]
                #[doc = " until the first block of the chain."]
                pub async fn genesis_slot(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<GenesisSlot>()?
                        == [
                            136u8, 244u8, 7u8, 142u8, 224u8, 33u8, 144u8, 186u8, 155u8, 144u8,
                            68u8, 81u8, 241u8, 57u8, 40u8, 207u8, 35u8, 39u8, 28u8, 41u8, 210u8,
                            213u8, 53u8, 195u8, 175u8, 119u8, 6u8, 175u8, 100u8, 192u8, 180u8,
                            73u8,
                        ]
                    {
                        let entry = GenesisSlot;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Current slot number."]
                pub async fn current_slot(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<CurrentSlot>()?
                        == [
                            233u8, 102u8, 77u8, 99u8, 103u8, 50u8, 151u8, 229u8, 46u8, 226u8,
                            181u8, 37u8, 117u8, 204u8, 234u8, 120u8, 116u8, 166u8, 80u8, 188u8,
                            92u8, 154u8, 137u8, 150u8, 79u8, 164u8, 29u8, 203u8, 2u8, 51u8, 123u8,
                            104u8,
                        ]
                    {
                        let entry = CurrentSlot;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The epoch randomness for the *current* epoch."]
                #[doc = ""]
                #[doc = " # Security"]
                #[doc = ""]
                #[doc = " This MUST NOT be used for gambling, as it can be influenced by a"]
                #[doc = " malicious validator in the short term. It MAY be used in many"]
                #[doc = " cryptographic protocols, however, so long as one remembers that this"]
                #[doc = " (like everything else on-chain) it is public. For example, it can be"]
                #[doc = " used where a number is needed that cannot have been chosen by an"]
                #[doc = " adversary, for purposes such as public-coin zero-knowledge proofs."]
                pub async fn randomness(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Randomness>()?
                        == [
                            191u8, 197u8, 25u8, 164u8, 104u8, 248u8, 247u8, 193u8, 244u8, 60u8,
                            181u8, 195u8, 248u8, 90u8, 41u8, 199u8, 82u8, 123u8, 72u8, 126u8, 18u8,
                            17u8, 128u8, 215u8, 34u8, 251u8, 227u8, 70u8, 166u8, 10u8, 104u8,
                            140u8,
                        ]
                    {
                        let entry = Randomness;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Pending epoch configuration change that will be applied when the next epoch is enacted."]
                pub async fn pending_epoch_config_change(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<PendingEpochConfigChange>()?
                        == [
                            98u8, 52u8, 22u8, 32u8, 76u8, 196u8, 89u8, 78u8, 119u8, 181u8, 17u8,
                            49u8, 220u8, 159u8, 195u8, 74u8, 33u8, 59u8, 15u8, 104u8, 26u8, 111u8,
                            165u8, 68u8, 147u8, 14u8, 86u8, 94u8, 250u8, 167u8, 146u8, 82u8,
                        ]
                    {
                        let entry = PendingEpochConfigChange;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Next epoch randomness."]
                pub async fn next_randomness(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<NextRandomness>()?
                        == [
                            185u8, 98u8, 45u8, 109u8, 253u8, 38u8, 238u8, 221u8, 240u8, 29u8, 38u8,
                            107u8, 118u8, 117u8, 131u8, 115u8, 21u8, 255u8, 203u8, 81u8, 243u8,
                            251u8, 91u8, 60u8, 163u8, 202u8, 125u8, 193u8, 173u8, 234u8, 166u8,
                            92u8,
                        ]
                    {
                        let entry = NextRandomness;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Next epoch authorities."]
                pub async fn next_authorities(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextAuthorities>()?
                        == [
                            211u8, 175u8, 218u8, 0u8, 212u8, 114u8, 210u8, 137u8, 146u8, 135u8,
                            78u8, 133u8, 85u8, 253u8, 140u8, 242u8, 101u8, 155u8, 159u8, 8u8,
                            217u8, 176u8, 234u8, 143u8, 212u8, 103u8, 198u8, 94u8, 121u8, 111u8,
                            56u8, 89u8,
                        ]
                    {
                        let entry = NextAuthorities;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Randomness under construction."]
                #[doc = ""]
                #[doc = " We make a trade-off between storage accesses and list length."]
                #[doc = " We store the under-construction randomness in segments of up to"]
                #[doc = " `UNDER_CONSTRUCTION_SEGMENT_LENGTH`."]
                #[doc = ""]
                #[doc = " Once a segment reaches this length, we begin the next one."]
                #[doc = " We reset all segments and return to `0` at the beginning of every"]
                #[doc = " epoch."]
                pub async fn segment_index(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<SegmentIndex>()?
                        == [
                            128u8, 45u8, 87u8, 58u8, 174u8, 152u8, 241u8, 156u8, 56u8, 192u8, 19u8,
                            45u8, 75u8, 160u8, 35u8, 253u8, 145u8, 11u8, 178u8, 81u8, 114u8, 117u8,
                            112u8, 107u8, 163u8, 208u8, 240u8, 151u8, 102u8, 176u8, 246u8, 5u8,
                        ]
                    {
                        let entry = SegmentIndex;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
                pub async fn under_construction(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<UnderConstruction>()?
                        == [
                            12u8, 167u8, 30u8, 96u8, 161u8, 63u8, 210u8, 63u8, 91u8, 199u8, 188u8,
                            78u8, 254u8, 255u8, 253u8, 202u8, 203u8, 26u8, 4u8, 105u8, 76u8, 125u8,
                            191u8, 245u8, 32u8, 97u8, 127u8, 129u8, 167u8, 80u8, 210u8, 123u8,
                        ]
                    {
                        let entry = UnderConstruction(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
                pub async fn under_construction_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<UnderConstruction>()?
                        == [
                            12u8, 167u8, 30u8, 96u8, 161u8, 63u8, 210u8, 63u8, 91u8, 199u8, 188u8,
                            78u8, 254u8, 255u8, 253u8, 202u8, 203u8, 26u8, 4u8, 105u8, 76u8, 125u8,
                            191u8, 245u8, 32u8, 97u8, 127u8, 129u8, 167u8, 80u8, 210u8, 123u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Temporary value (cleared at block finalization) which is `Some`"]
                #[doc = " if per-block initialization has already been called for current block."]
                pub async fn initialized(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Initialized>()?
                        == [
                            48u8, 206u8, 111u8, 118u8, 149u8, 175u8, 148u8, 53u8, 233u8, 82u8,
                            220u8, 57u8, 22u8, 164u8, 116u8, 228u8, 134u8, 237u8, 129u8, 195u8,
                            60u8, 169u8, 1u8, 164u8, 74u8, 177u8, 145u8, 112u8, 66u8, 198u8, 53u8,
                            157u8,
                        ]
                    {
                        let entry = Initialized;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " This field should always be populated during block processing unless"]
                #[doc = " secondary plain slots are enabled (which don't contain a VRF output)."]
                #[doc = ""]
                #[doc = " It is set in `on_initialize`, before it will contain the value from the last block."]
                pub async fn author_vrf_randomness(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<AuthorVrfRandomness>()?
                        == [
                            66u8, 235u8, 74u8, 252u8, 222u8, 135u8, 19u8, 28u8, 74u8, 191u8, 170u8,
                            197u8, 207u8, 127u8, 77u8, 121u8, 138u8, 138u8, 110u8, 187u8, 34u8,
                            14u8, 230u8, 43u8, 241u8, 241u8, 63u8, 163u8, 53u8, 179u8, 250u8,
                            247u8,
                        ]
                    {
                        let entry = AuthorVrfRandomness;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The block numbers when the last and current epoch have started, respectively `N-1` and"]
                #[doc = " `N`."]
                #[doc = " NOTE: We track this is in order to annotate the block number when a given pool of"]
                #[doc = " entropy was fixed (i.e. it was known to chain observers). Since epochs are defined in"]
                #[doc = " slots, which may be skipped, the block numbers may not line up with the slot numbers."]
                pub async fn epoch_start(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EpochStart>()?
                        == [
                            196u8, 39u8, 241u8, 20u8, 150u8, 180u8, 136u8, 4u8, 195u8, 205u8,
                            218u8, 10u8, 130u8, 131u8, 168u8, 243u8, 207u8, 249u8, 58u8, 195u8,
                            177u8, 119u8, 110u8, 243u8, 241u8, 3u8, 245u8, 56u8, 157u8, 5u8, 68u8,
                            60u8,
                        ]
                    {
                        let entry = EpochStart;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " How late the current block is compared to its parent."]
                #[doc = ""]
                #[doc = " This entry is populated as part of block execution and is cleaned up"]
                #[doc = " on block finalization. Querying this storage entry outside of block"]
                #[doc = " execution context should always yield zero."]
                pub async fn lateness(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Lateness>()?
                        == [
                            229u8, 230u8, 224u8, 89u8, 49u8, 213u8, 198u8, 236u8, 144u8, 56u8,
                            193u8, 234u8, 62u8, 242u8, 191u8, 199u8, 105u8, 131u8, 74u8, 63u8,
                            75u8, 1u8, 210u8, 49u8, 3u8, 128u8, 18u8, 77u8, 219u8, 146u8, 60u8,
                            88u8,
                        ]
                    {
                        let entry = Lateness;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The configuration for the current epoch. Should never be `None` as it is initialized in"]
                #[doc = " genesis."]
                pub async fn epoch_config(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EpochConfig>()?
                        == [
                            169u8, 189u8, 214u8, 159u8, 181u8, 232u8, 243u8, 4u8, 113u8, 24u8,
                            221u8, 229u8, 27u8, 35u8, 3u8, 121u8, 136u8, 88u8, 187u8, 193u8, 207u8,
                            153u8, 223u8, 225u8, 166u8, 183u8, 53u8, 3u8, 162u8, 207u8, 88u8,
                            133u8,
                        ]
                    {
                        let entry = EpochConfig;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The configuration for the next epoch, `None` if the config will not change"]
                #[doc = " (you can fallback to `EpochConfig` instead in that case)."]
                pub async fn next_epoch_config(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextEpochConfig>()?
                        == [
                            239u8, 125u8, 203u8, 223u8, 161u8, 107u8, 232u8, 54u8, 158u8, 100u8,
                            244u8, 140u8, 119u8, 58u8, 253u8, 245u8, 73u8, 236u8, 50u8, 67u8,
                            228u8, 162u8, 166u8, 168u8, 162u8, 152u8, 239u8, 246u8, 153u8, 223u8,
                            109u8, 121u8,
                        ]
                    {
                        let entry = NextEpochConfig;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The amount of time, in slots, that each epoch should last."]
                #[doc = " NOTE: Currently it is not possible to change the epoch duration after"]
                #[doc = " the chain has started. Attempting to do so will brick block production."]
                pub fn epoch_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Babe", "EpochDuration")?
                        == [
                            40u8, 54u8, 255u8, 20u8, 89u8, 2u8, 38u8, 235u8, 70u8, 145u8, 128u8,
                            227u8, 177u8, 3u8, 153u8, 91u8, 102u8, 159u8, 160u8, 139u8, 88u8,
                            111u8, 116u8, 90u8, 139u8, 12u8, 31u8, 236u8, 11u8, 113u8, 213u8,
                            254u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Babe")?;
                        let constant = pallet.constant("EpochDuration")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The expected average block time at which BABE should be creating"]
                #[doc = " blocks. Since BABE is probabilistic it is not trivial to figure out"]
                #[doc = " what the expected average block time should be based on the slot"]
                #[doc = " duration and the security parameter `c` (where `1 - c` represents"]
                #[doc = " the probability of a slot being empty)."]
                pub fn expected_block_time(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Babe", "ExpectedBlockTime")?
                        == [
                            249u8, 170u8, 37u8, 7u8, 132u8, 115u8, 106u8, 71u8, 116u8, 166u8, 78u8,
                            251u8, 242u8, 146u8, 99u8, 207u8, 204u8, 225u8, 157u8, 57u8, 19u8,
                            17u8, 202u8, 231u8, 50u8, 67u8, 17u8, 205u8, 238u8, 80u8, 154u8, 125u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Babe")?;
                        let constant = pallet.constant("ExpectedBlockTime")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Max number of authorities allowed"]
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Babe", "MaxAuthorities")?
                        == [
                            205u8, 23u8, 62u8, 93u8, 110u8, 248u8, 109u8, 245u8, 1u8, 57u8, 144u8,
                            146u8, 103u8, 0u8, 178u8, 246u8, 63u8, 80u8, 77u8, 155u8, 202u8, 208u8,
                            73u8, 194u8, 210u8, 49u8, 121u8, 99u8, 101u8, 222u8, 127u8, 206u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Babe")?;
                        let constant = pallet.constant("MaxAuthorities")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "Provide a set of uncles."]
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetUncles,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetUncles>()?
                        == [
                            5u8, 56u8, 71u8, 152u8, 103u8, 232u8, 101u8, 171u8, 200u8, 2u8, 177u8,
                            102u8, 0u8, 93u8, 210u8, 90u8, 56u8, 151u8, 5u8, 235u8, 227u8, 197u8,
                            189u8, 248u8, 2u8, 71u8, 49u8, 220u8, 212u8, 253u8, 235u8, 67u8,
                        ]
                    {
                        let call = SetUncles { new_uncles };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Uncles"]
                pub async fn uncles(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
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
                    if self.client.metadata().storage_hash::<Uncles>()?
                        == [
                            71u8, 135u8, 85u8, 172u8, 221u8, 165u8, 212u8, 2u8, 208u8, 50u8, 9u8,
                            92u8, 251u8, 25u8, 194u8, 123u8, 210u8, 4u8, 148u8, 30u8, 20u8, 146u8,
                            21u8, 210u8, 138u8, 128u8, 144u8, 152u8, 97u8, 57u8, 205u8, 231u8,
                        ]
                    {
                        let entry = Uncles;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Author of current block."]
                pub async fn author(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Author>()?
                        == [
                            191u8, 57u8, 3u8, 242u8, 220u8, 123u8, 103u8, 215u8, 149u8, 120u8,
                            20u8, 139u8, 146u8, 234u8, 180u8, 105u8, 129u8, 128u8, 114u8, 147u8,
                            114u8, 236u8, 23u8, 21u8, 15u8, 250u8, 180u8, 19u8, 177u8, 145u8, 77u8,
                            228u8,
                        ]
                    {
                        let entry = Author;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Whether uncles were already set in this block."]
                pub async fn did_set_uncles(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<DidSetUncles>()?
                        == [
                            64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
                            126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
                            83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
                        ]
                    {
                        let entry = DidSetUncles;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The number of blocks back we should accept uncles."]
                #[doc = " This means that we will deal with uncle-parents that are"]
                #[doc = " `UncleGenerations + 1` before `now`."]
                pub fn uncle_generations(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Authorship", "UncleGenerations")?
                        == [
                            0u8, 72u8, 57u8, 175u8, 222u8, 143u8, 191u8, 33u8, 163u8, 157u8, 202u8,
                            83u8, 186u8, 103u8, 162u8, 103u8, 227u8, 158u8, 239u8, 212u8, 205u8,
                            193u8, 226u8, 138u8, 5u8, 220u8, 221u8, 42u8, 7u8, 146u8, 173u8, 205u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Authorship")?;
                        let constant = pallet.constant("UncleGenerations")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                #[doc = "  types. See related functions below."]
                #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                #[doc = "  computation."]
                #[doc = ""]
                #[doc = "Related functions:"]
                #[doc = ""]
                #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                #[doc = "    that the transfer will not kill the origin account."]
                #[doc = "---------------------------------"]
                #[doc = "- Origin account is already in memory, so no DB operations for them."]
                #[doc = "# </weight>"]
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Transfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Transfer>()?
                        == [
                            250u8, 8u8, 164u8, 186u8, 80u8, 220u8, 134u8, 247u8, 142u8, 121u8,
                            34u8, 22u8, 169u8, 39u8, 6u8, 93u8, 72u8, 47u8, 44u8, 107u8, 9u8, 98u8,
                            203u8, 190u8, 136u8, 55u8, 251u8, 78u8, 216u8, 150u8, 98u8, 118u8,
                        ]
                    {
                        let call = Transfer { dest, value };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                #[doc = "If the new free or reserved balance is below the existential deposit,"]
                #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetBalance,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetBalance>()?
                        == [
                            232u8, 6u8, 27u8, 131u8, 163u8, 72u8, 148u8, 197u8, 14u8, 239u8, 94u8,
                            1u8, 32u8, 94u8, 17u8, 14u8, 123u8, 82u8, 39u8, 233u8, 77u8, 20u8,
                            40u8, 139u8, 222u8, 137u8, 103u8, 18u8, 126u8, 63u8, 200u8, 149u8,
                        ]
                    {
                        let call = SetBalance {
                            who,
                            new_free,
                            new_reserved,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "# <weight>"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                #[doc = "# </weight>"]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceTransfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceTransfer>()?
                        == [
                            120u8, 66u8, 111u8, 84u8, 176u8, 241u8, 214u8, 118u8, 219u8, 75u8,
                            127u8, 222u8, 45u8, 33u8, 204u8, 147u8, 126u8, 214u8, 101u8, 190u8,
                            37u8, 37u8, 159u8, 166u8, 61u8, 143u8, 22u8, 32u8, 15u8, 83u8, 221u8,
                            230u8,
                        ]
                    {
                        let call = ForceTransfer {
                            source,
                            dest,
                            value,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferKeepAlive,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferKeepAlive>()?
                        == [
                            111u8, 233u8, 125u8, 71u8, 223u8, 141u8, 112u8, 94u8, 157u8, 11u8,
                            88u8, 7u8, 239u8, 145u8, 247u8, 183u8, 245u8, 87u8, 157u8, 35u8, 49u8,
                            91u8, 54u8, 103u8, 101u8, 76u8, 110u8, 94u8, 81u8, 170u8, 153u8, 209u8,
                        ]
                    {
                        let call = TransferKeepAlive { dest, value };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true). # <weight>"]
                #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                #[doc = "  #</weight>"]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferAll,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferAll>()?
                        == [
                            240u8, 165u8, 185u8, 144u8, 24u8, 149u8, 15u8, 46u8, 60u8, 147u8, 19u8,
                            187u8, 96u8, 24u8, 150u8, 53u8, 151u8, 232u8, 200u8, 164u8, 176u8,
                            167u8, 8u8, 23u8, 63u8, 135u8, 68u8, 110u8, 5u8, 21u8, 35u8, 78u8,
                        ]
                    {
                        let call = TransferAll { dest, keep_alive };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceUnreserve,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceUnreserve>()?
                        == [
                            106u8, 42u8, 48u8, 136u8, 41u8, 155u8, 214u8, 112u8, 99u8, 122u8,
                            202u8, 250u8, 95u8, 60u8, 182u8, 13u8, 25u8, 149u8, 212u8, 212u8,
                            247u8, 191u8, 130u8, 95u8, 84u8, 252u8, 252u8, 197u8, 244u8, 149u8,
                            103u8, 67u8,
                        ]
                    {
                        let call = ForceUnreserve { who, amount };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
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
                #[doc = " The total units issued in the system."]
                pub async fn total_issuance(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<TotalIssuance>()?
                        == [
                            1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
                            156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
                            20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
                        ]
                    {
                        let entry = TotalIssuance;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            129u8, 169u8, 171u8, 206u8, 229u8, 178u8, 69u8, 118u8, 199u8, 64u8,
                            254u8, 67u8, 16u8, 154u8, 160u8, 197u8, 177u8, 161u8, 148u8, 199u8,
                            78u8, 219u8, 187u8, 83u8, 99u8, 110u8, 207u8, 252u8, 243u8, 39u8, 46u8,
                            106u8,
                        ]
                    {
                        let entry = Account(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub async fn account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            129u8, 169u8, 171u8, 206u8, 229u8, 178u8, 69u8, 118u8, 199u8, 64u8,
                            254u8, 67u8, 16u8, 154u8, 160u8, 197u8, 177u8, 161u8, 148u8, 199u8,
                            78u8, 219u8, 187u8, 83u8, 99u8, 110u8, 207u8, 252u8, 243u8, 39u8, 46u8,
                            106u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub async fn locks(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Locks>()?
                        == [
                            31u8, 76u8, 213u8, 60u8, 86u8, 11u8, 155u8, 151u8, 33u8, 212u8, 74u8,
                            89u8, 174u8, 74u8, 195u8, 107u8, 29u8, 163u8, 178u8, 34u8, 209u8, 8u8,
                            201u8, 237u8, 77u8, 99u8, 205u8, 212u8, 236u8, 132u8, 2u8, 252u8,
                        ]
                    {
                        let entry = Locks(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub async fn locks_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Locks>()?
                        == [
                            31u8, 76u8, 213u8, 60u8, 86u8, 11u8, 155u8, 151u8, 33u8, 212u8, 74u8,
                            89u8, 174u8, 74u8, 195u8, 107u8, 29u8, 163u8, 178u8, 34u8, 209u8, 8u8,
                            201u8, 237u8, 77u8, 99u8, 205u8, 212u8, 236u8, 132u8, 2u8, 252u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Named reserves on some account balances."]
                pub async fn reserves(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Reserves>()?
                        == [
                            103u8, 6u8, 69u8, 151u8, 81u8, 40u8, 146u8, 113u8, 56u8, 239u8, 104u8,
                            31u8, 168u8, 242u8, 141u8, 121u8, 213u8, 213u8, 114u8, 63u8, 62u8,
                            47u8, 91u8, 119u8, 57u8, 91u8, 95u8, 81u8, 19u8, 208u8, 59u8, 146u8,
                        ]
                    {
                        let entry = Reserves(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Named reserves on some account balances."]
                pub async fn reserves_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Reserves>()?
                        == [
                            103u8, 6u8, 69u8, 151u8, 81u8, 40u8, 146u8, 113u8, 56u8, 239u8, 104u8,
                            31u8, 168u8, 242u8, 141u8, 121u8, 213u8, 213u8, 114u8, 63u8, 62u8,
                            47u8, 91u8, 119u8, 57u8, 91u8, 95u8, 81u8, 19u8, 208u8, 59u8, 146u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Storage version of the pallet."]
                #[doc = ""]
                #[doc = " This is set to v2.0.0 for new networks."]
                pub async fn storage_version(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<StorageVersion>()?
                        == [
                            135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
                            235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
                            174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
                            173u8, 96u8,
                        ]
                    {
                        let entry = StorageVersion;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Balances", "ExistentialDeposit")?
                        == [
                            73u8, 138u8, 39u8, 150u8, 84u8, 112u8, 183u8, 185u8, 133u8, 100u8,
                            109u8, 141u8, 83u8, 77u8, 76u8, 142u8, 153u8, 35u8, 251u8, 35u8, 33u8,
                            220u8, 164u8, 198u8, 118u8, 229u8, 12u8, 175u8, 147u8, 188u8, 164u8,
                            12u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Balances")?;
                        let constant = pallet.constant("ExistentialDeposit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Balances", "MaxLocks")?
                        == [
                            250u8, 58u8, 19u8, 15u8, 35u8, 113u8, 227u8, 89u8, 39u8, 75u8, 21u8,
                            108u8, 202u8, 32u8, 163u8, 167u8, 207u8, 233u8, 69u8, 151u8, 53u8,
                            164u8, 230u8, 16u8, 14u8, 22u8, 172u8, 46u8, 36u8, 216u8, 29u8, 1u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Balances")?;
                        let constant = pallet.constant("MaxLocks")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Balances", "MaxReserves")?
                        == [
                            24u8, 30u8, 77u8, 89u8, 216u8, 114u8, 140u8, 11u8, 127u8, 252u8, 130u8,
                            203u8, 4u8, 55u8, 62u8, 240u8, 65u8, 182u8, 187u8, 189u8, 140u8, 6u8,
                            177u8, 216u8, 159u8, 108u8, 18u8, 73u8, 95u8, 67u8, 62u8, 50u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Balances")?;
                        let constant = pallet.constant("MaxReserves")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextFeeMultiplier>()?
                        == [
                            232u8, 48u8, 68u8, 202u8, 209u8, 29u8, 249u8, 71u8, 0u8, 84u8, 229u8,
                            250u8, 176u8, 203u8, 27u8, 26u8, 34u8, 55u8, 83u8, 183u8, 224u8, 40u8,
                            62u8, 127u8, 131u8, 88u8, 128u8, 9u8, 56u8, 178u8, 31u8, 183u8,
                        ]
                    {
                        let entry = NextFeeMultiplier;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn storage_version(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<StorageVersion>()?
                        == [
                            219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
                            200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
                            58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
                        ]
                    {
                        let entry = StorageVersion;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The fee to be paid for making a transaction; the per-byte portion."]
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("TransactionPayment", "TransactionByteFee")?
                        == [
                            183u8, 21u8, 105u8, 53u8, 108u8, 121u8, 186u8, 76u8, 202u8, 99u8,
                            230u8, 139u8, 18u8, 171u8, 99u8, 19u8, 189u8, 6u8, 194u8, 165u8, 226u8,
                            21u8, 230u8, 225u8, 53u8, 180u8, 49u8, 17u8, 0u8, 30u8, 213u8, 142u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("TransactionPayment")?;
                        let constant = pallet.constant("TransactionByteFee")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("TransactionPayment", "OperationalFeeMultiplier")?
                        == [
                            161u8, 232u8, 150u8, 43u8, 106u8, 83u8, 56u8, 248u8, 54u8, 123u8,
                            244u8, 73u8, 5u8, 49u8, 245u8, 150u8, 70u8, 92u8, 158u8, 207u8, 127u8,
                            115u8, 211u8, 21u8, 24u8, 136u8, 89u8, 44u8, 151u8, 211u8, 235u8,
                            196u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("TransactionPayment")?;
                        let constant = pallet.constant("OperationalFeeMultiplier")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The polynomial that is applied in order to derive fee from weight."]
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
                    if self
                        .client
                        .metadata()
                        .constant_hash("TransactionPayment", "WeightToFee")?
                        == [
                            236u8, 89u8, 172u8, 50u8, 101u8, 218u8, 151u8, 158u8, 128u8, 186u8,
                            120u8, 84u8, 103u8, 248u8, 220u8, 191u8, 9u8, 185u8, 114u8, 160u8,
                            104u8, 235u8, 167u8, 83u8, 228u8, 6u8, 56u8, 179u8, 160u8, 4u8, 230u8,
                            12u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("TransactionPayment")?;
                        let constant = pallet.constant("WeightToFee")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ForceSetIsActivated {
                pub is_activated: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceSetIsActivated {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_is_activated";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct ForceSetNextSetId {
                pub next_set_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceSetNextSetId {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_next_set_id";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Lock {
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Lock {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "lock";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct BurnAsset {
                pub asset_id: ::core::primitive::u32,
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for BurnAsset {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "burn_asset";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetAssetName {
                pub asset_name: ::std::vec::Vec<::core::primitive::u8>,
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetAssetName {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "set_asset_name";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TranferFromPalletAccount {
                pub receiver:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for TranferFromPalletAccount {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "tranfer_from_pallet_account";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct LockNft {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for LockNft {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "lock_nft";
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
                #[doc = "Submit observations."]
                pub fn submit_observations(
                    &self,
                    payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    signature: runtime_types::sp_runtime::MultiSignature,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SubmitObservations,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SubmitObservations>()?
                        == [
                            131u8, 234u8, 87u8, 150u8, 173u8, 60u8, 177u8, 246u8, 203u8, 153u8,
                            248u8, 208u8, 29u8, 55u8, 70u8, 11u8, 196u8, 185u8, 202u8, 141u8,
                            186u8, 11u8, 248u8, 87u8, 3u8, 149u8, 169u8, 247u8, 68u8, 173u8, 10u8,
                            85u8,
                        ]
                    {
                        let call = SubmitObservations { payload, signature };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn force_set_is_activated(
                    &self,
                    is_activated: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceSetIsActivated,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceSetIsActivated>()?
                        == [
                            82u8, 165u8, 95u8, 78u8, 177u8, 2u8, 9u8, 78u8, 62u8, 221u8, 119u8,
                            84u8, 52u8, 1u8, 196u8, 111u8, 216u8, 195u8, 53u8, 234u8, 116u8, 201u8,
                            163u8, 200u8, 190u8, 66u8, 30u8, 65u8, 12u8, 134u8, 63u8, 66u8,
                        ]
                    {
                        let call = ForceSetIsActivated { is_activated };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn force_set_next_set_id(
                    &self,
                    next_set_id: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceSetNextSetId,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceSetNextSetId>()?
                        == [
                            205u8, 181u8, 231u8, 95u8, 68u8, 241u8, 214u8, 131u8, 173u8, 85u8,
                            167u8, 205u8, 47u8, 218u8, 230u8, 242u8, 243u8, 118u8, 96u8, 112u8,
                            241u8, 93u8, 30u8, 16u8, 148u8, 181u8, 96u8, 56u8, 146u8, 67u8, 171u8,
                            13u8,
                        ]
                    {
                        let call = ForceSetNextSetId { next_set_id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn force_set_planned_validators(
                    &self,
                    validators: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceSetPlannedValidators,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .call_hash::<ForceSetPlannedValidators>()?
                        == [
                            241u8, 123u8, 44u8, 41u8, 232u8, 178u8, 105u8, 119u8, 13u8, 177u8,
                            228u8, 191u8, 70u8, 65u8, 209u8, 225u8, 89u8, 181u8, 8u8, 188u8, 198u8,
                            126u8, 17u8, 25u8, 213u8, 114u8, 202u8, 180u8, 57u8, 109u8, 111u8,
                            211u8,
                        ]
                    {
                        let call = ForceSetPlannedValidators { validators };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Emits `Locked` event when successful."]
                pub fn lock(
                    &self,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Lock, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Lock>()?
                        == [
                            59u8, 116u8, 159u8, 139u8, 246u8, 131u8, 64u8, 221u8, 230u8, 19u8,
                            15u8, 94u8, 163u8, 177u8, 34u8, 253u8, 31u8, 227u8, 70u8, 171u8, 188u8,
                            176u8, 131u8, 147u8, 81u8, 123u8, 20u8, 195u8, 187u8, 184u8, 25u8,
                            97u8,
                        ]
                    {
                        let call = Lock {
                            receiver_id,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        MintAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<MintAsset>()?
                        == [
                            21u8, 208u8, 168u8, 222u8, 234u8, 151u8, 158u8, 246u8, 111u8, 118u8,
                            110u8, 200u8, 199u8, 47u8, 250u8, 75u8, 105u8, 110u8, 207u8, 175u8,
                            253u8, 2u8, 79u8, 93u8, 141u8, 164u8, 198u8, 151u8, 144u8, 237u8,
                            103u8, 45u8,
                        ]
                    {
                        let call = MintAsset {
                            asset_id,
                            sender_id,
                            receiver,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn burn_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        BurnAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<BurnAsset>()?
                        == [
                            173u8, 20u8, 42u8, 239u8, 193u8, 223u8, 201u8, 117u8, 74u8, 90u8,
                            131u8, 231u8, 161u8, 240u8, 7u8, 185u8, 177u8, 167u8, 63u8, 103u8,
                            252u8, 231u8, 53u8, 231u8, 53u8, 154u8, 68u8, 180u8, 221u8, 15u8,
                            180u8, 233u8,
                        ]
                    {
                        let call = BurnAsset {
                            asset_id,
                            receiver_id,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn set_asset_name(
                    &self,
                    asset_name: ::std::vec::Vec<::core::primitive::u8>,
                    asset_id: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetAssetName,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetAssetName>()?
                        == [
                            177u8, 251u8, 214u8, 114u8, 89u8, 103u8, 116u8, 65u8, 235u8, 163u8,
                            111u8, 152u8, 64u8, 142u8, 94u8, 199u8, 216u8, 227u8, 205u8, 177u8,
                            156u8, 166u8, 20u8, 244u8, 7u8, 33u8, 56u8, 134u8, 33u8, 205u8, 252u8,
                            48u8,
                        ]
                    {
                        let call = SetAssetName {
                            asset_name,
                            asset_id,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn tranfer_from_pallet_account(
                    &self,
                    receiver: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TranferFromPalletAccount,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .call_hash::<TranferFromPalletAccount>()?
                        == [
                            251u8, 35u8, 177u8, 150u8, 171u8, 156u8, 141u8, 217u8, 58u8, 96u8,
                            222u8, 176u8, 182u8, 75u8, 51u8, 237u8, 28u8, 247u8, 74u8, 20u8, 26u8,
                            5u8, 197u8, 251u8, 225u8, 24u8, 244u8, 123u8, 103u8, 124u8, 200u8,
                            247u8,
                        ]
                    {
                        let call = TranferFromPalletAccount { receiver, amount };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn lock_nft(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        LockNft,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<LockNft>()?
                        == [
                            120u8, 154u8, 76u8, 198u8, 157u8, 253u8, 25u8, 14u8, 241u8, 19u8, 89u8,
                            10u8, 237u8, 126u8, 77u8, 102u8, 103u8, 85u8, 100u8, 82u8, 58u8, 234u8,
                            23u8, 126u8, 51u8, 186u8, 199u8, 225u8, 18u8, 137u8, 50u8, 102u8,
                        ]
                    {
                        let call = LockNft {
                            class,
                            instance,
                            receiver_id,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_appchain::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A new set of validators is waiting to be changed."]
            pub struct NewPlannedValidators {
                pub set_id: ::core::primitive::u32,
                pub validators: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::Event for NewPlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NewPlannedValidators";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An `amount` of native token has been locked in the appchain to indicate that"]
            #[doc = "it will be cross-chain transferred to the mainchain."]
            pub struct Locked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub receiver: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
                pub sequence: ::core::primitive::u64,
            }
            impl ::subxt::Event for Locked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Locked";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An `amount` was unlocked to `receiver` from `sender`."]
            pub struct Unlocked {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unlocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An `amount` unlock to `receiver` from `sender` failed."]
            pub struct UnlockFailed {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for UnlockFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "UnlockFailed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AssetMinted {
                pub asset_id: ::core::primitive::u32,
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for AssetMinted {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMinted";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AssetBurned {
                pub asset_id: ::core::primitive::u32,
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub receiver: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
                pub sequence: ::core::primitive::u64,
            }
            impl ::subxt::Event for AssetBurned {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetBurned";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AssetMintFailed {
                pub asset_id: ::core::primitive::u32,
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for AssetMintFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMintFailed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AssetIdGetFailed {
                pub token_id: ::std::vec::Vec<::core::primitive::u8>,
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for AssetIdGetFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetIdGetFailed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TransferredFromPallet {
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for TransferredFromPallet {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "TransferredFromPallet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct NftLocked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub receiver: ::std::vec::Vec<::core::primitive::u8>,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub sequence: ::core::primitive::u64,
            }
            impl ::subxt::Event for NftLocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NftLocked";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct NftUnlocked {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Event for NftUnlocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NftUnlocked";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct NftUnlockFailed {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::sp_core::crypto::AccountId32,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Event for NftUnlockFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NftUnlockFailed";
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
            pub struct OctopusPalletId;
            impl ::subxt::StorageEntry for OctopusPalletId {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "OctopusPalletId";
                type Value = ::core::option::Option<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NotificationHistory<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for NotificationHistory<'_> {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NotificationHistory";
                type Value = ::core::option::Option<
                    runtime_types::pallet_octopus_appchain::NotificationResult,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct GitVersion;
            impl ::subxt::StorageEntry for GitVersion {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "GitVersion";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
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
                pub async fn anchor_contract(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<AnchorContract>()?
                        == [
                            39u8, 254u8, 249u8, 168u8, 27u8, 186u8, 34u8, 81u8, 114u8, 252u8,
                            137u8, 120u8, 169u8, 110u8, 85u8, 144u8, 32u8, 155u8, 158u8, 251u8,
                            126u8, 107u8, 64u8, 213u8, 87u8, 20u8, 213u8, 218u8, 46u8, 1u8, 107u8,
                            208u8,
                        ]
                    {
                        let entry = AnchorContract;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn asset_id_by_name(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<AssetIdByName>()?
                        == [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ]
                    {
                        let entry = AssetIdByName(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn asset_id_by_name_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIdByName<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<AssetIdByName>()?
                        == [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Whether the appchain is activated."]
                #[doc = ""]
                #[doc = " Only an active appchain will communicate with the mainchain and pay block rewards."]
                pub async fn is_activated(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<IsActivated>()?
                        == [
                            79u8, 161u8, 11u8, 40u8, 81u8, 93u8, 156u8, 140u8, 189u8, 249u8, 217u8,
                            21u8, 180u8, 193u8, 93u8, 129u8, 149u8, 250u8, 97u8, 1u8, 205u8, 234u8,
                            123u8, 232u8, 167u8, 194u8, 188u8, 247u8, 182u8, 179u8, 196u8, 209u8,
                        ]
                    {
                        let entry = IsActivated;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn next_set_id(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<NextSetId>()?
                        == [
                            143u8, 235u8, 138u8, 252u8, 147u8, 133u8, 43u8, 104u8, 147u8, 238u8,
                            74u8, 115u8, 65u8, 100u8, 51u8, 239u8, 106u8, 122u8, 127u8, 146u8,
                            234u8, 160u8, 38u8, 18u8, 20u8, 59u8, 28u8, 254u8, 105u8, 194u8, 56u8,
                            31u8,
                        ]
                    {
                        let entry = NextSetId;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn planned_validators(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<PlannedValidators>()?
                        == [
                            43u8, 15u8, 180u8, 107u8, 112u8, 151u8, 132u8, 149u8, 199u8, 222u8,
                            232u8, 177u8, 36u8, 194u8, 139u8, 153u8, 17u8, 50u8, 203u8, 150u8,
                            242u8, 34u8, 89u8, 184u8, 158u8, 112u8, 73u8, 220u8, 96u8, 96u8, 160u8,
                            142u8,
                        ]
                    {
                        let entry = PlannedValidators;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn next_notification_id(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<NextNotificationId>()?
                        == [
                            116u8, 245u8, 172u8, 65u8, 54u8, 136u8, 117u8, 21u8, 20u8, 36u8, 37u8,
                            8u8, 204u8, 118u8, 33u8, 67u8, 247u8, 121u8, 86u8, 178u8, 130u8, 76u8,
                            18u8, 8u8, 135u8, 34u8, 142u8, 60u8, 197u8, 177u8, 161u8, 64u8,
                        ]
                    {
                        let entry = NextNotificationId;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn observations(
                    &self,
                    _0: &runtime_types::pallet_octopus_appchain::ObservationType,
                    _1: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_octopus_appchain::Observation<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Observations>()?
                        == [
                            21u8, 184u8, 9u8, 134u8, 224u8, 3u8, 125u8, 59u8, 231u8, 69u8, 173u8,
                            180u8, 184u8, 55u8, 118u8, 185u8, 116u8, 117u8, 29u8, 137u8, 142u8,
                            80u8, 194u8, 17u8, 18u8, 4u8, 125u8, 188u8, 208u8, 227u8, 115u8, 231u8,
                        ]
                    {
                        let entry = Observations(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn observations_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Observations<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Observations>()?
                        == [
                            21u8, 184u8, 9u8, 134u8, 224u8, 3u8, 125u8, 59u8, 231u8, 69u8, 173u8,
                            180u8, 184u8, 55u8, 118u8, 185u8, 116u8, 117u8, 29u8, 137u8, 142u8,
                            80u8, 194u8, 17u8, 18u8, 4u8, 125u8, 188u8, 208u8, 227u8, 115u8, 231u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn observing(
                    &self,
                    _0: &runtime_types::pallet_octopus_appchain::Observation<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Observing>()?
                        == [
                            114u8, 218u8, 121u8, 218u8, 217u8, 165u8, 24u8, 151u8, 67u8, 250u8,
                            65u8, 226u8, 7u8, 201u8, 126u8, 126u8, 227u8, 21u8, 123u8, 65u8, 39u8,
                            11u8, 80u8, 246u8, 253u8, 3u8, 222u8, 205u8, 176u8, 33u8, 4u8, 159u8,
                        ]
                    {
                        let entry = Observing(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn observing_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Observing<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Observing>()?
                        == [
                            114u8, 218u8, 121u8, 218u8, 217u8, 165u8, 24u8, 151u8, 67u8, 250u8,
                            65u8, 226u8, 7u8, 201u8, 126u8, 126u8, 227u8, 21u8, 123u8, 65u8, 39u8,
                            11u8, 80u8, 246u8, 253u8, 3u8, 222u8, 205u8, 176u8, 33u8, 4u8, 159u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn octopus_pallet_id(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<OctopusPalletId>()?
                        == [
                            40u8, 44u8, 123u8, 181u8, 124u8, 176u8, 194u8, 41u8, 209u8, 34u8, 81u8,
                            95u8, 168u8, 100u8, 48u8, 60u8, 108u8, 53u8, 86u8, 85u8, 227u8, 73u8,
                            92u8, 204u8, 37u8, 236u8, 196u8, 167u8, 168u8, 34u8, 1u8, 249u8,
                        ]
                    {
                        let entry = OctopusPalletId;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn notification_history(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_octopus_appchain::NotificationResult,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<NotificationHistory>()?
                        == [
                            225u8, 191u8, 108u8, 250u8, 203u8, 210u8, 122u8, 2u8, 99u8, 101u8,
                            88u8, 151u8, 134u8, 3u8, 238u8, 247u8, 178u8, 53u8, 184u8, 103u8,
                            168u8, 195u8, 157u8, 48u8, 119u8, 0u8, 100u8, 39u8, 217u8, 47u8, 146u8,
                            193u8,
                        ]
                    {
                        let entry = NotificationHistory(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn notification_history_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NotificationHistory<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<NotificationHistory>()?
                        == [
                            225u8, 191u8, 108u8, 250u8, 203u8, 210u8, 122u8, 2u8, 99u8, 101u8,
                            88u8, 151u8, 134u8, 3u8, 238u8, 247u8, 178u8, 53u8, 184u8, 103u8,
                            168u8, 195u8, 157u8, 48u8, 119u8, 0u8, 100u8, 39u8, 217u8, 47u8, 146u8,
                            193u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn git_version(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<GitVersion>()?
                        == [
                            10u8, 147u8, 169u8, 151u8, 159u8, 49u8, 156u8, 150u8, 193u8, 119u8,
                            142u8, 14u8, 209u8, 167u8, 213u8, 190u8, 32u8, 113u8, 55u8, 15u8, 65u8,
                            163u8, 137u8, 102u8, 26u8, 173u8, 93u8, 252u8, 234u8, 94u8, 5u8, 174u8,
                        ]
                    {
                        let entry = GitVersion;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                pub fn pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAppchain", "PalletId")?
                        == [
                            236u8, 223u8, 87u8, 151u8, 53u8, 214u8, 193u8, 211u8, 33u8, 7u8, 9u8,
                            168u8, 166u8, 90u8, 212u8, 102u8, 32u8, 118u8, 46u8, 106u8, 4u8, 241u8,
                            130u8, 67u8, 77u8, 46u8, 1u8, 145u8, 20u8, 139u8, 56u8, 25u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAppchain")?;
                        let constant = pallet.constant("PalletId")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A grace period after we send transaction."]
                #[doc = ""]
                #[doc = " To avoid sending too many transactions, we only attempt to send one"]
                #[doc = " every `GRACE_PERIOD` blocks. We use Local Storage to coordinate"]
                #[doc = " sending between distinct runs of this offchain worker."]
                pub fn grace_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAppchain", "GracePeriod")?
                        == [
                            186u8, 247u8, 209u8, 75u8, 234u8, 165u8, 168u8, 20u8, 46u8, 13u8,
                            122u8, 144u8, 170u8, 135u8, 80u8, 47u8, 189u8, 64u8, 113u8, 223u8,
                            206u8, 57u8, 246u8, 194u8, 60u8, 54u8, 0u8, 131u8, 124u8, 242u8, 41u8,
                            116u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAppchain")?;
                        let constant = pallet.constant("GracePeriod")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A configuration for base priority of unsigned transactions."]
                #[doc = ""]
                #[doc = " This is exposed so that it can be tuned for particular runtime, when"]
                #[doc = " multiple pallets send unsigned transactions."]
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAppchain", "UnsignedPriority")?
                        == [
                            235u8, 29u8, 23u8, 191u8, 31u8, 101u8, 167u8, 228u8, 144u8, 234u8,
                            94u8, 85u8, 113u8, 178u8, 135u8, 146u8, 65u8, 220u8, 233u8, 202u8,
                            200u8, 166u8, 217u8, 159u8, 109u8, 216u8, 3u8, 7u8, 47u8, 210u8, 35u8,
                            116u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAppchain")?;
                        let constant = pallet.constant("UnsignedPriority")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn request_event_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAppchain", "RequestEventLimit")?
                        == [
                            40u8, 22u8, 193u8, 130u8, 169u8, 43u8, 232u8, 243u8, 184u8, 208u8,
                            142u8, 85u8, 148u8, 194u8, 126u8, 240u8, 180u8, 80u8, 218u8, 198u8,
                            239u8, 182u8, 50u8, 117u8, 57u8, 231u8, 204u8, 127u8, 140u8, 93u8,
                            162u8, 181u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAppchain")?;
                        let constant = pallet.constant("RequestEventLimit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
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
                #[doc = "Set `HistoryDepth` value. This function will delete any history information"]
                #[doc = "when `HistoryDepth` is reduced."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `new_history_depth`: The new history depth you would like to set."]
                #[doc = "- `era_items_deleted`: The number of items that will be deleted by this dispatch. This"]
                #[doc = "  should report all the storage items that will be deleted by clearing old era history."]
                #[doc = "  Needed to report an accurate weight for the dispatch. Trusted by `Root` to report an"]
                #[doc = "  accurate number."]
                #[doc = ""]
                #[doc = "Origin must be root."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- E: Number of history depths removed, i.e. 10 -> 7 = 3"]
                #[doc = "- Weight: O(E)"]
                #[doc = "- DB Weight:"]
                #[doc = "    - Reads: Current Era, History Depth"]
                #[doc = "    - Writes: History Depth"]
                #[doc = "    - Clear Prefix Each: Era Stakers, EraStakersClipped, ErasValidatorPrefs"]
                #[doc = "    - Writes Each: ErasValidatorReward, ErasRewardPoints, ErasTotalStake,"]
                #[doc = "      ErasStartSessionIndex"]
                #[doc = "# </weight>"]
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetHistoryDepth,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetHistoryDepth>()?
                        == [
                            128u8, 149u8, 139u8, 192u8, 213u8, 239u8, 248u8, 215u8, 57u8, 145u8,
                            177u8, 225u8, 43u8, 214u8, 228u8, 14u8, 213u8, 181u8, 18u8, 40u8,
                            242u8, 1u8, 210u8, 87u8, 143u8, 78u8, 0u8, 23u8, 145u8, 46u8, 210u8,
                            168u8,
                        ]
                    {
                        let call = SetHistoryDepth {
                            new_history_depth,
                            era_items_deleted,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn force_set_era_payout(
                    &self,
                    era_payout: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceSetEraPayout,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceSetEraPayout>()?
                        == [
                            31u8, 235u8, 56u8, 254u8, 54u8, 192u8, 0u8, 180u8, 57u8, 94u8, 106u8,
                            16u8, 80u8, 207u8, 215u8, 86u8, 231u8, 74u8, 87u8, 174u8, 62u8, 71u8,
                            31u8, 202u8, 30u8, 251u8, 225u8, 108u8, 217u8, 26u8, 173u8, 62u8,
                        ]
                    {
                        let call = ForceSetEraPayout { era_payout };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_lpos::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Notifies the mainchain to prepare the next era."]
            pub struct PlanNewEra {
                pub era_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for PlanNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEra";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Failed to notify the mainchain to prepare the next era."]
            pub struct PlanNewEraFailed;
            impl ::subxt::Event for PlanNewEraFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEraFailed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Trigger new era."]
            pub struct TriggerNewEra;
            impl ::subxt::Event for TriggerNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "TriggerNewEra";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Notifies the mainchain to pay the validator rewards of `era_index`."]
            #[doc = "`excluded_validators` were excluded because they were not working properly."]
            pub struct EraPayout {
                pub era_index: ::core::primitive::u32,
                pub excluded_validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayout";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Failed to notify the mainchain to pay the validator rewards of `era_index`."]
            pub struct EraPayoutFailed {
                pub era_index: ::core::primitive::u32,
            }
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
                #[doc = " Number of eras to keep in history."]
                #[doc = ""]
                #[doc = " Information is kept for eras in `[current_era - history_depth; current_era]`."]
                #[doc = ""]
                #[doc = " Must be more than the number of eras delayed by session otherwise. I.e. active era must"]
                #[doc = " always be in history. I.e. `active_era > current_era - history_depth` must be"]
                #[doc = " guaranteed."]
                pub async fn history_depth(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<HistoryDepth>()?
                        == [
                            41u8, 54u8, 118u8, 245u8, 75u8, 136u8, 220u8, 25u8, 55u8, 255u8, 149u8,
                            177u8, 49u8, 155u8, 167u8, 188u8, 170u8, 29u8, 251u8, 44u8, 240u8,
                            250u8, 225u8, 205u8, 102u8, 74u8, 25u8, 47u8, 52u8, 235u8, 204u8,
                            167u8,
                        ]
                    {
                        let entry = HistoryDepth;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current era index."]
                #[doc = ""]
                #[doc = " This is the latest planned era, depending on how the Session pallet queues the validator"]
                #[doc = " set, it might be active or not."]
                pub async fn current_era(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<CurrentEra>()?
                        == [
                            105u8, 150u8, 49u8, 122u8, 4u8, 78u8, 8u8, 121u8, 34u8, 136u8, 157u8,
                            227u8, 59u8, 139u8, 7u8, 253u8, 7u8, 10u8, 117u8, 71u8, 240u8, 74u8,
                            86u8, 36u8, 198u8, 37u8, 153u8, 93u8, 196u8, 22u8, 192u8, 243u8,
                        ]
                    {
                        let entry = CurrentEra;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The active era information, it holds index and start."]
                #[doc = ""]
                #[doc = " The active era is the era being currently rewarded. Validator set of this era must be"]
                #[doc = " equal to [`SessionInterface::validators`]."]
                pub async fn active_era(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_octopus_lpos::ActiveEraInfo>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ActiveEra>()?
                        == [
                            134u8, 251u8, 154u8, 229u8, 115u8, 221u8, 194u8, 18u8, 186u8, 224u8,
                            229u8, 98u8, 62u8, 124u8, 56u8, 81u8, 224u8, 104u8, 114u8, 2u8, 81u8,
                            33u8, 217u8, 208u8, 137u8, 154u8, 31u8, 19u8, 253u8, 83u8, 55u8, 142u8,
                        ]
                    {
                        let entry = ActiveEra;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
                pub async fn eras_start_session_index(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ErasStartSessionIndex>()?
                        == [
                            92u8, 157u8, 168u8, 144u8, 132u8, 3u8, 212u8, 80u8, 230u8, 229u8,
                            251u8, 218u8, 97u8, 55u8, 79u8, 100u8, 163u8, 91u8, 32u8, 246u8, 122u8,
                            78u8, 149u8, 214u8, 103u8, 249u8, 119u8, 20u8, 101u8, 116u8, 110u8,
                            185u8,
                        ]
                    {
                        let entry = ErasStartSessionIndex(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
                pub async fn eras_start_session_index_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ErasStartSessionIndex>()?
                        == [
                            92u8, 157u8, 168u8, 144u8, 132u8, 3u8, 212u8, 80u8, 230u8, 229u8,
                            251u8, 218u8, 97u8, 55u8, 79u8, 100u8, 163u8, 91u8, 32u8, 246u8, 122u8,
                            78u8, 149u8, 214u8, 103u8, 249u8, 119u8, 20u8, 101u8, 116u8, 110u8,
                            185u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub async fn eras_stakers(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ErasStakers>()?
                        == [
                            85u8, 187u8, 55u8, 106u8, 130u8, 221u8, 44u8, 232u8, 36u8, 206u8, 79u8,
                            165u8, 220u8, 24u8, 252u8, 25u8, 176u8, 126u8, 122u8, 39u8, 241u8,
                            172u8, 250u8, 9u8, 191u8, 142u8, 223u8, 117u8, 255u8, 182u8, 26u8,
                            235u8,
                        ]
                    {
                        let entry = ErasStakers(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub async fn eras_stakers_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakers<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ErasStakers>()?
                        == [
                            85u8, 187u8, 55u8, 106u8, 130u8, 221u8, 44u8, 232u8, 36u8, 206u8, 79u8,
                            165u8, 220u8, 24u8, 252u8, 25u8, 176u8, 126u8, 122u8, 39u8, 241u8,
                            172u8, 250u8, 9u8, 191u8, 142u8, 223u8, 117u8, 255u8, 182u8, 26u8,
                            235u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
                pub async fn eras_validator_reward(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ErasValidatorReward>()?
                        == [
                            87u8, 80u8, 156u8, 123u8, 107u8, 77u8, 203u8, 37u8, 231u8, 84u8, 124u8,
                            155u8, 227u8, 212u8, 212u8, 179u8, 84u8, 161u8, 223u8, 255u8, 254u8,
                            107u8, 52u8, 89u8, 98u8, 169u8, 136u8, 241u8, 104u8, 3u8, 244u8, 161u8,
                        ]
                    {
                        let entry = ErasValidatorReward(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
                pub async fn eras_validator_reward_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ErasValidatorReward>()?
                        == [
                            87u8, 80u8, 156u8, 123u8, 107u8, 77u8, 203u8, 37u8, 231u8, 84u8, 124u8,
                            155u8, 227u8, 212u8, 212u8, 179u8, 84u8, 161u8, 223u8, 255u8, 254u8,
                            107u8, 52u8, 89u8, 98u8, 169u8, 136u8, 241u8, 104u8, 3u8, 244u8, 161u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
                pub async fn eras_reward_points(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_octopus_lpos::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ErasRewardPoints>()?
                        == [
                            76u8, 221u8, 158u8, 62u8, 3u8, 254u8, 139u8, 170u8, 103u8, 218u8,
                            191u8, 103u8, 57u8, 212u8, 208u8, 7u8, 105u8, 52u8, 117u8, 173u8, 8u8,
                            34u8, 82u8, 141u8, 51u8, 72u8, 243u8, 56u8, 206u8, 206u8, 48u8, 140u8,
                        ]
                    {
                        let entry = ErasRewardPoints(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
                pub async fn eras_reward_points_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasRewardPoints<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ErasRewardPoints>()?
                        == [
                            76u8, 221u8, 158u8, 62u8, 3u8, 254u8, 139u8, 170u8, 103u8, 218u8,
                            191u8, 103u8, 57u8, 212u8, 208u8, 7u8, 105u8, 52u8, 117u8, 173u8, 8u8,
                            34u8, 82u8, 141u8, 51u8, 72u8, 243u8, 56u8, 206u8, 206u8, 48u8, 140u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
                pub async fn eras_total_stake(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ErasTotalStake>()?
                        == [
                            224u8, 240u8, 168u8, 69u8, 148u8, 140u8, 249u8, 240u8, 4u8, 46u8, 77u8,
                            11u8, 224u8, 65u8, 26u8, 239u8, 1u8, 110u8, 53u8, 11u8, 247u8, 235u8,
                            142u8, 234u8, 22u8, 43u8, 24u8, 36u8, 37u8, 43u8, 170u8, 40u8,
                        ]
                    {
                        let entry = ErasTotalStake(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
                pub async fn eras_total_stake_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasTotalStake<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ErasTotalStake>()?
                        == [
                            224u8, 240u8, 168u8, 69u8, 148u8, 140u8, 249u8, 240u8, 4u8, 46u8, 77u8,
                            11u8, 224u8, 65u8, 26u8, 239u8, 1u8, 110u8, 53u8, 11u8, 247u8, 235u8,
                            142u8, 234u8, 22u8, 43u8, 24u8, 36u8, 37u8, 43u8, 170u8, 40u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A mapping from still-bonded eras to the first session index of that era."]
                #[doc = ""]
                #[doc = " Must contains information for eras for the range:"]
                #[doc = " `[active_era - bounding_duration; active_era]`"]
                pub async fn bonded_eras(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<BondedEras>()?
                        == [
                            243u8, 162u8, 236u8, 198u8, 122u8, 182u8, 37u8, 55u8, 171u8, 156u8,
                            235u8, 223u8, 226u8, 129u8, 89u8, 206u8, 2u8, 155u8, 222u8, 154u8,
                            116u8, 124u8, 4u8, 119u8, 155u8, 94u8, 248u8, 30u8, 171u8, 51u8, 78u8,
                            106u8,
                        ]
                    {
                        let entry = BondedEras;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The last planned session scheduled by the session pallet."]
                #[doc = ""]
                #[doc = " This is basically in sync with the call to [`SessionManager::new_session`]."]
                pub async fn current_planned_session(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<CurrentPlannedSession>()?
                        == [
                            38u8, 22u8, 56u8, 250u8, 17u8, 154u8, 99u8, 37u8, 155u8, 253u8, 100u8,
                            117u8, 5u8, 239u8, 31u8, 190u8, 53u8, 241u8, 11u8, 185u8, 163u8, 227u8,
                            10u8, 77u8, 210u8, 64u8, 156u8, 218u8, 105u8, 16u8, 1u8, 57u8,
                        ]
                    {
                        let entry = CurrentPlannedSession;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The payout for validators and the system for the current era."]
                pub async fn era_payout(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<EraPayout>()?
                        == [
                            170u8, 205u8, 212u8, 96u8, 48u8, 137u8, 153u8, 23u8, 221u8, 85u8, 55u8,
                            48u8, 57u8, 151u8, 98u8, 86u8, 82u8, 140u8, 29u8, 186u8, 244u8, 84u8,
                            188u8, 25u8, 69u8, 183u8, 21u8, 189u8, 164u8, 101u8, 25u8, 233u8,
                        ]
                    {
                        let entry = EraPayout;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Number of sessions per era."]
                pub fn sessions_per_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusLpos", "SessionsPerEra")?
                        == [
                            73u8, 207u8, 178u8, 212u8, 159u8, 9u8, 41u8, 31u8, 205u8, 221u8, 166u8,
                            159u8, 104u8, 218u8, 113u8, 160u8, 174u8, 66u8, 95u8, 0u8, 237u8, 42u8,
                            120u8, 171u8, 68u8, 78u8, 136u8, 162u8, 163u8, 225u8, 199u8, 138u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusLpos")?;
                        let constant = pallet.constant("SessionsPerEra")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn blocks_per_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusLpos", "BlocksPerEra")?
                        == [
                            237u8, 188u8, 9u8, 53u8, 207u8, 253u8, 43u8, 105u8, 115u8, 126u8,
                            252u8, 130u8, 131u8, 190u8, 103u8, 88u8, 221u8, 34u8, 253u8, 92u8,
                            121u8, 214u8, 5u8, 229u8, 245u8, 21u8, 34u8, 114u8, 115u8, 16u8, 69u8,
                            166u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusLpos")?;
                        let constant = pallet.constant("BlocksPerEra")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Number of eras that staked funds must remain bonded for."]
                pub fn bonding_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusLpos", "BondingDuration")?
                        == [
                            117u8, 2u8, 56u8, 16u8, 159u8, 102u8, 149u8, 196u8, 2u8, 213u8, 25u8,
                            224u8, 92u8, 126u8, 224u8, 29u8, 59u8, 121u8, 215u8, 129u8, 144u8,
                            144u8, 128u8, 154u8, 133u8, 111u8, 55u8, 39u8, 28u8, 45u8, 198u8, 55u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusLpos")?;
                        let constant = pallet.constant("BondingDuration")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<MessageQueue>()?
                        == [
                            142u8, 49u8, 124u8, 4u8, 98u8, 234u8, 187u8, 185u8, 244u8, 158u8,
                            156u8, 227u8, 130u8, 54u8, 183u8, 240u8, 234u8, 170u8, 100u8, 150u8,
                            198u8, 150u8, 108u8, 164u8, 58u8, 70u8, 46u8, 59u8, 176u8, 56u8, 172u8,
                            52u8,
                        ]
                    {
                        let entry = MessageQueue;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn nonce(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Nonce>()?
                        == [
                            122u8, 169u8, 95u8, 131u8, 85u8, 32u8, 154u8, 114u8, 143u8, 56u8, 12u8,
                            182u8, 64u8, 150u8, 241u8, 249u8, 254u8, 251u8, 160u8, 235u8, 192u8,
                            41u8, 101u8, 232u8, 186u8, 108u8, 187u8, 149u8, 210u8, 91u8, 179u8,
                            98u8,
                        ]
                    {
                        let entry = Nonce;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The limit for submit messages."]
                pub fn upward_messages_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUpwardMessages", "UpwardMessagesLimit")?
                        == [
                            58u8, 44u8, 120u8, 149u8, 124u8, 229u8, 134u8, 99u8, 16u8, 140u8,
                            161u8, 12u8, 225u8, 2u8, 2u8, 38u8, 10u8, 84u8, 178u8, 88u8, 209u8,
                            83u8, 147u8, 164u8, 33u8, 119u8, 56u8, 118u8, 210u8, 182u8, 56u8, 79u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUpwardMessages")?;
                        let constant = pallet.constant("UpwardMessagesLimit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Destroy {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub witness: runtime_types::pallet_assets::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct FreezeAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for FreezeAsset {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "freeze_asset";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ThawAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ThawAsset {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "thaw_asset";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ForceClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceClearMetadata {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "force_clear_metadata";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Touch {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for Touch {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "touch";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Refund {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub allow_burn: ::core::primitive::bool,
            }
            impl ::subxt::Call for Refund {
                const PALLET: &'static str = "OctopusAssets";
                const FUNCTION: &'static str = "refund";
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
                #[doc = "Issue a new class of fungible assets from a public origin."]
                #[doc = ""]
                #[doc = "This new asset class has no assets initially and its owner is the origin."]
                #[doc = ""]
                #[doc = "The origin must be Signed and the sender must have sufficient funds free."]
                #[doc = ""]
                #[doc = "Funds of sender are reserved by `AssetDeposit`."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                #[doc = "an existing asset."]
                #[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
                #[doc = "member of the asset class's admin team."]
                #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                #[doc = ""]
                #[doc = "Emits `Created` event when successful."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn create(
                    &self,
                    id: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Create, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Create>()?
                        == [
                            22u8, 132u8, 128u8, 87u8, 22u8, 196u8, 236u8, 106u8, 26u8, 235u8, 17u8,
                            225u8, 180u8, 30u8, 18u8, 182u8, 134u8, 142u8, 242u8, 223u8, 94u8,
                            12u8, 168u8, 219u8, 62u8, 169u8, 60u8, 246u8, 83u8, 202u8, 86u8, 49u8,
                        ]
                    {
                        let call = Create {
                            id,
                            admin,
                            min_balance,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Issue a new class of fungible assets from a privileged origin."]
                #[doc = ""]
                #[doc = "This new asset class has no assets initially."]
                #[doc = ""]
                #[doc = "The origin must conform to `ForceOrigin`."]
                #[doc = ""]
                #[doc = "Unlike `create`, no funds are reserved."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                #[doc = "an existing asset."]
                #[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
                #[doc = "over this asset, but may later change and configure the permissions using"]
                #[doc = "`transfer_ownership` and `set_team`."]
                #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                #[doc = ""]
                #[doc = "Emits `ForceCreated` event when successful."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn force_create(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    is_sufficient: ::core::primitive::bool,
                    min_balance: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceCreate,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceCreate>()?
                        == [
                            255u8, 135u8, 183u8, 58u8, 93u8, 70u8, 28u8, 45u8, 83u8, 139u8, 19u8,
                            96u8, 29u8, 75u8, 27u8, 51u8, 118u8, 77u8, 33u8, 76u8, 231u8, 24u8,
                            122u8, 177u8, 12u8, 2u8, 97u8, 144u8, 38u8, 63u8, 243u8, 164u8,
                        ]
                    {
                        let call = ForceCreate {
                            id,
                            owner,
                            is_sufficient,
                            min_balance,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Destroy a class of fungible assets."]
                #[doc = ""]
                #[doc = "The origin must conform to `ForceOrigin` or must be Signed and the sender must be the"]
                #[doc = "owner of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                #[doc = "asset."]
                #[doc = ""]
                #[doc = "Emits `Destroyed` event when successful."]
                #[doc = ""]
                #[doc = "NOTE: It can be helpful to first freeze an asset before destroying it so that you"]
                #[doc = "can provide accurate witness information and prevent users from manipulating state"]
                #[doc = "in a way that can make it harder to destroy."]
                #[doc = ""]
                #[doc = "Weight: `O(c + p + a)` where:"]
                #[doc = "- `c = (witness.accounts - witness.sufficients)`"]
                #[doc = "- `s = witness.sufficients`"]
                #[doc = "- `a = witness.approvals`"]
                pub fn destroy(
                    &self,
                    id: ::core::primitive::u32,
                    witness: runtime_types::pallet_assets::types::DestroyWitness,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Destroy,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Destroy>()?
                        == [
                            75u8, 150u8, 231u8, 156u8, 201u8, 102u8, 132u8, 29u8, 65u8, 54u8,
                            131u8, 27u8, 34u8, 254u8, 226u8, 60u8, 139u8, 241u8, 76u8, 182u8,
                            174u8, 132u8, 119u8, 210u8, 13u8, 198u8, 214u8, 224u8, 26u8, 205u8,
                            132u8, 45u8,
                        ]
                    {
                        let call = Destroy { id, witness };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Mint assets of a particular class."]
                #[doc = ""]
                #[doc = "The origin must be Signed and the sender must be the Issuer of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to have some amount minted."]
                #[doc = "- `beneficiary`: The account to be credited with the minted assets."]
                #[doc = "- `amount`: The amount of the asset to be minted."]
                #[doc = ""]
                #[doc = "Emits `Issued` event when successful."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                #[doc = "Modes: Pre-existing balance of `beneficiary`; Account pre-existence of `beneficiary`."]
                pub fn mint(
                    &self,
                    id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Mint, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Mint>()?
                        == [
                            176u8, 5u8, 185u8, 90u8, 101u8, 30u8, 92u8, 236u8, 132u8, 69u8, 249u8,
                            183u8, 224u8, 224u8, 28u8, 95u8, 178u8, 107u8, 47u8, 149u8, 102u8,
                            22u8, 23u8, 86u8, 95u8, 103u8, 207u8, 88u8, 152u8, 138u8, 203u8, 1u8,
                        ]
                    {
                        let call = Mint {
                            id,
                            beneficiary,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Reduce the balance of `who` by as much as possible up to `amount` assets of `id`."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Manager of the asset `id`."]
                #[doc = ""]
                #[doc = "Bails with `NoAccount` if the `who` is already dead."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to have some amount burned."]
                #[doc = "- `who`: The account to be debited from."]
                #[doc = "- `amount`: The maximum amount by which `who`'s balance should be reduced."]
                #[doc = ""]
                #[doc = "Emits `Burned` with the actual amount burned. If this takes the balance to below the"]
                #[doc = "minimum for the asset, then the amount burned is increased to take it to zero."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                #[doc = "Modes: Post-existence of `who`; Pre & post Zombie-status of `who`."]
                pub fn burn(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Burn, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Burn>()?
                        == [
                            141u8, 248u8, 72u8, 10u8, 198u8, 66u8, 50u8, 6u8, 176u8, 102u8, 21u8,
                            118u8, 94u8, 127u8, 26u8, 112u8, 220u8, 61u8, 77u8, 217u8, 61u8, 247u8,
                            147u8, 25u8, 44u8, 217u8, 69u8, 224u8, 91u8, 232u8, 58u8, 162u8,
                        ]
                    {
                        let call = Burn { id, who, amount };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Move some assets from the sender account to another."]
                #[doc = ""]
                #[doc = "Origin must be Signed."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                #[doc = "- `target`: The account to be credited."]
                #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                #[doc = "the minimum balance. Must be greater than zero."]
                #[doc = ""]
                #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                #[doc = "to zero."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                #[doc = "`target`."]
                pub fn transfer(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Transfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Transfer>()?
                        == [
                            107u8, 247u8, 199u8, 196u8, 111u8, 11u8, 46u8, 128u8, 201u8, 89u8,
                            196u8, 206u8, 164u8, 199u8, 56u8, 111u8, 148u8, 206u8, 237u8, 27u8,
                            239u8, 185u8, 90u8, 17u8, 113u8, 252u8, 22u8, 27u8, 158u8, 47u8, 100u8,
                            213u8,
                        ]
                    {
                        let call = Transfer { id, target, amount };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Move some assets from the sender account to another, keeping the sender account alive."]
                #[doc = ""]
                #[doc = "Origin must be Signed."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                #[doc = "- `target`: The account to be credited."]
                #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                #[doc = "the minimum balance. Must be greater than zero."]
                #[doc = ""]
                #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                #[doc = "to zero."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                #[doc = "`target`."]
                pub fn transfer_keep_alive(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferKeepAlive,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferKeepAlive>()?
                        == [
                            162u8, 209u8, 34u8, 46u8, 89u8, 14u8, 90u8, 101u8, 160u8, 250u8, 105u8,
                            13u8, 15u8, 245u8, 75u8, 30u8, 145u8, 177u8, 201u8, 157u8, 57u8, 221u8,
                            106u8, 38u8, 122u8, 184u8, 98u8, 231u8, 7u8, 114u8, 177u8, 229u8,
                        ]
                    {
                        let call = TransferKeepAlive { id, target, amount };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Move some assets from one account to another."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                #[doc = "- `source`: The account to be debited."]
                #[doc = "- `dest`: The account to be credited."]
                #[doc = "- `amount`: The amount by which the `source`'s balance of assets should be reduced and"]
                #[doc = "`dest`'s balance increased. The amount actually transferred may be slightly greater in"]
                #[doc = "the case that the transfer would otherwise take the `source` balance above zero but"]
                #[doc = "below the minimum balance. Must be greater than zero."]
                #[doc = ""]
                #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                #[doc = "to zero."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                #[doc = "Modes: Pre-existence of `dest`; Post-existence of `source`; Account pre-existence of"]
                #[doc = "`dest`."]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceTransfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceTransfer>()?
                        == [
                            42u8, 166u8, 74u8, 186u8, 185u8, 246u8, 0u8, 191u8, 198u8, 218u8, 76u8,
                            238u8, 47u8, 30u8, 206u8, 214u8, 214u8, 220u8, 211u8, 197u8, 210u8,
                            99u8, 232u8, 25u8, 225u8, 68u8, 238u8, 109u8, 87u8, 136u8, 213u8,
                            104u8,
                        ]
                    {
                        let call = ForceTransfer {
                            id,
                            source,
                            dest,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Disallow further unprivileged transfers from an account."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to be frozen."]
                #[doc = "- `who`: The account to be frozen."]
                #[doc = ""]
                #[doc = "Emits `Frozen`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn freeze(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Freeze, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Freeze>()?
                        == [
                            142u8, 1u8, 72u8, 133u8, 217u8, 28u8, 148u8, 48u8, 5u8, 154u8, 11u8,
                            106u8, 202u8, 177u8, 116u8, 118u8, 152u8, 99u8, 225u8, 89u8, 43u8,
                            25u8, 198u8, 55u8, 127u8, 174u8, 27u8, 47u8, 1u8, 161u8, 248u8, 123u8,
                        ]
                    {
                        let call = Freeze { id, who };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Allow unprivileged transfers from an account again."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to be frozen."]
                #[doc = "- `who`: The account to be unfrozen."]
                #[doc = ""]
                #[doc = "Emits `Thawed`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn thaw(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Thaw, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Thaw>()?
                        == [
                            23u8, 243u8, 155u8, 178u8, 118u8, 62u8, 57u8, 188u8, 112u8, 79u8,
                            205u8, 15u8, 247u8, 11u8, 227u8, 53u8, 24u8, 238u8, 50u8, 206u8, 199u8,
                            122u8, 129u8, 186u8, 234u8, 31u8, 32u8, 191u8, 69u8, 96u8, 180u8,
                            145u8,
                        ]
                    {
                        let call = Thaw { id, who };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Disallow further unprivileged transfers for the asset class."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to be frozen."]
                #[doc = ""]
                #[doc = "Emits `Frozen`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn freeze_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        FreezeAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<FreezeAsset>()?
                        == [
                            143u8, 188u8, 2u8, 145u8, 109u8, 95u8, 170u8, 60u8, 163u8, 226u8, 99u8,
                            18u8, 106u8, 64u8, 77u8, 168u8, 131u8, 87u8, 105u8, 203u8, 145u8, 97u8,
                            237u8, 194u8, 143u8, 176u8, 65u8, 164u8, 79u8, 163u8, 11u8, 230u8,
                        ]
                    {
                        let call = FreezeAsset { id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Allow unprivileged transfers for the asset again."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to be thawed."]
                #[doc = ""]
                #[doc = "Emits `Thawed`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn thaw_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ThawAsset,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ThawAsset>()?
                        == [
                            63u8, 222u8, 197u8, 201u8, 125u8, 210u8, 148u8, 180u8, 52u8, 93u8,
                            190u8, 63u8, 195u8, 10u8, 15u8, 140u8, 154u8, 95u8, 5u8, 32u8, 36u8,
                            189u8, 85u8, 25u8, 167u8, 49u8, 11u8, 122u8, 43u8, 11u8, 255u8, 187u8,
                        ]
                    {
                        let call = ThawAsset { id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Change the Owner of an asset."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset."]
                #[doc = "- `owner`: The new Owner of this asset."]
                #[doc = ""]
                #[doc = "Emits `OwnerChanged`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn transfer_ownership(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferOwnership,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferOwnership>()?
                        == [
                            121u8, 79u8, 188u8, 97u8, 241u8, 75u8, 186u8, 246u8, 105u8, 159u8,
                            229u8, 73u8, 244u8, 97u8, 0u8, 35u8, 31u8, 242u8, 220u8, 125u8, 62u8,
                            18u8, 21u8, 72u8, 15u8, 106u8, 184u8, 30u8, 126u8, 105u8, 9u8, 106u8,
                        ]
                    {
                        let call = TransferOwnership { id, owner };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Change the Issuer, Admin and Freezer of an asset."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to be frozen."]
                #[doc = "- `issuer`: The new Issuer of this asset."]
                #[doc = "- `admin`: The new Admin of this asset."]
                #[doc = "- `freezer`: The new Freezer of this asset."]
                #[doc = ""]
                #[doc = "Emits `TeamChanged`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetTeam,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetTeam>()?
                        == [
                            158u8, 23u8, 201u8, 88u8, 18u8, 219u8, 244u8, 74u8, 106u8, 19u8, 181u8,
                            71u8, 73u8, 73u8, 203u8, 89u8, 221u8, 202u8, 234u8, 120u8, 44u8, 52u8,
                            155u8, 137u8, 4u8, 234u8, 123u8, 255u8, 234u8, 33u8, 113u8, 57u8,
                        ]
                    {
                        let call = SetTeam {
                            id,
                            issuer,
                            admin,
                            freezer,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the metadata for an asset."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                #[doc = ""]
                #[doc = "Funds of sender are reserved according to the formula:"]
                #[doc = "`MetadataDepositBase + MetadataDepositPerByte * (name.len + symbol.len)` taking into"]
                #[doc = "account any already reserved funds."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to update."]
                #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                #[doc = ""]
                #[doc = "Emits `MetadataSet`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetMetadata>()?
                        == [
                            173u8, 223u8, 227u8, 51u8, 239u8, 95u8, 222u8, 28u8, 190u8, 232u8,
                            231u8, 226u8, 149u8, 147u8, 69u8, 135u8, 183u8, 147u8, 254u8, 80u8,
                            88u8, 101u8, 215u8, 250u8, 192u8, 17u8, 113u8, 243u8, 173u8, 184u8,
                            252u8, 88u8,
                        ]
                    {
                        let call = SetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Clear the metadata for an asset."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                #[doc = ""]
                #[doc = "Any deposit is freed for the asset owner."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to clear."]
                #[doc = ""]
                #[doc = "Emits `MetadataCleared`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ClearMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ClearMetadata>()?
                        == [
                            167u8, 244u8, 159u8, 98u8, 242u8, 73u8, 109u8, 217u8, 75u8, 20u8, 34u8,
                            94u8, 21u8, 190u8, 179u8, 182u8, 156u8, 14u8, 19u8, 91u8, 36u8, 130u8,
                            88u8, 196u8, 21u8, 97u8, 180u8, 0u8, 139u8, 209u8, 136u8, 8u8,
                        ]
                    {
                        let call = ClearMetadata { id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Force the metadata for an asset to some value."]
                #[doc = ""]
                #[doc = "Origin must be ForceOrigin."]
                #[doc = ""]
                #[doc = "Any deposit is left alone."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to update."]
                #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                #[doc = ""]
                #[doc = "Emits `MetadataSet`."]
                #[doc = ""]
                #[doc = "Weight: `O(N + S)` where N and S are the length of the name and symbol respectively."]
                pub fn force_set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                    is_frozen: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceSetMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceSetMetadata>()?
                        == [
                            30u8, 254u8, 44u8, 17u8, 82u8, 41u8, 93u8, 110u8, 113u8, 6u8, 75u8,
                            27u8, 7u8, 69u8, 221u8, 148u8, 47u8, 106u8, 111u8, 144u8, 18u8, 79u8,
                            0u8, 17u8, 137u8, 229u8, 175u8, 183u8, 7u8, 137u8, 148u8, 135u8,
                        ]
                    {
                        let call = ForceSetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                            is_frozen,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Clear the metadata for an asset."]
                #[doc = ""]
                #[doc = "Origin must be ForceOrigin."]
                #[doc = ""]
                #[doc = "Any deposit is returned."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset to clear."]
                #[doc = ""]
                #[doc = "Emits `MetadataCleared`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn force_clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceClearMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceClearMetadata>()?
                        == [
                            31u8, 209u8, 69u8, 120u8, 235u8, 248u8, 172u8, 247u8, 79u8, 199u8,
                            186u8, 52u8, 254u8, 240u8, 76u8, 59u8, 74u8, 6u8, 136u8, 142u8, 240u8,
                            200u8, 235u8, 55u8, 96u8, 78u8, 33u8, 232u8, 162u8, 57u8, 215u8, 26u8,
                        ]
                    {
                        let call = ForceClearMetadata { id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Alter the attributes of a given asset."]
                #[doc = ""]
                #[doc = "Origin must be `ForceOrigin`."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset."]
                #[doc = "- `owner`: The new Owner of this asset."]
                #[doc = "- `issuer`: The new Issuer of this asset."]
                #[doc = "- `admin`: The new Admin of this asset."]
                #[doc = "- `freezer`: The new Freezer of this asset."]
                #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                #[doc = "- `is_sufficient`: Whether a non-zero balance of this asset is deposit of sufficient"]
                #[doc = "value to account for the state bloat associated with its balance storage. If set to"]
                #[doc = "`true`, then non-zero balances may be stored without a `consumer` reference (and thus"]
                #[doc = "an ED in the Balances pallet or whatever else is used to control user-account state"]
                #[doc = "growth)."]
                #[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
                #[doc = "instructions."]
                #[doc = ""]
                #[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceAssetStatus,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceAssetStatus>()?
                        == [
                            15u8, 174u8, 37u8, 4u8, 197u8, 25u8, 16u8, 52u8, 7u8, 59u8, 34u8, 91u8,
                            158u8, 87u8, 237u8, 130u8, 75u8, 87u8, 103u8, 137u8, 130u8, 212u8,
                            182u8, 15u8, 52u8, 235u8, 162u8, 3u8, 202u8, 67u8, 190u8, 203u8,
                        ]
                    {
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
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Approve an amount of asset for transfer by a delegated third-party account."]
                #[doc = ""]
                #[doc = "Origin must be Signed."]
                #[doc = ""]
                #[doc = "Ensures that `ApprovalDeposit` worth of `Currency` is reserved from signing account"]
                #[doc = "for the purpose of holding the approval. If some non-zero amount of assets is already"]
                #[doc = "approved from signing account to `delegate`, then it is topped up or unreserved to"]
                #[doc = "meet the right value."]
                #[doc = ""]
                #[doc = "NOTE: The signing account does not need to own `amount` of assets at the point of"]
                #[doc = "making this call."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset."]
                #[doc = "- `delegate`: The account to delegate permission to transfer asset."]
                #[doc = "- `amount`: The amount of asset that may be transferred by `delegate`. If there is"]
                #[doc = "already an approval in place, then this acts additively."]
                #[doc = ""]
                #[doc = "Emits `ApprovedTransfer` on success."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn approve_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ApproveTransfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ApproveTransfer>()?
                        == [
                            142u8, 115u8, 147u8, 169u8, 36u8, 87u8, 10u8, 209u8, 67u8, 225u8, 1u8,
                            27u8, 156u8, 60u8, 215u8, 104u8, 35u8, 43u8, 181u8, 215u8, 3u8, 88u8,
                            218u8, 54u8, 2u8, 209u8, 192u8, 214u8, 245u8, 68u8, 90u8, 177u8,
                        ]
                    {
                        let call = ApproveTransfer {
                            id,
                            delegate,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                #[doc = ""]
                #[doc = "Origin must be Signed and there must be an approval in place between signer and"]
                #[doc = "`delegate`."]
                #[doc = ""]
                #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset."]
                #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                #[doc = ""]
                #[doc = "Emits `ApprovalCancelled` on success."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        CancelApproval,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<CancelApproval>()?
                        == [
                            64u8, 68u8, 198u8, 159u8, 170u8, 36u8, 215u8, 175u8, 51u8, 52u8, 211u8,
                            178u8, 187u8, 60u8, 97u8, 126u8, 104u8, 87u8, 145u8, 128u8, 124u8,
                            112u8, 200u8, 129u8, 18u8, 218u8, 32u8, 8u8, 188u8, 7u8, 91u8, 141u8,
                        ]
                    {
                        let call = CancelApproval { id, delegate };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                #[doc = ""]
                #[doc = "Origin must be either ForceOrigin or Signed origin with the signer being the Admin"]
                #[doc = "account of the asset `id`."]
                #[doc = ""]
                #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset."]
                #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                #[doc = ""]
                #[doc = "Emits `ApprovalCancelled` on success."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceCancelApproval,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceCancelApproval>()?
                        == [
                            55u8, 3u8, 145u8, 168u8, 228u8, 58u8, 62u8, 53u8, 79u8, 128u8, 191u8,
                            87u8, 218u8, 150u8, 165u8, 123u8, 52u8, 201u8, 168u8, 81u8, 188u8,
                            170u8, 190u8, 134u8, 208u8, 169u8, 45u8, 35u8, 254u8, 86u8, 72u8,
                            164u8,
                        ]
                    {
                        let call = ForceCancelApproval {
                            id,
                            owner,
                            delegate,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Transfer some asset balance from a previously delegated account to some third-party"]
                #[doc = "account."]
                #[doc = ""]
                #[doc = "Origin must be Signed and there must be an approval in place by the `owner` to the"]
                #[doc = "signer."]
                #[doc = ""]
                #[doc = "If the entire amount approved for transfer is transferred, then any deposit previously"]
                #[doc = "reserved by `approve_transfer` is unreserved."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset."]
                #[doc = "- `owner`: The account which previously approved for a transfer of at least `amount` and"]
                #[doc = "from which the asset balance will be withdrawn."]
                #[doc = "- `destination`: The account to which the asset balance of `amount` will be transferred."]
                #[doc = "- `amount`: The amount of assets to transfer."]
                #[doc = ""]
                #[doc = "Emits `TransferredApproved` on success."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferApproved,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferApproved>()?
                        == [
                            121u8, 143u8, 106u8, 161u8, 29u8, 103u8, 166u8, 228u8, 7u8, 23u8,
                            227u8, 159u8, 243u8, 74u8, 47u8, 115u8, 111u8, 53u8, 26u8, 182u8,
                            221u8, 251u8, 39u8, 75u8, 241u8, 5u8, 18u8, 65u8, 107u8, 252u8, 18u8,
                            187u8,
                        ]
                    {
                        let call = TransferApproved {
                            id,
                            owner,
                            destination,
                            amount,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Create an asset account for non-provider assets."]
                #[doc = ""]
                #[doc = "A deposit will be taken from the signer account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be Signed; the signer account must have sufficient funds for a deposit"]
                #[doc = "  to be taken."]
                #[doc = "- `id`: The identifier of the asset for the account to be created."]
                #[doc = ""]
                #[doc = "Emits `Touched` event when successful."]
                pub fn touch(
                    &self,
                    id: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Touch, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Touch>()?
                        == [
                            148u8, 89u8, 169u8, 34u8, 198u8, 67u8, 136u8, 9u8, 219u8, 214u8, 164u8,
                            144u8, 195u8, 48u8, 75u8, 218u8, 231u8, 109u8, 253u8, 88u8, 23u8,
                            159u8, 184u8, 54u8, 249u8, 68u8, 78u8, 91u8, 45u8, 168u8, 194u8, 165u8,
                        ]
                    {
                        let call = Touch { id };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Return the deposit (if any) of an asset account."]
                #[doc = ""]
                #[doc = "The origin must be Signed."]
                #[doc = ""]
                #[doc = "- `id`: The identifier of the asset for the account to be created."]
                #[doc = "- `allow_burn`: If `true` then assets may be destroyed in order to complete the refund."]
                #[doc = ""]
                #[doc = "Emits `Refunded` event when successful."]
                pub fn refund(
                    &self,
                    id: ::core::primitive::u32,
                    allow_burn: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Refund, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Refund>()?
                        == [
                            72u8, 197u8, 177u8, 250u8, 20u8, 37u8, 39u8, 132u8, 62u8, 24u8, 167u8,
                            155u8, 79u8, 88u8, 158u8, 209u8, 112u8, 40u8, 41u8, 83u8, 224u8, 224u8,
                            71u8, 115u8, 29u8, 84u8, 52u8, 49u8, 199u8, 78u8, 97u8, 230u8,
                        ]
                    {
                        let call = Refund { id, allow_burn };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some asset class was created."]
            pub struct Created {
                pub asset_id: ::core::primitive::u32,
                pub creator: ::subxt::sp_core::crypto::AccountId32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some assets were issued."]
            pub struct Issued {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub total_supply: ::core::primitive::u128,
            }
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some assets were transferred."]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some assets were destroyed."]
            pub struct Burned {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The management team changed."]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The owner changed."]
            pub struct OwnerChanged {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some account `who` was frozen."]
            pub struct Frozen {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some account `who` was thawed."]
            pub struct Thawed {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `asset_id` was frozen."]
            pub struct AssetFrozen {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetFrozen {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `asset_id` was thawed."]
            pub struct AssetThawed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetThawed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset class was destroyed."]
            pub struct Destroyed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some asset class was force-created."]
            pub struct ForceCreated {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "New metadata has been set for an asset."]
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
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Metadata has been cleared for an asset."]
            pub struct MetadataCleared {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "(Additional) funds have been approved for transfer to a destination account."]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An approval for account `delegate` was cancelled by `owner`."]
            pub struct ApprovalCancelled {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An `amount` was transferred in its entirety from `owner` to `destination` by"]
            #[doc = "the approved `delegate`."]
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
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset has had its attributes changed by the `Force` origin."]
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
                type Value = runtime_types::pallet_assets::types::AssetAccount<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                    (),
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
                #[doc = " Details of an asset."]
                pub async fn asset(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
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
                    if self.client.metadata().storage_hash::<Asset>()?
                        == [
                            171u8, 171u8, 67u8, 144u8, 214u8, 143u8, 54u8, 233u8, 197u8, 111u8,
                            154u8, 139u8, 7u8, 231u8, 194u8, 246u8, 190u8, 109u8, 168u8, 154u8,
                            247u8, 28u8, 227u8, 53u8, 108u8, 48u8, 4u8, 89u8, 119u8, 8u8, 79u8,
                            76u8,
                        ]
                    {
                        let entry = Asset(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Details of an asset."]
                pub async fn asset_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Asset>()?
                        == [
                            171u8, 171u8, 67u8, 144u8, 214u8, 143u8, 54u8, 233u8, 197u8, 111u8,
                            154u8, 139u8, 7u8, 231u8, 194u8, 246u8, 190u8, 109u8, 168u8, 154u8,
                            247u8, 28u8, 227u8, 53u8, 108u8, 48u8, 4u8, 89u8, 119u8, 8u8, 79u8,
                            76u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The holdings of a specific account for a specific asset."]
                pub async fn account(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::AssetAccount<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                            (),
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            189u8, 8u8, 93u8, 193u8, 123u8, 139u8, 0u8, 132u8, 208u8, 132u8, 162u8,
                            170u8, 142u8, 0u8, 81u8, 72u8, 143u8, 234u8, 207u8, 2u8, 140u8, 112u8,
                            135u8, 25u8, 79u8, 202u8, 25u8, 87u8, 147u8, 169u8, 0u8, 213u8,
                        ]
                    {
                        let entry = Account(_0, _1);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The holdings of a specific account for a specific asset."]
                pub async fn account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            189u8, 8u8, 93u8, 193u8, 123u8, 139u8, 0u8, 132u8, 208u8, 132u8, 162u8,
                            170u8, 142u8, 0u8, 81u8, 72u8, 143u8, 234u8, 207u8, 2u8, 140u8, 112u8,
                            135u8, 25u8, 79u8, 202u8, 25u8, 87u8, 147u8, 169u8, 0u8, 213u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Approved balance transfers. First balance is the amount approved for transfer. Second"]
                #[doc = " is the amount of `T::Currency` reserved for storing this."]
                #[doc = " First key is the asset ID, second key is the owner and third key is the delegate."]
                pub async fn approvals(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    _2: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Approvals>()?
                        == [
                            237u8, 98u8, 160u8, 91u8, 202u8, 124u8, 226u8, 49u8, 184u8, 96u8, 90u8,
                            61u8, 79u8, 66u8, 54u8, 237u8, 156u8, 232u8, 140u8, 67u8, 81u8, 55u8,
                            89u8, 132u8, 68u8, 125u8, 47u8, 254u8, 11u8, 104u8, 207u8, 96u8,
                        ]
                    {
                        let entry = Approvals(_0, _1, _2);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Approved balance transfers. First balance is the amount approved for transfer. Second"]
                #[doc = " is the amount of `T::Currency` reserved for storing this."]
                #[doc = " First key is the asset ID, second key is the owner and third key is the delegate."]
                pub async fn approvals_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Approvals<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Approvals>()?
                        == [
                            237u8, 98u8, 160u8, 91u8, 202u8, 124u8, 226u8, 49u8, 184u8, 96u8, 90u8,
                            61u8, 79u8, 66u8, 54u8, 237u8, 156u8, 232u8, 140u8, 67u8, 81u8, 55u8,
                            89u8, 132u8, 68u8, 125u8, 47u8, 254u8, 11u8, 104u8, 207u8, 96u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset."]
                pub async fn metadata(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Metadata>()?
                        == [
                            209u8, 23u8, 7u8, 106u8, 173u8, 10u8, 115u8, 39u8, 138u8, 26u8, 54u8,
                            230u8, 183u8, 233u8, 249u8, 85u8, 76u8, 84u8, 184u8, 54u8, 114u8,
                            213u8, 38u8, 238u8, 109u8, 30u8, 0u8, 246u8, 123u8, 200u8, 40u8, 34u8,
                        ]
                    {
                        let entry = Metadata(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset."]
                pub async fn metadata_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Metadata<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Metadata>()?
                        == [
                            209u8, 23u8, 7u8, 106u8, 173u8, 10u8, 115u8, 39u8, 138u8, 26u8, 54u8,
                            230u8, 183u8, 233u8, 249u8, 85u8, 76u8, 84u8, 184u8, 54u8, 114u8,
                            213u8, 38u8, 238u8, 109u8, 30u8, 0u8, 246u8, 123u8, 200u8, 40u8, 34u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The basic amount of funds that must be reserved for an asset."]
                pub fn asset_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAssets", "AssetDeposit")?
                        == [
                            120u8, 251u8, 49u8, 189u8, 118u8, 126u8, 35u8, 7u8, 192u8, 36u8, 209u8,
                            166u8, 76u8, 186u8, 19u8, 231u8, 236u8, 176u8, 226u8, 77u8, 214u8,
                            84u8, 210u8, 31u8, 67u8, 246u8, 12u8, 191u8, 209u8, 37u8, 166u8, 49u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAssets")?;
                        let constant = pallet.constant("AssetDeposit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The amount of funds that must be reserved for a non-provider asset account to be"]
                #[doc = " maintained."]
                pub fn asset_account_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAssets", "AssetAccountDeposit")?
                        == [
                            79u8, 195u8, 123u8, 184u8, 64u8, 11u8, 96u8, 8u8, 191u8, 95u8, 213u8,
                            157u8, 114u8, 36u8, 9u8, 111u8, 224u8, 97u8, 190u8, 223u8, 63u8, 168u8,
                            198u8, 108u8, 145u8, 100u8, 74u8, 86u8, 2u8, 8u8, 189u8, 192u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAssets")?;
                        let constant = pallet.constant("AssetAccountDeposit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The basic amount of funds that must be reserved when adding metadata to your asset."]
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAssets", "MetadataDepositBase")?
                        == [
                            206u8, 201u8, 26u8, 198u8, 159u8, 174u8, 212u8, 81u8, 73u8, 223u8,
                            96u8, 44u8, 229u8, 140u8, 251u8, 240u8, 199u8, 30u8, 40u8, 52u8, 229u8,
                            249u8, 104u8, 111u8, 138u8, 45u8, 97u8, 78u8, 131u8, 205u8, 119u8,
                            151u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAssets")?;
                        let constant = pallet.constant("MetadataDepositBase")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The additional funds that must be reserved for the number of bytes you store in your"]
                #[doc = " metadata."]
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAssets", "MetadataDepositPerByte")?
                        == [
                            151u8, 19u8, 126u8, 167u8, 151u8, 217u8, 225u8, 213u8, 132u8, 62u8,
                            163u8, 83u8, 72u8, 11u8, 37u8, 124u8, 0u8, 55u8, 119u8, 47u8, 9u8,
                            106u8, 184u8, 68u8, 74u8, 62u8, 87u8, 57u8, 5u8, 91u8, 85u8, 255u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAssets")?;
                        let constant = pallet.constant("MetadataDepositPerByte")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The amount of funds that must be reserved when creating a new approval."]
                pub fn approval_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAssets", "ApprovalDeposit")?
                        == [
                            203u8, 2u8, 76u8, 21u8, 80u8, 235u8, 119u8, 243u8, 16u8, 127u8, 215u8,
                            34u8, 196u8, 158u8, 88u8, 86u8, 223u8, 34u8, 253u8, 1u8, 125u8, 249u8,
                            157u8, 188u8, 94u8, 237u8, 100u8, 0u8, 103u8, 145u8, 166u8, 124u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAssets")?;
                        let constant = pallet.constant("ApprovalDeposit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum length of a name or symbol stored on-chain."]
                pub fn string_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusAssets", "StringLimit")?
                        == [
                            64u8, 204u8, 1u8, 240u8, 39u8, 64u8, 238u8, 105u8, 253u8, 194u8, 239u8,
                            211u8, 82u8, 163u8, 118u8, 129u8, 51u8, 252u8, 113u8, 145u8, 46u8,
                            73u8, 211u8, 136u8, 160u8, 117u8, 90u8, 181u8, 166u8, 203u8, 244u8,
                            185u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusAssets")?;
                        let constant = pallet.constant("StringLimit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod octopus_uniques {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Create {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ForceCreate {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub free_holding: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceCreate {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Destroy {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub witness: runtime_types::pallet_uniques::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Mint {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Mint {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Burn {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub check_owner: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for Burn {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Transfer {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Redeposit {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub instances: ::std::vec::Vec<::core::primitive::u128>,
            }
            impl ::subxt::Call for Redeposit {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "redeposit";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Freeze {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Thaw {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct FreezeClass {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Call for FreezeClass {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "freeze_class";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ThawClass {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Call for ThawClass {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "thaw_class";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for TransferOwnership {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetTeam {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetTeam {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for ApproveTransfer {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct CancelApproval {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub maybe_check_delegate: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for CancelApproval {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
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
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetAttribute {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub maybe_instance: ::core::option::Option<::core::primitive::u128>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for SetAttribute {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "set_attribute";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ClearAttribute {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub maybe_instance: ::core::option::Option<::core::primitive::u128>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for ClearAttribute {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "clear_attribute";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for SetClassMetadata {
                const PALLET: &'static str = "OctopusUniques";
                const FUNCTION: &'static str = "set_class_metadata";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ClearClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Call for ClearClassMetadata {
                const PALLET: &'static str = "OctopusUniques";
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
                #[doc = "Issue a new class of non-fungible assets from a public origin."]
                #[doc = ""]
                #[doc = "This new asset class has no assets initially and its owner is the origin."]
                #[doc = ""]
                #[doc = "The origin must be Signed and the sender must have sufficient funds free."]
                #[doc = ""]
                #[doc = "`AssetDeposit` funds of sender are reserved."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `class`: The identifier of the new asset class. This must not be currently in use."]
                #[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
                #[doc = "member of the asset class's admin team."]
                #[doc = ""]
                #[doc = "Emits `Created` event when successful."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn create(
                    &self,
                    class: ::core::primitive::u128,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Create, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Create>()?
                        == [
                            247u8, 34u8, 3u8, 210u8, 20u8, 107u8, 91u8, 114u8, 239u8, 4u8, 143u8,
                            223u8, 245u8, 170u8, 62u8, 161u8, 1u8, 151u8, 48u8, 162u8, 112u8,
                            117u8, 25u8, 103u8, 14u8, 143u8, 38u8, 188u8, 60u8, 14u8, 61u8, 212u8,
                        ]
                    {
                        let call = Create { class, admin };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Issue a new class of non-fungible assets from a privileged origin."]
                #[doc = ""]
                #[doc = "This new asset class has no assets initially."]
                #[doc = ""]
                #[doc = "The origin must conform to `ForceOrigin`."]
                #[doc = ""]
                #[doc = "Unlike `create`, no funds are reserved."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the new asset. This must not be currently in use."]
                #[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
                #[doc = "over this asset, but may later change and configure the permissions using"]
                #[doc = "`transfer_ownership` and `set_team`."]
                #[doc = ""]
                #[doc = "Emits `ForceCreated` event when successful."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn force_create(
                    &self,
                    class: ::core::primitive::u128,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    free_holding: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceCreate,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceCreate>()?
                        == [
                            188u8, 185u8, 9u8, 122u8, 215u8, 22u8, 153u8, 205u8, 169u8, 112u8,
                            68u8, 55u8, 220u8, 115u8, 54u8, 17u8, 31u8, 211u8, 2u8, 102u8, 61u8,
                            189u8, 160u8, 241u8, 106u8, 203u8, 172u8, 68u8, 171u8, 172u8, 37u8,
                            198u8,
                        ]
                    {
                        let call = ForceCreate {
                            class,
                            owner,
                            free_holding,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Destroy a class of fungible assets."]
                #[doc = ""]
                #[doc = "The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the"]
                #[doc = "owner of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset class to be destroyed."]
                #[doc = "- `witness`: Information on the instances minted in the asset class. This must be"]
                #[doc = "correct."]
                #[doc = ""]
                #[doc = "Emits `Destroyed` event when successful."]
                #[doc = ""]
                #[doc = "Weight: `O(n + m)` where:"]
                #[doc = "- `n = witness.instances`"]
                #[doc = "- `m = witness.instance_metadatas`"]
                #[doc = "- `a = witness.attributes`"]
                pub fn destroy(
                    &self,
                    class: ::core::primitive::u128,
                    witness: runtime_types::pallet_uniques::types::DestroyWitness,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Destroy,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Destroy>()?
                        == [
                            29u8, 224u8, 27u8, 160u8, 68u8, 160u8, 3u8, 212u8, 177u8, 195u8, 133u8,
                            6u8, 222u8, 190u8, 119u8, 154u8, 193u8, 245u8, 21u8, 115u8, 88u8, 81u8,
                            250u8, 187u8, 103u8, 31u8, 41u8, 168u8, 17u8, 43u8, 47u8, 32u8,
                        ]
                    {
                        let call = Destroy { class, witness };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Mint an asset instance of a particular class."]
                #[doc = ""]
                #[doc = "The origin must be Signed and the sender must be the Issuer of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The class of the asset to be minted."]
                #[doc = "- `instance`: The instance value of the asset to be minted."]
                #[doc = "- `beneficiary`: The initial owner of the minted asset."]
                #[doc = ""]
                #[doc = "Emits `Issued` event when successful."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn mint(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Mint, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Mint>()?
                        == [
                            27u8, 85u8, 153u8, 112u8, 19u8, 35u8, 166u8, 253u8, 85u8, 67u8, 1u8,
                            160u8, 60u8, 238u8, 137u8, 63u8, 191u8, 79u8, 196u8, 253u8, 6u8, 216u8,
                            252u8, 41u8, 34u8, 192u8, 159u8, 182u8, 127u8, 45u8, 151u8, 249u8,
                        ]
                    {
                        let call = Mint {
                            class,
                            instance,
                            owner,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Destroy a single asset instance."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Admin of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The class of the asset to be burned."]
                #[doc = "- `instance`: The instance of the asset to be burned."]
                #[doc = "- `check_owner`: If `Some` then the operation will fail with `WrongOwner` unless the"]
                #[doc = "  asset is owned by this value."]
                #[doc = ""]
                #[doc = "Emits `Burned` with the actual amount burned."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                #[doc = "Modes: `check_owner.is_some()`."]
                pub fn burn(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    check_owner: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Burn, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Burn>()?
                        == [
                            20u8, 11u8, 245u8, 20u8, 69u8, 167u8, 67u8, 133u8, 228u8, 231u8, 161u8,
                            116u8, 31u8, 205u8, 199u8, 0u8, 79u8, 105u8, 73u8, 166u8, 66u8, 18u8,
                            135u8, 171u8, 212u8, 148u8, 55u8, 158u8, 254u8, 120u8, 238u8, 94u8,
                        ]
                    {
                        let call = Burn {
                            class,
                            instance,
                            check_owner,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Move an asset from the sender account to another."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the signing account must be either:"]
                #[doc = "- the Admin of the asset `class`;"]
                #[doc = "- the Owner of the asset `instance`;"]
                #[doc = "- the approved delegate for the asset `instance` (in this case, the approval is reset)."]
                #[doc = ""]
                #[doc = "Arguments:"]
                #[doc = "- `class`: The class of the asset to be transferred."]
                #[doc = "- `instance`: The instance of the asset to be transferred."]
                #[doc = "- `dest`: The account to receive ownership of the asset."]
                #[doc = ""]
                #[doc = "Emits `Transferred`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn transfer(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Transfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Transfer>()?
                        == [
                            222u8, 245u8, 117u8, 163u8, 14u8, 194u8, 121u8, 164u8, 151u8, 36u8,
                            207u8, 101u8, 226u8, 131u8, 153u8, 119u8, 149u8, 39u8, 163u8, 9u8,
                            48u8, 20u8, 219u8, 37u8, 110u8, 213u8, 203u8, 201u8, 4u8, 94u8, 194u8,
                            31u8,
                        ]
                    {
                        let call = Transfer {
                            class,
                            instance,
                            dest,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Reevaluate the deposits on some assets."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Owner of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The class of the asset to be frozen."]
                #[doc = "- `instances`: The instances of the asset class whose deposits will be reevaluated."]
                #[doc = ""]
                #[doc = "NOTE: This exists as a best-effort function. Any asset instances which are unknown or"]
                #[doc = "in the case that the owner account does not have reservable funds to pay for a"]
                #[doc = "deposit increase are ignored. Generally the owner isn't going to call this on instances"]
                #[doc = "whose existing deposit is less than the refreshed deposit as it would only cost them,"]
                #[doc = "so it's of little consequence."]
                #[doc = ""]
                #[doc = "It will still return an error in the case that the class is unknown of the signer is"]
                #[doc = "not permitted to call it."]
                #[doc = ""]
                #[doc = "Weight: `O(instances.len())`"]
                pub fn redeposit(
                    &self,
                    class: ::core::primitive::u128,
                    instances: ::std::vec::Vec<::core::primitive::u128>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Redeposit,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Redeposit>()?
                        == [
                            26u8, 183u8, 73u8, 30u8, 195u8, 113u8, 102u8, 64u8, 244u8, 181u8, 17u8,
                            132u8, 160u8, 167u8, 168u8, 32u8, 0u8, 91u8, 184u8, 232u8, 48u8, 122u8,
                            13u8, 3u8, 216u8, 96u8, 29u8, 15u8, 228u8, 53u8, 242u8, 133u8,
                        ]
                    {
                        let call = Redeposit { class, instances };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Disallow further unprivileged transfer of an asset instance."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The class of the asset to be frozen."]
                #[doc = "- `instance`: The instance of the asset to be frozen."]
                #[doc = ""]
                #[doc = "Emits `Frozen`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn freeze(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Freeze, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Freeze>()?
                        == [
                            63u8, 169u8, 254u8, 159u8, 243u8, 63u8, 201u8, 226u8, 38u8, 159u8,
                            20u8, 200u8, 60u8, 19u8, 0u8, 55u8, 31u8, 79u8, 74u8, 226u8, 206u8,
                            142u8, 143u8, 27u8, 11u8, 183u8, 199u8, 129u8, 196u8, 232u8, 178u8,
                            132u8,
                        ]
                    {
                        let call = Freeze { class, instance };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Re-allow unprivileged transfer of an asset instance."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The class of the asset to be thawed."]
                #[doc = "- `instance`: The instance of the asset to be thawed."]
                #[doc = ""]
                #[doc = "Emits `Thawed`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn thaw(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Thaw, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Thaw>()?
                        == [
                            59u8, 190u8, 79u8, 193u8, 89u8, 47u8, 37u8, 194u8, 15u8, 241u8, 44u8,
                            105u8, 9u8, 252u8, 110u8, 28u8, 13u8, 40u8, 216u8, 28u8, 30u8, 6u8,
                            114u8, 179u8, 12u8, 4u8, 32u8, 60u8, 216u8, 235u8, 251u8, 47u8,
                        ]
                    {
                        let call = Thaw { class, instance };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Disallow further unprivileged transfers for a whole asset class."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The asset class to be frozen."]
                #[doc = ""]
                #[doc = "Emits `ClassFrozen`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn freeze_class(
                    &self,
                    class: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        FreezeClass,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<FreezeClass>()?
                        == [
                            51u8, 100u8, 25u8, 136u8, 7u8, 148u8, 50u8, 157u8, 129u8, 132u8, 72u8,
                            91u8, 70u8, 62u8, 151u8, 254u8, 125u8, 64u8, 159u8, 230u8, 218u8, 84u8,
                            71u8, 129u8, 162u8, 167u8, 229u8, 70u8, 139u8, 88u8, 60u8, 21u8,
                        ]
                    {
                        let call = FreezeClass { class };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Re-allow unprivileged transfers for a whole asset class."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Admin of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The class to be thawed."]
                #[doc = ""]
                #[doc = "Emits `ClassThawed`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn thaw_class(
                    &self,
                    class: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ThawClass,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ThawClass>()?
                        == [
                            90u8, 34u8, 138u8, 61u8, 209u8, 49u8, 101u8, 71u8, 81u8, 40u8, 14u8,
                            216u8, 166u8, 227u8, 31u8, 106u8, 220u8, 5u8, 101u8, 143u8, 233u8,
                            97u8, 185u8, 50u8, 135u8, 220u8, 205u8, 193u8, 145u8, 84u8, 41u8,
                            197u8,
                        ]
                    {
                        let call = ThawClass { class };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Change the Owner of an asset class."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Owner of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The asset class whose owner should be changed."]
                #[doc = "- `owner`: The new Owner of this asset class."]
                #[doc = ""]
                #[doc = "Emits `OwnerChanged`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn transfer_ownership(
                    &self,
                    class: ::core::primitive::u128,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferOwnership,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferOwnership>()?
                        == [
                            241u8, 200u8, 130u8, 24u8, 232u8, 13u8, 91u8, 126u8, 176u8, 36u8, 71u8,
                            127u8, 101u8, 91u8, 146u8, 205u8, 252u8, 235u8, 72u8, 207u8, 127u8,
                            30u8, 226u8, 178u8, 113u8, 83u8, 163u8, 119u8, 12u8, 110u8, 236u8,
                            203u8,
                        ]
                    {
                        let call = TransferOwnership { class, owner };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Change the Issuer, Admin and Freezer of an asset class."]
                #[doc = ""]
                #[doc = "Origin must be Signed and the sender should be the Owner of the asset `class`."]
                #[doc = ""]
                #[doc = "- `class`: The asset class whose team should be changed."]
                #[doc = "- `issuer`: The new Issuer of this asset class."]
                #[doc = "- `admin`: The new Admin of this asset class."]
                #[doc = "- `freezer`: The new Freezer of this asset class."]
                #[doc = ""]
                #[doc = "Emits `TeamChanged`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn set_team(
                    &self,
                    class: ::core::primitive::u128,
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetTeam,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetTeam>()?
                        == [
                            101u8, 215u8, 217u8, 42u8, 173u8, 23u8, 17u8, 168u8, 236u8, 126u8,
                            185u8, 1u8, 220u8, 116u8, 90u8, 16u8, 125u8, 106u8, 205u8, 253u8,
                            154u8, 58u8, 132u8, 13u8, 253u8, 228u8, 45u8, 238u8, 189u8, 252u8,
                            235u8, 37u8,
                        ]
                    {
                        let call = SetTeam {
                            class,
                            issuer,
                            admin,
                            freezer,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Approve an instance to be transferred by a delegated third-party account."]
                #[doc = ""]
                #[doc = "Origin must be Signed and must be the owner of the asset `instance`."]
                #[doc = ""]
                #[doc = "- `class`: The class of the asset to be approved for delegated transfer."]
                #[doc = "- `instance`: The instance of the asset to be approved for delegated transfer."]
                #[doc = "- `delegate`: The account to delegate permission to transfer the asset."]
                #[doc = ""]
                #[doc = "Emits `ApprovedTransfer` on success."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn approve_transfer(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ApproveTransfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ApproveTransfer>()?
                        == [
                            51u8, 134u8, 191u8, 188u8, 145u8, 203u8, 198u8, 27u8, 250u8, 82u8,
                            96u8, 46u8, 137u8, 19u8, 200u8, 63u8, 13u8, 49u8, 32u8, 50u8, 130u8,
                            206u8, 29u8, 158u8, 143u8, 82u8, 63u8, 88u8, 91u8, 210u8, 231u8, 119u8,
                        ]
                    {
                        let call = ApproveTransfer {
                            class,
                            instance,
                            delegate,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Cancel the prior approval for the transfer of an asset by a delegate."]
                #[doc = ""]
                #[doc = "Origin must be either:"]
                #[doc = "- the `Force` origin;"]
                #[doc = "- `Signed` with the signer being the Admin of the asset `class`;"]
                #[doc = "- `Signed` with the signer being the Owner of the asset `instance`;"]
                #[doc = ""]
                #[doc = "Arguments:"]
                #[doc = "- `class`: The class of the asset of whose approval will be cancelled."]
                #[doc = "- `instance`: The instance of the asset of whose approval will be cancelled."]
                #[doc = "- `maybe_check_delegate`: If `Some` will ensure that the given account is the one to"]
                #[doc = "  which permission of transfer is delegated."]
                #[doc = ""]
                #[doc = "Emits `ApprovalCancelled` on success."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn cancel_approval(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    maybe_check_delegate: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        CancelApproval,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<CancelApproval>()?
                        == [
                            0u8, 150u8, 8u8, 91u8, 17u8, 97u8, 46u8, 109u8, 133u8, 29u8, 133u8,
                            97u8, 216u8, 93u8, 54u8, 150u8, 125u8, 117u8, 93u8, 102u8, 226u8,
                            163u8, 62u8, 205u8, 178u8, 115u8, 165u8, 209u8, 88u8, 87u8, 45u8,
                            118u8,
                        ]
                    {
                        let call = CancelApproval {
                            class,
                            instance,
                            maybe_check_delegate,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Alter the attributes of a given asset."]
                #[doc = ""]
                #[doc = "Origin must be `ForceOrigin`."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset."]
                #[doc = "- `owner`: The new Owner of this asset."]
                #[doc = "- `issuer`: The new Issuer of this asset."]
                #[doc = "- `admin`: The new Admin of this asset."]
                #[doc = "- `freezer`: The new Freezer of this asset."]
                #[doc = "- `free_holding`: Whether a deposit is taken for holding an instance of this asset"]
                #[doc = "  class."]
                #[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
                #[doc = "instructions."]
                #[doc = ""]
                #[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn force_asset_status(
                    &self,
                    class: ::core::primitive::u128,
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
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceAssetStatus,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceAssetStatus>()?
                        == [
                            223u8, 7u8, 2u8, 208u8, 61u8, 152u8, 114u8, 203u8, 169u8, 221u8, 82u8,
                            7u8, 202u8, 171u8, 132u8, 169u8, 81u8, 107u8, 6u8, 183u8, 49u8, 206u8,
                            148u8, 253u8, 3u8, 236u8, 49u8, 72u8, 10u8, 28u8, 112u8, 247u8,
                        ]
                    {
                        let call = ForceAssetStatus {
                            class,
                            owner,
                            issuer,
                            admin,
                            freezer,
                            free_holding,
                            is_frozen,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set an attribute for an asset class or instance."]
                #[doc = ""]
                #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                #[doc = "asset `class`."]
                #[doc = ""]
                #[doc = "If the origin is Signed, then funds of signer are reserved according to the formula:"]
                #[doc = "`MetadataDepositBase + DepositPerByte * (key.len + value.len)` taking into"]
                #[doc = "account any already reserved funds."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset class whose instance's metadata to set."]
                #[doc = "- `maybe_instance`: The identifier of the asset instance whose metadata to set."]
                #[doc = "- `key`: The key of the attribute."]
                #[doc = "- `value`: The value to which to set the attribute."]
                #[doc = ""]
                #[doc = "Emits `AttributeSet`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn set_attribute(
                    &self,
                    class: ::core::primitive::u128,
                    maybe_instance: ::core::option::Option<::core::primitive::u128>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetAttribute,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetAttribute>()?
                        == [
                            201u8, 58u8, 143u8, 57u8, 65u8, 57u8, 59u8, 160u8, 140u8, 140u8, 100u8,
                            2u8, 145u8, 198u8, 155u8, 29u8, 209u8, 63u8, 51u8, 91u8, 114u8, 139u8,
                            30u8, 159u8, 81u8, 57u8, 237u8, 34u8, 70u8, 216u8, 37u8, 102u8,
                        ]
                    {
                        let call = SetAttribute {
                            class,
                            maybe_instance,
                            key,
                            value,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Clear an attribute for an asset class or instance."]
                #[doc = ""]
                #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                #[doc = "asset `class`."]
                #[doc = ""]
                #[doc = "Any deposit is freed for the asset class owner."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset class whose instance's metadata to clear."]
                #[doc = "- `maybe_instance`: The identifier of the asset instance whose metadata to clear."]
                #[doc = "- `key`: The key of the attribute."]
                #[doc = ""]
                #[doc = "Emits `AttributeCleared`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn clear_attribute(
                    &self,
                    class: ::core::primitive::u128,
                    maybe_instance: ::core::option::Option<::core::primitive::u128>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ClearAttribute,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ClearAttribute>()?
                        == [
                            1u8, 54u8, 59u8, 1u8, 98u8, 171u8, 69u8, 245u8, 59u8, 70u8, 70u8, 1u8,
                            203u8, 168u8, 83u8, 252u8, 135u8, 147u8, 26u8, 62u8, 133u8, 129u8,
                            237u8, 37u8, 161u8, 4u8, 201u8, 20u8, 210u8, 35u8, 160u8, 9u8,
                        ]
                    {
                        let call = ClearAttribute {
                            class,
                            maybe_instance,
                            key,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the metadata for an asset instance."]
                #[doc = ""]
                #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                #[doc = "asset `class`."]
                #[doc = ""]
                #[doc = "If the origin is Signed, then funds of signer are reserved according to the formula:"]
                #[doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"]
                #[doc = "account any already reserved funds."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset class whose instance's metadata to set."]
                #[doc = "- `instance`: The identifier of the asset instance whose metadata to set."]
                #[doc = "- `data`: The general information of this asset. Limited in length by `StringLimit`."]
                #[doc = "- `is_frozen`: Whether the metadata should be frozen against further changes."]
                #[doc = ""]
                #[doc = "Emits `MetadataSet`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn set_metadata(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetMetadata>()?
                        == [
                            108u8, 117u8, 161u8, 103u8, 80u8, 183u8, 100u8, 166u8, 122u8, 219u8,
                            127u8, 140u8, 165u8, 88u8, 125u8, 71u8, 31u8, 33u8, 1u8, 66u8, 124u8,
                            234u8, 73u8, 224u8, 81u8, 199u8, 204u8, 141u8, 65u8, 74u8, 18u8, 89u8,
                        ]
                    {
                        let call = SetMetadata {
                            class,
                            instance,
                            data,
                            is_frozen,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Clear the metadata for an asset instance."]
                #[doc = ""]
                #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                #[doc = "asset `instance`."]
                #[doc = ""]
                #[doc = "Any deposit is freed for the asset class owner."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset class whose instance's metadata to clear."]
                #[doc = "- `instance`: The identifier of the asset instance whose metadata to clear."]
                #[doc = ""]
                #[doc = "Emits `MetadataCleared`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn clear_metadata(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ClearMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ClearMetadata>()?
                        == [
                            42u8, 6u8, 221u8, 228u8, 104u8, 250u8, 23u8, 226u8, 183u8, 239u8, 46u8,
                            108u8, 184u8, 40u8, 10u8, 196u8, 72u8, 88u8, 230u8, 154u8, 250u8, 30u8,
                            99u8, 209u8, 158u8, 191u8, 75u8, 165u8, 85u8, 31u8, 75u8, 149u8,
                        ]
                    {
                        let call = ClearMetadata { class, instance };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the metadata for an asset class."]
                #[doc = ""]
                #[doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"]
                #[doc = "the asset `class`."]
                #[doc = ""]
                #[doc = "If the origin is `Signed`, then funds of signer are reserved according to the formula:"]
                #[doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"]
                #[doc = "account any already reserved funds."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset whose metadata to update."]
                #[doc = "- `data`: The general information of this asset. Limited in length by `StringLimit`."]
                #[doc = "- `is_frozen`: Whether the metadata should be frozen against further changes."]
                #[doc = ""]
                #[doc = "Emits `ClassMetadataSet`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn set_class_metadata(
                    &self,
                    class: ::core::primitive::u128,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetClassMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetClassMetadata>()?
                        == [
                            137u8, 199u8, 225u8, 169u8, 109u8, 52u8, 46u8, 219u8, 114u8, 135u8,
                            210u8, 6u8, 125u8, 238u8, 95u8, 55u8, 142u8, 13u8, 192u8, 74u8, 106u8,
                            235u8, 22u8, 66u8, 165u8, 231u8, 97u8, 93u8, 227u8, 189u8, 186u8, 73u8,
                        ]
                    {
                        let call = SetClassMetadata {
                            class,
                            data,
                            is_frozen,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Clear the metadata for an asset class."]
                #[doc = ""]
                #[doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"]
                #[doc = "the asset `class`."]
                #[doc = ""]
                #[doc = "Any deposit is freed for the asset class owner."]
                #[doc = ""]
                #[doc = "- `class`: The identifier of the asset class whose metadata to clear."]
                #[doc = ""]
                #[doc = "Emits `ClassMetadataCleared`."]
                #[doc = ""]
                #[doc = "Weight: `O(1)`"]
                pub fn clear_class_metadata(
                    &self,
                    class: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ClearClassMetadata,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ClearClassMetadata>()?
                        == [
                            196u8, 232u8, 76u8, 203u8, 83u8, 169u8, 24u8, 119u8, 77u8, 85u8, 143u8,
                            238u8, 7u8, 183u8, 196u8, 43u8, 93u8, 47u8, 47u8, 147u8, 207u8, 67u8,
                            244u8, 85u8, 6u8, 104u8, 10u8, 73u8, 159u8, 132u8, 16u8, 221u8,
                        ]
                    {
                        let call = ClearClassMetadata { class };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_uniques::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An asset class was created."]
            pub struct Created {
                pub class: ::core::primitive::u128,
                pub creator: ::subxt::sp_core::crypto::AccountId32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An asset class was force-created."]
            pub struct ForceCreated {
                pub class: ::core::primitive::u128,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset `class` was destroyed."]
            pub struct Destroyed {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An asset `instance` was issued."]
            pub struct Issued {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An asset `instance` was transferred."]
            pub struct Transferred {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An asset `instance` was destroyed."]
            pub struct Burned {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some asset `instance` was frozen."]
            pub struct Frozen {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some asset `instance` was thawed."]
            pub struct Thawed {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `class` was frozen."]
            pub struct ClassFrozen {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Event for ClassFrozen {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassFrozen";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `class` was thawed."]
            pub struct ClassThawed {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Event for ClassThawed {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassThawed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The owner changed."]
            pub struct OwnerChanged {
                pub class: ::core::primitive::u128,
                pub new_owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The management team changed."]
            pub struct TeamChanged {
                pub class: ::core::primitive::u128,
                pub issuer: ::subxt::sp_core::crypto::AccountId32,
                pub admin: ::subxt::sp_core::crypto::AccountId32,
                pub freezer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An `instance` of an asset `class` has been approved by the `owner` for transfer by a"]
            #[doc = "`delegate`."]
            pub struct ApprovedTransfer {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An approval for a `delegate` account to transfer the `instance` of an asset `class` was"]
            #[doc = "cancelled by its `owner`."]
            pub struct ApprovalCancelled {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset `class` has had its attributes changed by the `Force` origin."]
            pub struct AssetStatusChanged {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "AssetStatusChanged";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "New metadata has been set for an asset class."]
            pub struct ClassMetadataSet {
                pub class: ::core::primitive::u128,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for ClassMetadataSet {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassMetadataSet";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "Metadata has been cleared for an asset class."]
            pub struct ClassMetadataCleared {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::Event for ClassMetadataCleared {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassMetadataCleared";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "New metadata has been set for an asset instance."]
            pub struct MetadataSet {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Metadata has been cleared for an asset instance."]
            pub struct MetadataCleared {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Metadata has been cleared for an asset instance."]
            pub struct Redeposited {
                pub class: ::core::primitive::u128,
                pub successful_instances: ::std::vec::Vec<::core::primitive::u128>,
            }
            impl ::subxt::Event for Redeposited {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Redeposited";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "New attribute metadata has been set for an asset class or instance."]
            pub struct AttributeSet {
                pub class: ::core::primitive::u128,
                pub maybe_instance: ::core::option::Option<::core::primitive::u128>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Event for AttributeSet {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "AttributeSet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Attribute metadata has been cleared for an asset class or instance."]
            pub struct AttributeCleared {
                pub class: ::core::primitive::u128,
                pub maybe_instance: ::core::option::Option<::core::primitive::u128>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Event for AttributeCleared {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "AttributeCleared";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Class<'a>(pub &'a ::core::primitive::u128);
            impl ::subxt::StorageEntry for Class<'_> {
                const PALLET: &'static str = "OctopusUniques";
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
                pub &'a ::core::primitive::u128,
                pub &'a ::core::primitive::u128,
            );
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "OctopusUniques";
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
            pub struct ClassAccount<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a ::core::primitive::u128,
            );
            impl ::subxt::StorageEntry for ClassAccount<'_> {
                const PALLET: &'static str = "OctopusUniques";
                const STORAGE: &'static str = "ClassAccount";
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
                    ])
                }
            }
            pub struct Asset<'a>(
                pub &'a ::core::primitive::u128,
                pub &'a ::core::primitive::u128,
            );
            impl ::subxt::StorageEntry for Asset<'_> {
                const PALLET: &'static str = "OctopusUniques";
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
            pub struct ClassMetadataOf<'a>(pub &'a ::core::primitive::u128);
            impl ::subxt::StorageEntry for ClassMetadataOf<'_> {
                const PALLET: &'static str = "OctopusUniques";
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
                pub &'a ::core::primitive::u128,
                pub &'a ::core::primitive::u128,
            );
            impl ::subxt::StorageEntry for InstanceMetadataOf<'_> {
                const PALLET: &'static str = "OctopusUniques";
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
                pub &'a ::core::primitive::u128,
                pub &'a ::core::option::Option<::core::primitive::u128>,
                pub  &'a runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            );
            impl ::subxt::StorageEntry for Attribute<'_> {
                const PALLET: &'static str = "OctopusUniques";
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
                #[doc = " Details of an asset class."]
                pub async fn class(
                    &self,
                    _0: &::core::primitive::u128,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Class>()?
                        == [
                            79u8, 27u8, 236u8, 67u8, 45u8, 11u8, 57u8, 242u8, 46u8, 132u8, 64u8,
                            212u8, 32u8, 114u8, 35u8, 201u8, 115u8, 172u8, 165u8, 150u8, 94u8,
                            185u8, 26u8, 160u8, 11u8, 144u8, 186u8, 43u8, 195u8, 39u8, 36u8, 155u8,
                        ]
                    {
                        let entry = Class(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Details of an asset class."]
                pub async fn class_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Class<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Class>()?
                        == [
                            79u8, 27u8, 236u8, 67u8, 45u8, 11u8, 57u8, 242u8, 46u8, 132u8, 64u8,
                            212u8, 32u8, 114u8, 35u8, 201u8, 115u8, 172u8, 165u8, 150u8, 94u8,
                            185u8, 26u8, 160u8, 11u8, 144u8, 186u8, 43u8, 195u8, 39u8, 36u8, 155u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The assets held by any given account; set out this way so that assets owned by a single"]
                #[doc = " account can be enumerated."]
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    _1: &::core::primitive::u128,
                    _2: &::core::primitive::u128,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            29u8, 248u8, 71u8, 245u8, 81u8, 70u8, 34u8, 134u8, 248u8, 53u8, 155u8,
                            234u8, 150u8, 196u8, 113u8, 75u8, 142u8, 180u8, 87u8, 214u8, 201u8,
                            166u8, 190u8, 136u8, 160u8, 232u8, 225u8, 139u8, 233u8, 49u8, 28u8,
                            24u8,
                        ]
                    {
                        let entry = Account(_0, _1, _2);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The assets held by any given account; set out this way so that assets owned by a single"]
                #[doc = " account can be enumerated."]
                pub async fn account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            29u8, 248u8, 71u8, 245u8, 81u8, 70u8, 34u8, 134u8, 248u8, 53u8, 155u8,
                            234u8, 150u8, 196u8, 113u8, 75u8, 142u8, 180u8, 87u8, 214u8, 201u8,
                            166u8, 190u8, 136u8, 160u8, 232u8, 225u8, 139u8, 233u8, 49u8, 28u8,
                            24u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The classes owned by any given account; set out this way so that classes owned by a single"]
                #[doc = " account can be enumerated."]
                pub async fn class_account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    _1: &::core::primitive::u128,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ClassAccount>()?
                        == [
                            132u8, 19u8, 191u8, 42u8, 253u8, 236u8, 186u8, 175u8, 183u8, 0u8,
                            160u8, 148u8, 24u8, 117u8, 21u8, 162u8, 70u8, 239u8, 45u8, 133u8,
                            200u8, 217u8, 202u8, 230u8, 186u8, 8u8, 18u8, 90u8, 188u8, 33u8, 71u8,
                            7u8,
                        ]
                    {
                        let entry = ClassAccount(_0, _1);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The classes owned by any given account; set out this way so that classes owned by a single"]
                #[doc = " account can be enumerated."]
                pub async fn class_account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClassAccount<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ClassAccount>()?
                        == [
                            132u8, 19u8, 191u8, 42u8, 253u8, 236u8, 186u8, 175u8, 183u8, 0u8,
                            160u8, 148u8, 24u8, 117u8, 21u8, 162u8, 70u8, 239u8, 45u8, 133u8,
                            200u8, 217u8, 202u8, 230u8, 186u8, 8u8, 18u8, 90u8, 188u8, 33u8, 71u8,
                            7u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The assets in existence and their ownership details."]
                pub async fn asset(
                    &self,
                    _0: &::core::primitive::u128,
                    _1: &::core::primitive::u128,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Asset>()?
                        == [
                            59u8, 240u8, 185u8, 223u8, 178u8, 217u8, 8u8, 249u8, 60u8, 96u8, 186u8,
                            215u8, 1u8, 187u8, 63u8, 194u8, 109u8, 238u8, 61u8, 109u8, 36u8, 20u8,
                            80u8, 75u8, 110u8, 95u8, 217u8, 136u8, 161u8, 11u8, 56u8, 10u8,
                        ]
                    {
                        let entry = Asset(_0, _1);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The assets in existence and their ownership details."]
                pub async fn asset_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Asset>()?
                        == [
                            59u8, 240u8, 185u8, 223u8, 178u8, 217u8, 8u8, 249u8, 60u8, 96u8, 186u8,
                            215u8, 1u8, 187u8, 63u8, 194u8, 109u8, 238u8, 61u8, 109u8, 36u8, 20u8,
                            80u8, 75u8, 110u8, 95u8, 217u8, 136u8, 161u8, 11u8, 56u8, 10u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset class."]
                pub async fn class_metadata_of(
                    &self,
                    _0: &::core::primitive::u128,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ClassMetadataOf>()?
                        == [
                            45u8, 181u8, 68u8, 155u8, 228u8, 213u8, 134u8, 119u8, 143u8, 49u8,
                            249u8, 160u8, 92u8, 114u8, 56u8, 220u8, 22u8, 70u8, 133u8, 34u8, 17u8,
                            243u8, 234u8, 159u8, 121u8, 207u8, 14u8, 27u8, 130u8, 169u8, 98u8,
                            170u8,
                        ]
                    {
                        let entry = ClassMetadataOf(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset class."]
                pub async fn class_metadata_of_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClassMetadataOf<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ClassMetadataOf>()?
                        == [
                            45u8, 181u8, 68u8, 155u8, 228u8, 213u8, 134u8, 119u8, 143u8, 49u8,
                            249u8, 160u8, 92u8, 114u8, 56u8, 220u8, 22u8, 70u8, 133u8, 34u8, 17u8,
                            243u8, 234u8, 159u8, 121u8, 207u8, 14u8, 27u8, 130u8, 169u8, 98u8,
                            170u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset instance."]
                pub async fn instance_metadata_of(
                    &self,
                    _0: &::core::primitive::u128,
                    _1: &::core::primitive::u128,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<InstanceMetadataOf>()?
                        == [
                            120u8, 60u8, 235u8, 113u8, 230u8, 91u8, 175u8, 133u8, 92u8, 212u8,
                            90u8, 30u8, 45u8, 220u8, 187u8, 83u8, 192u8, 98u8, 196u8, 193u8, 27u8,
                            221u8, 253u8, 109u8, 183u8, 11u8, 197u8, 127u8, 181u8, 74u8, 132u8,
                            213u8,
                        ]
                    {
                        let entry = InstanceMetadataOf(_0, _1);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset instance."]
                pub async fn instance_metadata_of_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InstanceMetadataOf<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<InstanceMetadataOf>()?
                        == [
                            120u8, 60u8, 235u8, 113u8, 230u8, 91u8, 175u8, 133u8, 92u8, 212u8,
                            90u8, 30u8, 45u8, 220u8, 187u8, 83u8, 192u8, 98u8, 196u8, 193u8, 27u8,
                            221u8, 253u8, 109u8, 183u8, 11u8, 197u8, 127u8, 181u8, 74u8, 132u8,
                            213u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset class."]
                pub async fn attribute(
                    &self,
                    _0: &::core::primitive::u128,
                    _1: &::core::option::Option<::core::primitive::u128>,
                    _2: &runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Attribute>()?
                        == [
                            107u8, 204u8, 167u8, 152u8, 109u8, 129u8, 199u8, 120u8, 79u8, 149u8,
                            174u8, 199u8, 160u8, 17u8, 238u8, 227u8, 217u8, 115u8, 122u8, 50u8,
                            53u8, 140u8, 203u8, 243u8, 89u8, 73u8, 197u8, 203u8, 113u8, 147u8,
                            89u8, 239u8,
                        ]
                    {
                        let entry = Attribute(_0, _1, _2);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Metadata of an asset class."]
                pub async fn attribute_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Attribute<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Attribute>()?
                        == [
                            107u8, 204u8, 167u8, 152u8, 109u8, 129u8, 199u8, 120u8, 79u8, 149u8,
                            174u8, 199u8, 160u8, 17u8, 238u8, 227u8, 217u8, 115u8, 122u8, 50u8,
                            53u8, 140u8, 203u8, 243u8, 89u8, 73u8, 197u8, 203u8, 113u8, 147u8,
                            89u8, 239u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The basic amount of funds that must be reserved for an asset class."]
                pub fn class_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "ClassDeposit")?
                        == [
                            179u8, 90u8, 186u8, 104u8, 152u8, 33u8, 93u8, 165u8, 193u8, 254u8,
                            82u8, 250u8, 21u8, 19u8, 29u8, 155u8, 98u8, 244u8, 113u8, 82u8, 134u8,
                            137u8, 141u8, 75u8, 8u8, 148u8, 64u8, 3u8, 222u8, 42u8, 69u8, 70u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("ClassDeposit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The basic amount of funds that must be reserved for an asset instance."]
                pub fn instance_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "InstanceDeposit")?
                        == [
                            43u8, 80u8, 222u8, 41u8, 217u8, 254u8, 1u8, 168u8, 232u8, 202u8, 122u8,
                            202u8, 202u8, 201u8, 194u8, 76u8, 210u8, 97u8, 95u8, 217u8, 39u8,
                            168u8, 125u8, 20u8, 73u8, 46u8, 146u8, 80u8, 59u8, 149u8, 248u8, 36u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("InstanceDeposit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The basic amount of funds that must be reserved when adding metadata to your asset."]
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "MetadataDepositBase")?
                        == [
                            206u8, 201u8, 26u8, 198u8, 159u8, 174u8, 212u8, 81u8, 73u8, 223u8,
                            96u8, 44u8, 229u8, 140u8, 251u8, 240u8, 199u8, 30u8, 40u8, 52u8, 229u8,
                            249u8, 104u8, 111u8, 138u8, 45u8, 97u8, 78u8, 131u8, 205u8, 119u8,
                            151u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("MetadataDepositBase")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The basic amount of funds that must be reserved when adding an attribute to an asset."]
                pub fn attribute_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "AttributeDepositBase")?
                        == [
                            63u8, 111u8, 114u8, 85u8, 48u8, 17u8, 244u8, 44u8, 74u8, 218u8, 209u8,
                            150u8, 227u8, 58u8, 148u8, 159u8, 91u8, 152u8, 25u8, 216u8, 50u8,
                            101u8, 24u8, 246u8, 1u8, 166u8, 198u8, 151u8, 50u8, 153u8, 93u8, 102u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("AttributeDepositBase")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The additional funds that must be reserved for the number of bytes store in metadata,"]
                #[doc = " either \"normal\" metadata or attribute metadata."]
                pub fn deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "DepositPerByte")?
                        == [
                            88u8, 82u8, 164u8, 201u8, 241u8, 110u8, 252u8, 26u8, 70u8, 120u8,
                            219u8, 206u8, 109u8, 91u8, 140u8, 163u8, 7u8, 34u8, 4u8, 171u8, 43u8,
                            236u8, 24u8, 23u8, 105u8, 77u8, 20u8, 161u8, 61u8, 78u8, 32u8, 58u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("DepositPerByte")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum length of data stored on-chain."]
                pub fn string_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "StringLimit")?
                        == [
                            64u8, 204u8, 1u8, 240u8, 39u8, 64u8, 238u8, 105u8, 253u8, 194u8, 239u8,
                            211u8, 82u8, 163u8, 118u8, 129u8, 51u8, 252u8, 113u8, 145u8, 46u8,
                            73u8, 211u8, 136u8, 160u8, 117u8, 90u8, 181u8, 166u8, 203u8, 244u8,
                            185u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("StringLimit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum length of an attribute key."]
                pub fn key_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "KeyLimit")?
                        == [
                            37u8, 214u8, 216u8, 115u8, 253u8, 41u8, 189u8, 100u8, 84u8, 70u8,
                            100u8, 115u8, 85u8, 221u8, 169u8, 79u8, 201u8, 130u8, 8u8, 51u8, 200u8,
                            248u8, 19u8, 135u8, 94u8, 81u8, 117u8, 242u8, 70u8, 169u8, 133u8,
                            117u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("KeyLimit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum length of an attribute value."]
                pub fn value_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("OctopusUniques", "ValueLimit")?
                        == [
                            29u8, 180u8, 129u8, 166u8, 237u8, 165u8, 240u8, 161u8, 119u8, 217u8,
                            132u8, 181u8, 138u8, 134u8, 165u8, 162u8, 239u8, 6u8, 239u8, 185u8,
                            112u8, 116u8, 46u8, 63u8, 252u8, 40u8, 23u8, 169u8, 121u8, 39u8, 32u8,
                            184u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("OctopusUniques")?;
                        let constant = pallet.constant("ValueLimit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetKeys {
                pub keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "Sets the session key(s) of the function caller to `keys`."]
                #[doc = "Allows an account to set its session key prior to becoming a validator."]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be signed."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
                #[doc = "  `T::Keys::key_ids()` which is fixed."]
                #[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
                #[doc = "- DbWrites: `origin account`, `NextKeys`"]
                #[doc = "- DbReads per key id: `KeyOwner`"]
                #[doc = "- DbWrites per key id: `KeyOwner`"]
                #[doc = "# </weight>"]
                pub fn set_keys(
                    &self,
                    keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetKeys,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetKeys>()?
                        == [
                            224u8, 124u8, 236u8, 244u8, 237u8, 183u8, 209u8, 133u8, 254u8, 17u8,
                            41u8, 153u8, 177u8, 1u8, 127u8, 11u8, 170u8, 233u8, 53u8, 216u8, 222u8,
                            230u8, 87u8, 129u8, 104u8, 215u8, 162u8, 68u8, 110u8, 4u8, 108u8, 0u8,
                        ]
                    {
                        let call = SetKeys { keys, proof };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Removes any session key(s) of the function caller."]
                #[doc = ""]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                #[doc = "usually means being a stash account)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
                #[doc = "  of `T::Keys::key_ids()` which is fixed."]
                #[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
                #[doc = "- DbWrites: `NextKeys`, `origin account`"]
                #[doc = "- DbWrites per key id: `KeyOwner`"]
                #[doc = "# </weight>"]
                pub fn purge_keys(
                    &self,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        PurgeKeys,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<PurgeKeys>()?
                        == [
                            200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
                            35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
                            71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
                        ]
                    {
                        let call = PurgeKeys {};
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            #[doc = "New session has happened. Note that the argument is the session index, not the"]
            #[doc = "block number as the type might suggest."]
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
                #[doc = " The current set of validators."]
                pub async fn validators(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Validators>()?
                        == [
                            186u8, 248u8, 234u8, 74u8, 245u8, 141u8, 90u8, 152u8, 226u8, 220u8,
                            255u8, 104u8, 174u8, 1u8, 37u8, 152u8, 23u8, 208u8, 25u8, 49u8, 33u8,
                            253u8, 254u8, 251u8, 141u8, 16u8, 18u8, 175u8, 196u8, 188u8, 163u8,
                            209u8,
                        ]
                    {
                        let entry = Validators;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Current index of the session."]
                pub async fn current_index(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<CurrentIndex>()?
                        == [
                            148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
                            183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
                            159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
                            130u8, 40u8,
                        ]
                    {
                        let entry = CurrentIndex;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " True if the underlying economic identities or weighting behind the validators"]
                #[doc = " has changed in the queued validator set."]
                pub async fn queued_changed(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<QueuedChanged>()?
                        == [
                            105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
                            244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
                            247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
                        ]
                    {
                        let entry = QueuedChanged;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The queued keys for the next session. When the next session begins, these keys"]
                #[doc = " will be used to determine the validator's session keys."]
                pub async fn queued_keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<QueuedKeys>()?
                        == [
                            139u8, 177u8, 78u8, 136u8, 47u8, 183u8, 45u8, 83u8, 48u8, 164u8, 218u8,
                            50u8, 51u8, 203u8, 196u8, 246u8, 48u8, 241u8, 9u8, 157u8, 254u8, 22u8,
                            123u8, 219u8, 243u8, 42u8, 36u8, 175u8, 159u8, 238u8, 242u8, 204u8,
                        ]
                    {
                        let entry = QueuedKeys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Indices of disabled validators."]
                #[doc = ""]
                #[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
                #[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
                #[doc = " a new set of identities."]
                pub async fn disabled_validators(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<DisabledValidators>()?
                        == [
                            135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
                            189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
                            110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
                            84u8,
                        ]
                    {
                        let entry = DisabledValidators;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The next session keys for a validator."]
                pub async fn next_keys(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextKeys>()?
                        == [
                            25u8, 113u8, 96u8, 196u8, 191u8, 47u8, 173u8, 101u8, 50u8, 237u8,
                            231u8, 247u8, 60u8, 130u8, 31u8, 185u8, 57u8, 206u8, 185u8, 84u8, 45u8,
                            32u8, 242u8, 196u8, 181u8, 159u8, 121u8, 140u8, 12u8, 53u8, 174u8,
                            144u8,
                        ]
                    {
                        let entry = NextKeys(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The next session keys for a validator."]
                pub async fn next_keys_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextKeys<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextKeys>()?
                        == [
                            25u8, 113u8, 96u8, 196u8, 191u8, 47u8, 173u8, 101u8, 50u8, 237u8,
                            231u8, 247u8, 60u8, 130u8, 31u8, 185u8, 57u8, 206u8, 185u8, 84u8, 45u8,
                            32u8, 242u8, 196u8, 181u8, 159u8, 121u8, 140u8, 12u8, 53u8, 174u8,
                            144u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub async fn key_owner(
                    &self,
                    _0: &runtime_types::sp_core::crypto::KeyTypeId,
                    _1: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<KeyOwner>()?
                        == [
                            49u8, 245u8, 212u8, 141u8, 211u8, 208u8, 109u8, 102u8, 249u8, 161u8,
                            41u8, 93u8, 220u8, 230u8, 14u8, 59u8, 251u8, 176u8, 33u8, 127u8, 93u8,
                            149u8, 205u8, 229u8, 113u8, 129u8, 162u8, 177u8, 155u8, 216u8, 151u8,
                            57u8,
                        ]
                    {
                        let entry = KeyOwner(_0, _1);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub async fn key_owner_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, KeyOwner<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<KeyOwner>()?
                        == [
                            49u8, 245u8, 212u8, 141u8, 211u8, 208u8, 109u8, 102u8, 249u8, 161u8,
                            41u8, 93u8, 220u8, 230u8, 14u8, 59u8, 251u8, 176u8, 33u8, 127u8, 93u8,
                            149u8, 205u8, 229u8, 113u8, 129u8, 162u8, 177u8, 155u8, 216u8, 151u8,
                            57u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocation,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ReportEquivocation>()?
                        == [
                            230u8, 252u8, 24u8, 207u8, 164u8, 127u8, 177u8, 30u8, 113u8, 175u8,
                            207u8, 252u8, 230u8, 225u8, 181u8, 190u8, 236u8, 110u8, 145u8, 168u8,
                            200u8, 134u8, 88u8, 234u8, 231u8, 45u8, 149u8, 169u8, 155u8, 114u8,
                            62u8, 65u8,
                        ]
                    {
                        let call = ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocationUnsigned,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .call_hash::<ReportEquivocationUnsigned>()?
                        == [
                            141u8, 235u8, 27u8, 135u8, 124u8, 124u8, 234u8, 51u8, 100u8, 105u8,
                            188u8, 248u8, 133u8, 10u8, 84u8, 14u8, 40u8, 235u8, 14u8, 107u8, 63u8,
                            148u8, 107u8, 172u8, 136u8, 159u8, 86u8, 23u8, 145u8, 221u8, 93u8,
                            206u8,
                        ]
                    {
                        let call = ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has"]
                #[doc = "stalled. This will trigger a forced authority set change at the beginning"]
                #[doc = "of the next session, to be enacted `delay` blocks after that. The delay"]
                #[doc = "should be high enough to safely assume that the block signalling the"]
                #[doc = "forced change will not be re-orged (e.g. 1000 blocks). The GRANDPA voters"]
                #[doc = "will start the new authority set using the given finalized block as base."]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        NoteStalled,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<NoteStalled>()?
                        == [
                            227u8, 98u8, 249u8, 158u8, 96u8, 124u8, 72u8, 188u8, 27u8, 215u8, 73u8,
                            62u8, 103u8, 79u8, 38u8, 48u8, 212u8, 88u8, 233u8, 187u8, 11u8, 95u8,
                            39u8, 247u8, 55u8, 184u8, 228u8, 102u8, 13u8, 251u8, 52u8, 206u8,
                        ]
                    {
                        let call = NoteStalled {
                            delay,
                            best_finalized_block_number,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "New authority set has been applied."]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Current authority set has been resumed."]
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
                #[doc = " State of the current authority set."]
                pub async fn state(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<State>()?
                        == [
                            159u8, 75u8, 78u8, 23u8, 98u8, 89u8, 239u8, 230u8, 192u8, 67u8, 139u8,
                            222u8, 151u8, 237u8, 216u8, 20u8, 235u8, 247u8, 180u8, 24u8, 64u8,
                            160u8, 58u8, 15u8, 205u8, 191u8, 120u8, 68u8, 32u8, 5u8, 161u8, 106u8,
                        ]
                    {
                        let entry = State;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub async fn pending_change(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<PendingChange>()?
                        == [
                            128u8, 176u8, 209u8, 41u8, 231u8, 111u8, 205u8, 198u8, 154u8, 44u8,
                            228u8, 231u8, 44u8, 110u8, 74u8, 9u8, 31u8, 86u8, 128u8, 244u8, 112u8,
                            21u8, 120u8, 176u8, 50u8, 213u8, 122u8, 46u8, 85u8, 255u8, 40u8, 173u8,
                        ]
                    {
                        let entry = PendingChange;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " next block number where we can force a change."]
                pub async fn next_forced(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextForced>()?
                        == [
                            99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8, 67u8,
                            6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8, 19u8, 169u8,
                            196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8, 156u8,
                        ]
                    {
                        let entry = NextForced;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " `true` if we are currently stalled."]
                pub async fn stalled(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Stalled>()?
                        == [
                            219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8, 186u8,
                            189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8, 231u8,
                            109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8, 24u8,
                            185u8,
                        ]
                    {
                        let entry = Stalled;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub async fn current_set_id(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<CurrentSetId>()?
                        == [
                            129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8, 20u8,
                            178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8, 105u8,
                            52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8, 7u8,
                            108u8,
                        ]
                    {
                        let entry = CurrentSetId;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub async fn set_id_session(
                    &self,
                    _0: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<SetIdSession>()?
                        == [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ]
                    {
                        let entry = SetIdSession(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub async fn set_id_session_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<SetIdSession>()?
                        == [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Grandpa", "MaxAuthorities")?
                        == [
                            205u8, 23u8, 62u8, 93u8, 110u8, 248u8, 109u8, 245u8, 1u8, 57u8, 144u8,
                            146u8, 103u8, 0u8, 178u8, 246u8, 63u8, 80u8, 77u8, 155u8, 202u8, 208u8,
                            73u8, 194u8, 210u8, 49u8, 121u8, 99u8, 101u8, 222u8, 127u8, 206u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Grandpa")?;
                        let constant = pallet.constant("MaxAuthorities")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"]
                #[doc = "  length of `heartbeat.network_state.external_address`"]
                #[doc = "  - `O(K)`: decoding of length `K`"]
                #[doc = "  - `O(E)`: decoding/encoding of length `E`"]
                #[doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"]
                #[doc = "  `ReceivedHeartbeats`"]
                #[doc = "- DbWrites: `ReceivedHeartbeats`"]
                #[doc = "# </weight>"]
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Heartbeat,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Heartbeat>()?
                        == [
                            246u8, 83u8, 28u8, 233u8, 69u8, 55u8, 28u8, 178u8, 82u8, 159u8, 56u8,
                            241u8, 111u8, 78u8, 194u8, 15u8, 14u8, 250u8, 172u8, 148u8, 208u8,
                            52u8, 33u8, 106u8, 159u8, 210u8, 196u8, 79u8, 138u8, 194u8, 150u8,
                            201u8,
                        ]
                    {
                        let call = Heartbeat {
                            heartbeat,
                            signature,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A new heartbeat was received from `AuthorityId`."]
            pub struct HeartbeatReceived {
                pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            }
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "At the end of the session, no offence was committed."]
            pub struct AllGood;
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "At the end of the session, at least one validator was found to be offline."]
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
                #[doc = " The block number after which it's ok to send heartbeats in the current"]
                #[doc = " session."]
                #[doc = ""]
                #[doc = " At the beginning of each session we set this to a value that should fall"]
                #[doc = " roughly in the middle of the session duration. The idea is to first wait for"]
                #[doc = " the validators to produce a block in the current session, so that the"]
                #[doc = " heartbeat later on will not be necessary."]
                #[doc = ""]
                #[doc = " This value will only be used as a fallback if we fail to get a proper session"]
                #[doc = " progress estimate from `NextSessionRotation`, as those estimates should be"]
                #[doc = " more accurate then the value we calculate for `HeartbeatAfter`."]
                pub async fn heartbeat_after(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<HeartbeatAfter>()?
                        == [
                            108u8, 100u8, 85u8, 198u8, 226u8, 122u8, 94u8, 225u8, 97u8, 154u8,
                            135u8, 95u8, 106u8, 28u8, 185u8, 78u8, 192u8, 196u8, 35u8, 191u8, 12u8,
                            19u8, 163u8, 46u8, 232u8, 235u8, 193u8, 81u8, 126u8, 204u8, 25u8,
                            228u8,
                        ]
                    {
                        let entry = HeartbeatAfter;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current set of keys that may issue a heartbeat."]
                pub async fn keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Keys>()?
                        == [
                            105u8, 250u8, 99u8, 106u8, 9u8, 29u8, 73u8, 176u8, 158u8, 247u8, 28u8,
                            171u8, 95u8, 1u8, 109u8, 11u8, 231u8, 52u8, 54u8, 102u8, 142u8, 105u8,
                            209u8, 31u8, 132u8, 60u8, 89u8, 181u8, 89u8, 193u8, 241u8, 130u8,
                        ]
                    {
                        let entry = Keys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
                #[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
                pub async fn received_heartbeats(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ReceivedHeartbeats>()?
                        == [
                            29u8, 40u8, 67u8, 222u8, 59u8, 104u8, 24u8, 193u8, 249u8, 200u8, 152u8,
                            225u8, 72u8, 243u8, 140u8, 114u8, 121u8, 216u8, 54u8, 145u8, 205u8,
                            82u8, 133u8, 128u8, 109u8, 54u8, 153u8, 118u8, 66u8, 147u8, 251u8,
                            148u8,
                        ]
                    {
                        let entry = ReceivedHeartbeats(_0, _1);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
                #[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
                pub async fn received_heartbeats_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ReceivedHeartbeats>()?
                        == [
                            29u8, 40u8, 67u8, 222u8, 59u8, 104u8, 24u8, 193u8, 249u8, 200u8, 152u8,
                            225u8, 72u8, 243u8, 140u8, 114u8, 121u8, 216u8, 54u8, 145u8, 205u8,
                            82u8, 133u8, 128u8, 109u8, 54u8, 153u8, 118u8, 66u8, 147u8, 251u8,
                            148u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
                #[doc = " number of blocks authored by the given authority."]
                pub async fn authored_blocks(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<AuthoredBlocks>()?
                        == [
                            94u8, 193u8, 107u8, 126u8, 3u8, 13u8, 28u8, 151u8, 197u8, 226u8, 224u8,
                            48u8, 138u8, 113u8, 31u8, 57u8, 111u8, 184u8, 218u8, 215u8, 185u8,
                            83u8, 209u8, 139u8, 114u8, 241u8, 68u8, 110u8, 157u8, 208u8, 16u8,
                            22u8,
                        ]
                    {
                        let entry = AuthoredBlocks(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
                #[doc = " number of blocks authored by the given authority."]
                pub async fn authored_blocks_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthoredBlocks<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<AuthoredBlocks>()?
                        == [
                            94u8, 193u8, 107u8, 126u8, 3u8, 13u8, 28u8, 151u8, 197u8, 226u8, 224u8,
                            48u8, 138u8, 113u8, 31u8, 57u8, 111u8, 184u8, 218u8, 215u8, 185u8,
                            83u8, 209u8, 139u8, 114u8, 241u8, 68u8, 110u8, 157u8, 208u8, 16u8,
                            22u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " A configuration for base priority of unsigned transactions."]
                #[doc = ""]
                #[doc = " This is exposed so that it can be tuned for particular runtime, when"]
                #[doc = " multiple pallets send unsigned transactions."]
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("ImOnline", "UnsignedPriority")?
                        == [
                            78u8, 226u8, 84u8, 70u8, 162u8, 23u8, 167u8, 100u8, 156u8, 228u8,
                            119u8, 16u8, 28u8, 202u8, 21u8, 71u8, 72u8, 244u8, 3u8, 255u8, 243u8,
                            55u8, 109u8, 238u8, 26u8, 180u8, 207u8, 175u8, 221u8, 27u8, 213u8,
                            217u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("ImOnline")?;
                        let constant = pallet.constant("UnsignedPriority")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Latest MMR Root hash."]
                pub async fn root_hash(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<RootHash>()?
                        == [
                            156u8, 176u8, 7u8, 77u8, 96u8, 23u8, 240u8, 140u8, 74u8, 33u8, 12u8,
                            124u8, 160u8, 228u8, 78u8, 8u8, 139u8, 164u8, 109u8, 52u8, 168u8,
                            234u8, 221u8, 194u8, 100u8, 2u8, 250u8, 5u8, 188u8, 203u8, 13u8, 117u8,
                        ]
                    {
                        let entry = RootHash;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Current size of the MMR (number of leaves)."]
                pub async fn number_of_leaves(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<NumberOfLeaves>()?
                        == [
                            138u8, 124u8, 23u8, 186u8, 255u8, 231u8, 187u8, 122u8, 213u8, 160u8,
                            29u8, 24u8, 88u8, 98u8, 171u8, 36u8, 195u8, 216u8, 27u8, 190u8, 192u8,
                            152u8, 8u8, 13u8, 210u8, 232u8, 45u8, 184u8, 240u8, 255u8, 156u8,
                            204u8,
                        ]
                    {
                        let entry = NumberOfLeaves;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Hashes of the nodes in the MMR."]
                #[doc = ""]
                #[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
                #[doc = " are pruned and only stored in the Offchain DB."]
                pub async fn nodes(
                    &self,
                    _0: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Nodes>()?
                        == [
                            150u8, 135u8, 92u8, 131u8, 236u8, 239u8, 244u8, 0u8, 254u8, 50u8,
                            117u8, 224u8, 131u8, 70u8, 100u8, 212u8, 191u8, 58u8, 146u8, 207u8,
                            94u8, 72u8, 187u8, 139u8, 80u8, 136u8, 215u8, 68u8, 131u8, 249u8,
                            190u8, 247u8,
                        ]
                    {
                        let entry = Nodes(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Hashes of the nodes in the MMR."]
                #[doc = ""]
                #[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
                #[doc = " are pruned and only stored in the Offchain DB."]
                pub async fn nodes_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nodes<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Nodes>()?
                        == [
                            150u8, 135u8, 92u8, 131u8, 236u8, 239u8, 244u8, 0u8, 254u8, 50u8,
                            117u8, 224u8, 131u8, 70u8, 100u8, 212u8, 191u8, 58u8, 146u8, 207u8,
                            94u8, 72u8, 187u8, 139u8, 80u8, 136u8, 215u8, 68u8, 131u8, 249u8,
                            190u8, 247u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " The current authorities set"]
                pub async fn authorities(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Authorities>()?
                        == [
                            45u8, 197u8, 244u8, 25u8, 113u8, 204u8, 231u8, 240u8, 124u8, 4u8,
                            153u8, 160u8, 92u8, 242u8, 251u8, 64u8, 146u8, 82u8, 161u8, 154u8,
                            238u8, 220u8, 206u8, 186u8, 244u8, 49u8, 238u8, 244u8, 122u8, 26u8,
                            159u8, 168u8,
                        ]
                    {
                        let entry = Authorities;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current validator set id"]
                pub async fn validator_set_id(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ValidatorSetId>()?
                        == [
                            132u8, 47u8, 139u8, 239u8, 214u8, 179u8, 24u8, 63u8, 55u8, 154u8,
                            248u8, 206u8, 73u8, 7u8, 52u8, 135u8, 54u8, 111u8, 250u8, 106u8, 71u8,
                            78u8, 44u8, 44u8, 235u8, 177u8, 36u8, 112u8, 17u8, 122u8, 252u8, 80u8,
                        ]
                    {
                        let entry = ValidatorSetId;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Authorities set scheduled to be used with the next session"]
                pub async fn next_authorities(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextAuthorities>()?
                        == [
                            217u8, 12u8, 213u8, 100u8, 67u8, 73u8, 155u8, 134u8, 236u8, 210u8,
                            129u8, 96u8, 191u8, 83u8, 200u8, 17u8, 181u8, 124u8, 201u8, 155u8,
                            14u8, 246u8, 203u8, 23u8, 57u8, 221u8, 95u8, 174u8, 128u8, 9u8, 32u8,
                            1u8,
                        ]
                    {
                        let entry = NextAuthorities;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[doc = " Details of next BEEFY authority set."]
                #[doc = ""]
                #[doc = " This storage entry is used as cache for calls to `update_beefy_next_authority_set`."]
                pub async fn beefy_next_authorities(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<BeefyNextAuthorities>()?
                        == [
                            176u8, 52u8, 163u8, 20u8, 65u8, 149u8, 67u8, 119u8, 40u8, 134u8, 178u8,
                            89u8, 67u8, 6u8, 201u8, 226u8, 207u8, 218u8, 138u8, 14u8, 201u8, 41u8,
                            40u8, 110u8, 227u8, 200u8, 56u8, 164u8, 187u8, 168u8, 116u8, 127u8,
                        ]
                    {
                        let entry = BeefyNextAuthorities;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Sudo, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Sudo>()?
                        == [
                            175u8, 118u8, 40u8, 58u8, 7u8, 81u8, 19u8, 5u8, 183u8, 177u8, 167u8,
                            151u8, 199u8, 232u8, 165u8, 43u8, 50u8, 128u8, 235u8, 197u8, 47u8,
                            226u8, 175u8, 177u8, 170u8, 194u8, 149u8, 18u8, 224u8, 98u8, 124u8,
                            247u8,
                        ]
                    {
                        let call = Sudo {
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- The weight of this call is defined by the caller."]
                #[doc = "# </weight>"]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SudoUncheckedWeight,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SudoUncheckedWeight>()?
                        == [
                            164u8, 85u8, 94u8, 110u8, 198u8, 102u8, 244u8, 163u8, 45u8, 100u8,
                            147u8, 61u8, 104u8, 35u8, 22u8, 174u8, 48u8, 154u8, 116u8, 96u8, 106u8,
                            239u8, 237u8, 67u8, 214u8, 37u8, 84u8, 28u8, 1u8, 111u8, 221u8, 218u8,
                        ]
                    {
                        let call = SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB change."]
                #[doc = "# </weight>"]
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, SetKey, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetKey>()?
                        == [
                            77u8, 253u8, 211u8, 157u8, 74u8, 92u8, 1u8, 102u8, 178u8, 103u8, 126u8,
                            56u8, 156u8, 105u8, 45u8, 44u8, 64u8, 154u8, 163u8, 102u8, 93u8, 93u8,
                            212u8, 5u8, 148u8, 184u8, 22u8, 135u8, 110u8, 102u8, 44u8, 172u8,
                        ]
                    {
                        let call = SetKey { new };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::node_template_runtime::Call,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, SudoAs, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SudoAs>()?
                        == [
                            185u8, 205u8, 172u8, 244u8, 214u8, 109u8, 74u8, 94u8, 129u8, 106u8,
                            253u8, 33u8, 13u8, 209u8, 52u8, 35u8, 218u8, 140u8, 65u8, 206u8, 94u8,
                            166u8, 229u8, 194u8, 60u8, 237u8, 56u8, 181u8, 99u8, 22u8, 203u8, 66u8,
                        ]
                    {
                        let call = SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
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
                #[doc = " The `AccountId` of the sudo key."]
                pub async fn key(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Key>()?
                        == [
                            222u8, 90u8, 158u8, 233u8, 184u8, 23u8, 141u8, 135u8, 81u8, 187u8,
                            47u8, 100u8, 30u8, 81u8, 239u8, 197u8, 249u8, 253u8, 73u8, 207u8,
                            161u8, 141u8, 174u8, 59u8, 74u8, 181u8, 10u8, 90u8, 22u8, 109u8, 62u8,
                            27u8,
                        ]
                    {
                        let entry = Key;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct DoSomething {
                pub something: ::core::primitive::u32,
            }
            impl ::subxt::Call for DoSomething {
                const PALLET: &'static str = "TemplateModule";
                const FUNCTION: &'static str = "do_something";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                pub fn do_something(
                    &self,
                    something: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        DoSomething,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<DoSomething>()?
                        == [
                            118u8, 135u8, 114u8, 170u8, 240u8, 97u8, 215u8, 4u8, 153u8, 184u8,
                            191u8, 161u8, 106u8, 187u8, 143u8, 60u8, 254u8, 16u8, 244u8, 150u8,
                            224u8, 18u8, 74u8, 157u8, 89u8, 229u8, 39u8, 88u8, 157u8, 140u8, 184u8,
                            51u8,
                        ]
                    {
                        let call = DoSomething { something };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "An example dispatchable that may throw a custom error."]
                pub fn cause_error(
                    &self,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        CauseError,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<CauseError>()?
                        == [
                            130u8, 169u8, 2u8, 174u8, 179u8, 44u8, 35u8, 37u8, 1u8, 110u8, 218u8,
                            248u8, 55u8, 69u8, 235u8, 186u8, 204u8, 99u8, 5u8, 33u8, 20u8, 68u8,
                            138u8, 254u8, 126u8, 2u8, 199u8, 51u8, 194u8, 6u8, 53u8, 243u8,
                        ]
                    {
                        let call = CauseError {};
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_template::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Event documentation should end with an array that provides descriptive names for event"]
            #[doc = "parameters. [something, who]"]
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
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Something>()?
                        == [
                            133u8, 15u8, 68u8, 243u8, 54u8, 252u8, 134u8, 227u8, 161u8, 207u8,
                            221u8, 164u8, 33u8, 75u8, 249u8, 148u8, 184u8, 225u8, 102u8, 168u8,
                            104u8, 72u8, 191u8, 148u8, 253u8, 173u8, 33u8, 89u8, 108u8, 218u8,
                            70u8, 179u8,
                        ]
                    {
                        let entry = Something;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Deliver {
                pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                pub tmp: ::core::primitive::u8,
            }
            impl ::subxt::Call for Deliver {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "deliver";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct UpdateClientState {
                pub client_id: ::std::vec::Vec<::core::primitive::u8>,
                pub mmr_root: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for UpdateClientState {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "update_client_state";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct DeleteSendPacketEvent;
            impl ::subxt::Call for DeleteSendPacketEvent {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "delete_send_packet_event";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct DeleteAckPacketEvent;
            impl ::subxt::Call for DeleteAckPacketEvent {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "delete_ack_packet_event";
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
                #[doc = "This function acts as an entry for all of the IBC request(except MMR root update)."]
                #[doc = "I.e., create clients, update clients, handshakes to create channels, ...etc"]
                pub fn deliver(
                    &self,
                    messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                    tmp: ::core::primitive::u8,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Deliver,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Deliver>()?
                        == [
                            94u8, 89u8, 78u8, 130u8, 130u8, 148u8, 176u8, 178u8, 139u8, 78u8,
                            235u8, 190u8, 160u8, 65u8, 229u8, 28u8, 249u8, 7u8, 142u8, 146u8, 42u8,
                            182u8, 224u8, 88u8, 37u8, 191u8, 35u8, 71u8, 168u8, 117u8, 212u8,
                            173u8,
                        ]
                    {
                        let call = Deliver { messages, tmp };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Update the MMR root stored in client_state"]
                #[doc = "Example of invoking this function via subxt"]
                pub fn update_client_state(
                    &self,
                    client_id: ::std::vec::Vec<::core::primitive::u8>,
                    mmr_root: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        UpdateClientState,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<UpdateClientState>()?
                        == [
                            121u8, 232u8, 249u8, 204u8, 195u8, 137u8, 227u8, 218u8, 6u8, 153u8,
                            10u8, 131u8, 93u8, 53u8, 91u8, 145u8, 202u8, 32u8, 187u8, 104u8, 36u8,
                            165u8, 236u8, 31u8, 135u8, 50u8, 150u8, 87u8, 30u8, 107u8, 169u8,
                            166u8,
                        ]
                    {
                        let call = UpdateClientState {
                            client_id,
                            mmr_root,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Transfer interface for user test by explore"]
                pub fn transfer(
                    &self,
                    source_port: ::std::vec::Vec<::core::primitive::u8>,
                    source_channel: ::std::vec::Vec<::core::primitive::u8>,
                    token: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                    receiver: ::std::vec::Vec<::core::primitive::u8>,
                    timeout_height: ::core::primitive::u64,
                    timeout_timestamp: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Transfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Transfer>()?
                        == [
                            166u8, 115u8, 135u8, 47u8, 243u8, 145u8, 189u8, 95u8, 87u8, 133u8,
                            97u8, 104u8, 13u8, 103u8, 246u8, 226u8, 164u8, 41u8, 175u8, 24u8,
                            110u8, 40u8, 244u8, 17u8, 137u8, 12u8, 147u8, 12u8, 208u8, 215u8, 88u8,
                            173u8,
                        ]
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
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn delete_send_packet_event(
                    &self,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        DeleteSendPacketEvent,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .call_hash::<DeleteSendPacketEvent>()?
                        == [
                            118u8, 248u8, 190u8, 235u8, 207u8, 82u8, 39u8, 30u8, 234u8, 197u8,
                            70u8, 76u8, 249u8, 227u8, 10u8, 48u8, 200u8, 136u8, 193u8, 248u8,
                            144u8, 104u8, 69u8, 52u8, 136u8, 217u8, 146u8, 236u8, 115u8, 107u8,
                            227u8, 111u8,
                        ]
                    {
                        let call = DeleteSendPacketEvent {};
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub fn delete_ack_packet_event(
                    &self,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        DeleteAckPacketEvent,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<DeleteAckPacketEvent>()?
                        == [
                            75u8, 24u8, 246u8, 1u8, 96u8, 231u8, 39u8, 112u8, 114u8, 137u8, 83u8,
                            104u8, 138u8, 161u8, 208u8, 234u8, 100u8, 210u8, 250u8, 54u8, 80u8,
                            71u8, 117u8, 43u8, 169u8, 249u8, 109u8, 181u8, 86u8, 13u8, 207u8,
                            150u8,
                        ]
                    {
                        let call = DeleteAckPacketEvent {};
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_ibc::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit new block event"]
            pub struct NewBlock(pub runtime_types::pallet_ibc::event::primitive::Height);
            impl ::subxt::Event for NewBlock {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "NewBlock";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit create client event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit updte client event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit update client state event"]
            pub struct UpdateClientState(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::ClientState,
            );
            impl ::subxt::Event for UpdateClientState {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpdateClientState";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit upgrade client event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit client misbehaviour event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open init connection event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open try connection event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open ack connection event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open confirm connection event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open init channel event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open try channel event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open ack channel event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit open confirm channel event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit close init channel event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit close confirm channel event"]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit send packet event"]
            pub struct SendPacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for SendPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "SendPacket";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit receive packet"]
            pub struct ReceivePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for ReceivePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ReceivePacket";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit write acknowledgement packet event"]
            pub struct WriteAcknowledgement(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for WriteAcknowledgement {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "WriteAcknowledgement";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit acknowledgement packet event"]
            pub struct AcknowledgePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for AcknowledgePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "AcknowledgePacket";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit timeout packet event"]
            pub struct TimeoutPacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for TimeoutPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutPacket";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit timeout on close packet event"]
            pub struct TimeoutOnClosePacket(
                pub runtime_types::pallet_ibc::event::primitive::Height,
                pub runtime_types::pallet_ibc::event::primitive::Packet,
            );
            impl ::subxt::Event for TimeoutOnClosePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutOnClosePacket";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit empty event"]
            pub struct Empty(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::Event for Empty {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "Empty";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit chain error event"]
            pub struct ChainError(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::Event for ChainError {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ChainError";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit escrow token"]
            pub struct EscrowToken(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for EscrowToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "EscrowToken";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "emit burn token"]
            pub struct BurnToken(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BurnToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "BurnToken";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "unescrow token"]
            pub struct UnEscrowToken(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UnEscrowToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UnEscrowToken";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "mint token"]
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
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ConsensusStatesKeys;
            impl ::subxt::StorageEntry for ConsensusStatesKeys {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "ConsensusStatesKeys";
                type Value = ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
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
                #[doc = " ClientStatePath(client_id) => ClientState"]
                pub async fn client_states(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ClientStates>()?
                        == [
                            152u8, 61u8, 20u8, 185u8, 66u8, 82u8, 87u8, 198u8, 200u8, 82u8, 17u8,
                            70u8, 194u8, 161u8, 141u8, 18u8, 150u8, 161u8, 211u8, 177u8, 40u8,
                            189u8, 70u8, 104u8, 51u8, 190u8, 94u8, 171u8, 157u8, 254u8, 120u8,
                            254u8,
                        ]
                    {
                        let entry = ClientStates(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " ClientStatePath(client_id) => ClientState"]
                pub async fn client_states_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClientStates<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ClientStates>()?
                        == [
                            152u8, 61u8, 20u8, 185u8, 66u8, 82u8, 87u8, 198u8, 200u8, 82u8, 17u8,
                            70u8, 194u8, 161u8, 141u8, 18u8, 150u8, 161u8, 211u8, 177u8, 40u8,
                            189u8, 70u8, 104u8, 51u8, 190u8, 94u8, 171u8, 157u8, 254u8, 120u8,
                            254u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " vector client_id for rpc"]
                pub async fn client_states_keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ClientStatesKeys>()?
                        == [
                            186u8, 115u8, 117u8, 47u8, 245u8, 8u8, 9u8, 61u8, 249u8, 242u8, 110u8,
                            237u8, 220u8, 76u8, 229u8, 45u8, 234u8, 58u8, 129u8, 139u8, 84u8, 71u8,
                            56u8, 124u8, 38u8, 198u8, 209u8, 3u8, 159u8, 197u8, 11u8, 214u8,
                        ]
                    {
                        let entry = ClientStatesKeys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (client_id, height) => timestamp"]
                pub async fn client_processed_times(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ClientProcessedTimes>()?
                        == [
                            217u8, 78u8, 5u8, 80u8, 212u8, 239u8, 150u8, 48u8, 18u8, 200u8, 42u8,
                            80u8, 249u8, 199u8, 99u8, 174u8, 43u8, 226u8, 178u8, 95u8, 127u8,
                            156u8, 217u8, 23u8, 192u8, 200u8, 29u8, 235u8, 8u8, 188u8, 35u8, 179u8,
                        ]
                    {
                        let entry = ClientProcessedTimes(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (client_id, height) => timestamp"]
                pub async fn client_processed_times_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClientProcessedTimes<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ClientProcessedTimes>()?
                        == [
                            217u8, 78u8, 5u8, 80u8, 212u8, 239u8, 150u8, 48u8, 18u8, 200u8, 42u8,
                            80u8, 249u8, 199u8, 99u8, 174u8, 43u8, 226u8, 178u8, 95u8, 127u8,
                            156u8, 217u8, 23u8, 192u8, 200u8, 29u8, 235u8, 8u8, 188u8, 35u8, 179u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (client_id, height) => host_height"]
                pub async fn client_processed_heights(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ClientProcessedHeights>()?
                        == [
                            132u8, 248u8, 6u8, 234u8, 122u8, 247u8, 165u8, 252u8, 28u8, 81u8, 54u8,
                            120u8, 116u8, 201u8, 65u8, 159u8, 212u8, 8u8, 64u8, 215u8, 59u8, 121u8,
                            69u8, 34u8, 59u8, 194u8, 112u8, 30u8, 238u8, 248u8, 115u8, 14u8,
                        ]
                    {
                        let entry = ClientProcessedHeights(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (client_id, height) => host_height"]
                pub async fn client_processed_heights_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClientProcessedHeights<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ClientProcessedHeights>()?
                        == [
                            132u8, 248u8, 6u8, 234u8, 122u8, 247u8, 165u8, 252u8, 28u8, 81u8, 54u8,
                            120u8, 116u8, 201u8, 65u8, 159u8, 212u8, 8u8, 64u8, 215u8, 59u8, 121u8,
                            69u8, 34u8, 59u8, 194u8, 112u8, 30u8, 238u8, 248u8, 115u8, 14u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " ClientConsensusStatePath(client_id, Height) => ConsensusState"]
                pub async fn consensus_states(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ConsensusStates>()?
                        == [
                            249u8, 84u8, 86u8, 26u8, 68u8, 70u8, 245u8, 236u8, 213u8, 72u8, 162u8,
                            47u8, 13u8, 158u8, 147u8, 129u8, 241u8, 182u8, 52u8, 149u8, 156u8,
                            241u8, 212u8, 252u8, 29u8, 127u8, 184u8, 60u8, 228u8, 138u8, 103u8,
                            221u8,
                        ]
                    {
                        let entry = ConsensusStates(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " ClientConsensusStatePath(client_id, Height) => ConsensusState"]
                pub async fn consensus_states_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConsensusStates<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ConsensusStates>()?
                        == [
                            249u8, 84u8, 86u8, 26u8, 68u8, 70u8, 245u8, 236u8, 213u8, 72u8, 162u8,
                            47u8, 13u8, 158u8, 147u8, 129u8, 241u8, 182u8, 52u8, 149u8, 156u8,
                            241u8, 212u8, 252u8, 29u8, 127u8, 184u8, 60u8, 228u8, 138u8, 103u8,
                            221u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " vector (client_id, height) for rpc"]
                pub async fn consensus_states_keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ConsensusStatesKeys>()?
                        == [
                            216u8, 230u8, 234u8, 85u8, 201u8, 228u8, 76u8, 236u8, 142u8, 53u8,
                            28u8, 101u8, 245u8, 84u8, 33u8, 171u8, 187u8, 92u8, 28u8, 106u8, 43u8,
                            212u8, 58u8, 64u8, 177u8, 244u8, 241u8, 32u8, 96u8, 42u8, 18u8, 115u8,
                        ]
                    {
                        let entry = ConsensusStatesKeys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " connection_id => ConnectionEnd"]
                #[doc = " Need ConnectionsPath"]
                pub async fn connections(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Connections>()?
                        == [
                            37u8, 64u8, 201u8, 194u8, 200u8, 243u8, 164u8, 32u8, 192u8, 132u8,
                            162u8, 108u8, 130u8, 185u8, 100u8, 253u8, 190u8, 135u8, 162u8, 24u8,
                            69u8, 214u8, 50u8, 186u8, 139u8, 178u8, 132u8, 250u8, 230u8, 252u8,
                            225u8, 209u8,
                        ]
                    {
                        let entry = Connections(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " connection_id => ConnectionEnd"]
                #[doc = " Need ConnectionsPath"]
                pub async fn connections_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Connections<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Connections>()?
                        == [
                            37u8, 64u8, 201u8, 194u8, 200u8, 243u8, 164u8, 32u8, 192u8, 132u8,
                            162u8, 108u8, 130u8, 185u8, 100u8, 253u8, 190u8, 135u8, 162u8, 24u8,
                            69u8, 214u8, 50u8, 186u8, 139u8, 178u8, 132u8, 250u8, 230u8, 252u8,
                            225u8, 209u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " vector connection id for rpc"]
                pub async fn connections_keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ConnectionsKeys>()?
                        == [
                            236u8, 255u8, 13u8, 166u8, 147u8, 14u8, 61u8, 139u8, 94u8, 219u8,
                            235u8, 218u8, 31u8, 242u8, 202u8, 174u8, 22u8, 144u8, 174u8, 16u8,
                            144u8, 66u8, 243u8, 12u8, 34u8, 140u8, 242u8, 97u8, 236u8, 52u8, 105u8,
                            169u8,
                        ]
                    {
                        let entry = ConnectionsKeys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_identifier, channel_identifier) => ChannelEnd"]
                #[doc = " Need ChannelEndPath"]
                pub async fn channels(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Channels>()?
                        == [
                            132u8, 125u8, 252u8, 86u8, 67u8, 210u8, 214u8, 68u8, 117u8, 174u8,
                            15u8, 92u8, 78u8, 54u8, 84u8, 180u8, 38u8, 63u8, 63u8, 157u8, 40u8,
                            118u8, 53u8, 100u8, 155u8, 68u8, 117u8, 246u8, 215u8, 200u8, 109u8,
                            96u8,
                        ]
                    {
                        let entry = Channels(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_identifier, channel_identifier) => ChannelEnd"]
                #[doc = " Need ChannelEndPath"]
                pub async fn channels_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Channels<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Channels>()?
                        == [
                            132u8, 125u8, 252u8, 86u8, 67u8, 210u8, 214u8, 68u8, 117u8, 174u8,
                            15u8, 92u8, 78u8, 54u8, 84u8, 180u8, 38u8, 63u8, 63u8, 157u8, 40u8,
                            118u8, 53u8, 100u8, 155u8, 68u8, 117u8, 246u8, 215u8, 200u8, 109u8,
                            96u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " vector of (port id, channel id) for rpc"]
                pub async fn channels_keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ChannelsKeys>()?
                        == [
                            165u8, 91u8, 121u8, 209u8, 83u8, 253u8, 157u8, 207u8, 194u8, 152u8,
                            190u8, 144u8, 109u8, 196u8, 200u8, 6u8, 148u8, 163u8, 80u8, 61u8, 14u8,
                            216u8, 95u8, 56u8, 140u8, 48u8, 209u8, 201u8, 189u8, 228u8, 192u8,
                            159u8,
                        ]
                    {
                        let entry = ChannelsKeys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " connection_id => Vec<(port_id, channel_id)>"]
                #[doc = " Need ConnectionsPath <-> ChannelEndPath"]
                pub async fn channels_connection(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ChannelsConnection>()?
                        == [
                            175u8, 74u8, 214u8, 39u8, 82u8, 72u8, 28u8, 110u8, 105u8, 136u8, 218u8,
                            218u8, 110u8, 111u8, 182u8, 21u8, 180u8, 80u8, 66u8, 44u8, 85u8, 138u8,
                            56u8, 102u8, 121u8, 201u8, 111u8, 240u8, 73u8, 7u8, 8u8, 115u8,
                        ]
                    {
                        let entry = ChannelsConnection(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " connection_id => Vec<(port_id, channel_id)>"]
                #[doc = " Need ConnectionsPath <-> ChannelEndPath"]
                pub async fn channels_connection_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ChannelsConnection<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<ChannelsConnection>()?
                        == [
                            175u8, 74u8, 214u8, 39u8, 82u8, 72u8, 28u8, 110u8, 105u8, 136u8, 218u8,
                            218u8, 110u8, 111u8, 182u8, 21u8, 180u8, 80u8, 66u8, 44u8, 85u8, 138u8,
                            56u8, 102u8, 121u8, 201u8, 111u8, 240u8, 73u8, 7u8, 8u8, 115u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id) => sequence"]
                #[doc = " Maybe Need SeqSendsPath"]
                pub async fn next_sequence_send(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<NextSequenceSend>()?
                        == [
                            235u8, 6u8, 179u8, 32u8, 206u8, 228u8, 221u8, 192u8, 182u8, 77u8,
                            147u8, 61u8, 1u8, 18u8, 211u8, 30u8, 212u8, 95u8, 17u8, 21u8, 110u8,
                            247u8, 249u8, 27u8, 255u8, 35u8, 199u8, 66u8, 151u8, 35u8, 38u8, 89u8,
                        ]
                    {
                        let entry = NextSequenceSend(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id) => sequence"]
                #[doc = " Maybe Need SeqSendsPath"]
                pub async fn next_sequence_send_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextSequenceSend<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextSequenceSend>()?
                        == [
                            235u8, 6u8, 179u8, 32u8, 206u8, 228u8, 221u8, 192u8, 182u8, 77u8,
                            147u8, 61u8, 1u8, 18u8, 211u8, 30u8, 212u8, 95u8, 17u8, 21u8, 110u8,
                            247u8, 249u8, 27u8, 255u8, 35u8, 199u8, 66u8, 151u8, 35u8, 38u8, 89u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id) => sequence"]
                #[doc = " Need SeqRecvsPath"]
                pub async fn next_sequence_recv(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<NextSequenceRecv>()?
                        == [
                            80u8, 41u8, 118u8, 188u8, 162u8, 6u8, 96u8, 13u8, 225u8, 7u8, 106u8,
                            250u8, 6u8, 88u8, 54u8, 179u8, 0u8, 177u8, 51u8, 106u8, 108u8, 215u8,
                            168u8, 112u8, 71u8, 116u8, 8u8, 206u8, 5u8, 229u8, 9u8, 96u8,
                        ]
                    {
                        let entry = NextSequenceRecv(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id) => sequence"]
                #[doc = " Need SeqRecvsPath"]
                pub async fn next_sequence_recv_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextSequenceRecv<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextSequenceRecv>()?
                        == [
                            80u8, 41u8, 118u8, 188u8, 162u8, 6u8, 96u8, 13u8, 225u8, 7u8, 106u8,
                            250u8, 6u8, 88u8, 54u8, 179u8, 0u8, 177u8, 51u8, 106u8, 108u8, 215u8,
                            168u8, 112u8, 71u8, 116u8, 8u8, 206u8, 5u8, 229u8, 9u8, 96u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id) => sequence"]
                #[doc = " Maybe Ned SeqAcksPath"]
                pub async fn next_sequence_ack(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<NextSequenceAck>()?
                        == [
                            147u8, 124u8, 181u8, 38u8, 195u8, 193u8, 149u8, 254u8, 197u8, 131u8,
                            190u8, 11u8, 205u8, 208u8, 18u8, 157u8, 211u8, 200u8, 166u8, 38u8,
                            15u8, 178u8, 151u8, 102u8, 207u8, 91u8, 254u8, 191u8, 182u8, 106u8,
                            30u8, 86u8,
                        ]
                    {
                        let entry = NextSequenceAck(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id) => sequence"]
                #[doc = " Maybe Ned SeqAcksPath"]
                pub async fn next_sequence_ack_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextSequenceAck<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextSequenceAck>()?
                        == [
                            147u8, 124u8, 181u8, 38u8, 195u8, 193u8, 149u8, 254u8, 197u8, 131u8,
                            190u8, 11u8, 205u8, 208u8, 18u8, 157u8, 211u8, 200u8, 166u8, 38u8,
                            15u8, 178u8, 151u8, 102u8, 207u8, 91u8, 254u8, 191u8, 182u8, 106u8,
                            30u8, 86u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => hash of acknowledgement"]
                #[doc = " Need AcksPath"]
                pub async fn acknowledgements(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Acknowledgements>()?
                        == [
                            1u8, 123u8, 213u8, 4u8, 209u8, 19u8, 159u8, 193u8, 82u8, 81u8, 140u8,
                            223u8, 230u8, 175u8, 54u8, 169u8, 119u8, 163u8, 162u8, 82u8, 144u8,
                            46u8, 169u8, 106u8, 250u8, 204u8, 89u8, 220u8, 36u8, 46u8, 233u8, 1u8,
                        ]
                    {
                        let entry = Acknowledgements(_0, _1, _2);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => hash of acknowledgement"]
                #[doc = " Need AcksPath"]
                pub async fn acknowledgements_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Acknowledgements<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Acknowledgements>()?
                        == [
                            1u8, 123u8, 213u8, 4u8, 209u8, 19u8, 159u8, 193u8, 82u8, 81u8, 140u8,
                            223u8, 230u8, 175u8, 54u8, 169u8, 119u8, 163u8, 162u8, 82u8, 144u8,
                            46u8, 169u8, 106u8, 250u8, 204u8, 89u8, 220u8, 36u8, 46u8, 233u8, 1u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " vector of (port_identifier, channel_identifier, sequence) for rpc"]
                pub async fn acknowledgements_keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<AcknowledgementsKeys>()?
                        == [
                            136u8, 99u8, 130u8, 194u8, 222u8, 150u8, 83u8, 22u8, 125u8, 123u8,
                            24u8, 247u8, 123u8, 252u8, 247u8, 106u8, 60u8, 251u8, 26u8, 41u8, 32u8,
                            98u8, 192u8, 234u8, 225u8, 122u8, 224u8, 232u8, 248u8, 121u8, 159u8,
                            42u8,
                        ]
                    {
                        let entry = AcknowledgementsKeys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " ClientTypePath(client_id) => client_type"]
                pub async fn clients(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Clients>()?
                        == [
                            239u8, 211u8, 78u8, 91u8, 26u8, 160u8, 9u8, 221u8, 209u8, 43u8, 118u8,
                            199u8, 130u8, 221u8, 246u8, 23u8, 153u8, 204u8, 137u8, 253u8, 108u8,
                            38u8, 149u8, 191u8, 248u8, 65u8, 239u8, 43u8, 133u8, 6u8, 153u8, 234u8,
                        ]
                    {
                        let entry = Clients(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " ClientTypePath(client_id) => client_type"]
                pub async fn clients_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Clients<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Clients>()?
                        == [
                            239u8, 211u8, 78u8, 91u8, 26u8, 160u8, 9u8, 221u8, 209u8, 43u8, 118u8,
                            199u8, 130u8, 221u8, 246u8, 23u8, 153u8, 204u8, 137u8, 253u8, 108u8,
                            38u8, 149u8, 191u8, 248u8, 65u8, 239u8, 43u8, 133u8, 6u8, 153u8, 234u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " client counter"]
                pub async fn client_counter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ClientCounter>()?
                        == [
                            1u8, 150u8, 194u8, 56u8, 39u8, 130u8, 126u8, 87u8, 194u8, 216u8, 27u8,
                            64u8, 125u8, 9u8, 89u8, 203u8, 105u8, 87u8, 27u8, 160u8, 235u8, 137u8,
                            164u8, 201u8, 8u8, 147u8, 164u8, 123u8, 247u8, 14u8, 190u8, 12u8,
                        ]
                    {
                        let entry = ClientCounter;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " connection counter"]
                pub async fn connection_counter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ConnectionCounter>()?
                        == [
                            118u8, 133u8, 237u8, 98u8, 216u8, 34u8, 98u8, 106u8, 121u8, 19u8, 26u8,
                            46u8, 199u8, 28u8, 6u8, 100u8, 34u8, 176u8, 78u8, 81u8, 114u8, 75u8,
                            19u8, 41u8, 148u8, 84u8, 130u8, 223u8, 220u8, 211u8, 5u8, 114u8,
                        ]
                    {
                        let entry = ConnectionCounter;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " channel counter"]
                pub async fn channel_counter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ChannelCounter>()?
                        == [
                            14u8, 251u8, 41u8, 5u8, 204u8, 203u8, 45u8, 151u8, 66u8, 199u8, 1u8,
                            166u8, 123u8, 240u8, 123u8, 121u8, 19u8, 159u8, 131u8, 59u8, 13u8,
                            12u8, 52u8, 26u8, 7u8, 110u8, 137u8, 200u8, 4u8, 234u8, 96u8, 143u8,
                        ]
                    {
                        let entry = ChannelCounter;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " client_id => connection_id"]
                #[doc = " ClientsStatePath <-> ConnectionEndPath"]
                pub async fn connection_client(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ConnectionClient>()?
                        == [
                            97u8, 22u8, 25u8, 124u8, 71u8, 112u8, 42u8, 26u8, 50u8, 121u8, 187u8,
                            234u8, 234u8, 220u8, 6u8, 206u8, 83u8, 51u8, 87u8, 125u8, 65u8, 230u8,
                            61u8, 17u8, 126u8, 142u8, 200u8, 243u8, 103u8, 163u8, 105u8, 26u8,
                        ]
                    {
                        let entry = ConnectionClient(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " client_id => connection_id"]
                #[doc = " ClientsStatePath <-> ConnectionEndPath"]
                pub async fn connection_client_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConnectionClient<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ConnectionClient>()?
                        == [
                            97u8, 22u8, 25u8, 124u8, 71u8, 112u8, 42u8, 26u8, 50u8, 121u8, 187u8,
                            234u8, 234u8, 220u8, 6u8, 206u8, 83u8, 51u8, 87u8, 125u8, 65u8, 230u8,
                            61u8, 17u8, 126u8, 142u8, 200u8, 243u8, 103u8, 163u8, 105u8, 26u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => receipt"]
                #[doc = " Need ReceiptsPath"]
                pub async fn packet_receipt(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<PacketReceipt>()?
                        == [
                            12u8, 200u8, 119u8, 241u8, 2u8, 190u8, 224u8, 111u8, 57u8, 191u8, 44u8,
                            104u8, 128u8, 206u8, 163u8, 98u8, 77u8, 107u8, 122u8, 53u8, 130u8,
                            205u8, 69u8, 108u8, 66u8, 145u8, 255u8, 80u8, 191u8, 220u8, 143u8,
                            194u8,
                        ]
                    {
                        let entry = PacketReceipt(_0, _1, _2);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => receipt"]
                #[doc = " Need ReceiptsPath"]
                pub async fn packet_receipt_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PacketReceipt<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<PacketReceipt>()?
                        == [
                            12u8, 200u8, 119u8, 241u8, 2u8, 190u8, 224u8, 111u8, 57u8, 191u8, 44u8,
                            104u8, 128u8, 206u8, 163u8, 98u8, 77u8, 107u8, 122u8, 53u8, 130u8,
                            205u8, 69u8, 108u8, 66u8, 145u8, 255u8, 80u8, 191u8, 220u8, 143u8,
                            194u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => hash of (timestamp, height, packet)"]
                #[doc = " Need CommitmentsPath"]
                pub async fn packet_commitment(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<PacketCommitment>()?
                        == [
                            106u8, 237u8, 231u8, 50u8, 205u8, 153u8, 220u8, 147u8, 20u8, 253u8,
                            209u8, 96u8, 212u8, 30u8, 123u8, 192u8, 224u8, 213u8, 127u8, 37u8,
                            169u8, 222u8, 33u8, 201u8, 52u8, 7u8, 231u8, 76u8, 71u8, 119u8, 34u8,
                            96u8,
                        ]
                    {
                        let entry = PacketCommitment(_0, _1, _2);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => hash of (timestamp, height, packet)"]
                #[doc = " Need CommitmentsPath"]
                pub async fn packet_commitment_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PacketCommitment<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<PacketCommitment>()?
                        == [
                            106u8, 237u8, 231u8, 50u8, 205u8, 153u8, 220u8, 147u8, 20u8, 253u8,
                            209u8, 96u8, 212u8, 30u8, 123u8, 192u8, 224u8, 213u8, 127u8, 37u8,
                            169u8, 222u8, 33u8, 201u8, 52u8, 7u8, 231u8, 76u8, 71u8, 119u8, 34u8,
                            96u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " vector of (port_id, channel_id, sequence) for rpc"]
                pub async fn packet_commitment_keys(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<PacketCommitmentKeys>()?
                        == [
                            78u8, 154u8, 86u8, 207u8, 4u8, 156u8, 201u8, 146u8, 12u8, 22u8, 231u8,
                            206u8, 71u8, 205u8, 39u8, 232u8, 165u8, 214u8, 154u8, 96u8, 99u8, 9u8,
                            183u8, 25u8, 49u8, 43u8, 34u8, 238u8, 54u8, 198u8, 248u8, 178u8,
                        ]
                    {
                        let entry = PacketCommitmentKeys;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (height, port_id, channel_id, sequence) => sendpacket event"]
                pub async fn send_packet_event(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<SendPacketEvent>()?
                        == [
                            88u8, 140u8, 15u8, 238u8, 88u8, 79u8, 47u8, 111u8, 163u8, 158u8, 181u8,
                            65u8, 135u8, 60u8, 13u8, 96u8, 80u8, 215u8, 62u8, 97u8, 235u8, 236u8,
                            105u8, 54u8, 201u8, 233u8, 22u8, 34u8, 168u8, 132u8, 217u8, 58u8,
                        ]
                    {
                        let entry = SendPacketEvent(_0, _1, _2);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (height, port_id, channel_id, sequence) => sendpacket event"]
                pub async fn send_packet_event_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SendPacketEvent<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<SendPacketEvent>()?
                        == [
                            88u8, 140u8, 15u8, 238u8, 88u8, 79u8, 47u8, 111u8, 163u8, 158u8, 181u8,
                            65u8, 135u8, 60u8, 13u8, 96u8, 80u8, 215u8, 62u8, 97u8, 235u8, 236u8,
                            105u8, 54u8, 201u8, 233u8, 22u8, 34u8, 168u8, 132u8, 217u8, 58u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => writeack event"]
                pub async fn write_ack_packet_event(
                    &self,
                    _0: &[::core::primitive::u8],
                    _1: &[::core::primitive::u8],
                    _2: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<WriteAckPacketEvent>()?
                        == [
                            107u8, 75u8, 137u8, 252u8, 105u8, 139u8, 38u8, 13u8, 26u8, 206u8, 92u8,
                            82u8, 227u8, 107u8, 43u8, 222u8, 107u8, 189u8, 119u8, 142u8, 23u8,
                            95u8, 68u8, 148u8, 207u8, 174u8, 127u8, 231u8, 13u8, 0u8, 16u8, 204u8,
                        ]
                    {
                        let entry = WriteAckPacketEvent(_0, _1, _2);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " (port_id, channel_id, sequence) => writeack event"]
                pub async fn write_ack_packet_event_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, WriteAckPacketEvent<'a>>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<WriteAckPacketEvent>()?
                        == [
                            107u8, 75u8, 137u8, 252u8, 105u8, 139u8, 38u8, 13u8, 26u8, 206u8, 92u8,
                            82u8, 227u8, 107u8, 43u8, 222u8, 107u8, 189u8, 119u8, 142u8, 23u8,
                            95u8, 68u8, 148u8, 207u8, 174u8, 127u8, 231u8, 13u8, 0u8, 16u8, 204u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " store latest height"]
                pub async fn latest_height(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<LatestHeight>()?
                        == [
                            207u8, 46u8, 16u8, 118u8, 150u8, 32u8, 84u8, 137u8, 81u8, 241u8, 168u8,
                            46u8, 77u8, 118u8, 236u8, 6u8, 146u8, 77u8, 38u8, 48u8, 238u8, 120u8,
                            17u8, 22u8, 81u8, 23u8, 142u8, 1u8, 197u8, 240u8, 1u8, 216u8,
                        ]
                    {
                        let entry = LatestHeight;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn old_height(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<OldHeight>()?
                        == [
                            99u8, 216u8, 180u8, 203u8, 123u8, 207u8, 2u8, 234u8, 253u8, 100u8,
                            107u8, 176u8, 159u8, 168u8, 234u8, 220u8, 35u8, 202u8, 234u8, 27u8,
                            60u8, 90u8, 197u8, 146u8, 82u8, 214u8, 129u8, 56u8, 8u8, 154u8, 250u8,
                            181u8,
                        ]
                    {
                        let entry = OldHeight;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " sha256(tracePath + \"/\" + baseDenom) => DenomTrace"]
                pub async fn denomination(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Denomination>()?
                        == [
                            249u8, 85u8, 198u8, 110u8, 211u8, 39u8, 28u8, 25u8, 188u8, 232u8,
                            102u8, 243u8, 2u8, 164u8, 179u8, 2u8, 7u8, 51u8, 65u8, 74u8, 251u8,
                            101u8, 20u8, 241u8, 146u8, 240u8, 107u8, 57u8, 21u8, 218u8, 50u8,
                            121u8,
                        ]
                    {
                        let entry = Denomination(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " sha256(tracePath + \"/\" + baseDenom) => DenomTrace"]
                pub async fn denomination_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Denomination<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Denomination>()?
                        == [
                            249u8, 85u8, 198u8, 110u8, 211u8, 39u8, 28u8, 25u8, 188u8, 232u8,
                            102u8, 243u8, 2u8, 164u8, 179u8, 2u8, 7u8, 51u8, 65u8, 74u8, 251u8,
                            101u8, 20u8, 241u8, 146u8, 240u8, 107u8, 57u8, 21u8, 218u8, 50u8,
                            121u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn escrow_addresses(
                    &self,
                    _0: &runtime_types::pallet_ibc::event::primitive::PortId,
                    _1: &runtime_types::pallet_ibc::event::primitive::ChannelId,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EscrowAddresses>()?
                        == [
                            216u8, 119u8, 214u8, 39u8, 219u8, 181u8, 163u8, 17u8, 80u8, 98u8, 70u8,
                            251u8, 227u8, 190u8, 167u8, 251u8, 13u8, 209u8, 214u8, 156u8, 169u8,
                            159u8, 125u8, 251u8, 172u8, 59u8, 30u8, 148u8, 164u8, 2u8, 113u8, 70u8,
                        ]
                    {
                        let entry = EscrowAddresses(_0, _1);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn escrow_addresses_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EscrowAddresses<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EscrowAddresses>()?
                        == [
                            216u8, 119u8, 214u8, 39u8, 219u8, 181u8, 163u8, 17u8, 80u8, 98u8, 70u8,
                            251u8, 227u8, 190u8, 167u8, 251u8, 13u8, 209u8, 214u8, 156u8, 169u8,
                            159u8, 125u8, 251u8, 172u8, 59u8, 30u8, 148u8, 164u8, 2u8, 113u8, 70u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " key-value asserid with asset name"]
                pub async fn asset_id_by_name(
                    &self,
                    _0: &[::core::primitive::u8],
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<AssetIdByName>()?
                        == [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ]
                    {
                        let entry = AssetIdByName(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " key-value asserid with asset name"]
                pub async fn asset_id_by_name_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIdByName<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<AssetIdByName>()?
                        == [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
            pub mod mmr {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BeefyNextAuthoritySet<_0> {
                    pub id: ::core::primitive::u64,
                    pub len: ::core::primitive::u32,
                    pub root: _0,
                }
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                            :: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug,
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`"]
                    #[doc = "# </weight>"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                    #[doc = "  expensive)."]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                    #[doc = "expensive. We will treat this as a full block."]
                    #[doc = "# </weight>"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                    #[doc = "block. # </weight>"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        hash: ::subxt::sp_core::H256,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct SessionKeys {
                    pub babe: runtime_types::sp_consensus_babe::app::Public,
                    pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                    pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    pub beefy: runtime_types::beefy_primitives::crypto::Public,
                    pub octopus: runtime_types::pallet_octopus_appchain::crypto::Public,
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 4)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 7)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Call),
                #[codec(index = 8)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Call),
                #[codec(index = 9)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Call),
                #[codec(index = 10)]
                OctopusAssets(runtime_types::pallet_assets::pallet::Call),
                #[codec(index = 11)]
                OctopusUniques(runtime_types::pallet_uniques::pallet::Call),
                #[codec(index = 12)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 13)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 14)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 17)]
                Beefy(runtime_types::pallet_beefy::pallet::Call),
                #[codec(index = 19)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 20)]
                TemplateModule(runtime_types::pallet_template::pallet::Call),
                #[codec(index = 21)]
                Ibc(runtime_types::pallet_ibc::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 7)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Event),
                #[codec(index = 8)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Event),
                #[codec(index = 9)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Event),
                #[codec(index = 10)]
                OctopusAssets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 11)]
                OctopusUniques(runtime_types::pallet_uniques::pallet::Event),
                #[codec(index = 12)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 13)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 14)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 19)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 20)]
                TemplateModule(runtime_types::pallet_template::pallet::Event),
                #[codec(index = 21)]
                Ibc(runtime_types::pallet_ibc::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Runtime;
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Issue a new class of fungible assets from a public origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially and its owner is the origin."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and the sender must have sufficient funds free."]
                    #[doc = ""]
                    #[doc = "Funds of sender are reserved by `AssetDeposit`."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                    #[doc = "an existing asset."]
                    #[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
                    #[doc = "member of the asset class's admin team."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = ""]
                    #[doc = "Emits `Created` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
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
                    #[doc = "Issue a new class of fungible assets from a privileged origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "Unlike `create`, no funds are reserved."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                    #[doc = "an existing asset."]
                    #[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
                    #[doc = "over this asset, but may later change and configure the permissions using"]
                    #[doc = "`transfer_ownership` and `set_team`."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = ""]
                    #[doc = "Emits `ForceCreated` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
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
                    #[doc = "Destroy a class of fungible assets."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin` or must be Signed and the sender must be the"]
                    #[doc = "owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "asset."]
                    #[doc = ""]
                    #[doc = "Emits `Destroyed` event when successful."]
                    #[doc = ""]
                    #[doc = "NOTE: It can be helpful to first freeze an asset before destroying it so that you"]
                    #[doc = "can provide accurate witness information and prevent users from manipulating state"]
                    #[doc = "in a way that can make it harder to destroy."]
                    #[doc = ""]
                    #[doc = "Weight: `O(c + p + a)` where:"]
                    #[doc = "- `c = (witness.accounts - witness.sufficients)`"]
                    #[doc = "- `s = witness.sufficients`"]
                    #[doc = "- `a = witness.approvals`"]
                    destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        witness: runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    #[doc = "Mint assets of a particular class."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and the sender must be the Issuer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount minted."]
                    #[doc = "- `beneficiary`: The account to be credited with the minted assets."]
                    #[doc = "- `amount`: The amount of the asset to be minted."]
                    #[doc = ""]
                    #[doc = "Emits `Issued` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existing balance of `beneficiary`; Account pre-existence of `beneficiary`."]
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
                    #[doc = "Reduce the balance of `who` by as much as possible up to `amount` assets of `id`."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Manager of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Bails with `NoAccount` if the `who` is already dead."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount burned."]
                    #[doc = "- `who`: The account to be debited from."]
                    #[doc = "- `amount`: The maximum amount by which `who`'s balance should be reduced."]
                    #[doc = ""]
                    #[doc = "Emits `Burned` with the actual amount burned. If this takes the balance to below the"]
                    #[doc = "minimum for the asset, then the amount burned is increased to take it to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Post-existence of `who`; Pre & post Zombie-status of `who`."]
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
                    #[doc = "Move some assets from the sender account to another."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `target`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                    #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                    #[doc = "the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                    #[doc = "`target`."]
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
                    #[doc = "Move some assets from the sender account to another, keeping the sender account alive."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `target`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                    #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                    #[doc = "the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                    #[doc = "`target`."]
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
                    #[doc = "Move some assets from one account to another."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `source`: The account to be debited."]
                    #[doc = "- `dest`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the `source`'s balance of assets should be reduced and"]
                    #[doc = "`dest`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the `source` balance above zero but"]
                    #[doc = "below the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `dest`; Post-existence of `source`; Account pre-existence of"]
                    #[doc = "`dest`."]
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
                    #[doc = "Disallow further unprivileged transfers from an account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `who`: The account to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 9)]
                    #[doc = "Allow unprivileged transfers from an account again."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `who`: The account to be unfrozen."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 10)]
                    #[doc = "Disallow further unprivileged transfers for the asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    #[doc = "Allow unprivileged transfers for the asset again."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be thawed."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    #[doc = "Change the Owner of an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The new Owner of this asset."]
                    #[doc = ""]
                    #[doc = "Emits `OwnerChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    #[doc = "Change the Issuer, Admin and Freezer of an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `issuer`: The new Issuer of this asset."]
                    #[doc = "- `admin`: The new Admin of this asset."]
                    #[doc = "- `freezer`: The new Freezer of this asset."]
                    #[doc = ""]
                    #[doc = "Emits `TeamChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
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
                    #[doc = "Set the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Funds of sender are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + MetadataDepositPerByte * (name.len + symbol.len)` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to update."]
                    #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 15)]
                    #[doc = "Clear the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the asset owner."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    #[doc = "Force the metadata for an asset to some value."]
                    #[doc = ""]
                    #[doc = "Origin must be ForceOrigin."]
                    #[doc = ""]
                    #[doc = "Any deposit is left alone."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to update."]
                    #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(N + S)` where N and S are the length of the name and symbol respectively."]
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 17)]
                    #[doc = "Clear the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be ForceOrigin."]
                    #[doc = ""]
                    #[doc = "Any deposit is returned."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 18)]
                    #[doc = "Alter the attributes of a given asset."]
                    #[doc = ""]
                    #[doc = "Origin must be `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The new Owner of this asset."]
                    #[doc = "- `issuer`: The new Issuer of this asset."]
                    #[doc = "- `admin`: The new Admin of this asset."]
                    #[doc = "- `freezer`: The new Freezer of this asset."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = "- `is_sufficient`: Whether a non-zero balance of this asset is deposit of sufficient"]
                    #[doc = "value to account for the state bloat associated with its balance storage. If set to"]
                    #[doc = "`true`, then non-zero balances may be stored without a `consumer` reference (and thus"]
                    #[doc = "an ED in the Balances pallet or whatever else is used to control user-account state"]
                    #[doc = "growth)."]
                    #[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
                    #[doc = "instructions."]
                    #[doc = ""]
                    #[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
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
                    #[doc = "Approve an amount of asset for transfer by a delegated third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "Ensures that `ApprovalDeposit` worth of `Currency` is reserved from signing account"]
                    #[doc = "for the purpose of holding the approval. If some non-zero amount of assets is already"]
                    #[doc = "approved from signing account to `delegate`, then it is topped up or unreserved to"]
                    #[doc = "meet the right value."]
                    #[doc = ""]
                    #[doc = "NOTE: The signing account does not need to own `amount` of assets at the point of"]
                    #[doc = "making this call."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account to delegate permission to transfer asset."]
                    #[doc = "- `amount`: The amount of asset that may be transferred by `delegate`. If there is"]
                    #[doc = "already an approval in place, then this acts additively."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovedTransfer` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
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
                    #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and there must be an approval in place between signer and"]
                    #[doc = "`delegate`."]
                    #[doc = ""]
                    #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 21)]
                    #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be either ForceOrigin or Signed origin with the signer being the Admin"]
                    #[doc = "account of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
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
                    #[doc = "Transfer some asset balance from a previously delegated account to some third-party"]
                    #[doc = "account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and there must be an approval in place by the `owner` to the"]
                    #[doc = "signer."]
                    #[doc = ""]
                    #[doc = "If the entire amount approved for transfer is transferred, then any deposit previously"]
                    #[doc = "reserved by `approve_transfer` is unreserved."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The account which previously approved for a transfer of at least `amount` and"]
                    #[doc = "from which the asset balance will be withdrawn."]
                    #[doc = "- `destination`: The account to which the asset balance of `amount` will be transferred."]
                    #[doc = "- `amount`: The amount of assets to transfer."]
                    #[doc = ""]
                    #[doc = "Emits `TransferredApproved` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
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
                    #[codec(index = 23)]
                    #[doc = "Create an asset account for non-provider assets."]
                    #[doc = ""]
                    #[doc = "A deposit will be taken from the signer account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Signed; the signer account must have sufficient funds for a deposit"]
                    #[doc = "  to be taken."]
                    #[doc = "- `id`: The identifier of the asset for the account to be created."]
                    #[doc = ""]
                    #[doc = "Emits `Touched` event when successful."]
                    touch {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 24)]
                    #[doc = "Return the deposit (if any) of an asset account."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset for the account to be created."]
                    #[doc = "- `allow_burn`: If `true` then assets may be destroyed in order to complete the refund."]
                    #[doc = ""]
                    #[doc = "Emits `Refunded` event when successful."]
                    refund {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        allow_burn: ::core::primitive::bool,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Account balance must be greater than or equal to the transfer amount."]
                    BalanceLow,
                    #[codec(index = 1)]
                    #[doc = "The account to alter does not exist."]
                    NoAccount,
                    #[codec(index = 2)]
                    #[doc = "The signing account has no permission to do the operation."]
                    NoPermission,
                    #[codec(index = 3)]
                    #[doc = "The given asset ID is unknown."]
                    Unknown,
                    #[codec(index = 4)]
                    #[doc = "The origin account is frozen."]
                    Frozen,
                    #[codec(index = 5)]
                    #[doc = "The asset ID is already taken."]
                    InUse,
                    #[codec(index = 6)]
                    #[doc = "Invalid witness data given."]
                    BadWitness,
                    #[codec(index = 7)]
                    #[doc = "Minimum balance should be non-zero."]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    #[doc = "Unable to increment the consumer reference counters on the account. Either no provider"]
                    #[doc = "reference exists to allow a non-zero balance of a non-self-sufficient asset, or the"]
                    #[doc = "maximum number of consumers has been reached."]
                    NoProvider,
                    #[codec(index = 9)]
                    #[doc = "Invalid metadata given."]
                    BadMetadata,
                    #[codec(index = 10)]
                    #[doc = "No approval exists that would allow the transfer."]
                    Unapproved,
                    #[codec(index = 11)]
                    #[doc = "The source account would not survive the transfer and it needs to stay alive."]
                    WouldDie,
                    #[codec(index = 12)]
                    #[doc = "The asset-account already exists."]
                    AlreadyExists,
                    #[codec(index = 13)]
                    #[doc = "The asset-account doesn't have an associated deposit."]
                    NoDeposit,
                    #[codec(index = 14)]
                    #[doc = "The operation would result in funds being burned."]
                    WouldBurn,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Some asset class was created."]
                    Created {
                        asset_id: ::core::primitive::u32,
                        creator: ::subxt::sp_core::crypto::AccountId32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Some assets were issued."]
                    Issued {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        total_supply: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Some assets were transferred."]
                    Transferred {
                        asset_id: ::core::primitive::u32,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Some assets were destroyed."]
                    Burned {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "The management team changed."]
                    TeamChanged {
                        asset_id: ::core::primitive::u32,
                        issuer: ::subxt::sp_core::crypto::AccountId32,
                        admin: ::subxt::sp_core::crypto::AccountId32,
                        freezer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "The owner changed."]
                    OwnerChanged {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some account `who` was frozen."]
                    Frozen {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some account `who` was thawed."]
                    Thawed {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some asset `asset_id` was frozen."]
                    AssetFrozen { asset_id: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    #[doc = "Some asset `asset_id` was thawed."]
                    AssetThawed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    #[doc = "An asset class was destroyed."]
                    Destroyed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    #[doc = "Some asset class was force-created."]
                    ForceCreated {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 12)]
                    #[doc = "New metadata has been set for an asset."]
                    MetadataSet {
                        asset_id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 13)]
                    #[doc = "Metadata has been cleared for an asset."]
                    MetadataCleared { asset_id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    #[doc = "(Additional) funds have been approved for transfer to a destination account."]
                    ApprovedTransfer {
                        asset_id: ::core::primitive::u32,
                        source: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 15)]
                    #[doc = "An approval for account `delegate` was cancelled by `owner`."]
                    ApprovalCancelled {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 16)]
                    #[doc = "An `amount` was transferred in its entirety from `owner` to `destination` by"]
                    #[doc = "the approved `delegate`."]
                    TransferredApproved {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        destination: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 17)]
                    #[doc = "An asset has had its attributes changed by the `Force` origin."]
                    AssetStatusChanged { asset_id: ::core::primitive::u32 },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct AssetAccount<_0, _1, _2> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0>,
                    pub extra: _2,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub accounts: ::core::primitive::u32,
                    #[codec(compact)]
                    pub sufficients: ::core::primitive::u32,
                    #[codec(compact)]
                    pub approvals: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum ExistenceReason<_0> {
                    #[codec(index = 0)]
                    Consumer,
                    #[codec(index = 1)]
                    Sufficient,
                    #[codec(index = 2)]
                    DepositHeld(_0),
                    #[codec(index = 3)]
                    DepositRefunded,
                }
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Provide a set of uncles."]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The uncle parent not in the chain."]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    #[doc = "Uncles already set in the block."]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    #[doc = "Too many uncles."]
                    TooManyUncles,
                    #[codec(index = 3)]
                    #[doc = "The uncle is genesis."]
                    GenesisUncle,
                    #[codec(index = 4)]
                    #[doc = "The uncle is too high in chain."]
                    TooHighUncle,
                    #[codec(index = 5)]
                    #[doc = "The uncle is already included."]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    #[doc = "The uncle isn't recent enough to be included."]
                    OldUncle,
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report authority equivocation/misbehavior. This method will verify"]
                    #[doc = "the equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence will"]
                    #[doc = "be reported."]
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
                    #[doc = "Report authority equivocation/misbehavior. This method will verify"]
                    #[doc = "the equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence will"]
                    #[doc = "be reported."]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
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
                    #[doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"]
                    #[doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."]
                    #[doc = "Multiple calls to this method will replace any existing planned config change that had"]
                    #[doc = "not been enacted yet."]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                    #[doc = "  types. See related functions below."]
                    #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                    #[doc = "  computation."]
                    #[doc = ""]
                    #[doc = "Related functions:"]
                    #[doc = ""]
                    #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                    #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                    #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                    #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                    #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                    #[doc = "    that the transfer will not kill the origin account."]
                    #[doc = "---------------------------------"]
                    #[doc = "- Origin account is already in memory, so no DB operations for them."]
                    #[doc = "# </weight>"]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                    #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                    #[doc = "If the new free or reserved balance is below the existential deposit,"]
                    #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
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
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                    #[doc = "specified."]
                    #[doc = "# <weight>"]
                    #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                    #[doc = "  assumed to be in the overlay."]
                    #[doc = "# </weight>"]
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
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                    #[doc = "origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true). # <weight>"]
                    #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                    #[doc = "  #</weight>"]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value"]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal"]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value"]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit"]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"]
                    KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account"]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed MaxReserves"]
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_beefy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {}
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
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
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
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
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has"]
                    #[doc = "stalled. This will trigger a forced authority set change at the beginning"]
                    #[doc = "of the next session, to be enacted `delay` blocks after that. The delay"]
                    #[doc = "should be high enough to safely assume that the block signalling the"]
                    #[doc = "forced change will not be re-orged (e.g. 1000 blocks). The GRANDPA voters"]
                    #[doc = "will start the new authority set using the given finalized block as base."]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct ChannelId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct ClientId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub enum ClientType {
                        #[codec(index = 0)]
                        Tendermint,
                        #[codec(index = 1)]
                        Grandpa,
                    }
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct ConnectionId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Height {
                        pub revision_number: ::core::primitive::u64,
                        pub revision_height: ::core::primitive::u64,
                    }
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct PortId(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(
                        :: subxt :: codec :: CompactAs,
                        :: subxt :: codec :: Decode,
                        :: subxt :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Sequence(pub ::core::primitive::u64);
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Timestamp {
                        pub time: ::std::vec::Vec<::core::primitive::u8>,
                    }
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "This function acts as an entry for all of the IBC request(except MMR root update)."]
                    #[doc = "I.e., create clients, update clients, handshakes to create channels, ...etc"]
                    deliver {
                        messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                        tmp: ::core::primitive::u8,
                    },
                    #[codec(index = 1)]
                    #[doc = "Update the MMR root stored in client_state"]
                    #[doc = "Example of invoking this function via subxt"]
                    update_client_state {
                        client_id: ::std::vec::Vec<::core::primitive::u8>,
                        mmr_root: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer interface for user test by explore"]
                    transfer {
                        source_port: ::std::vec::Vec<::core::primitive::u8>,
                        source_channel: ::std::vec::Vec<::core::primitive::u8>,
                        token: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        timeout_height: ::core::primitive::u64,
                        timeout_timestamp: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    delete_send_packet_event,
                    #[codec(index = 4)]
                    delete_ack_packet_event,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "update the beefy light client failure!"]
                    UpdateBeefyLightClientFailure,
                    #[codec(index = 1)]
                    #[doc = "receive mmr root block number less than client_state.latest_commitment.block_number"]
                    ReceiveMmrRootBlockNumberLessThanClientStateLatestCommitmentBlockNumber,
                    #[codec(index = 2)]
                    #[doc = "client id not found"]
                    ClientIdNotFound,
                    #[codec(index = 3)]
                    #[doc = "Encode error"]
                    InvalidEncode,
                    #[codec(index = 4)]
                    #[doc = "Decode Error"]
                    InvalidDecode,
                    #[codec(index = 5)]
                    #[doc = "FromUtf8Error"]
                    InvalidFromUtf8,
                    #[codec(index = 6)]
                    #[doc = "ics26router error"]
                    Ics26Error,
                    #[codec(index = 7)]
                    #[doc = "invalid signer"]
                    InvalidSigner,
                    #[codec(index = 8)]
                    #[doc = "empty channel id"]
                    EmptyChannelId,
                    #[codec(index = 9)]
                    #[doc = "ics20 error"]
                    Ics20Error,
                    #[codec(index = 10)]
                    #[doc = "parse ibc packet error"]
                    InvalidPacket,
                    #[codec(index = 11)]
                    #[doc = "invalid signed_commitment"]
                    InvalidSignedCommitment,
                    #[codec(index = 12)]
                    #[doc = "invalid identifier"]
                    InvalidIdentifier,
                    #[codec(index = 13)]
                    #[doc = "invalid timestamp"]
                    InvalidTimestamp,
                    #[codec(index = 14)]
                    #[doc = "empty latest_commitment"]
                    EmptyLatestCommitment,
                    #[codec(index = 15)]
                    #[doc = "send packet error"]
                    SendPacketError,
                    #[codec(index = 16)]
                    #[doc = "ReceivePacket error"]
                    ReceivePacketError,
                    #[codec(index = 17)]
                    #[doc = "TimeoutPacket error"]
                    TimeoutPacketError,
                    #[codec(index = 18)]
                    #[doc = "AcknowledgePacket error"]
                    AcknowledgePacketError,
                    #[codec(index = 19)]
                    #[doc = "OpenInitChannel error"]
                    OpenInitChannelError,
                    #[codec(index = 20)]
                    #[doc = "OpenTryChannel error"]
                    OpenTryChannelError,
                    #[codec(index = 21)]
                    #[doc = "OpenAckChannel error"]
                    OpenAckChannelError,
                    #[codec(index = 22)]
                    #[doc = "OpenConfirmChannel error"]
                    OpenConfirmChannelError,
                    #[codec(index = 23)]
                    #[doc = "CloseInitChannel error"]
                    CloseInitChannelError,
                    #[codec(index = 24)]
                    #[doc = "CloseConfirmChannel error"]
                    CloseConfirmChannelError,
                    #[codec(index = 25)]
                    #[doc = "AmountOverflow"]
                    AmountOverflow,
                    #[codec(index = 26)]
                    SerdeIBCFungibleTokenPacketDataError,
                    #[codec(index = 27)]
                    #[doc = "Invalid parse"]
                    InvalidParse,
                    #[codec(index = 28)]
                    #[doc = "parse denom trace error"]
                    ParseDenomTraceError,
                    #[codec(index = 29)]
                    #[doc = "acknowledgement_response_empty"]
                    AcknowledgementResponseEmpty,
                    #[codec(index = 30)]
                    #[doc = "Get Ibc denom Error"]
                    GetIbcDenomError,
                    #[codec(index = 31)]
                    #[doc = "invalid_validation"]
                    InvalidValidation,
                    #[codec(index = 32)]
                    #[doc = "store packet result error"]
                    StorePacketResultError,
                    #[codec(index = 33)]
                    #[doc = "invalid token id"]
                    InvalidTokenId,
                    #[codec(index = 34)]
                    #[doc = "wrong assert id"]
                    WrongAssetId,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "emit new block event"]
                    NewBlock(runtime_types::pallet_ibc::event::primitive::Height),
                    #[codec(index = 1)]
                    #[doc = "emit create client event"]
                    CreateClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 2)]
                    #[doc = "emit updte client event"]
                    UpdateClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 3)]
                    #[doc = "emit update client state event"]
                    UpdateClientState(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientState,
                    ),
                    #[codec(index = 4)]
                    #[doc = "emit upgrade client event"]
                    UpgradeClient(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 5)]
                    #[doc = "emit client misbehaviour event"]
                    ClientMisbehaviour(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::ClientId,
                        runtime_types::pallet_ibc::event::primitive::ClientType,
                        runtime_types::pallet_ibc::event::primitive::Height,
                    ),
                    #[codec(index = 6)]
                    #[doc = "emit open init connection event"]
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
                    #[doc = "emit open try connection event"]
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
                    #[doc = "emit open ack connection event"]
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
                    #[doc = "emit open confirm connection event"]
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
                    #[doc = "emit open init channel event"]
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
                    #[doc = "emit open try channel event"]
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
                    #[doc = "emit open ack channel event"]
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
                    #[doc = "emit open confirm channel event"]
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
                    #[doc = "emit close init channel event"]
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
                    #[doc = "emit close confirm channel event"]
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
                    #[doc = "emit send packet event"]
                    SendPacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 17)]
                    #[doc = "emit receive packet"]
                    ReceivePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 18)]
                    #[doc = "emit write acknowledgement packet event"]
                    WriteAcknowledgement(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 19)]
                    #[doc = "emit acknowledgement packet event"]
                    AcknowledgePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 20)]
                    #[doc = "emit timeout packet event"]
                    TimeoutPacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 21)]
                    #[doc = "emit timeout on close packet event"]
                    TimeoutOnClosePacket(
                        runtime_types::pallet_ibc::event::primitive::Height,
                        runtime_types::pallet_ibc::event::primitive::Packet,
                    ),
                    #[codec(index = 22)]
                    #[doc = "emit empty event"]
                    Empty(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 23)]
                    #[doc = "emit chain error event"]
                    ChainError(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 24)]
                    #[doc = "emit escrow token"]
                    EscrowToken(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 25)]
                    #[doc = "emit burn token"]
                    BurnToken(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 26)]
                    #[doc = "unescrow token"]
                    UnEscrowToken(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 27)]
                    #[doc = "mint token"]
                    MintToken(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Any {
                pub type_url: ::std::vec::Vec<::core::primitive::u8>,
                pub value: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"]
                    #[doc = "  length of `heartbeat.network_state.external_address`"]
                    #[doc = "  - `O(K)`: decoding of length `K`"]
                    #[doc = "  - `O(E)`: decoding/encoding of length `E`"]
                    #[doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"]
                    #[doc = "  `ReceivedHeartbeats`"]
                    #[doc = "- DbWrites: `ReceivedHeartbeats`"]
                    #[doc = "# </weight>"]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Non existent public key."]
                    InvalidKey,
                    #[codec(index = 1)]
                    #[doc = "Duplicated heartbeat."]
                    DuplicatedHeartbeat,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new heartbeat was received from `AuthorityId`."]
                    HeartbeatReceived {
                        authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    },
                    #[codec(index = 1)]
                    #[doc = "At the end of the session, no offence was committed."]
                    AllGood,
                    #[codec(index = 2)]
                    #[doc = "At the end of the session, at least one validator was found to be offline."]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Submit observations."]
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
                    #[doc = "Emits `Locked` event when successful."]
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
                    #[codec(index = 7)]
                    set_asset_name {
                        asset_name: ::std::vec::Vec<::core::primitive::u8>,
                        asset_id: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    tranfer_from_pallet_account {
                        receiver: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    lock_nft {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The set id of new validator set was wrong."]
                    WrongSetId,
                    #[codec(index = 1)]
                    #[doc = "Invalid notification id of observation."]
                    InvalidNotificationId,
                    #[codec(index = 2)]
                    #[doc = "Must be a validator."]
                    NotValidator,
                    #[codec(index = 3)]
                    #[doc = "Amount overflow."]
                    AmountOverflow,
                    #[codec(index = 4)]
                    #[doc = "Next notification Id overflow."]
                    NextNotificationIdOverflow,
                    #[codec(index = 5)]
                    #[doc = "Wrong Asset Id."]
                    WrongAssetId,
                    #[codec(index = 6)]
                    #[doc = "Invalid active total stake."]
                    InvalidActiveTotalStake,
                    #[codec(index = 7)]
                    #[doc = "Appchain is not activated."]
                    NotActivated,
                    #[codec(index = 8)]
                    #[doc = "ReceiverId is not a valid utf8 string."]
                    InvalidReceiverId,
                    #[codec(index = 9)]
                    #[doc = "Token is not a valid utf8 string."]
                    InvalidTokenId,
                    #[codec(index = 10)]
                    #[doc = "Next set Id overflow."]
                    NextSetIdOverflow,
                    #[codec(index = 11)]
                    #[doc = "Observations exceeded limit."]
                    ObservationsExceededLimit,
                    #[codec(index = 12)]
                    #[doc = "Asset name has been set."]
                    AssetNameHasSet,
                    #[codec(index = 13)]
                    #[doc = "Asset id in use."]
                    AssetIdInUse,
                    #[codec(index = 14)]
                    #[doc = "Not implement nep171 convertor."]
                    ConvertorNotImplement,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new set of validators is waiting to be changed."]
                    NewPlannedValidators {
                        set_id: ::core::primitive::u32,
                        validators: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "An `amount` of native token has been locked in the appchain to indicate that"]
                    #[doc = "it will be cross-chain transferred to the mainchain."]
                    Locked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                        sequence: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "An `amount` was unlocked to `receiver` from `sender`."]
                    Unlocked {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "An `amount` unlock to `receiver` from `sender` failed."]
                    UnlockFailed {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    AssetMinted {
                        asset_id: ::core::primitive::u32,
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    AssetBurned {
                        asset_id: ::core::primitive::u32,
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                        sequence: ::core::primitive::u64,
                    },
                    #[codec(index = 6)]
                    AssetMintFailed {
                        asset_id: ::core::primitive::u32,
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    AssetIdGetFailed {
                        token_id: ::std::vec::Vec<::core::primitive::u8>,
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    TransferredFromPallet {
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    NftLocked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        sequence: ::core::primitive::u64,
                    },
                    #[codec(index = 10)]
                    NftUnlocked {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    NftUnlockFailed {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_core::crypto::AccountId32,
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct BurnEvent<_0> {
                pub index: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct BurnNftEvent<_0> {
                pub index: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct LockAssetEvent<_0> {
                pub index: ::core::primitive::u32,
                pub token_id: ::std::vec::Vec<::core::primitive::u8>,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum NotificationResult {
                #[codec(index = 0)]
                Success,
                #[codec(index = 1)]
                UnlockFailed,
                #[codec(index = 2)]
                AssetMintFailed,
                #[codec(index = 3)]
                AssetGetFailed,
                #[codec(index = 4)]
                NftUnlockFailed,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Observation<_0> {
                #[codec(index = 0)]
                UpdateValidatorSet(runtime_types::pallet_octopus_appchain::ValidatorSet<_0>),
                #[codec(index = 1)]
                LockAsset(runtime_types::pallet_octopus_appchain::LockAssetEvent<_0>),
                #[codec(index = 2)]
                Burn(runtime_types::pallet_octopus_appchain::BurnEvent<_0>),
                #[codec(index = 3)]
                BurnNft(runtime_types::pallet_octopus_appchain::BurnNftEvent<_0>),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum ObservationType {
                #[codec(index = 0)]
                UpdateValidatorSet,
                #[codec(index = 1)]
                Burn,
                #[codec(index = 2)]
                LockAsset,
                #[codec(index = 3)]
                BurnNft,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ObservationsPayload<_0, _1, _2> {
                pub public: _0,
                pub block_number: _1,
                pub observations:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Observation<_2>>,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Validator<_0> {
                pub validator_id_in_appchain: _0,
                pub total_stake: ::core::primitive::u128,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set `HistoryDepth` value. This function will delete any history information"]
                    #[doc = "when `HistoryDepth` is reduced."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `new_history_depth`: The new history depth you would like to set."]
                    #[doc = "- `era_items_deleted`: The number of items that will be deleted by this dispatch. This"]
                    #[doc = "  should report all the storage items that will be deleted by clearing old era history."]
                    #[doc = "  Needed to report an accurate weight for the dispatch. Trusted by `Root` to report an"]
                    #[doc = "  accurate number."]
                    #[doc = ""]
                    #[doc = "Origin must be root."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- E: Number of history depths removed, i.e. 10 -> 7 = 3"]
                    #[doc = "- Weight: O(E)"]
                    #[doc = "- DB Weight:"]
                    #[doc = "    - Reads: Current Era, History Depth"]
                    #[doc = "    - Writes: History Depth"]
                    #[doc = "    - Clear Prefix Each: Era Stakers, EraStakersClipped, ErasValidatorPrefs"]
                    #[doc = "    - Writes Each: ErasValidatorReward, ErasRewardPoints, ErasTotalStake,"]
                    #[doc = "      ErasStartSessionIndex"]
                    #[doc = "# </weight>"]
                    set_history_depth {
                        #[codec(compact)]
                        new_history_depth: ::core::primitive::u32,
                        #[codec(compact)]
                        era_items_deleted: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    force_set_era_payout { era_payout: ::core::primitive::u128 },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Not a controller account."]
                    NotController,
                    #[codec(index = 1)]
                    #[doc = "Not a stash account."]
                    NotStash,
                    #[codec(index = 2)]
                    #[doc = "Stash is already bonded."]
                    AlreadyBonded,
                    #[codec(index = 3)]
                    #[doc = "Controller is already paired."]
                    AlreadyPaired,
                    #[codec(index = 4)]
                    #[doc = "Targets cannot be empty."]
                    EmptyTargets,
                    #[codec(index = 5)]
                    #[doc = "Duplicate index."]
                    DuplicateIndex,
                    #[codec(index = 6)]
                    #[doc = "Slash record index out of bounds."]
                    InvalidSlashIndex,
                    #[codec(index = 7)]
                    #[doc = "Can not bond with value less than minimum required."]
                    InsufficientBond,
                    #[codec(index = 8)]
                    #[doc = "Can not schedule more unlock chunks."]
                    NoMoreChunks,
                    #[codec(index = 9)]
                    #[doc = "Can not rebond without unlocking chunks."]
                    NoUnlockChunk,
                    #[codec(index = 10)]
                    #[doc = "Attempting to target a stash that still has funds."]
                    FundedTarget,
                    #[codec(index = 11)]
                    #[doc = "Invalid era to reward."]
                    InvalidEraToReward,
                    #[codec(index = 12)]
                    #[doc = "Invalid number of nominations."]
                    InvalidNumberOfNominations,
                    #[codec(index = 13)]
                    #[doc = "Items are not sorted and unique."]
                    NotSortedAndUnique,
                    #[codec(index = 14)]
                    #[doc = "Rewards for this era have already been claimed for this validator."]
                    AlreadyClaimed,
                    #[codec(index = 15)]
                    #[doc = "Incorrect previous history depth input provided."]
                    IncorrectHistoryDepth,
                    #[codec(index = 16)]
                    #[doc = "Incorrect number of slashing spans provided."]
                    IncorrectSlashingSpans,
                    #[codec(index = 17)]
                    #[doc = "Internal state has become somehow corrupted and the operation cannot continue."]
                    BadState,
                    #[codec(index = 18)]
                    #[doc = "Too many nomination targets supplied."]
                    TooManyTargets,
                    #[codec(index = 19)]
                    #[doc = "A nomination target was supplied that was blocked or otherwise not a validator."]
                    BadTarget,
                    #[codec(index = 20)]
                    #[doc = "The user has enough bond and thus cannot be chilled forcefully by an external person."]
                    CannotChillOther,
                    #[codec(index = 21)]
                    #[doc = "There are too many nominators in the system. Governance needs to adjust the staking"]
                    #[doc = "settings to keep things safe for the runtime."]
                    TooManyNominators,
                    #[codec(index = 22)]
                    #[doc = "There are too many validators in the system. Governance needs to adjust the staking"]
                    #[doc = "settings to keep things safe for the runtime."]
                    TooManyValidators,
                    #[codec(index = 23)]
                    #[doc = "There are not claimed rewards for this validator."]
                    NoClaimedRewards,
                    #[codec(index = 24)]
                    #[doc = "Amount overflow."]
                    AmountOverflow,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Notifies the mainchain to prepare the next era."]
                    PlanNewEra { era_index: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "Failed to notify the mainchain to prepare the next era."]
                    PlanNewEraFailed,
                    #[codec(index = 2)]
                    #[doc = "Trigger new era."]
                    TriggerNewEra,
                    #[codec(index = 3)]
                    #[doc = "Notifies the mainchain to pay the validator rewards of `era_index`."]
                    #[doc = "`excluded_validators` were excluded because they were not working properly."]
                    EraPayout {
                        era_index: ::core::primitive::u32,
                        excluded_validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Failed to notify the mainchain to pay the validator rewards of `era_index`."]
                    EraPayoutFailed { era_index: ::core::primitive::u32 },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub set_id: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::subxt::KeyedVec<_0, ::core::primitive::u32>,
            }
        }
        pub mod pallet_octopus_support {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum PayloadType {
                    #[codec(index = 0)]
                    Lock,
                    #[codec(index = 1)]
                    BurnAsset,
                    #[codec(index = 2)]
                    PlanNewEra,
                    #[codec(index = 3)]
                    EraPayout,
                    #[codec(index = 4)]
                    LockNft,
                }
            }
        }
        pub mod pallet_octopus_upward_messages {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {}
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Nonce overflow."]
                    NonceOverflow,
                    #[codec(index = 1)]
                    #[doc = "Queue size limit reached."]
                    QueueSizeLimitReached,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {}
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Sets the session key(s) of the function caller to `keys`."]
                    #[doc = "Allows an account to set its session key prior to becoming a validator."]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be signed."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
                    #[doc = "  `T::Keys::key_ids()` which is fixed."]
                    #[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
                    #[doc = "- DbWrites: `origin account`, `NextKeys`"]
                    #[doc = "- DbReads per key id: `KeyOwner`"]
                    #[doc = "- DbWrites per key id: `KeyOwner`"]
                    #[doc = "# </weight>"]
                    set_keys {
                        keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Removes any session key(s) of the function caller."]
                    #[doc = ""]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                    #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                    #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                    #[doc = "usually means being a stash account)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
                    #[doc = "  of `T::Keys::key_ids()` which is fixed."]
                    #[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
                    #[doc = "- DbWrites: `NextKeys`, `origin account`"]
                    #[doc = "- DbWrites per key id: `KeyOwner`"]
                    #[doc = "# </weight>"]
                    purge_keys,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid ownership proof."]
                    InvalidProof,
                    #[codec(index = 1)]
                    #[doc = "No associated validator ID for account."]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    #[doc = "Registered duplicate key."]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    #[doc = "No keys are associated with this account."]
                    NoKeys,
                    #[codec(index = 4)]
                    #[doc = "Key setting account is not live, so it's impossible to associate keys."]
                    NoAccount,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New session has happened. Note that the argument is the session index, not the"]
                    #[doc = "block number as the type might suggest."]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- The weight of this call is defined by the caller."]
                    #[doc = "# </weight>"]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB change."]
                    #[doc = "# </weight>"]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                    #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "An example dispatchable that may throw a custom error."]
                    cause_error,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Error names should be descriptive."]
                    NoneValue,
                    #[codec(index = 1)]
                    #[doc = "Errors should have helpful documentation associated with them."]
                    StorageOverflow,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event documentation should end with an array that provides descriptive names for event"]
                    #[doc = "parameters. [something, who]"]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    #[doc = "# </weight>"]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Issue a new class of non-fungible assets from a public origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially and its owner is the origin."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and the sender must have sufficient funds free."]
                    #[doc = ""]
                    #[doc = "`AssetDeposit` funds of sender are reserved."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `class`: The identifier of the new asset class. This must not be currently in use."]
                    #[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
                    #[doc = "member of the asset class's admin team."]
                    #[doc = ""]
                    #[doc = "Emits `Created` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    create {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Issue a new class of non-fungible assets from a privileged origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "Unlike `create`, no funds are reserved."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the new asset. This must not be currently in use."]
                    #[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
                    #[doc = "over this asset, but may later change and configure the permissions using"]
                    #[doc = "`transfer_ownership` and `set_team`."]
                    #[doc = ""]
                    #[doc = "Emits `ForceCreated` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_create {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        free_holding: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    #[doc = "Destroy a class of fungible assets."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the"]
                    #[doc = "owner of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset class to be destroyed."]
                    #[doc = "- `witness`: Information on the instances minted in the asset class. This must be"]
                    #[doc = "correct."]
                    #[doc = ""]
                    #[doc = "Emits `Destroyed` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(n + m)` where:"]
                    #[doc = "- `n = witness.instances`"]
                    #[doc = "- `m = witness.instance_metadatas`"]
                    #[doc = "- `a = witness.attributes`"]
                    destroy {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        witness: runtime_types::pallet_uniques::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    #[doc = "Mint an asset instance of a particular class."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and the sender must be the Issuer of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The class of the asset to be minted."]
                    #[doc = "- `instance`: The instance value of the asset to be minted."]
                    #[doc = "- `beneficiary`: The initial owner of the minted asset."]
                    #[doc = ""]
                    #[doc = "Emits `Issued` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    mint {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Destroy a single asset instance."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The class of the asset to be burned."]
                    #[doc = "- `instance`: The instance of the asset to be burned."]
                    #[doc = "- `check_owner`: If `Some` then the operation will fail with `WrongOwner` unless the"]
                    #[doc = "  asset is owned by this value."]
                    #[doc = ""]
                    #[doc = "Emits `Burned` with the actual amount burned."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: `check_owner.is_some()`."]
                    burn {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                        check_owner: ::core::option::Option<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Move an asset from the sender account to another."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the signing account must be either:"]
                    #[doc = "- the Admin of the asset `class`;"]
                    #[doc = "- the Owner of the asset `instance`;"]
                    #[doc = "- the approved delegate for the asset `instance` (in this case, the approval is reset)."]
                    #[doc = ""]
                    #[doc = "Arguments:"]
                    #[doc = "- `class`: The class of the asset to be transferred."]
                    #[doc = "- `instance`: The instance of the asset to be transferred."]
                    #[doc = "- `dest`: The account to receive ownership of the asset."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Reevaluate the deposits on some assets."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The class of the asset to be frozen."]
                    #[doc = "- `instances`: The instances of the asset class whose deposits will be reevaluated."]
                    #[doc = ""]
                    #[doc = "NOTE: This exists as a best-effort function. Any asset instances which are unknown or"]
                    #[doc = "in the case that the owner account does not have reservable funds to pay for a"]
                    #[doc = "deposit increase are ignored. Generally the owner isn't going to call this on instances"]
                    #[doc = "whose existing deposit is less than the refreshed deposit as it would only cost them,"]
                    #[doc = "so it's of little consequence."]
                    #[doc = ""]
                    #[doc = "It will still return an error in the case that the class is unknown of the signer is"]
                    #[doc = "not permitted to call it."]
                    #[doc = ""]
                    #[doc = "Weight: `O(instances.len())`"]
                    redeposit {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        instances: ::std::vec::Vec<::core::primitive::u128>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Disallow further unprivileged transfer of an asset instance."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The class of the asset to be frozen."]
                    #[doc = "- `instance`: The instance of the asset to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Re-allow unprivileged transfer of an asset instance."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The class of the asset to be thawed."]
                    #[doc = "- `instance`: The instance of the asset to be thawed."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Disallow further unprivileged transfers for a whole asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The asset class to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `ClassFrozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze_class {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Re-allow unprivileged transfers for a whole asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The class to be thawed."]
                    #[doc = ""]
                    #[doc = "Emits `ClassThawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw_class {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Change the Owner of an asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The asset class whose owner should be changed."]
                    #[doc = "- `owner`: The new Owner of this asset class."]
                    #[doc = ""]
                    #[doc = "Emits `OwnerChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer_ownership {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 12)]
                    #[doc = "Change the Issuer, Admin and Freezer of an asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `class`."]
                    #[doc = ""]
                    #[doc = "- `class`: The asset class whose team should be changed."]
                    #[doc = "- `issuer`: The new Issuer of this asset class."]
                    #[doc = "- `admin`: The new Admin of this asset class."]
                    #[doc = "- `freezer`: The new Freezer of this asset class."]
                    #[doc = ""]
                    #[doc = "Emits `TeamChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_team {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
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
                    #[doc = "Approve an instance to be transferred by a delegated third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and must be the owner of the asset `instance`."]
                    #[doc = ""]
                    #[doc = "- `class`: The class of the asset to be approved for delegated transfer."]
                    #[doc = "- `instance`: The instance of the asset to be approved for delegated transfer."]
                    #[doc = "- `delegate`: The account to delegate permission to transfer the asset."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovedTransfer` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    approve_transfer {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    #[doc = "Cancel the prior approval for the transfer of an asset by a delegate."]
                    #[doc = ""]
                    #[doc = "Origin must be either:"]
                    #[doc = "- the `Force` origin;"]
                    #[doc = "- `Signed` with the signer being the Admin of the asset `class`;"]
                    #[doc = "- `Signed` with the signer being the Owner of the asset `instance`;"]
                    #[doc = ""]
                    #[doc = "Arguments:"]
                    #[doc = "- `class`: The class of the asset of whose approval will be cancelled."]
                    #[doc = "- `instance`: The instance of the asset of whose approval will be cancelled."]
                    #[doc = "- `maybe_check_delegate`: If `Some` will ensure that the given account is the one to"]
                    #[doc = "  which permission of transfer is delegated."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    cancel_approval {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                        maybe_check_delegate: ::core::option::Option<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 15)]
                    #[doc = "Alter the attributes of a given asset."]
                    #[doc = ""]
                    #[doc = "Origin must be `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset."]
                    #[doc = "- `owner`: The new Owner of this asset."]
                    #[doc = "- `issuer`: The new Issuer of this asset."]
                    #[doc = "- `admin`: The new Admin of this asset."]
                    #[doc = "- `freezer`: The new Freezer of this asset."]
                    #[doc = "- `free_holding`: Whether a deposit is taken for holding an instance of this asset"]
                    #[doc = "  class."]
                    #[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
                    #[doc = "instructions."]
                    #[doc = ""]
                    #[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_asset_status {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
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
                    #[doc = "Set an attribute for an asset class or instance."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "asset `class`."]
                    #[doc = ""]
                    #[doc = "If the origin is Signed, then funds of signer are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + DepositPerByte * (key.len + value.len)` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset class whose instance's metadata to set."]
                    #[doc = "- `maybe_instance`: The identifier of the asset instance whose metadata to set."]
                    #[doc = "- `key`: The key of the attribute."]
                    #[doc = "- `value`: The value to which to set the attribute."]
                    #[doc = ""]
                    #[doc = "Emits `AttributeSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_attribute {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        maybe_instance: ::core::option::Option<::core::primitive::u128>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 17)]
                    #[doc = "Clear an attribute for an asset class or instance."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "asset `class`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the asset class owner."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset class whose instance's metadata to clear."]
                    #[doc = "- `maybe_instance`: The identifier of the asset instance whose metadata to clear."]
                    #[doc = "- `key`: The key of the attribute."]
                    #[doc = ""]
                    #[doc = "Emits `AttributeCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_attribute {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        maybe_instance: ::core::option::Option<::core::primitive::u128>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 18)]
                    #[doc = "Set the metadata for an asset instance."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "asset `class`."]
                    #[doc = ""]
                    #[doc = "If the origin is Signed, then funds of signer are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset class whose instance's metadata to set."]
                    #[doc = "- `instance`: The identifier of the asset instance whose metadata to set."]
                    #[doc = "- `data`: The general information of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `is_frozen`: Whether the metadata should be frozen against further changes."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    #[doc = "Clear the metadata for an asset instance."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "asset `instance`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the asset class owner."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset class whose instance's metadata to clear."]
                    #[doc = "- `instance`: The identifier of the asset instance whose metadata to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        #[codec(compact)]
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Set the metadata for an asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"]
                    #[doc = "the asset `class`."]
                    #[doc = ""]
                    #[doc = "If the origin is `Signed`, then funds of signer are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset whose metadata to update."]
                    #[doc = "- `data`: The general information of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `is_frozen`: Whether the metadata should be frozen against further changes."]
                    #[doc = ""]
                    #[doc = "Emits `ClassMetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_class_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 21)]
                    #[doc = "Clear the metadata for an asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"]
                    #[doc = "the asset `class`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the asset class owner."]
                    #[doc = ""]
                    #[doc = "- `class`: The identifier of the asset class whose metadata to clear."]
                    #[doc = ""]
                    #[doc = "Emits `ClassMetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_class_metadata {
                        #[codec(compact)]
                        class: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The signing account has no permission to do the operation."]
                    NoPermission,
                    #[codec(index = 1)]
                    #[doc = "The given asset ID is unknown."]
                    UnknownClass,
                    #[codec(index = 2)]
                    #[doc = "The asset instance ID has already been used for an asset."]
                    AlreadyExists,
                    #[codec(index = 3)]
                    #[doc = "The owner turned out to be different to what was expected."]
                    WrongOwner,
                    #[codec(index = 4)]
                    #[doc = "Invalid witness data given."]
                    BadWitness,
                    #[codec(index = 5)]
                    #[doc = "The asset ID is already taken."]
                    InUse,
                    #[codec(index = 6)]
                    #[doc = "The asset instance or class is frozen."]
                    Frozen,
                    #[codec(index = 7)]
                    #[doc = "The delegate turned out to be different to what was expected."]
                    WrongDelegate,
                    #[codec(index = 8)]
                    #[doc = "There is no delegate approved."]
                    NoDelegate,
                    #[codec(index = 9)]
                    #[doc = "No approval exists that would allow the transfer."]
                    Unapproved,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An asset class was created."]
                    Created {
                        class: ::core::primitive::u128,
                        creator: ::subxt::sp_core::crypto::AccountId32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "An asset class was force-created."]
                    ForceCreated {
                        class: ::core::primitive::u128,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "An asset `class` was destroyed."]
                    Destroyed { class: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    #[doc = "An asset `instance` was issued."]
                    Issued {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An asset `instance` was transferred."]
                    Transferred {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "An asset `instance` was destroyed."]
                    Burned {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some asset `instance` was frozen."]
                    Frozen {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some asset `instance` was thawed."]
                    Thawed {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some asset `class` was frozen."]
                    ClassFrozen { class: ::core::primitive::u128 },
                    #[codec(index = 9)]
                    #[doc = "Some asset `class` was thawed."]
                    ClassThawed { class: ::core::primitive::u128 },
                    #[codec(index = 10)]
                    #[doc = "The owner changed."]
                    OwnerChanged {
                        class: ::core::primitive::u128,
                        new_owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 11)]
                    #[doc = "The management team changed."]
                    TeamChanged {
                        class: ::core::primitive::u128,
                        issuer: ::subxt::sp_core::crypto::AccountId32,
                        admin: ::subxt::sp_core::crypto::AccountId32,
                        freezer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 12)]
                    #[doc = "An `instance` of an asset `class` has been approved by the `owner` for transfer by a"]
                    #[doc = "`delegate`."]
                    ApprovedTransfer {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 13)]
                    #[doc = "An approval for a `delegate` account to transfer the `instance` of an asset `class` was"]
                    #[doc = "cancelled by its `owner`."]
                    ApprovalCancelled {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 14)]
                    #[doc = "An asset `class` has had its attributes changed by the `Force` origin."]
                    AssetStatusChanged { class: ::core::primitive::u128 },
                    #[codec(index = 15)]
                    #[doc = "New metadata has been set for an asset class."]
                    ClassMetadataSet {
                        class: ::core::primitive::u128,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    #[doc = "Metadata has been cleared for an asset class."]
                    ClassMetadataCleared { class: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "New metadata has been set for an asset instance."]
                    MetadataSet {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 18)]
                    #[doc = "Metadata has been cleared for an asset instance."]
                    MetadataCleared {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Metadata has been cleared for an asset instance."]
                    Redeposited {
                        class: ::core::primitive::u128,
                        successful_instances: ::std::vec::Vec<::core::primitive::u128>,
                    },
                    #[codec(index = 20)]
                    #[doc = "New attribute metadata has been set for an asset class or instance."]
                    AttributeSet {
                        class: ::core::primitive::u128,
                        maybe_instance: ::core::option::Option<::core::primitive::u128>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 21)]
                    #[doc = "Attribute metadata has been cleared for an asset class or instance."]
                    AttributeCleared {
                        class: ::core::primitive::u128,
                        maybe_instance: ::core::option::Option<::core::primitive::u128>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct ClassMetadata<_0> {
                    pub deposit: _0,
                    pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub instances: ::core::primitive::u32,
                    #[codec(compact)]
                    pub instance_metadatas: ::core::primitive::u32,
                    #[codec(compact)]
                    pub attributes: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct InstanceDetails<_0, _1> {
                    pub owner: _0,
                    pub approved: ::core::option::Option<_0>,
                    pub is_frozen: ::core::primitive::bool,
                    pub deposit: _1,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Decode,
                    :: subxt :: codec :: Encode,
                    Debug,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Decode,
                    :: subxt :: codec :: Encode,
                    Debug,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _2, _1, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BlakeTwo256;
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: ::core::primitive::u8,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
        fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
            if let Self::Module(module_error) = self {
                Some(::subxt::ModuleErrorData {
                    pallet_index: module_error.index,
                    error: [module_error.error, 0, 0, 0],
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
        pub fn validate_metadata(&'a self) -> Result<(), ::subxt::MetadataError> {
            if self.client.metadata().metadata_hash(&PALLETS)
                != [
                    229u8, 239u8, 188u8, 5u8, 106u8, 80u8, 79u8, 136u8, 181u8, 252u8, 151u8, 125u8,
                    105u8, 252u8, 229u8, 137u8, 45u8, 102u8, 232u8, 158u8, 67u8, 44u8, 162u8,
                    238u8, 91u8, 205u8, 162u8, 117u8, 225u8, 240u8, 41u8, 102u8,
                ]
            {
                Err(::subxt::MetadataError::IncompatibleMetadata)
            } else {
                Ok(())
            }
        }
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
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi<'a, T> {
            timestamp::constants::ConstantsApi::new(self.client)
        }
        pub fn babe(&self) -> babe::constants::ConstantsApi<'a, T> {
            babe::constants::ConstantsApi::new(self.client)
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
        pub fn octopus_uniques(&self) -> octopus_uniques::constants::ConstantsApi<'a, T> {
            octopus_uniques::constants::ConstantsApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi<'a, T> {
            grandpa::constants::ConstantsApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi<'a, T> {
            im_online::constants::ConstantsApi::new(self.client)
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
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
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
        pub fn octopus_uniques(&self) -> octopus_uniques::storage::StorageApi<'a, T> {
            octopus_uniques::storage::StorageApi::new(self.client)
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
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::storage::StorageApi<'a, T> {
            template_module::storage::StorageApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::storage::StorageApi<'a, T> {
            ibc::storage::StorageApi::new(self.client)
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
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T, X> {
            babe::calls::TransactionApi::new(self.client)
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
        pub fn octopus_uniques(&self) -> octopus_uniques::calls::TransactionApi<'a, T, X> {
            octopus_uniques::calls::TransactionApi::new(self.client)
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
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::calls::TransactionApi<'a, T, X> {
            template_module::calls::TransactionApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::calls::TransactionApi<'a, T, X> {
            ibc::calls::TransactionApi::new(self.client)
        }
    }
}
