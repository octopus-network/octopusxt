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
    #[derive(:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug)]
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
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::tx::StaticTxPayload<FillBlock> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "fill_block",
                        FillBlock { ratio },
                        [
                            48u8, 18u8, 205u8, 90u8, 222u8, 4u8, 20u8, 251u8, 173u8, 76u8, 167u8,
                            4u8, 83u8, 203u8, 160u8, 89u8, 132u8, 218u8, 191u8, 145u8, 130u8,
                            245u8, 177u8, 201u8, 169u8, 129u8, 173u8, 105u8, 88u8, 45u8, 136u8,
                            191u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<Remark> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark",
                        Remark { remark },
                        [
                            101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8,
                            147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8,
                            146u8, 87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8,
                            160u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetHeapPages> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_heap_pages",
                        SetHeapPages { pages },
                        [
                            43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8,
                            86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8,
                            125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<SetCode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code",
                        SetCode { code },
                        [
                            27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8,
                            244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8,
                            4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code_without_checks",
                        SetCodeWithoutChecks { code },
                        [
                            102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8,
                            159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8,
                            40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<SetStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_storage",
                        SetStorage { items },
                        [
                            74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8,
                            45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
                            140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::tx::StaticTxPayload<KillStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_storage",
                        KillStorage { keys },
                        [
                            174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8,
                            85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8,
                            67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8,
                            15u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_prefix",
                        KillPrefix { prefix, subkeys },
                        [
                            203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8,
                            193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8,
                            80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8,
                            48u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark_with_event",
                        RemarkWithEvent { remark },
                        [
                            123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8,
                            134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8,
                            136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8,
                            46u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
                pub hash: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
                            114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
                            42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Account",
                        Vec::new(),
                        [
                            176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
                            114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
                            42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
                            222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
                            41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::weights::PerDispatchClass<
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            91u8, 211u8, 177u8, 36u8, 147u8, 249u8, 55u8, 164u8, 48u8, 49u8, 55u8,
                            11u8, 121u8, 193u8, 103u8, 69u8, 38u8, 142u8, 148u8, 36u8, 137u8, 41u8,
                            115u8, 195u8, 31u8, 174u8, 163u8, 125u8, 69u8, 5u8, 94u8, 79u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
                            254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
                            219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
                            134u8, 60u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
                            21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
                            80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockHash",
                        Vec::new(),
                        [
                            50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
                            21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
                            80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicData",
                        Vec::new(),
                        [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Number",
                        vec![],
                        [
                            228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
                            246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
                            36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
                            164u8, 191u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8,
                            169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8,
                            15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8,
                            36u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::generic::digest::Digest,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Digest",
                        vec![],
                        [
                            83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8,
                            31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8,
                            216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: This storage item is explicitly unbounded since it is never intended to be read"]
                #[doc = " from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::frame_system::EventRecord<
                                runtime_types::node_template_runtime::Event,
                                ::subxt::ext::sp_core::H256,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Events",
                        vec![],
                        [
                            54u8, 230u8, 50u8, 11u8, 108u8, 35u8, 124u8, 17u8, 61u8, 192u8, 72u8,
                            159u8, 219u8, 209u8, 111u8, 152u8, 191u8, 9u8, 180u8, 8u8, 44u8, 245u8,
                            49u8, 192u8, 95u8, 103u8, 194u8, 83u8, 35u8, 151u8, 122u8, 159u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
                            203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
                            161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
                            112u8,
                        ],
                    )
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
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
                            63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
                            111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
                            38u8,
                        ],
                    )
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
                pub fn event_topics_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventTopics",
                        Vec::new(),
                        [
                            205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
                            63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
                            111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
                            38u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8,
                            126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8,
                            185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
                            178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
                            83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
                            210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
                            155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8,
                            103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8,
                            201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8,
                            172u8, 93u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockWeights,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockWeights",
                        [
                            153u8, 164u8, 86u8, 79u8, 97u8, 114u8, 248u8, 181u8, 179u8, 186u8,
                            214u8, 124u8, 215u8, 96u8, 116u8, 109u8, 215u8, 182u8, 61u8, 10u8,
                            77u8, 74u8, 29u8, 125u8, 131u8, 111u8, 249u8, 208u8, 233u8, 170u8,
                            11u8, 14u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockLength,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockLength",
                        [
                            116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8, 220u8, 234u8, 198u8,
                            150u8, 108u8, 205u8, 87u8, 194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8,
                            12u8, 200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8, 205u8, 203u8,
                            236u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::weights::RuntimeDbWeight,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "DbWeight",
                        [
                            124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8,
                            199u8, 181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8,
                            20u8, 129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8,
                            118u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "Version",
                        [
                            93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
                            47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
                            165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
                        ],
                    )
                }
                #[doc = " The designated SS85 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub fn random_material(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::subxt::ext::sp_core::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "RandomnessCollectiveFlip",
                        "RandomMaterial",
                        vec![],
                        [
                            152u8, 126u8, 73u8, 88u8, 54u8, 147u8, 6u8, 19u8, 214u8, 40u8, 159u8,
                            30u8, 236u8, 61u8, 240u8, 65u8, 178u8, 94u8, 146u8, 152u8, 135u8,
                            252u8, 160u8, 86u8, 123u8, 114u8, 251u8, 140u8, 98u8, 143u8, 217u8,
                            242u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                ) -> ::subxt::tx::StaticTxPayload<Set> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Timestamp",
                        "set",
                        Set { now },
                        [
                            6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8,
                            102u8, 85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8,
                            214u8, 182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
                            221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
                            185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
                            13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
                            27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod babe {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PlanConfigChange {
                pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocation> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Babe",
                        "report_equivocation",
                        ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            177u8, 237u8, 107u8, 138u8, 237u8, 233u8, 30u8, 195u8, 112u8, 176u8,
                            185u8, 113u8, 157u8, 221u8, 134u8, 151u8, 62u8, 151u8, 64u8, 164u8,
                            254u8, 112u8, 2u8, 94u8, 175u8, 79u8, 160u8, 3u8, 72u8, 145u8, 244u8,
                            137u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Babe",
                        "report_equivocation_unsigned",
                        ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            56u8, 103u8, 238u8, 118u8, 61u8, 192u8, 222u8, 87u8, 254u8, 24u8,
                            138u8, 219u8, 210u8, 85u8, 201u8, 147u8, 128u8, 49u8, 199u8, 144u8,
                            46u8, 158u8, 163u8, 31u8, 101u8, 224u8, 72u8, 98u8, 68u8, 120u8, 215u8,
                            19u8,
                        ],
                    )
                }
                #[doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"]
                #[doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."]
                #[doc = "Multiple calls to this method will replace any existing planned config change that had"]
                #[doc = "not been enacted yet."]
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> ::subxt::tx::StaticTxPayload<PlanConfigChange> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Babe",
                        "plan_config_change",
                        PlanConfigChange { config },
                        [
                            229u8, 157u8, 41u8, 58u8, 56u8, 4u8, 52u8, 107u8, 104u8, 20u8, 42u8,
                            110u8, 1u8, 17u8, 45u8, 196u8, 30u8, 135u8, 63u8, 46u8, 40u8, 137u8,
                            209u8, 37u8, 24u8, 108u8, 251u8, 189u8, 77u8, 208u8, 74u8, 32u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current epoch index."]
                pub fn epoch_index(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "EpochIndex",
                        vec![],
                        [
                            51u8, 27u8, 91u8, 156u8, 118u8, 99u8, 46u8, 219u8, 190u8, 147u8, 205u8,
                            23u8, 106u8, 169u8, 121u8, 218u8, 208u8, 235u8, 135u8, 127u8, 243u8,
                            41u8, 55u8, 243u8, 235u8, 122u8, 57u8, 86u8, 37u8, 90u8, 208u8, 71u8,
                        ],
                    )
                }
                #[doc = " Current epoch authorities."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                            runtime_types::sp_consensus_babe::app::Public,
                            ::core::primitive::u64,
                        )>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "Authorities",
                        vec![],
                        [
                            61u8, 8u8, 133u8, 111u8, 169u8, 120u8, 0u8, 213u8, 31u8, 159u8, 204u8,
                            212u8, 18u8, 205u8, 93u8, 84u8, 140u8, 108u8, 136u8, 209u8, 234u8,
                            107u8, 145u8, 9u8, 204u8, 224u8, 105u8, 9u8, 238u8, 241u8, 65u8, 30u8,
                        ],
                    )
                }
                #[doc = " The slot at which the first epoch actually started. This is 0"]
                #[doc = " until the first block of the chain."]
                pub fn genesis_slot(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "GenesisSlot",
                        vec![],
                        [
                            234u8, 127u8, 243u8, 100u8, 124u8, 160u8, 66u8, 248u8, 48u8, 218u8,
                            61u8, 52u8, 54u8, 142u8, 158u8, 77u8, 32u8, 63u8, 156u8, 39u8, 94u8,
                            255u8, 192u8, 238u8, 170u8, 118u8, 58u8, 42u8, 199u8, 61u8, 199u8,
                            77u8,
                        ],
                    )
                }
                #[doc = " Current slot number."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "CurrentSlot",
                        vec![],
                        [
                            139u8, 237u8, 185u8, 137u8, 251u8, 179u8, 69u8, 167u8, 133u8, 168u8,
                            204u8, 64u8, 178u8, 123u8, 92u8, 250u8, 119u8, 190u8, 208u8, 178u8,
                            208u8, 176u8, 124u8, 187u8, 74u8, 165u8, 33u8, 78u8, 161u8, 206u8, 8u8,
                            108u8,
                        ],
                    )
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
                pub fn randomness(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 32usize]>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "Randomness",
                        vec![],
                        [
                            191u8, 197u8, 25u8, 164u8, 104u8, 248u8, 247u8, 193u8, 244u8, 60u8,
                            181u8, 195u8, 248u8, 90u8, 41u8, 199u8, 82u8, 123u8, 72u8, 126u8, 18u8,
                            17u8, 128u8, 215u8, 34u8, 251u8, 227u8, 70u8, 166u8, 10u8, 104u8,
                            140u8,
                        ],
                    )
                }
                #[doc = " Pending epoch configuration change that will be applied when the next epoch is enacted."]
                pub fn pending_epoch_config_change(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "PendingEpochConfigChange",
                        vec![],
                        [
                            4u8, 201u8, 0u8, 204u8, 47u8, 246u8, 4u8, 185u8, 163u8, 242u8, 242u8,
                            152u8, 29u8, 222u8, 71u8, 127u8, 49u8, 203u8, 206u8, 180u8, 244u8,
                            50u8, 80u8, 49u8, 199u8, 97u8, 3u8, 170u8, 156u8, 139u8, 106u8, 113u8,
                        ],
                    )
                }
                #[doc = " Next epoch randomness."]
                pub fn next_randomness(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 32usize]>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "NextRandomness",
                        vec![],
                        [
                            185u8, 98u8, 45u8, 109u8, 253u8, 38u8, 238u8, 221u8, 240u8, 29u8, 38u8,
                            107u8, 118u8, 117u8, 131u8, 115u8, 21u8, 255u8, 203u8, 81u8, 243u8,
                            251u8, 91u8, 60u8, 163u8, 202u8, 125u8, 193u8, 173u8, 234u8, 166u8,
                            92u8,
                        ],
                    )
                }
                #[doc = " Next epoch authorities."]
                pub fn next_authorities(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                            runtime_types::sp_consensus_babe::app::Public,
                            ::core::primitive::u64,
                        )>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "NextAuthorities",
                        vec![],
                        [
                            201u8, 193u8, 164u8, 18u8, 155u8, 253u8, 124u8, 163u8, 143u8, 73u8,
                            212u8, 20u8, 241u8, 108u8, 110u8, 5u8, 171u8, 66u8, 224u8, 208u8, 10u8,
                            65u8, 148u8, 164u8, 1u8, 12u8, 216u8, 83u8, 20u8, 226u8, 254u8, 183u8,
                        ],
                    )
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
                pub fn segment_index(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "SegmentIndex",
                        vec![],
                        [
                            128u8, 45u8, 87u8, 58u8, 174u8, 152u8, 241u8, 156u8, 56u8, 192u8, 19u8,
                            45u8, 75u8, 160u8, 35u8, 253u8, 145u8, 11u8, 178u8, 81u8, 114u8, 117u8,
                            112u8, 107u8, 163u8, 208u8, 240u8, 151u8, 102u8, 176u8, 246u8, 5u8,
                        ],
                    )
                }
                #[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
                pub fn under_construction(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            [::core::primitive::u8; 32usize],
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "UnderConstruction",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
                            182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
                            89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
                        ],
                    )
                }
                #[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
                pub fn under_construction_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            [::core::primitive::u8; 32usize],
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "UnderConstruction",
                        Vec::new(),
                        [
                            180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
                            182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
                            89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
                        ],
                    )
                }
                #[doc = " Temporary value (cleared at block finalization) which is `Some`"]
                #[doc = " if per-block initialization has already been called for current block."]
                pub fn initialized(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "Initialized",
                        vec![],
                        [
                            48u8, 206u8, 111u8, 118u8, 149u8, 175u8, 148u8, 53u8, 233u8, 82u8,
                            220u8, 57u8, 22u8, 164u8, 116u8, 228u8, 134u8, 237u8, 129u8, 195u8,
                            60u8, 169u8, 1u8, 164u8, 74u8, 177u8, 145u8, 112u8, 66u8, 198u8, 53u8,
                            157u8,
                        ],
                    )
                }
                #[doc = " This field should always be populated during block processing unless"]
                #[doc = " secondary plain slots are enabled (which don't contain a VRF output)."]
                #[doc = ""]
                #[doc = " It is set in `on_initialize`, before it will contain the value from the last block."]
                pub fn author_vrf_randomness(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "AuthorVrfRandomness",
                        vec![],
                        [
                            66u8, 235u8, 74u8, 252u8, 222u8, 135u8, 19u8, 28u8, 74u8, 191u8, 170u8,
                            197u8, 207u8, 127u8, 77u8, 121u8, 138u8, 138u8, 110u8, 187u8, 34u8,
                            14u8, 230u8, 43u8, 241u8, 241u8, 63u8, 163u8, 53u8, 179u8, 250u8,
                            247u8,
                        ],
                    )
                }
                #[doc = " The block numbers when the last and current epoch have started, respectively `N-1` and"]
                #[doc = " `N`."]
                #[doc = " NOTE: We track this is in order to annotate the block number when a given pool of"]
                #[doc = " entropy was fixed (i.e. it was known to chain observers). Since epochs are defined in"]
                #[doc = " slots, which may be skipped, the block numbers may not line up with the slot numbers."]
                pub fn epoch_start(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "EpochStart",
                        vec![],
                        [
                            196u8, 39u8, 241u8, 20u8, 150u8, 180u8, 136u8, 4u8, 195u8, 205u8,
                            218u8, 10u8, 130u8, 131u8, 168u8, 243u8, 207u8, 249u8, 58u8, 195u8,
                            177u8, 119u8, 110u8, 243u8, 241u8, 3u8, 245u8, 56u8, 157u8, 5u8, 68u8,
                            60u8,
                        ],
                    )
                }
                #[doc = " How late the current block is compared to its parent."]
                #[doc = ""]
                #[doc = " This entry is populated as part of block execution and is cleaned up"]
                #[doc = " on block finalization. Querying this storage entry outside of block"]
                #[doc = " execution context should always yield zero."]
                pub fn lateness(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "Lateness",
                        vec![],
                        [
                            229u8, 230u8, 224u8, 89u8, 49u8, 213u8, 198u8, 236u8, 144u8, 56u8,
                            193u8, 234u8, 62u8, 242u8, 191u8, 199u8, 105u8, 131u8, 74u8, 63u8,
                            75u8, 1u8, 210u8, 49u8, 3u8, 128u8, 18u8, 77u8, 219u8, 146u8, 60u8,
                            88u8,
                        ],
                    )
                }
                #[doc = " The configuration for the current epoch. Should never be `None` as it is initialized in"]
                #[doc = " genesis."]
                pub fn epoch_config(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "EpochConfig",
                        vec![],
                        [
                            41u8, 118u8, 141u8, 244u8, 72u8, 17u8, 125u8, 203u8, 43u8, 153u8,
                            203u8, 119u8, 117u8, 223u8, 123u8, 133u8, 73u8, 235u8, 130u8, 21u8,
                            160u8, 167u8, 16u8, 173u8, 177u8, 35u8, 117u8, 97u8, 149u8, 49u8,
                            220u8, 24u8,
                        ],
                    )
                }
                #[doc = " The configuration for the next epoch, `None` if the config will not change"]
                #[doc = " (you can fallback to `EpochConfig` instead in that case)."]
                pub fn next_epoch_config(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Babe",
                        "NextEpochConfig",
                        vec![],
                        [
                            111u8, 182u8, 144u8, 180u8, 92u8, 146u8, 102u8, 249u8, 196u8, 229u8,
                            226u8, 30u8, 25u8, 198u8, 133u8, 9u8, 136u8, 95u8, 11u8, 151u8, 139u8,
                            156u8, 105u8, 228u8, 181u8, 12u8, 175u8, 148u8, 174u8, 33u8, 233u8,
                            228u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The amount of time, in slots, that each epoch should last."]
                #[doc = " NOTE: Currently it is not possible to change the epoch duration after"]
                #[doc = " the chain has started. Attempting to do so will brick block production."]
                pub fn epoch_duration(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Babe",
                        "EpochDuration",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " The expected average block time at which BABE should be creating"]
                #[doc = " blocks. Since BABE is probabilistic it is not trivial to figure out"]
                #[doc = " what the expected average block time should be based on the slot"]
                #[doc = " duration and the security parameter `c` (where `1 - c` represents"]
                #[doc = " the probability of a slot being empty)."]
                pub fn expected_block_time(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Babe",
                        "ExpectedBlockTime",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " Max number of authorities allowed"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Babe",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod authorship {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Provide a set of uncles."]
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetUncles> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Authorship",
                        "set_uncles",
                        SetUncles { new_uncles },
                        [
                            181u8, 70u8, 222u8, 83u8, 154u8, 215u8, 200u8, 64u8, 154u8, 228u8,
                            115u8, 247u8, 117u8, 89u8, 229u8, 102u8, 128u8, 189u8, 90u8, 60u8,
                            223u8, 19u8, 111u8, 172u8, 5u8, 223u8, 132u8, 37u8, 235u8, 119u8, 42u8,
                            64u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Uncles"]
                pub fn uncles(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::pallet_authorship::UncleEntryItem<
                                ::core::primitive::u32,
                                ::subxt::ext::sp_core::H256,
                                ::subxt::ext::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Authorship",
                        "Uncles",
                        vec![],
                        [
                            43u8, 181u8, 75u8, 158u8, 153u8, 32u8, 210u8, 36u8, 194u8, 34u8, 146u8,
                            179u8, 154u8, 141u8, 75u8, 29u8, 51u8, 116u8, 94u8, 82u8, 90u8, 74u8,
                            103u8, 216u8, 86u8, 27u8, 30u8, 213u8, 174u8, 80u8, 193u8, 51u8,
                        ],
                    )
                }
                #[doc = " Author of current block."]
                pub fn author(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Authorship",
                        "Author",
                        vec![],
                        [
                            149u8, 42u8, 33u8, 147u8, 190u8, 207u8, 174u8, 227u8, 190u8, 110u8,
                            25u8, 131u8, 5u8, 167u8, 237u8, 188u8, 188u8, 33u8, 177u8, 126u8,
                            181u8, 49u8, 126u8, 118u8, 46u8, 128u8, 154u8, 95u8, 15u8, 91u8, 103u8,
                            113u8,
                        ],
                    )
                }
                #[doc = " Whether uncles were already set in this block."]
                pub fn did_set_uncles(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Authorship",
                        "DidSetUncles",
                        vec![],
                        [
                            64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
                            126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
                            83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The number of blocks back we should accept uncles."]
                #[doc = " This means that we will deal with uncle-parents that are"]
                #[doc = " `UncleGenerations + 1` before `now`."]
                pub fn uncle_generations(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Authorship",
                        "UncleGenerations",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Transfer {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetBalance {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceTransfer {
                pub source: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceUnreserve {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub amount: ::core::primitive::u128,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer",
                        Transfer { dest, value },
                        [
                            111u8, 222u8, 32u8, 56u8, 171u8, 77u8, 252u8, 29u8, 194u8, 155u8,
                            200u8, 192u8, 198u8, 81u8, 23u8, 115u8, 236u8, 91u8, 218u8, 114u8,
                            107u8, 141u8, 138u8, 100u8, 237u8, 21u8, 58u8, 172u8, 3u8, 20u8, 216u8,
                            38u8,
                        ],
                    )
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
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SetBalance> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "set_balance",
                        SetBalance {
                            who,
                            new_free,
                            new_reserved,
                        },
                        [
                            234u8, 215u8, 97u8, 98u8, 243u8, 199u8, 57u8, 76u8, 59u8, 161u8, 118u8,
                            207u8, 34u8, 197u8, 198u8, 61u8, 231u8, 210u8, 169u8, 235u8, 150u8,
                            137u8, 173u8, 49u8, 28u8, 77u8, 84u8, 149u8, 143u8, 210u8, 139u8,
                            193u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "# <weight>"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                #[doc = "# </weight>"]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_transfer",
                        ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            79u8, 174u8, 212u8, 108u8, 184u8, 33u8, 170u8, 29u8, 232u8, 254u8,
                            195u8, 218u8, 221u8, 134u8, 57u8, 99u8, 6u8, 70u8, 181u8, 227u8, 56u8,
                            239u8, 243u8, 158u8, 157u8, 245u8, 36u8, 162u8, 11u8, 237u8, 147u8,
                            15u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_keep_alive",
                        TransferKeepAlive { dest, value },
                        [
                            112u8, 179u8, 75u8, 168u8, 193u8, 221u8, 9u8, 82u8, 190u8, 113u8,
                            253u8, 13u8, 130u8, 134u8, 170u8, 216u8, 136u8, 111u8, 242u8, 220u8,
                            202u8, 112u8, 47u8, 79u8, 73u8, 244u8, 226u8, 59u8, 240u8, 188u8,
                            210u8, 208u8,
                        ],
                    )
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
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<TransferAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_all",
                        TransferAll { dest, keep_alive },
                        [
                            46u8, 129u8, 29u8, 177u8, 221u8, 107u8, 245u8, 69u8, 238u8, 126u8,
                            145u8, 26u8, 219u8, 208u8, 14u8, 80u8, 149u8, 1u8, 214u8, 63u8, 67u8,
                            201u8, 144u8, 45u8, 129u8, 145u8, 174u8, 71u8, 238u8, 113u8, 208u8,
                            34u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_unreserve",
                        ForceUnreserve { who, amount },
                        [
                            160u8, 146u8, 137u8, 76u8, 157u8, 187u8, 66u8, 148u8, 207u8, 76u8,
                            32u8, 254u8, 82u8, 215u8, 35u8, 161u8, 213u8, 52u8, 32u8, 98u8, 102u8,
                            106u8, 234u8, 123u8, 6u8, 175u8, 184u8, 188u8, 174u8, 106u8, 176u8,
                            78u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
                            156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
                            20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
                        ],
                    )
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
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
                            40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
                            217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
                        ],
                    )
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
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
                            40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
                            217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
                            58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
                            136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
                            58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
                            136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
                            167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
                            183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
                            167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
                            183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
                #[doc = " Storage version of the pallet."]
                #[doc = ""]
                #[doc = " This is set to v2.0.0 for new networks."]
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::Releases>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "StorageVersion",
                        vec![],
                        [
                            135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
                            235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
                            174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
                            173u8, 96u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
                            197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
                            237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
                            180u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_transaction_payment::Releases,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
                            200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
                            58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The fee to be paid for making a transaction; the per-byte portion."]
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TransactionPayment",
                        "TransactionByteFee",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
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
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The polynomial that is applied in order to derive fee from weight."]
                pub fn weight_to_fee(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::frame_support::weights::WeightToFeeCoefficient<
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TransactionPayment",
                        "WeightToFee",
                        [
                            15u8, 119u8, 137u8, 69u8, 18u8, 105u8, 232u8, 166u8, 200u8, 253u8,
                            17u8, 71u8, 185u8, 155u8, 82u8, 215u8, 166u8, 74u8, 238u8, 28u8, 61u8,
                            206u8, 89u8, 133u8, 136u8, 131u8, 67u8, 243u8, 19u8, 238u8, 147u8,
                            226u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod octopus_appchain {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SubmitObservations {
                pub payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                    runtime_types::sp_runtime::MultiSigner,
                    ::core::primitive::u32,
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
                pub signature: runtime_types::sp_runtime::MultiSignature,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceSetIsActivated {
                pub is_activated: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ForceSetNextSetId {
                pub next_set_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceSetPlannedValidators {
                pub validators: ::std::vec::Vec<(
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Lock {
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MintAsset {
                pub asset_id: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnAsset {
                pub asset_id: ::core::primitive::u32,
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetAssetName {
                pub asset_name: ::std::vec::Vec<::core::primitive::u8>,
                pub asset_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TranferFromPalletAccount {
                pub receiver: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct LockNft {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Submit observations."]
                pub fn submit_observations(
                    &self,
                    payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                    signature: runtime_types::sp_runtime::MultiSignature,
                ) -> ::subxt::tx::StaticTxPayload<SubmitObservations> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "submit_observations",
                        SubmitObservations { payload, signature },
                        [
                            96u8, 235u8, 89u8, 242u8, 60u8, 142u8, 192u8, 33u8, 146u8, 136u8,
                            120u8, 10u8, 78u8, 248u8, 16u8, 104u8, 152u8, 110u8, 45u8, 146u8,
                            236u8, 81u8, 15u8, 74u8, 121u8, 153u8, 191u8, 188u8, 210u8, 107u8,
                            233u8, 150u8,
                        ],
                    )
                }
                pub fn force_set_is_activated(
                    &self,
                    is_activated: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<ForceSetIsActivated> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "force_set_is_activated",
                        ForceSetIsActivated { is_activated },
                        [
                            135u8, 58u8, 213u8, 72u8, 139u8, 244u8, 159u8, 254u8, 74u8, 171u8,
                            196u8, 111u8, 119u8, 140u8, 4u8, 28u8, 138u8, 0u8, 200u8, 96u8, 169u8,
                            99u8, 210u8, 233u8, 101u8, 130u8, 229u8, 170u8, 205u8, 190u8, 223u8,
                            191u8,
                        ],
                    )
                }
                pub fn force_set_next_set_id(
                    &self,
                    next_set_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<ForceSetNextSetId> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "force_set_next_set_id",
                        ForceSetNextSetId { next_set_id },
                        [
                            37u8, 219u8, 73u8, 61u8, 195u8, 213u8, 68u8, 185u8, 30u8, 214u8, 144u8,
                            232u8, 90u8, 49u8, 149u8, 161u8, 172u8, 91u8, 150u8, 66u8, 232u8, 32u8,
                            253u8, 240u8, 14u8, 237u8, 108u8, 93u8, 15u8, 17u8, 10u8, 101u8,
                        ],
                    )
                }
                pub fn force_set_planned_validators(
                    &self,
                    validators: ::std::vec::Vec<(
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<ForceSetPlannedValidators> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "force_set_planned_validators",
                        ForceSetPlannedValidators { validators },
                        [
                            27u8, 128u8, 217u8, 217u8, 113u8, 10u8, 168u8, 159u8, 203u8, 250u8,
                            145u8, 92u8, 84u8, 236u8, 87u8, 250u8, 191u8, 48u8, 225u8, 230u8,
                            148u8, 201u8, 156u8, 141u8, 69u8, 118u8, 58u8, 212u8, 254u8, 146u8,
                            43u8, 243u8,
                        ],
                    )
                }
                #[doc = "Emits `Locked` event when successful."]
                pub fn lock(
                    &self,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Lock> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "lock",
                        Lock {
                            receiver_id,
                            amount,
                        },
                        [
                            89u8, 186u8, 79u8, 131u8, 188u8, 39u8, 222u8, 144u8, 209u8, 230u8,
                            152u8, 137u8, 217u8, 162u8, 25u8, 15u8, 2u8, 20u8, 153u8, 115u8, 55u8,
                            82u8, 71u8, 129u8, 197u8, 167u8, 129u8, 138u8, 101u8, 170u8, 50u8,
                            253u8,
                        ],
                    )
                }
                pub fn mint_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    sender_id: ::std::vec::Vec<::core::primitive::u8>,
                    receiver: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<MintAsset> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "mint_asset",
                        MintAsset {
                            asset_id,
                            sender_id,
                            receiver,
                            amount,
                        },
                        [
                            100u8, 40u8, 241u8, 219u8, 61u8, 14u8, 180u8, 169u8, 155u8, 254u8,
                            16u8, 167u8, 28u8, 199u8, 80u8, 127u8, 102u8, 122u8, 214u8, 202u8,
                            94u8, 233u8, 0u8, 105u8, 166u8, 65u8, 1u8, 151u8, 172u8, 104u8, 138u8,
                            10u8,
                        ],
                    )
                }
                pub fn burn_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<BurnAsset> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "burn_asset",
                        BurnAsset {
                            asset_id,
                            receiver_id,
                            amount,
                        },
                        [
                            50u8, 156u8, 61u8, 60u8, 88u8, 224u8, 243u8, 72u8, 83u8, 133u8, 189u8,
                            2u8, 190u8, 165u8, 150u8, 25u8, 229u8, 149u8, 212u8, 45u8, 143u8, 92u8,
                            61u8, 226u8, 210u8, 183u8, 182u8, 219u8, 94u8, 67u8, 97u8, 117u8,
                        ],
                    )
                }
                pub fn set_asset_name(
                    &self,
                    asset_name: ::std::vec::Vec<::core::primitive::u8>,
                    asset_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetAssetName> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "set_asset_name",
                        SetAssetName {
                            asset_name,
                            asset_id,
                        },
                        [
                            242u8, 39u8, 109u8, 159u8, 136u8, 76u8, 141u8, 218u8, 187u8, 98u8,
                            100u8, 221u8, 110u8, 136u8, 255u8, 6u8, 21u8, 234u8, 53u8, 152u8,
                            230u8, 71u8, 164u8, 186u8, 247u8, 149u8, 245u8, 165u8, 143u8, 50u8,
                            66u8, 203u8,
                        ],
                    )
                }
                pub fn tranfer_from_pallet_account(
                    &self,
                    receiver: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TranferFromPalletAccount> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "tranfer_from_pallet_account",
                        TranferFromPalletAccount { receiver, amount },
                        [
                            71u8, 132u8, 86u8, 157u8, 158u8, 251u8, 226u8, 128u8, 191u8, 92u8,
                            228u8, 238u8, 174u8, 36u8, 143u8, 173u8, 129u8, 33u8, 143u8, 242u8,
                            204u8, 36u8, 196u8, 136u8, 96u8, 97u8, 177u8, 27u8, 157u8, 217u8,
                            250u8, 191u8,
                        ],
                    )
                }
                pub fn lock_nft(
                    &self,
                    class: ::core::primitive::u128,
                    instance: ::core::primitive::u128,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<LockNft> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAppchain",
                        "lock_nft",
                        LockNft {
                            class,
                            instance,
                            receiver_id,
                        },
                        [
                            247u8, 44u8, 8u8, 247u8, 155u8, 147u8, 53u8, 81u8, 88u8, 86u8, 88u8,
                            137u8, 136u8, 104u8, 45u8, 124u8, 101u8, 238u8, 25u8, 120u8, 222u8,
                            246u8, 244u8, 4u8, 106u8, 200u8, 168u8, 246u8, 129u8, 166u8, 250u8,
                            38u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_octopus_appchain::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A new set of validators is waiting to be changed."]
            pub struct NewPlannedValidators {
                pub set_id: ::core::primitive::u32,
                pub validators: ::std::vec::Vec<(
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::events::StaticEvent for NewPlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NewPlannedValidators";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An `amount` of native token has been locked in the appchain to indicate that"]
            #[doc = "it will be cross-chain transferred to the mainchain."]
            pub struct Locked {
                pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
                pub receiver: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
                pub sequence: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for Locked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An `amount` was unlocked to `receiver` from `sender`."]
            pub struct Unlocked {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An `amount` unlock to `receiver` from `sender` failed."]
            pub struct UnlockFailed {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for UnlockFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "UnlockFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AssetMinted {
                pub asset_id: ::core::primitive::u32,
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AssetMinted {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMinted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AssetBurned {
                pub asset_id: ::core::primitive::u32,
                pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
                pub receiver: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
                pub sequence: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for AssetBurned {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetBurned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AssetMintFailed {
                pub asset_id: ::core::primitive::u32,
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AssetMintFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMintFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AssetIdGetFailed {
                pub token_id: ::std::vec::Vec<::core::primitive::u8>,
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AssetIdGetFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetIdGetFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferredFromPallet {
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransferredFromPallet {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "TransferredFromPallet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NftLocked {
                pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
                pub receiver: ::std::vec::Vec<::core::primitive::u8>,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub sequence: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for NftLocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NftLocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NftUnlocked {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for NftUnlocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NftUnlocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NftUnlockFailed {
                pub sender: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for NftUnlockFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NftUnlockFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn anchor_contract(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "AnchorContract",
                        vec![],
                        [
                            39u8, 254u8, 249u8, 168u8, 27u8, 186u8, 34u8, 81u8, 114u8, 252u8,
                            137u8, 120u8, 169u8, 110u8, 85u8, 144u8, 32u8, 155u8, 158u8, 251u8,
                            126u8, 107u8, 64u8, 213u8, 87u8, 20u8, 213u8, 218u8, 46u8, 1u8, 107u8,
                            208u8,
                        ],
                    )
                }
                pub fn asset_id_by_name(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "AssetIdByName",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ],
                    )
                }
                pub fn asset_id_by_name_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "AssetIdByName",
                        Vec::new(),
                        [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ],
                    )
                }
                #[doc = " Whether the appchain is activated."]
                #[doc = ""]
                #[doc = " Only an active appchain will communicate with the mainchain and pay block rewards."]
                pub fn is_activated(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "IsActivated",
                        vec![],
                        [
                            79u8, 161u8, 11u8, 40u8, 81u8, 93u8, 156u8, 140u8, 189u8, 249u8, 217u8,
                            21u8, 180u8, 193u8, 93u8, 129u8, 149u8, 250u8, 97u8, 1u8, 205u8, 234u8,
                            123u8, 232u8, 167u8, 194u8, 188u8, 247u8, 182u8, 179u8, 196u8, 209u8,
                        ],
                    )
                }
                pub fn next_set_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "NextSetId",
                        vec![],
                        [
                            143u8, 235u8, 138u8, 252u8, 147u8, 133u8, 43u8, 104u8, 147u8, 238u8,
                            74u8, 115u8, 65u8, 100u8, 51u8, 239u8, 106u8, 122u8, 127u8, 146u8,
                            234u8, 160u8, 38u8, 18u8, 20u8, 59u8, 28u8, 254u8, 105u8, 194u8, 56u8,
                            31u8,
                        ],
                    )
                }
                pub fn planned_validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "PlannedValidators",
                        vec![],
                        [
                            103u8, 160u8, 192u8, 18u8, 133u8, 96u8, 218u8, 99u8, 212u8, 93u8,
                            224u8, 243u8, 140u8, 40u8, 63u8, 61u8, 211u8, 171u8, 49u8, 50u8, 130u8,
                            188u8, 134u8, 128u8, 54u8, 189u8, 120u8, 12u8, 105u8, 159u8, 168u8,
                            255u8,
                        ],
                    )
                }
                pub fn next_notification_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "NextNotificationId",
                        vec![],
                        [
                            116u8, 245u8, 172u8, 65u8, 54u8, 136u8, 117u8, 21u8, 20u8, 36u8, 37u8,
                            8u8, 204u8, 118u8, 33u8, 67u8, 247u8, 121u8, 86u8, 178u8, 130u8, 76u8,
                            18u8, 8u8, 135u8, 34u8, 142u8, 60u8, 197u8, 177u8, 161u8, 64u8,
                        ],
                    )
                }
                pub fn observations(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::pallet_octopus_appchain::ObservationType,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::pallet_octopus_appchain::Observation<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "Observations",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                        ],
                        [
                            197u8, 90u8, 16u8, 29u8, 169u8, 27u8, 55u8, 111u8, 169u8, 144u8, 106u8,
                            222u8, 154u8, 246u8, 62u8, 160u8, 152u8, 200u8, 120u8, 238u8, 128u8,
                            147u8, 213u8, 53u8, 102u8, 5u8, 198u8, 216u8, 151u8, 127u8, 77u8, 41u8,
                        ],
                    )
                }
                pub fn observations_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::pallet_octopus_appchain::Observation<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "Observations",
                        Vec::new(),
                        [
                            197u8, 90u8, 16u8, 29u8, 169u8, 27u8, 55u8, 111u8, 169u8, 144u8, 106u8,
                            222u8, 154u8, 246u8, 62u8, 160u8, 152u8, 200u8, 120u8, 238u8, 128u8,
                            147u8, 213u8, 53u8, 102u8, 5u8, 198u8, 216u8, 151u8, 127u8, 77u8, 41u8,
                        ],
                    )
                }
                pub fn observing(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::pallet_octopus_appchain::Observation<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "Observing",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            34u8, 108u8, 55u8, 222u8, 88u8, 71u8, 16u8, 136u8, 189u8, 207u8, 221u8,
                            188u8, 105u8, 66u8, 202u8, 39u8, 84u8, 132u8, 166u8, 11u8, 77u8, 23u8,
                            150u8, 240u8, 170u8, 117u8, 144u8, 15u8, 33u8, 2u8, 95u8, 157u8,
                        ],
                    )
                }
                pub fn observing_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "Observing",
                        Vec::new(),
                        [
                            34u8, 108u8, 55u8, 222u8, 88u8, 71u8, 16u8, 136u8, 189u8, 207u8, 221u8,
                            188u8, 105u8, 66u8, 202u8, 39u8, 84u8, 132u8, 166u8, 11u8, 77u8, 23u8,
                            150u8, 240u8, 170u8, 117u8, 144u8, 15u8, 33u8, 2u8, 95u8, 157u8,
                        ],
                    )
                }
                pub fn octopus_pallet_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "OctopusPalletId",
                        vec![],
                        [
                            75u8, 75u8, 63u8, 77u8, 173u8, 21u8, 57u8, 219u8, 70u8, 152u8, 62u8,
                            128u8, 20u8, 142u8, 89u8, 97u8, 221u8, 128u8, 108u8, 117u8, 234u8,
                            23u8, 250u8, 93u8, 190u8, 99u8, 250u8, 66u8, 119u8, 106u8, 147u8,
                            243u8,
                        ],
                    )
                }
                pub fn notification_history(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::option::Option<
                            runtime_types::pallet_octopus_appchain::NotificationResult,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "NotificationHistory",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            225u8, 191u8, 108u8, 250u8, 203u8, 210u8, 122u8, 2u8, 99u8, 101u8,
                            88u8, 151u8, 134u8, 3u8, 238u8, 247u8, 178u8, 53u8, 184u8, 103u8,
                            168u8, 195u8, 157u8, 48u8, 119u8, 0u8, 100u8, 39u8, 217u8, 47u8, 146u8,
                            193u8,
                        ],
                    )
                }
                pub fn notification_history_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::option::Option<
                            runtime_types::pallet_octopus_appchain::NotificationResult,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "NotificationHistory",
                        Vec::new(),
                        [
                            225u8, 191u8, 108u8, 250u8, 203u8, 210u8, 122u8, 2u8, 99u8, 101u8,
                            88u8, 151u8, 134u8, 3u8, 238u8, 247u8, 178u8, 53u8, 184u8, 103u8,
                            168u8, 195u8, 157u8, 48u8, 119u8, 0u8, 100u8, 39u8, 217u8, 47u8, 146u8,
                            193u8,
                        ],
                    )
                }
                pub fn git_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAppchain",
                        "GitVersion",
                        vec![],
                        [
                            145u8, 197u8, 251u8, 207u8, 72u8, 121u8, 239u8, 47u8, 19u8, 183u8,
                            210u8, 143u8, 110u8, 184u8, 121u8, 48u8, 171u8, 232u8, 176u8, 111u8,
                            201u8, 110u8, 182u8, 161u8, 126u8, 15u8, 3u8, 79u8, 0u8, 12u8, 205u8,
                            177u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn pallet_id(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::frame_support::PalletId>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAppchain",
                        "PalletId",
                        [
                            139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
                            174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
                            9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
                        ],
                    )
                }
                #[doc = " A grace period after we send transaction."]
                #[doc = ""]
                #[doc = " To avoid sending too many transactions, we only attempt to send one"]
                #[doc = " every `GRACE_PERIOD` blocks. We use Local Storage to coordinate"]
                #[doc = " sending between distinct runs of this offchain worker."]
                pub fn grace_period(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAppchain",
                        "GracePeriod",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " A configuration for base priority of unsigned transactions."]
                #[doc = ""]
                #[doc = " This is exposed so that it can be tuned for particular runtime, when"]
                #[doc = " multiple pallets send unsigned transactions."]
                pub fn unsigned_priority(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAppchain",
                        "UnsignedPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn request_event_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAppchain",
                        "RequestEventLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod octopus_lpos {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ForceSetEraPayout {
                pub era_payout: ::core::primitive::u128,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                ) -> ::subxt::tx::StaticTxPayload<SetHistoryDepth> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusLpos",
                        "set_history_depth",
                        SetHistoryDepth {
                            new_history_depth,
                            era_items_deleted,
                        },
                        [
                            174u8, 55u8, 231u8, 132u8, 219u8, 215u8, 118u8, 202u8, 13u8, 151u8,
                            193u8, 248u8, 141u8, 180u8, 56u8, 103u8, 90u8, 182u8, 194u8, 198u8,
                            120u8, 251u8, 143u8, 218u8, 81u8, 59u8, 13u8, 161u8, 247u8, 57u8,
                            178u8, 122u8,
                        ],
                    )
                }
                pub fn force_set_era_payout(
                    &self,
                    era_payout: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceSetEraPayout> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusLpos",
                        "force_set_era_payout",
                        ForceSetEraPayout { era_payout },
                        [
                            58u8, 27u8, 230u8, 121u8, 74u8, 166u8, 22u8, 186u8, 182u8, 194u8, 93u8,
                            49u8, 64u8, 24u8, 4u8, 158u8, 200u8, 99u8, 38u8, 74u8, 68u8, 42u8,
                            244u8, 42u8, 74u8, 212u8, 135u8, 7u8, 44u8, 140u8, 118u8, 176u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_octopus_lpos::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Notifies the mainchain to prepare the next era."]
            pub struct PlanNewEra {
                pub era_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for PlanNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEra";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Failed to notify the mainchain to prepare the next era."]
            pub struct PlanNewEraFailed;
            impl ::subxt::events::StaticEvent for PlanNewEraFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEraFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Trigger new era."]
            pub struct TriggerNewEra;
            impl ::subxt::events::StaticEvent for TriggerNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "TriggerNewEra";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Notifies the mainchain to pay the validator rewards of `era_index`."]
            #[doc = "`excluded_validators` were excluded because they were not working properly."]
            pub struct EraPayout {
                pub era_index: ::core::primitive::u32,
                pub excluded_validators:
                    ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayout";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Failed to notify the mainchain to pay the validator rewards of `era_index`."]
            pub struct EraPayoutFailed {
                pub era_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for EraPayoutFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayoutFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Number of eras to keep in history."]
                #[doc = ""]
                #[doc = " Information is kept for eras in `[current_era - history_depth; current_era]`."]
                #[doc = ""]
                #[doc = " Must be more than the number of eras delayed by session otherwise. I.e. active era must"]
                #[doc = " always be in history. I.e. `active_era > current_era - history_depth` must be"]
                #[doc = " guaranteed."]
                pub fn history_depth(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "HistoryDepth",
                        vec![],
                        [
                            41u8, 54u8, 118u8, 245u8, 75u8, 136u8, 220u8, 25u8, 55u8, 255u8, 149u8,
                            177u8, 49u8, 155u8, 167u8, 188u8, 170u8, 29u8, 251u8, 44u8, 240u8,
                            250u8, 225u8, 205u8, 102u8, 74u8, 25u8, 47u8, 52u8, 235u8, 204u8,
                            167u8,
                        ],
                    )
                }
                #[doc = " The current era index."]
                #[doc = ""]
                #[doc = " This is the latest planned era, depending on how the Session pallet queues the validator"]
                #[doc = " set, it might be active or not."]
                pub fn current_era(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "CurrentEra",
                        vec![],
                        [
                            105u8, 150u8, 49u8, 122u8, 4u8, 78u8, 8u8, 121u8, 34u8, 136u8, 157u8,
                            227u8, 59u8, 139u8, 7u8, 253u8, 7u8, 10u8, 117u8, 71u8, 240u8, 74u8,
                            86u8, 36u8, 198u8, 37u8, 153u8, 93u8, 196u8, 22u8, 192u8, 243u8,
                        ],
                    )
                }
                #[doc = " The active era information, it holds index and start."]
                #[doc = ""]
                #[doc = " The active era is the era being currently rewarded. Validator set of this era must be"]
                #[doc = " equal to [`SessionInterface::validators`]."]
                pub fn active_era(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_octopus_lpos::ActiveEraInfo,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ActiveEra",
                        vec![],
                        [
                            10u8, 172u8, 177u8, 21u8, 176u8, 70u8, 160u8, 180u8, 27u8, 43u8, 140u8,
                            13u8, 200u8, 5u8, 127u8, 249u8, 51u8, 156u8, 240u8, 192u8, 190u8,
                            197u8, 23u8, 5u8, 38u8, 127u8, 34u8, 199u8, 161u8, 178u8, 75u8, 124u8,
                        ],
                    )
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
                pub fn eras_start_session_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasStartSessionIndex",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            92u8, 157u8, 168u8, 144u8, 132u8, 3u8, 212u8, 80u8, 230u8, 229u8,
                            251u8, 218u8, 97u8, 55u8, 79u8, 100u8, 163u8, 91u8, 32u8, 246u8, 122u8,
                            78u8, 149u8, 214u8, 103u8, 249u8, 119u8, 20u8, 101u8, 116u8, 110u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
                pub fn eras_start_session_index_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasStartSessionIndex",
                        Vec::new(),
                        [
                            92u8, 157u8, 168u8, 144u8, 132u8, 3u8, 212u8, 80u8, 230u8, 229u8,
                            251u8, 218u8, 97u8, 55u8, 79u8, 100u8, 163u8, 91u8, 32u8, 246u8, 122u8,
                            78u8, 149u8, 214u8, 103u8, 249u8, 119u8, 20u8, 101u8, 116u8, 110u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasStakers",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                        ],
                        [
                            57u8, 126u8, 174u8, 228u8, 118u8, 104u8, 66u8, 138u8, 8u8, 134u8,
                            183u8, 80u8, 68u8, 156u8, 235u8, 60u8, 254u8, 181u8, 6u8, 5u8, 149u8,
                            32u8, 19u8, 18u8, 236u8, 244u8, 145u8, 248u8, 134u8, 185u8, 109u8,
                            106u8,
                        ],
                    )
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasStakers",
                        Vec::new(),
                        [
                            57u8, 126u8, 174u8, 228u8, 118u8, 104u8, 66u8, 138u8, 8u8, 134u8,
                            183u8, 80u8, 68u8, 156u8, 235u8, 60u8, 254u8, 181u8, 6u8, 5u8, 149u8,
                            32u8, 19u8, 18u8, 236u8, 244u8, 145u8, 248u8, 134u8, 185u8, 109u8,
                            106u8,
                        ],
                    )
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
                pub fn eras_validator_reward(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasValidatorReward",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            87u8, 80u8, 156u8, 123u8, 107u8, 77u8, 203u8, 37u8, 231u8, 84u8, 124u8,
                            155u8, 227u8, 212u8, 212u8, 179u8, 84u8, 161u8, 223u8, 255u8, 254u8,
                            107u8, 52u8, 89u8, 98u8, 169u8, 136u8, 241u8, 104u8, 3u8, 244u8, 161u8,
                        ],
                    )
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
                pub fn eras_validator_reward_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasValidatorReward",
                        Vec::new(),
                        [
                            87u8, 80u8, 156u8, 123u8, 107u8, 77u8, 203u8, 37u8, 231u8, 84u8, 124u8,
                            155u8, 227u8, 212u8, 212u8, 179u8, 84u8, 161u8, 223u8, 255u8, 254u8,
                            107u8, 52u8, 89u8, 98u8, 169u8, 136u8, 241u8, 104u8, 3u8, 244u8, 161u8,
                        ],
                    )
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
                pub fn eras_reward_points(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_octopus_lpos::EraRewardPoints<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasRewardPoints",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            194u8, 29u8, 20u8, 83u8, 200u8, 47u8, 158u8, 102u8, 88u8, 65u8, 24u8,
                            255u8, 120u8, 178u8, 23u8, 232u8, 15u8, 64u8, 206u8, 0u8, 170u8, 40u8,
                            18u8, 149u8, 45u8, 90u8, 179u8, 127u8, 52u8, 59u8, 37u8, 192u8,
                        ],
                    )
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
                pub fn eras_reward_points_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_octopus_lpos::EraRewardPoints<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasRewardPoints",
                        Vec::new(),
                        [
                            194u8, 29u8, 20u8, 83u8, 200u8, 47u8, 158u8, 102u8, 88u8, 65u8, 24u8,
                            255u8, 120u8, 178u8, 23u8, 232u8, 15u8, 64u8, 206u8, 0u8, 170u8, 40u8,
                            18u8, 149u8, 45u8, 90u8, 179u8, 127u8, 52u8, 59u8, 37u8, 192u8,
                        ],
                    )
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
                pub fn eras_total_stake(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasTotalStake",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            224u8, 240u8, 168u8, 69u8, 148u8, 140u8, 249u8, 240u8, 4u8, 46u8, 77u8,
                            11u8, 224u8, 65u8, 26u8, 239u8, 1u8, 110u8, 53u8, 11u8, 247u8, 235u8,
                            142u8, 234u8, 22u8, 43u8, 24u8, 36u8, 37u8, 43u8, 170u8, 40u8,
                        ],
                    )
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
                pub fn eras_total_stake_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "ErasTotalStake",
                        Vec::new(),
                        [
                            224u8, 240u8, 168u8, 69u8, 148u8, 140u8, 249u8, 240u8, 4u8, 46u8, 77u8,
                            11u8, 224u8, 65u8, 26u8, 239u8, 1u8, 110u8, 53u8, 11u8, 247u8, 235u8,
                            142u8, 234u8, 22u8, 43u8, 24u8, 36u8, 37u8, 43u8, 170u8, 40u8,
                        ],
                    )
                }
                #[doc = " A mapping from still-bonded eras to the first session index of that era."]
                #[doc = ""]
                #[doc = " Must contains information for eras for the range:"]
                #[doc = " `[active_era - bounding_duration; active_era]`"]
                pub fn bonded_eras(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "BondedEras",
                        vec![],
                        [
                            243u8, 162u8, 236u8, 198u8, 122u8, 182u8, 37u8, 55u8, 171u8, 156u8,
                            235u8, 223u8, 226u8, 129u8, 89u8, 206u8, 2u8, 155u8, 222u8, 154u8,
                            116u8, 124u8, 4u8, 119u8, 155u8, 94u8, 248u8, 30u8, 171u8, 51u8, 78u8,
                            106u8,
                        ],
                    )
                }
                #[doc = " The last planned session scheduled by the session pallet."]
                #[doc = ""]
                #[doc = " This is basically in sync with the call to [`SessionManager::new_session`]."]
                pub fn current_planned_session(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "CurrentPlannedSession",
                        vec![],
                        [
                            38u8, 22u8, 56u8, 250u8, 17u8, 154u8, 99u8, 37u8, 155u8, 253u8, 100u8,
                            117u8, 5u8, 239u8, 31u8, 190u8, 53u8, 241u8, 11u8, 185u8, 163u8, 227u8,
                            10u8, 77u8, 210u8, 64u8, 156u8, 218u8, 105u8, 16u8, 1u8, 57u8,
                        ],
                    )
                }
                #[doc = " The payout for validators and the system for the current era."]
                pub fn era_payout(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusLpos",
                        "EraPayout",
                        vec![],
                        [
                            170u8, 205u8, 212u8, 96u8, 48u8, 137u8, 153u8, 23u8, 221u8, 85u8, 55u8,
                            48u8, 57u8, 151u8, 98u8, 86u8, 82u8, 140u8, 29u8, 186u8, 244u8, 84u8,
                            188u8, 25u8, 69u8, 183u8, 21u8, 189u8, 164u8, 101u8, 25u8, 233u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Number of sessions per era."]
                pub fn sessions_per_era(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusLpos",
                        "SessionsPerEra",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn blocks_per_era(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusLpos",
                        "BlocksPerEra",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of eras that staked funds must remain bonded for."]
                pub fn bonding_duration(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusLpos",
                        "BondingDuration",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod octopus_upward_messages {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi;
            impl TransactionApi {}
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_octopus_upward_messages::pallet::Event;
        pub mod events {
            use super::runtime_types;
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn message_queue(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUpwardMessages",
                        "MessageQueue",
                        vec![],
                        [
                            135u8, 173u8, 92u8, 141u8, 240u8, 206u8, 55u8, 65u8, 22u8, 207u8,
                            252u8, 131u8, 151u8, 100u8, 92u8, 84u8, 172u8, 43u8, 101u8, 218u8,
                            25u8, 172u8, 102u8, 212u8, 28u8, 26u8, 177u8, 12u8, 227u8, 245u8,
                            175u8, 188u8,
                        ],
                    )
                }
                pub fn nonce(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUpwardMessages",
                        "Nonce",
                        vec![],
                        [
                            122u8, 169u8, 95u8, 131u8, 85u8, 32u8, 154u8, 114u8, 143u8, 56u8, 12u8,
                            182u8, 64u8, 150u8, 241u8, 249u8, 254u8, 251u8, 160u8, 235u8, 192u8,
                            41u8, 101u8, 232u8, 186u8, 108u8, 187u8, 149u8, 210u8, 91u8, 179u8,
                            98u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The limit for submit messages."]
                pub fn upward_messages_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUpwardMessages",
                        "UpwardMessagesLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod octopus_assets {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Create {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub admin: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub min_balance: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceCreate {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub is_sufficient: ::core::primitive::bool,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Destroy {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub witness: runtime_types::pallet_assets::types::DestroyWitness,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Mint {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Burn {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Transfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferKeepAlive {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub source: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Freeze {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Thaw {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FreezeAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ThawAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetTeam {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub issuer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub admin: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub freezer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceSetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub issuer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub admin: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub freezer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u128,
                pub is_sufficient: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceCancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub delegate: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferApproved {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub destination: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Touch {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Refund {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub allow_burn: ::core::primitive::bool,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                    admin: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Create> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "create",
                        Create {
                            id,
                            admin,
                            min_balance,
                        },
                        [
                            173u8, 91u8, 250u8, 119u8, 145u8, 115u8, 29u8, 163u8, 99u8, 95u8, 89u8,
                            231u8, 200u8, 205u8, 3u8, 226u8, 144u8, 66u8, 168u8, 39u8, 63u8, 69u8,
                            255u8, 116u8, 61u8, 67u8, 195u8, 219u8, 102u8, 112u8, 155u8, 67u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    is_sufficient: ::core::primitive::bool,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceCreate> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "force_create",
                        ForceCreate {
                            id,
                            owner,
                            is_sufficient,
                            min_balance,
                        },
                        [
                            45u8, 129u8, 55u8, 141u8, 100u8, 83u8, 74u8, 183u8, 70u8, 83u8, 158u8,
                            89u8, 86u8, 102u8, 228u8, 71u8, 182u8, 43u8, 22u8, 126u8, 42u8, 195u8,
                            204u8, 173u8, 178u8, 166u8, 155u8, 105u8, 13u8, 178u8, 4u8, 254u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<Destroy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "destroy",
                        Destroy { id, witness },
                        [
                            243u8, 230u8, 96u8, 223u8, 56u8, 13u8, 69u8, 28u8, 165u8, 163u8, 192u8,
                            203u8, 100u8, 170u8, 10u8, 85u8, 85u8, 144u8, 108u8, 32u8, 64u8, 84u8,
                            149u8, 15u8, 75u8, 57u8, 24u8, 249u8, 146u8, 157u8, 52u8, 166u8,
                        ],
                    )
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
                    beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Mint> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "mint",
                        Mint {
                            id,
                            beneficiary,
                            amount,
                        },
                        [
                            142u8, 88u8, 145u8, 221u8, 194u8, 149u8, 206u8, 99u8, 206u8, 71u8,
                            101u8, 130u8, 175u8, 218u8, 130u8, 9u8, 169u8, 28u8, 82u8, 41u8, 102u8,
                            159u8, 131u8, 145u8, 249u8, 54u8, 38u8, 168u8, 48u8, 15u8, 2u8, 96u8,
                        ],
                    )
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
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Burn> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "burn",
                        Burn { id, who, amount },
                        [
                            27u8, 30u8, 250u8, 220u8, 38u8, 224u8, 142u8, 28u8, 70u8, 122u8, 241u8,
                            79u8, 31u8, 163u8, 54u8, 87u8, 44u8, 6u8, 14u8, 161u8, 32u8, 181u8,
                            94u8, 117u8, 34u8, 161u8, 97u8, 161u8, 7u8, 163u8, 223u8, 124u8,
                        ],
                    )
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
                    target: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "transfer",
                        Transfer { id, target, amount },
                        [
                            211u8, 37u8, 7u8, 179u8, 233u8, 146u8, 149u8, 140u8, 54u8, 97u8, 141u8,
                            213u8, 149u8, 84u8, 127u8, 185u8, 205u8, 93u8, 119u8, 179u8, 47u8,
                            112u8, 7u8, 17u8, 94u8, 125u8, 44u8, 28u8, 103u8, 17u8, 209u8, 61u8,
                        ],
                    )
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
                    target: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "transfer_keep_alive",
                        TransferKeepAlive { id, target, amount },
                        [
                            45u8, 221u8, 40u8, 14u8, 110u8, 12u8, 134u8, 20u8, 220u8, 73u8, 131u8,
                            43u8, 6u8, 214u8, 34u8, 13u8, 200u8, 198u8, 44u8, 150u8, 58u8, 252u8,
                            2u8, 136u8, 238u8, 253u8, 118u8, 238u8, 241u8, 172u8, 151u8, 153u8,
                        ],
                    )
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
                    source: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "force_transfer",
                        ForceTransfer {
                            id,
                            source,
                            dest,
                            amount,
                        },
                        [
                            203u8, 81u8, 11u8, 97u8, 79u8, 101u8, 170u8, 89u8, 107u8, 10u8, 220u8,
                            133u8, 229u8, 94u8, 228u8, 255u8, 216u8, 239u8, 161u8, 15u8, 50u8,
                            113u8, 6u8, 131u8, 107u8, 60u8, 112u8, 146u8, 245u8, 67u8, 15u8, 220u8,
                        ],
                    )
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
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Freeze> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "freeze",
                        Freeze { id, who },
                        [
                            9u8, 164u8, 132u8, 206u8, 71u8, 56u8, 255u8, 255u8, 169u8, 236u8, 79u8,
                            148u8, 201u8, 242u8, 125u8, 120u8, 179u8, 148u8, 225u8, 7u8, 139u8,
                            193u8, 33u8, 68u8, 61u8, 133u8, 230u8, 13u8, 232u8, 2u8, 235u8, 112u8,
                        ],
                    )
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
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Thaw> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "thaw",
                        Thaw { id, who },
                        [
                            121u8, 134u8, 54u8, 1u8, 81u8, 234u8, 61u8, 112u8, 120u8, 213u8, 153u8,
                            137u8, 206u8, 129u8, 87u8, 90u8, 135u8, 211u8, 151u8, 2u8, 195u8, 40u8,
                            218u8, 16u8, 87u8, 119u8, 204u8, 180u8, 97u8, 233u8, 14u8, 168u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<FreezeAsset> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "freeze_asset",
                        FreezeAsset { id },
                        [
                            208u8, 101u8, 0u8, 73u8, 41u8, 192u8, 227u8, 44u8, 189u8, 231u8, 40u8,
                            124u8, 189u8, 147u8, 136u8, 210u8, 76u8, 32u8, 249u8, 183u8, 68u8,
                            58u8, 150u8, 136u8, 192u8, 47u8, 173u8, 178u8, 225u8, 84u8, 110u8, 1u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ThawAsset> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "thaw_asset",
                        ThawAsset { id },
                        [
                            18u8, 198u8, 141u8, 158u8, 182u8, 167u8, 160u8, 227u8, 20u8, 74u8,
                            80u8, 164u8, 89u8, 46u8, 168u8, 139u8, 251u8, 83u8, 155u8, 91u8, 91u8,
                            46u8, 205u8, 55u8, 171u8, 175u8, 167u8, 188u8, 116u8, 155u8, 79u8,
                            117u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<TransferOwnership> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "transfer_ownership",
                        TransferOwnership { id, owner },
                        [
                            146u8, 254u8, 44u8, 100u8, 99u8, 215u8, 140u8, 15u8, 152u8, 73u8, 84u8,
                            213u8, 7u8, 176u8, 63u8, 202u8, 58u8, 94u8, 133u8, 58u8, 191u8, 108u8,
                            137u8, 137u8, 76u8, 131u8, 145u8, 188u8, 241u8, 45u8, 88u8, 87u8,
                        ],
                    )
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
                    issuer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetTeam> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "set_team",
                        SetTeam {
                            id,
                            issuer,
                            admin,
                            freezer,
                        },
                        [
                            206u8, 78u8, 41u8, 85u8, 189u8, 77u8, 76u8, 150u8, 213u8, 233u8, 68u8,
                            12u8, 75u8, 181u8, 158u8, 105u8, 158u8, 209u8, 94u8, 155u8, 100u8,
                            91u8, 95u8, 77u8, 10u8, 192u8, 138u8, 243u8, 42u8, 155u8, 253u8, 165u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<SetMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "set_metadata",
                        SetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                        },
                        [
                            15u8, 184u8, 50u8, 46u8, 164u8, 27u8, 105u8, 186u8, 35u8, 115u8, 194u8,
                            247u8, 74u8, 252u8, 139u8, 242u8, 108u8, 221u8, 122u8, 15u8, 139u8,
                            74u8, 123u8, 17u8, 192u8, 138u8, 182u8, 163u8, 77u8, 7u8, 124u8, 18u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ClearMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "clear_metadata",
                        ClearMetadata { id },
                        [
                            192u8, 41u8, 71u8, 183u8, 13u8, 128u8, 244u8, 255u8, 175u8, 36u8, 99u8,
                            175u8, 15u8, 129u8, 228u8, 76u8, 107u8, 214u8, 166u8, 116u8, 244u8,
                            139u8, 60u8, 31u8, 123u8, 61u8, 203u8, 59u8, 213u8, 146u8, 116u8,
                            126u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ForceSetMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "force_set_metadata",
                        ForceSetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                            is_frozen,
                        },
                        [
                            7u8, 30u8, 55u8, 233u8, 217u8, 113u8, 196u8, 21u8, 29u8, 122u8, 168u8,
                            225u8, 63u8, 104u8, 57u8, 78u8, 76u8, 145u8, 121u8, 118u8, 91u8, 149u8,
                            87u8, 26u8, 26u8, 125u8, 44u8, 241u8, 143u8, 138u8, 144u8, 8u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ForceClearMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "force_clear_metadata",
                        ForceClearMetadata { id },
                        [
                            71u8, 191u8, 101u8, 72u8, 188u8, 223u8, 215u8, 187u8, 200u8, 206u8,
                            3u8, 42u8, 4u8, 62u8, 117u8, 106u8, 26u8, 2u8, 68u8, 202u8, 162u8,
                            142u8, 172u8, 123u8, 48u8, 196u8, 247u8, 89u8, 147u8, 75u8, 84u8,
                            109u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    issuer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                    is_sufficient: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<ForceAssetStatus> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "force_asset_status",
                        ForceAssetStatus {
                            id,
                            owner,
                            issuer,
                            admin,
                            freezer,
                            min_balance,
                            is_sufficient,
                            is_frozen,
                        },
                        [
                            181u8, 168u8, 215u8, 229u8, 27u8, 78u8, 26u8, 171u8, 50u8, 95u8, 9u8,
                            112u8, 142u8, 125u8, 230u8, 68u8, 188u8, 24u8, 208u8, 203u8, 226u8,
                            17u8, 231u8, 69u8, 172u8, 24u8, 119u8, 22u8, 232u8, 11u8, 70u8, 248u8,
                        ],
                    )
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
                    delegate: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ApproveTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "approve_transfer",
                        ApproveTransfer {
                            id,
                            delegate,
                            amount,
                        },
                        [
                            188u8, 247u8, 242u8, 152u8, 209u8, 38u8, 128u8, 25u8, 79u8, 17u8, 31u8,
                            236u8, 171u8, 237u8, 175u8, 49u8, 86u8, 157u8, 164u8, 220u8, 5u8,
                            225u8, 124u8, 157u8, 174u8, 61u8, 39u8, 78u8, 22u8, 2u8, 37u8, 31u8,
                        ],
                    )
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
                    delegate: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<CancelApproval> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "cancel_approval",
                        CancelApproval { id, delegate },
                        [
                            176u8, 30u8, 130u8, 224u8, 220u8, 236u8, 186u8, 160u8, 21u8, 177u8,
                            57u8, 65u8, 12u8, 85u8, 195u8, 254u8, 189u8, 180u8, 229u8, 25u8, 240u8,
                            200u8, 101u8, 223u8, 110u8, 66u8, 246u8, 81u8, 44u8, 135u8, 228u8,
                            220u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    delegate: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ForceCancelApproval> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "force_cancel_approval",
                        ForceCancelApproval {
                            id,
                            owner,
                            delegate,
                        },
                        [
                            6u8, 80u8, 184u8, 209u8, 50u8, 16u8, 2u8, 236u8, 101u8, 140u8, 94u8,
                            0u8, 56u8, 77u8, 119u8, 220u8, 141u8, 144u8, 82u8, 189u8, 6u8, 52u8,
                            212u8, 102u8, 170u8, 143u8, 171u8, 140u8, 150u8, 86u8, 247u8, 17u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    destination: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TransferApproved> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "transfer_approved",
                        TransferApproved {
                            id,
                            owner,
                            destination,
                            amount,
                        },
                        [
                            159u8, 239u8, 168u8, 140u8, 203u8, 198u8, 2u8, 11u8, 113u8, 160u8,
                            63u8, 131u8, 204u8, 70u8, 84u8, 41u8, 161u8, 166u8, 87u8, 79u8, 106u8,
                            14u8, 136u8, 53u8, 14u8, 239u8, 28u8, 188u8, 172u8, 242u8, 249u8,
                            129u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<Touch> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "touch",
                        Touch { id },
                        [
                            114u8, 149u8, 179u8, 168u8, 115u8, 117u8, 32u8, 50u8, 39u8, 77u8,
                            148u8, 238u8, 123u8, 96u8, 193u8, 174u8, 113u8, 141u8, 34u8, 228u8,
                            228u8, 214u8, 71u8, 111u8, 55u8, 126u8, 103u8, 181u8, 133u8, 77u8,
                            116u8, 105u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<Refund> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusAssets",
                        "refund",
                        Refund { id, allow_burn },
                        [
                            20u8, 139u8, 248u8, 67u8, 123u8, 221u8, 7u8, 106u8, 239u8, 156u8, 68u8,
                            59u8, 81u8, 184u8, 47u8, 188u8, 195u8, 227u8, 75u8, 168u8, 126u8,
                            176u8, 91u8, 187u8, 30u8, 34u8, 24u8, 223u8, 108u8, 101u8, 88u8, 83u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some asset class was created."]
            pub struct Created {
                pub asset_id: ::core::primitive::u32,
                pub creator: ::subxt::ext::sp_core::crypto::AccountId32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Created {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some assets were issued."]
            pub struct Issued {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
                pub total_supply: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some assets were transferred."]
            pub struct Transferred {
                pub asset_id: ::core::primitive::u32,
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transferred {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some assets were destroyed."]
            pub struct Burned {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Burned {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The management team changed."]
            pub struct TeamChanged {
                pub asset_id: ::core::primitive::u32,
                pub issuer: ::subxt::ext::sp_core::crypto::AccountId32,
                pub admin: ::subxt::ext::sp_core::crypto::AccountId32,
                pub freezer: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for TeamChanged {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The owner changed."]
            pub struct OwnerChanged {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for OwnerChanged {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some account `who` was frozen."]
            pub struct Frozen {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some account `who` was thawed."]
            pub struct Thawed {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `asset_id` was frozen."]
            pub struct AssetFrozen {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for AssetFrozen {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `asset_id` was thawed."]
            pub struct AssetThawed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for AssetThawed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset class was destroyed."]
            pub struct Destroyed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Destroyed {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some asset class was force-created."]
            pub struct ForceCreated {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ForceCreated {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "New metadata has been set for an asset."]
            pub struct MetadataSet {
                pub asset_id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::events::StaticEvent for MetadataSet {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Metadata has been cleared for an asset."]
            pub struct MetadataCleared {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for MetadataCleared {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "(Additional) funds have been approved for transfer to a destination account."]
            pub struct ApprovedTransfer {
                pub asset_id: ::core::primitive::u32,
                pub source: ::subxt::ext::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for ApprovedTransfer {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An approval for account `delegate` was cancelled by `owner`."]
            pub struct ApprovalCancelled {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ApprovalCancelled {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An `amount` was transferred in its entirety from `owner` to `destination` by"]
            #[doc = "the approved `delegate`."]
            pub struct TransferredApproved {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                pub destination: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransferredApproved {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset has had its attributes changed by the `Force` origin."]
            pub struct AssetStatusChanged {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for AssetStatusChanged {
                const PALLET: &'static str = "OctopusAssets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Details of an asset."]
                pub fn asset(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u128,
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Asset",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            65u8, 19u8, 120u8, 233u8, 154u8, 59u8, 71u8, 35u8, 10u8, 35u8, 125u8,
                            99u8, 186u8, 18u8, 239u8, 118u8, 169u8, 104u8, 80u8, 204u8, 85u8,
                            193u8, 145u8, 83u8, 132u8, 19u8, 117u8, 227u8, 67u8, 62u8, 123u8,
                            109u8,
                        ],
                    )
                }
                #[doc = " Details of an asset."]
                pub fn asset_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u128,
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Asset",
                        Vec::new(),
                        [
                            65u8, 19u8, 120u8, 233u8, 154u8, 59u8, 71u8, 35u8, 10u8, 35u8, 125u8,
                            99u8, 186u8, 18u8, 239u8, 118u8, 169u8, 104u8, 80u8, 204u8, 85u8,
                            193u8, 145u8, 83u8, 132u8, 19u8, 117u8, 227u8, 67u8, 62u8, 123u8,
                            109u8,
                        ],
                    )
                }
                #[doc = " The holdings of a specific account for a specific asset."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::AssetAccount<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                            (),
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Account",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            109u8, 245u8, 93u8, 133u8, 206u8, 68u8, 94u8, 233u8, 29u8, 113u8,
                            245u8, 201u8, 241u8, 2u8, 200u8, 179u8, 37u8, 199u8, 128u8, 243u8,
                            49u8, 50u8, 122u8, 139u8, 135u8, 48u8, 201u8, 109u8, 195u8, 38u8,
                            205u8, 32u8,
                        ],
                    )
                }
                #[doc = " The holdings of a specific account for a specific asset."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::AssetAccount<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                            (),
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Account",
                        Vec::new(),
                        [
                            109u8, 245u8, 93u8, 133u8, 206u8, 68u8, 94u8, 233u8, 29u8, 113u8,
                            245u8, 201u8, 241u8, 2u8, 200u8, 179u8, 37u8, 199u8, 128u8, 243u8,
                            49u8, 50u8, 122u8, 139u8, 135u8, 48u8, 201u8, 109u8, 195u8, 38u8,
                            205u8, 32u8,
                        ],
                    )
                }
                #[doc = " Approved balance transfers. First balance is the amount approved for transfer. Second"]
                #[doc = " is the amount of `T::Currency` reserved for storing this."]
                #[doc = " First key is the asset ID, second key is the owner and third key is the delegate."]
                pub fn approvals(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                    _2: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Approvals",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _2.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            210u8, 147u8, 203u8, 49u8, 232u8, 215u8, 116u8, 154u8, 43u8, 154u8,
                            69u8, 159u8, 241u8, 28u8, 238u8, 101u8, 108u8, 162u8, 242u8, 121u8,
                            138u8, 164u8, 217u8, 243u8, 72u8, 173u8, 75u8, 109u8, 194u8, 9u8,
                            196u8, 163u8,
                        ],
                    )
                }
                #[doc = " Approved balance transfers. First balance is the amount approved for transfer. Second"]
                #[doc = " is the amount of `T::Currency` reserved for storing this."]
                #[doc = " First key is the asset ID, second key is the owner and third key is the delegate."]
                pub fn approvals_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Approvals",
                        Vec::new(),
                        [
                            210u8, 147u8, 203u8, 49u8, 232u8, 215u8, 116u8, 154u8, 43u8, 154u8,
                            69u8, 159u8, 241u8, 28u8, 238u8, 101u8, 108u8, 162u8, 242u8, 121u8,
                            138u8, 164u8, 217u8, 243u8, 72u8, 173u8, 75u8, 109u8, 194u8, 9u8,
                            196u8, 163u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset."]
                pub fn metadata(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::AssetMetadata<
                            ::core::primitive::u128,
                            runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Metadata",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            80u8, 115u8, 155u8, 115u8, 136u8, 108u8, 82u8, 93u8, 65u8, 130u8,
                            143u8, 228u8, 170u8, 234u8, 182u8, 170u8, 229u8, 217u8, 168u8, 71u8,
                            81u8, 80u8, 16u8, 112u8, 209u8, 82u8, 8u8, 165u8, 80u8, 137u8, 58u8,
                            170u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset."]
                pub fn metadata_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_assets::types::AssetMetadata<
                            ::core::primitive::u128,
                            runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusAssets",
                        "Metadata",
                        Vec::new(),
                        [
                            80u8, 115u8, 155u8, 115u8, 136u8, 108u8, 82u8, 93u8, 65u8, 130u8,
                            143u8, 228u8, 170u8, 234u8, 182u8, 170u8, 229u8, 217u8, 168u8, 71u8,
                            81u8, 80u8, 16u8, 112u8, 209u8, 82u8, 8u8, 165u8, 80u8, 137u8, 58u8,
                            170u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The basic amount of funds that must be reserved for an asset."]
                pub fn asset_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAssets",
                        "AssetDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of funds that must be reserved for a non-provider asset account to be"]
                #[doc = " maintained."]
                pub fn asset_account_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAssets",
                        "AssetAccountDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The basic amount of funds that must be reserved when adding metadata to your asset."]
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAssets",
                        "MetadataDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The additional funds that must be reserved for the number of bytes you store in your"]
                #[doc = " metadata."]
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAssets",
                        "MetadataDepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of funds that must be reserved when creating a new approval."]
                pub fn approval_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAssets",
                        "ApprovalDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a name or symbol stored on-chain."]
                pub fn string_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusAssets",
                        "StringLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod octopus_uniques {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Create {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub admin: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceCreate {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub free_holding: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Destroy {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub witness: runtime_types::pallet_uniques::types::DestroyWitness,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Mint {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Burn {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub check_owner: ::core::option::Option<
                    ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Transfer {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Redeposit {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub instances: ::std::vec::Vec<::core::primitive::u128>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Freeze {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Thaw {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FreezeClass {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ThawClass {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetTeam {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub issuer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub admin: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub freezer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub delegate: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CancelApproval {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
                pub maybe_check_delegate: ::core::option::Option<
                    ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub issuer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub admin: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub freezer: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub free_holding: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClearAttribute {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub maybe_instance: ::core::option::Option<::core::primitive::u128>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                #[codec(compact)]
                pub instance: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClearClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u128,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                    admin: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Create> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "create",
                        Create { class, admin },
                        [
                            7u8, 222u8, 31u8, 126u8, 187u8, 32u8, 129u8, 145u8, 233u8, 226u8,
                            130u8, 205u8, 87u8, 47u8, 48u8, 85u8, 92u8, 69u8, 253u8, 176u8, 76u8,
                            112u8, 219u8, 173u8, 42u8, 150u8, 250u8, 232u8, 186u8, 43u8, 170u8,
                            5u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    free_holding: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<ForceCreate> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "force_create",
                        ForceCreate {
                            class,
                            owner,
                            free_holding,
                        },
                        [
                            63u8, 101u8, 100u8, 174u8, 199u8, 178u8, 8u8, 149u8, 71u8, 134u8,
                            232u8, 195u8, 220u8, 134u8, 6u8, 238u8, 141u8, 160u8, 202u8, 183u8,
                            183u8, 101u8, 56u8, 68u8, 200u8, 248u8, 114u8, 187u8, 169u8, 124u8,
                            84u8, 251u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<Destroy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "destroy",
                        Destroy { class, witness },
                        [
                            95u8, 172u8, 171u8, 186u8, 50u8, 22u8, 38u8, 206u8, 76u8, 140u8, 56u8,
                            51u8, 114u8, 91u8, 85u8, 173u8, 89u8, 93u8, 77u8, 3u8, 34u8, 198u8,
                            161u8, 189u8, 101u8, 43u8, 51u8, 128u8, 203u8, 75u8, 117u8, 34u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Mint> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "mint",
                        Mint {
                            class,
                            instance,
                            owner,
                        },
                        [
                            129u8, 222u8, 0u8, 121u8, 231u8, 108u8, 26u8, 116u8, 198u8, 209u8,
                            169u8, 129u8, 145u8, 165u8, 28u8, 191u8, 86u8, 92u8, 237u8, 223u8,
                            63u8, 48u8, 125u8, 126u8, 29u8, 113u8, 247u8, 75u8, 107u8, 108u8,
                            245u8, 42u8,
                        ],
                    )
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
                        ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Burn> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "burn",
                        Burn {
                            class,
                            instance,
                            check_owner,
                        },
                        [
                            62u8, 164u8, 86u8, 241u8, 198u8, 98u8, 85u8, 201u8, 66u8, 27u8, 87u8,
                            43u8, 107u8, 10u8, 186u8, 120u8, 123u8, 61u8, 111u8, 219u8, 195u8,
                            112u8, 20u8, 105u8, 237u8, 6u8, 198u8, 206u8, 204u8, 93u8, 179u8, 26u8,
                        ],
                    )
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
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "transfer",
                        Transfer {
                            class,
                            instance,
                            dest,
                        },
                        [
                            167u8, 207u8, 158u8, 144u8, 213u8, 143u8, 35u8, 191u8, 209u8, 157u8,
                            111u8, 116u8, 87u8, 220u8, 59u8, 120u8, 195u8, 64u8, 82u8, 27u8, 161u8,
                            28u8, 111u8, 211u8, 233u8, 125u8, 79u8, 206u8, 26u8, 174u8, 104u8,
                            64u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<Redeposit> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "redeposit",
                        Redeposit { class, instances },
                        [
                            164u8, 220u8, 233u8, 106u8, 166u8, 205u8, 145u8, 91u8, 21u8, 231u8,
                            79u8, 85u8, 59u8, 131u8, 195u8, 121u8, 205u8, 150u8, 253u8, 244u8, 5u8,
                            188u8, 33u8, 18u8, 193u8, 130u8, 171u8, 181u8, 117u8, 253u8, 139u8,
                            65u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<Freeze> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "freeze",
                        Freeze { class, instance },
                        [
                            54u8, 30u8, 245u8, 36u8, 175u8, 167u8, 3u8, 232u8, 228u8, 7u8, 116u8,
                            191u8, 65u8, 108u8, 137u8, 15u8, 253u8, 131u8, 87u8, 246u8, 225u8,
                            92u8, 21u8, 133u8, 183u8, 250u8, 7u8, 87u8, 152u8, 157u8, 127u8, 111u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<Thaw> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "thaw",
                        Thaw { class, instance },
                        [
                            226u8, 215u8, 252u8, 211u8, 109u8, 166u8, 120u8, 218u8, 165u8, 243u8,
                            237u8, 89u8, 94u8, 23u8, 12u8, 73u8, 70u8, 217u8, 83u8, 246u8, 98u8,
                            100u8, 175u8, 209u8, 52u8, 213u8, 150u8, 50u8, 165u8, 193u8, 157u8,
                            168u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<FreezeClass> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "freeze_class",
                        FreezeClass { class },
                        [
                            219u8, 58u8, 72u8, 221u8, 105u8, 144u8, 28u8, 227u8, 15u8, 187u8,
                            216u8, 152u8, 248u8, 215u8, 176u8, 82u8, 15u8, 18u8, 114u8, 96u8,
                            227u8, 47u8, 154u8, 230u8, 231u8, 44u8, 67u8, 251u8, 232u8, 95u8,
                            171u8, 113u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ThawClass> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "thaw_class",
                        ThawClass { class },
                        [
                            95u8, 85u8, 97u8, 214u8, 111u8, 122u8, 55u8, 25u8, 234u8, 206u8, 215u8,
                            81u8, 109u8, 160u8, 100u8, 17u8, 162u8, 47u8, 205u8, 213u8, 237u8,
                            251u8, 35u8, 226u8, 106u8, 234u8, 165u8, 138u8, 122u8, 255u8, 35u8,
                            100u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<TransferOwnership> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "transfer_ownership",
                        TransferOwnership { class, owner },
                        [
                            197u8, 226u8, 121u8, 117u8, 230u8, 250u8, 252u8, 95u8, 132u8, 127u8,
                            175u8, 171u8, 194u8, 144u8, 252u8, 109u8, 135u8, 15u8, 58u8, 152u8,
                            151u8, 83u8, 165u8, 185u8, 214u8, 71u8, 194u8, 220u8, 76u8, 1u8, 20u8,
                            152u8,
                        ],
                    )
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
                    issuer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetTeam> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "set_team",
                        SetTeam {
                            class,
                            issuer,
                            admin,
                            freezer,
                        },
                        [
                            173u8, 36u8, 149u8, 20u8, 142u8, 241u8, 238u8, 69u8, 124u8, 244u8,
                            101u8, 249u8, 8u8, 33u8, 116u8, 65u8, 83u8, 89u8, 58u8, 110u8, 100u8,
                            65u8, 76u8, 6u8, 75u8, 71u8, 66u8, 90u8, 224u8, 215u8, 169u8, 83u8,
                        ],
                    )
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
                    delegate: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ApproveTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "approve_transfer",
                        ApproveTransfer {
                            class,
                            instance,
                            delegate,
                        },
                        [
                            209u8, 73u8, 153u8, 248u8, 159u8, 108u8, 36u8, 193u8, 40u8, 174u8,
                            58u8, 233u8, 197u8, 247u8, 117u8, 230u8, 239u8, 225u8, 162u8, 215u8,
                            203u8, 138u8, 41u8, 202u8, 34u8, 230u8, 17u8, 149u8, 218u8, 107u8,
                            175u8, 186u8,
                        ],
                    )
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
                        ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<CancelApproval> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "cancel_approval",
                        CancelApproval {
                            class,
                            instance,
                            maybe_check_delegate,
                        },
                        [
                            124u8, 3u8, 24u8, 243u8, 92u8, 241u8, 77u8, 79u8, 86u8, 220u8, 9u8,
                            154u8, 46u8, 87u8, 67u8, 212u8, 216u8, 215u8, 30u8, 187u8, 227u8,
                            200u8, 112u8, 172u8, 251u8, 204u8, 52u8, 145u8, 76u8, 102u8, 173u8,
                            163u8,
                        ],
                    )
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
                    owner: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    issuer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    free_holding: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<ForceAssetStatus> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "force_asset_status",
                        ForceAssetStatus {
                            class,
                            owner,
                            issuer,
                            admin,
                            freezer,
                            free_holding,
                            is_frozen,
                        },
                        [
                            183u8, 209u8, 78u8, 159u8, 121u8, 250u8, 254u8, 180u8, 145u8, 133u8,
                            92u8, 133u8, 4u8, 120u8, 255u8, 216u8, 113u8, 139u8, 187u8, 19u8,
                            153u8, 141u8, 89u8, 51u8, 208u8, 189u8, 204u8, 87u8, 34u8, 207u8, 59u8,
                            99u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<SetAttribute> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "set_attribute",
                        SetAttribute {
                            class,
                            maybe_instance,
                            key,
                            value,
                        },
                        [
                            2u8, 102u8, 242u8, 228u8, 33u8, 129u8, 241u8, 107u8, 137u8, 207u8,
                            141u8, 75u8, 75u8, 59u8, 101u8, 204u8, 82u8, 207u8, 68u8, 38u8, 130u8,
                            66u8, 253u8, 165u8, 155u8, 104u8, 118u8, 89u8, 9u8, 79u8, 192u8, 0u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ClearAttribute> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "clear_attribute",
                        ClearAttribute {
                            class,
                            maybe_instance,
                            key,
                        },
                        [
                            187u8, 0u8, 127u8, 112u8, 98u8, 56u8, 142u8, 151u8, 128u8, 96u8, 132u8,
                            241u8, 153u8, 114u8, 140u8, 217u8, 54u8, 147u8, 189u8, 163u8, 146u8,
                            162u8, 220u8, 173u8, 165u8, 231u8, 208u8, 223u8, 121u8, 86u8, 153u8,
                            58u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<SetMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "set_metadata",
                        SetMetadata {
                            class,
                            instance,
                            data,
                            is_frozen,
                        },
                        [
                            134u8, 104u8, 141u8, 224u8, 4u8, 146u8, 144u8, 10u8, 21u8, 120u8,
                            232u8, 181u8, 101u8, 34u8, 114u8, 124u8, 14u8, 84u8, 123u8, 179u8,
                            199u8, 216u8, 28u8, 69u8, 153u8, 142u8, 158u8, 21u8, 91u8, 125u8,
                            113u8, 40u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ClearMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "clear_metadata",
                        ClearMetadata { class, instance },
                        [
                            165u8, 34u8, 145u8, 250u8, 61u8, 128u8, 236u8, 212u8, 59u8, 4u8, 83u8,
                            146u8, 37u8, 65u8, 132u8, 62u8, 213u8, 0u8, 52u8, 143u8, 238u8, 177u8,
                            72u8, 6u8, 231u8, 98u8, 68u8, 30u8, 211u8, 127u8, 52u8, 88u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<SetClassMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "set_class_metadata",
                        SetClassMetadata {
                            class,
                            data,
                            is_frozen,
                        },
                        [
                            41u8, 212u8, 65u8, 91u8, 220u8, 84u8, 218u8, 112u8, 139u8, 231u8,
                            173u8, 17u8, 206u8, 69u8, 160u8, 143u8, 202u8, 239u8, 102u8, 200u8,
                            55u8, 17u8, 42u8, 205u8, 23u8, 195u8, 134u8, 206u8, 136u8, 140u8,
                            216u8, 85u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<ClearClassMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "OctopusUniques",
                        "clear_class_metadata",
                        ClearClassMetadata { class },
                        [
                            145u8, 8u8, 147u8, 88u8, 148u8, 99u8, 130u8, 102u8, 182u8, 148u8, 92u8,
                            13u8, 102u8, 15u8, 28u8, 81u8, 110u8, 226u8, 42u8, 19u8, 145u8, 180u8,
                            227u8, 195u8, 171u8, 240u8, 72u8, 140u8, 126u8, 120u8, 142u8, 34u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_uniques::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An asset class was created."]
            pub struct Created {
                pub class: ::core::primitive::u128,
                pub creator: ::subxt::ext::sp_core::crypto::AccountId32,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Created {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An asset class was force-created."]
            pub struct ForceCreated {
                pub class: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ForceCreated {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset `class` was destroyed."]
            pub struct Destroyed {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Destroyed {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An asset `instance` was issued."]
            pub struct Issued {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An asset `instance` was transferred."]
            pub struct Transferred {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Transferred {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An asset `instance` was destroyed."]
            pub struct Burned {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Burned {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some asset `instance` was frozen."]
            pub struct Frozen {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some asset `instance` was thawed."]
            pub struct Thawed {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `class` was frozen."]
            pub struct ClassFrozen {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for ClassFrozen {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassFrozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some asset `class` was thawed."]
            pub struct ClassThawed {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for ClassThawed {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassThawed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The owner changed."]
            pub struct OwnerChanged {
                pub class: ::core::primitive::u128,
                pub new_owner: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for OwnerChanged {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The management team changed."]
            pub struct TeamChanged {
                pub class: ::core::primitive::u128,
                pub issuer: ::subxt::ext::sp_core::crypto::AccountId32,
                pub admin: ::subxt::ext::sp_core::crypto::AccountId32,
                pub freezer: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for TeamChanged {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An `instance` of an asset `class` has been approved by the `owner` for transfer by a"]
            #[doc = "`delegate`."]
            pub struct ApprovedTransfer {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ApprovedTransfer {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An approval for a `delegate` account to transfer the `instance` of an asset `class` was"]
            #[doc = "cancelled by its `owner`."]
            pub struct ApprovalCancelled {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ApprovalCancelled {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "An asset `class` has had its attributes changed by the `Force` origin."]
            pub struct AssetStatusChanged {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for AssetStatusChanged {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "AssetStatusChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "New metadata has been set for an asset class."]
            pub struct ClassMetadataSet {
                pub class: ::core::primitive::u128,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::events::StaticEvent for ClassMetadataSet {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassMetadataSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Metadata has been cleared for an asset class."]
            pub struct ClassMetadataCleared {
                pub class: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for ClassMetadataCleared {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "ClassMetadataCleared";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "New metadata has been set for an asset instance."]
            pub struct MetadataSet {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::events::StaticEvent for MetadataSet {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Metadata has been cleared for an asset instance."]
            pub struct MetadataCleared {
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for MetadataCleared {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Metadata has been cleared for an asset instance."]
            pub struct Redeposited {
                pub class: ::core::primitive::u128,
                pub successful_instances: ::std::vec::Vec<::core::primitive::u128>,
            }
            impl ::subxt::events::StaticEvent for Redeposited {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "Redeposited";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            impl ::subxt::events::StaticEvent for AttributeSet {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "AttributeSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Attribute metadata has been cleared for an asset class or instance."]
            pub struct AttributeCleared {
                pub class: ::core::primitive::u128,
                pub maybe_instance: ::core::option::Option<::core::primitive::u128>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::events::StaticEvent for AttributeCleared {
                const PALLET: &'static str = "OctopusUniques";
                const EVENT: &'static str = "AttributeCleared";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Details of an asset class."]
                pub fn class(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::ClassDetails<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Class",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            49u8, 248u8, 50u8, 191u8, 240u8, 205u8, 66u8, 57u8, 234u8, 250u8,
                            229u8, 18u8, 13u8, 94u8, 1u8, 13u8, 236u8, 175u8, 252u8, 174u8, 155u8,
                            170u8, 107u8, 224u8, 13u8, 145u8, 207u8, 241u8, 116u8, 79u8, 224u8,
                            81u8,
                        ],
                    )
                }
                #[doc = " Details of an asset class."]
                pub fn class_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::ClassDetails<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Class",
                        Vec::new(),
                        [
                            49u8, 248u8, 50u8, 191u8, 240u8, 205u8, 66u8, 57u8, 234u8, 250u8,
                            229u8, 18u8, 13u8, 94u8, 1u8, 13u8, 236u8, 175u8, 252u8, 174u8, 155u8,
                            170u8, 107u8, 224u8, 13u8, 145u8, 207u8, 241u8, 116u8, 79u8, 224u8,
                            81u8,
                        ],
                    )
                }
                #[doc = " The assets held by any given account; set out this way so that assets owned by a single"]
                #[doc = " account can be enumerated."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _2: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<()>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Account",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _2.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            32u8, 122u8, 196u8, 149u8, 33u8, 199u8, 234u8, 192u8, 192u8, 122u8,
                            37u8, 155u8, 175u8, 87u8, 174u8, 96u8, 64u8, 10u8, 255u8, 46u8, 48u8,
                            129u8, 227u8, 210u8, 217u8, 33u8, 50u8, 159u8, 231u8, 14u8, 134u8,
                            11u8,
                        ],
                    )
                }
                #[doc = " The assets held by any given account; set out this way so that assets owned by a single"]
                #[doc = " account can be enumerated."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<()>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Account",
                        Vec::new(),
                        [
                            32u8, 122u8, 196u8, 149u8, 33u8, 199u8, 234u8, 192u8, 192u8, 122u8,
                            37u8, 155u8, 175u8, 87u8, 174u8, 96u8, 64u8, 10u8, 255u8, 46u8, 48u8,
                            129u8, 227u8, 210u8, 217u8, 33u8, 50u8, 159u8, 231u8, 14u8, 134u8,
                            11u8,
                        ],
                    )
                }
                #[doc = " The classes owned by any given account; set out this way so that classes owned by a single"]
                #[doc = " account can be enumerated."]
                pub fn class_account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<()>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "ClassAccount",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            200u8, 188u8, 203u8, 83u8, 8u8, 27u8, 228u8, 89u8, 164u8, 131u8, 168u8,
                            214u8, 176u8, 159u8, 161u8, 6u8, 132u8, 118u8, 215u8, 33u8, 184u8,
                            71u8, 21u8, 222u8, 18u8, 197u8, 35u8, 138u8, 181u8, 222u8, 79u8, 118u8,
                        ],
                    )
                }
                #[doc = " The classes owned by any given account; set out this way so that classes owned by a single"]
                #[doc = " account can be enumerated."]
                pub fn class_account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<()>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "ClassAccount",
                        Vec::new(),
                        [
                            200u8, 188u8, 203u8, 83u8, 8u8, 27u8, 228u8, 89u8, 164u8, 131u8, 168u8,
                            214u8, 176u8, 159u8, 161u8, 6u8, 132u8, 118u8, 215u8, 33u8, 184u8,
                            71u8, 21u8, 222u8, 18u8, 197u8, 35u8, 138u8, 181u8, 222u8, 79u8, 118u8,
                        ],
                    )
                }
                #[doc = " The assets in existence and their ownership details."]
                pub fn asset(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::InstanceDetails<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Asset",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            42u8, 109u8, 162u8, 137u8, 209u8, 83u8, 168u8, 87u8, 167u8, 157u8,
                            165u8, 109u8, 171u8, 108u8, 228u8, 180u8, 14u8, 16u8, 91u8, 115u8,
                            99u8, 204u8, 32u8, 2u8, 149u8, 237u8, 214u8, 21u8, 11u8, 99u8, 194u8,
                            86u8,
                        ],
                    )
                }
                #[doc = " The assets in existence and their ownership details."]
                pub fn asset_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::InstanceDetails<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Asset",
                        Vec::new(),
                        [
                            42u8, 109u8, 162u8, 137u8, 209u8, 83u8, 168u8, 87u8, 167u8, 157u8,
                            165u8, 109u8, 171u8, 108u8, 228u8, 180u8, 14u8, 16u8, 91u8, 115u8,
                            99u8, 204u8, 32u8, 2u8, 149u8, 237u8, 214u8, 21u8, 11u8, 99u8, 194u8,
                            86u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset class."]
                pub fn class_metadata_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::ClassMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "ClassMetadataOf",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            68u8, 135u8, 53u8, 248u8, 179u8, 205u8, 29u8, 10u8, 64u8, 53u8, 119u8,
                            143u8, 240u8, 22u8, 162u8, 137u8, 191u8, 39u8, 197u8, 233u8, 211u8,
                            186u8, 199u8, 143u8, 5u8, 49u8, 53u8, 24u8, 8u8, 238u8, 245u8, 155u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset class."]
                pub fn class_metadata_of_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::ClassMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "ClassMetadataOf",
                        Vec::new(),
                        [
                            68u8, 135u8, 53u8, 248u8, 179u8, 205u8, 29u8, 10u8, 64u8, 53u8, 119u8,
                            143u8, 240u8, 22u8, 162u8, 137u8, 191u8, 39u8, 197u8, 233u8, 211u8,
                            186u8, 199u8, 143u8, 5u8, 49u8, 53u8, 24u8, 8u8, 238u8, 245u8, 155u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset instance."]
                pub fn instance_metadata_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u128>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::InstanceMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "InstanceMetadataOf",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            17u8, 14u8, 154u8, 18u8, 177u8, 67u8, 52u8, 248u8, 147u8, 208u8, 212u8,
                            49u8, 129u8, 184u8, 33u8, 6u8, 105u8, 3u8, 132u8, 10u8, 217u8, 148u8,
                            208u8, 125u8, 203u8, 245u8, 254u8, 124u8, 63u8, 13u8, 19u8, 228u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset instance."]
                pub fn instance_metadata_of_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_uniques::types::InstanceMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "InstanceMetadataOf",
                        Vec::new(),
                        [
                            17u8, 14u8, 154u8, 18u8, 177u8, 67u8, 52u8, 248u8, 147u8, 208u8, 212u8,
                            49u8, 129u8, 184u8, 33u8, 6u8, 105u8, 3u8, 132u8, 10u8, 217u8, 148u8,
                            208u8, 125u8, 203u8, 245u8, 254u8, 124u8, 63u8, 13u8, 19u8, 228u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset class."]
                pub fn attribute(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u128>,
                    _1: impl ::std::borrow::Borrow<::core::option::Option<::core::primitive::u128>>,
                    _2: impl ::std::borrow::Borrow<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Attribute",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _2.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            145u8, 26u8, 138u8, 247u8, 184u8, 26u8, 165u8, 157u8, 117u8, 2u8,
                            147u8, 240u8, 34u8, 25u8, 247u8, 7u8, 148u8, 117u8, 158u8, 183u8, 39u8,
                            36u8, 162u8, 61u8, 160u8, 57u8, 24u8, 29u8, 169u8, 237u8, 49u8, 105u8,
                        ],
                    )
                }
                #[doc = " Metadata of an asset class."]
                pub fn attribute_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::u128,
                    )>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "OctopusUniques",
                        "Attribute",
                        Vec::new(),
                        [
                            145u8, 26u8, 138u8, 247u8, 184u8, 26u8, 165u8, 157u8, 117u8, 2u8,
                            147u8, 240u8, 34u8, 25u8, 247u8, 7u8, 148u8, 117u8, 158u8, 183u8, 39u8,
                            36u8, 162u8, 61u8, 160u8, 57u8, 24u8, 29u8, 169u8, 237u8, 49u8, 105u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The basic amount of funds that must be reserved for an asset class."]
                pub fn class_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "ClassDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The basic amount of funds that must be reserved for an asset instance."]
                pub fn instance_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "InstanceDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The basic amount of funds that must be reserved when adding metadata to your asset."]
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "MetadataDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The basic amount of funds that must be reserved when adding an attribute to an asset."]
                pub fn attribute_deposit_base(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "AttributeDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The additional funds that must be reserved for the number of bytes store in metadata,"]
                #[doc = " either \"normal\" metadata or attribute metadata."]
                pub fn deposit_per_byte(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "DepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum length of data stored on-chain."]
                pub fn string_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "StringLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum length of an attribute key."]
                pub fn key_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "KeyLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum length of an attribute value."]
                pub fn value_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "OctopusUniques",
                        "ValueLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetKeys {
                pub keys: runtime_types::node_template_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PurgeKeys;
            pub struct TransactionApi;
            impl TransactionApi {
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
                ) -> ::subxt::tx::StaticTxPayload<SetKeys> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Session",
                        "set_keys",
                        SetKeys { keys, proof },
                        [
                            41u8, 165u8, 81u8, 93u8, 173u8, 214u8, 165u8, 133u8, 112u8, 196u8,
                            102u8, 232u8, 197u8, 71u8, 111u8, 45u8, 46u8, 73u8, 44u8, 232u8, 176u8,
                            79u8, 144u8, 24u8, 34u8, 72u8, 241u8, 220u8, 88u8, 59u8, 105u8, 49u8,
                        ],
                    )
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
                pub fn purge_keys(&self) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Session",
                        "purge_keys",
                        PurgeKeys {},
                        [
                            200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
                            35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
                            71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "New session has happened. Note that the argument is the session index, not the"]
            #[doc = "block number as the type might suggest."]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current set of validators."]
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "Validators",
                        vec![],
                        [
                            144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
                            242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
                            32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
                            68u8,
                        ],
                    )
                }
                #[doc = " Current index of the session."]
                pub fn current_index(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "CurrentIndex",
                        vec![],
                        [
                            148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
                            183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
                            159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
                            130u8, 40u8,
                        ],
                    )
                }
                #[doc = " True if the underlying economic identities or weighting behind the validators"]
                #[doc = " has changed in the queued validator set."]
                pub fn queued_changed(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "QueuedChanged",
                        vec![],
                        [
                            105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
                            244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
                            247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
                        ],
                    )
                }
                #[doc = " The queued keys for the next session. When the next session begins, these keys"]
                #[doc = " will be used to determine the validator's session keys."]
                pub fn queued_keys(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            runtime_types::node_template_runtime::opaque::SessionKeys,
                        )>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "QueuedKeys",
                        vec![],
                        [
                            190u8, 138u8, 22u8, 146u8, 29u8, 14u8, 46u8, 152u8, 28u8, 104u8, 56u8,
                            70u8, 226u8, 110u8, 238u8, 42u8, 43u8, 105u8, 36u8, 151u8, 170u8, 48u8,
                            155u8, 137u8, 186u8, 153u8, 51u8, 197u8, 125u8, 149u8, 59u8, 108u8,
                        ],
                    )
                }
                #[doc = " Indices of disabled validators."]
                #[doc = ""]
                #[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
                #[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
                #[doc = " a new set of identities."]
                pub fn disabled_validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "DisabledValidators",
                        vec![],
                        [
                            135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
                            189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
                            110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
                            84u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "NextKeys",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            134u8, 155u8, 16u8, 143u8, 204u8, 179u8, 108u8, 209u8, 55u8, 128u8,
                            91u8, 44u8, 232u8, 230u8, 32u8, 106u8, 206u8, 178u8, 77u8, 71u8, 36u8,
                            49u8, 62u8, 205u8, 156u8, 113u8, 153u8, 236u8, 115u8, 182u8, 78u8,
                            180u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::node_template_runtime::opaque::SessionKeys,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "NextKeys",
                        Vec::new(),
                        [
                            134u8, 155u8, 16u8, 143u8, 204u8, 179u8, 108u8, 209u8, 55u8, 128u8,
                            91u8, 44u8, 232u8, 230u8, 32u8, 106u8, 206u8, 178u8, 77u8, 71u8, 36u8,
                            49u8, 62u8, 205u8, 156u8, 113u8, 153u8, 236u8, 115u8, 182u8, 78u8,
                            180u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "KeyOwner",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
                            199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
                            0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "KeyOwner",
                        Vec::new(),
                        [
                            4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
                            199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
                            0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::ext::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::ext::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::ext::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocation> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "report_equivocation",
                        ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            156u8, 162u8, 189u8, 89u8, 60u8, 156u8, 129u8, 176u8, 62u8, 35u8,
                            214u8, 7u8, 68u8, 245u8, 130u8, 117u8, 30u8, 3u8, 73u8, 218u8, 142u8,
                            82u8, 13u8, 141u8, 124u8, 19u8, 53u8, 138u8, 70u8, 4u8, 40u8, 32u8,
                        ],
                    )
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
                        ::subxt::ext::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            166u8, 26u8, 217u8, 185u8, 215u8, 37u8, 174u8, 170u8, 137u8, 160u8,
                            151u8, 43u8, 246u8, 86u8, 58u8, 18u8, 248u8, 73u8, 99u8, 161u8, 158u8,
                            93u8, 212u8, 186u8, 224u8, 253u8, 234u8, 18u8, 151u8, 111u8, 227u8,
                            249u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<NoteStalled> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "note_stalled",
                        NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            197u8, 236u8, 137u8, 32u8, 46u8, 200u8, 144u8, 13u8, 89u8, 181u8,
                            235u8, 73u8, 167u8, 131u8, 174u8, 93u8, 42u8, 136u8, 238u8, 59u8,
                            129u8, 60u8, 83u8, 100u8, 5u8, 182u8, 99u8, 250u8, 145u8, 180u8, 1u8,
                            199u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "State",
                        vec![],
                        [
                            211u8, 149u8, 114u8, 217u8, 206u8, 194u8, 115u8, 67u8, 12u8, 218u8,
                            246u8, 213u8, 208u8, 29u8, 216u8, 104u8, 2u8, 39u8, 123u8, 172u8,
                            252u8, 210u8, 52u8, 129u8, 147u8, 237u8, 244u8, 68u8, 252u8, 169u8,
                            97u8, 148u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "PendingChange",
                        vec![],
                        [
                            178u8, 24u8, 140u8, 7u8, 8u8, 196u8, 18u8, 58u8, 3u8, 226u8, 181u8,
                            47u8, 155u8, 160u8, 70u8, 12u8, 75u8, 189u8, 38u8, 255u8, 104u8, 141u8,
                            64u8, 34u8, 134u8, 201u8, 102u8, 21u8, 75u8, 81u8, 218u8, 60u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "NextForced",
                        vec![],
                        [
                            99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8, 67u8,
                            6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8, 19u8, 169u8,
                            196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8, 156u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "Stalled",
                        vec![],
                        [
                            219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8, 186u8,
                            189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8, 231u8,
                            109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8, 24u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "CurrentSetId",
                        vec![],
                        [
                            129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8, 20u8,
                            178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8, 105u8,
                            52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8, 7u8,
                            108u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "SetIdSession",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "SetIdSession",
                        Vec::new(),
                        [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod im_online {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                ) -> ::subxt::tx::StaticTxPayload<Heartbeat> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ImOnline",
                        "heartbeat",
                        Heartbeat {
                            heartbeat,
                            signature,
                        },
                        [
                            212u8, 23u8, 174u8, 246u8, 60u8, 220u8, 178u8, 137u8, 53u8, 146u8,
                            165u8, 225u8, 179u8, 209u8, 233u8, 152u8, 129u8, 210u8, 126u8, 32u8,
                            216u8, 22u8, 76u8, 196u8, 255u8, 128u8, 246u8, 161u8, 30u8, 186u8,
                            249u8, 34u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A new heartbeat was received from `AuthorityId`."]
            pub struct HeartbeatReceived {
                pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            }
            impl ::subxt::events::StaticEvent for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "At the end of the session, no offence was committed."]
            pub struct AllGood;
            impl ::subxt::events::StaticEvent for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "At the end of the session, at least one validator was found to be offline."]
            pub struct SomeOffline {
                pub offline: ::std::vec::Vec<(
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::events::StaticEvent for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
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
                pub fn heartbeat_after(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "HeartbeatAfter",
                        vec![],
                        [
                            108u8, 100u8, 85u8, 198u8, 226u8, 122u8, 94u8, 225u8, 97u8, 154u8,
                            135u8, 95u8, 106u8, 28u8, 185u8, 78u8, 192u8, 196u8, 35u8, 191u8, 12u8,
                            19u8, 163u8, 46u8, 232u8, 235u8, 193u8, 81u8, 126u8, 204u8, 25u8,
                            228u8,
                        ],
                    )
                }
                #[doc = " The current set of keys that may issue a heartbeat."]
                pub fn keys(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "Keys",
                        vec![],
                        [
                            6u8, 198u8, 221u8, 58u8, 14u8, 166u8, 245u8, 103u8, 191u8, 20u8, 69u8,
                            233u8, 147u8, 245u8, 24u8, 64u8, 207u8, 180u8, 39u8, 208u8, 252u8,
                            236u8, 247u8, 112u8, 187u8, 97u8, 70u8, 11u8, 248u8, 148u8, 208u8,
                            106u8,
                        ],
                    )
                }
                #[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
                #[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
                pub fn received_heartbeats(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "ReceivedHeartbeats",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                        ],
                        [
                            233u8, 128u8, 140u8, 233u8, 55u8, 146u8, 172u8, 54u8, 54u8, 57u8,
                            141u8, 106u8, 168u8, 59u8, 147u8, 253u8, 119u8, 48u8, 50u8, 251u8,
                            242u8, 109u8, 251u8, 2u8, 136u8, 80u8, 146u8, 121u8, 180u8, 219u8,
                            245u8, 37u8,
                        ],
                    )
                }
                #[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
                #[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
                pub fn received_heartbeats_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "ReceivedHeartbeats",
                        Vec::new(),
                        [
                            233u8, 128u8, 140u8, 233u8, 55u8, 146u8, 172u8, 54u8, 54u8, 57u8,
                            141u8, 106u8, 168u8, 59u8, 147u8, 253u8, 119u8, 48u8, 50u8, 251u8,
                            242u8, 109u8, 251u8, 2u8, 136u8, 80u8, 146u8, 121u8, 180u8, 219u8,
                            245u8, 37u8,
                        ],
                    )
                }
                #[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
                #[doc = " number of blocks authored by the given authority."]
                pub fn authored_blocks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "AuthoredBlocks",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                        ],
                        [
                            50u8, 4u8, 242u8, 240u8, 247u8, 184u8, 114u8, 245u8, 233u8, 170u8,
                            24u8, 197u8, 18u8, 245u8, 8u8, 28u8, 33u8, 115u8, 166u8, 245u8, 221u8,
                            223u8, 56u8, 144u8, 33u8, 139u8, 10u8, 227u8, 228u8, 223u8, 103u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
                #[doc = " number of blocks authored by the given authority."]
                pub fn authored_blocks_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "AuthoredBlocks",
                        Vec::new(),
                        [
                            50u8, 4u8, 242u8, 240u8, 247u8, 184u8, 114u8, 245u8, 233u8, 170u8,
                            24u8, 197u8, 18u8, 245u8, 8u8, 28u8, 33u8, 115u8, 166u8, 245u8, 221u8,
                            223u8, 56u8, 144u8, 33u8, 139u8, 10u8, 227u8, 228u8, 223u8, 103u8,
                            151u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A configuration for base priority of unsigned transactions."]
                #[doc = ""]
                #[doc = " This is exposed so that it can be tuned for particular runtime, when"]
                #[doc = " multiple pallets send unsigned transactions."]
                pub fn unsigned_priority(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ImOnline",
                        "UnsignedPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
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
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Latest MMR Root hash."]
                pub fn root_hash(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Mmr",
                        "RootHash",
                        vec![],
                        [
                            182u8, 163u8, 37u8, 44u8, 2u8, 163u8, 57u8, 184u8, 97u8, 55u8, 1u8,
                            116u8, 55u8, 169u8, 23u8, 221u8, 182u8, 5u8, 174u8, 217u8, 111u8, 55u8,
                            180u8, 161u8, 69u8, 120u8, 212u8, 73u8, 2u8, 1u8, 39u8, 224u8,
                        ],
                    )
                }
                #[doc = " Current size of the MMR (number of leaves)."]
                pub fn number_of_leaves(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Mmr",
                        "NumberOfLeaves",
                        vec![],
                        [
                            138u8, 124u8, 23u8, 186u8, 255u8, 231u8, 187u8, 122u8, 213u8, 160u8,
                            29u8, 24u8, 88u8, 98u8, 171u8, 36u8, 195u8, 216u8, 27u8, 190u8, 192u8,
                            152u8, 8u8, 13u8, 210u8, 232u8, 45u8, 184u8, 240u8, 255u8, 156u8,
                            204u8,
                        ],
                    )
                }
                #[doc = " Hashes of the nodes in the MMR."]
                #[doc = ""]
                #[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
                #[doc = " are pruned and only stored in the Offchain DB."]
                pub fn nodes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Mmr",
                        "Nodes",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            188u8, 148u8, 126u8, 226u8, 142u8, 91u8, 61u8, 52u8, 213u8, 36u8,
                            120u8, 232u8, 20u8, 11u8, 61u8, 1u8, 130u8, 155u8, 81u8, 34u8, 153u8,
                            149u8, 210u8, 232u8, 113u8, 242u8, 249u8, 8u8, 61u8, 51u8, 148u8, 98u8,
                        ],
                    )
                }
                #[doc = " Hashes of the nodes in the MMR."]
                #[doc = ""]
                #[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
                #[doc = " are pruned and only stored in the Offchain DB."]
                pub fn nodes_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Mmr",
                        "Nodes",
                        Vec::new(),
                        [
                            188u8, 148u8, 126u8, 226u8, 142u8, 91u8, 61u8, 52u8, 213u8, 36u8,
                            120u8, 232u8, 20u8, 11u8, 61u8, 1u8, 130u8, 155u8, 81u8, 34u8, 153u8,
                            149u8, 210u8, 232u8, 113u8, 242u8, 249u8, 8u8, 61u8, 51u8, 148u8, 98u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod beefy {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi;
            impl TransactionApi {}
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authorities set"]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Beefy",
                        "Authorities",
                        vec![],
                        [
                            220u8, 61u8, 168u8, 252u8, 28u8, 159u8, 44u8, 11u8, 58u8, 48u8, 39u8,
                            136u8, 156u8, 90u8, 195u8, 166u8, 244u8, 243u8, 110u8, 146u8, 168u8,
                            44u8, 4u8, 19u8, 203u8, 14u8, 209u8, 11u8, 162u8, 87u8, 101u8, 43u8,
                        ],
                    )
                }
                #[doc = " The current validator set id"]
                pub fn validator_set_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Beefy",
                        "ValidatorSetId",
                        vec![],
                        [
                            132u8, 47u8, 139u8, 239u8, 214u8, 179u8, 24u8, 63u8, 55u8, 154u8,
                            248u8, 206u8, 73u8, 7u8, 52u8, 135u8, 54u8, 111u8, 250u8, 106u8, 71u8,
                            78u8, 44u8, 44u8, 235u8, 177u8, 36u8, 112u8, 17u8, 122u8, 252u8, 80u8,
                        ],
                    )
                }
                #[doc = " Authorities set scheduled to be used with the next session"]
                pub fn next_authorities(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Beefy",
                        "NextAuthorities",
                        vec![],
                        [
                            40u8, 244u8, 137u8, 129u8, 46u8, 26u8, 80u8, 125u8, 170u8, 230u8, 63u8,
                            72u8, 127u8, 251u8, 240u8, 247u8, 211u8, 221u8, 6u8, 147u8, 72u8, 6u8,
                            1u8, 190u8, 6u8, 226u8, 96u8, 81u8, 88u8, 68u8, 218u8, 130u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod mmr_leaf {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Details of next BEEFY authority set."]
                #[doc = ""]
                #[doc = " This storage entry is used as cache for calls to `update_beefy_next_authority_set`."]
                pub fn beefy_next_authorities(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                            ::subxt::ext::sp_core::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "MmrLeaf",
                        "BeefyNextAuthorities",
                        vec![],
                        [
                            39u8, 40u8, 15u8, 157u8, 20u8, 100u8, 124u8, 98u8, 13u8, 243u8, 221u8,
                            133u8, 245u8, 210u8, 99u8, 159u8, 240u8, 158u8, 33u8, 140u8, 142u8,
                            216u8, 86u8, 227u8, 42u8, 224u8, 148u8, 200u8, 70u8, 105u8, 87u8,
                            155u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetKey {
                pub new: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SudoAs {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
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
                ) -> ::subxt::tx::StaticTxPayload<Sudo> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo",
                        Sudo {
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            185u8, 83u8, 217u8, 219u8, 8u8, 96u8, 161u8, 41u8, 8u8, 75u8, 51u8,
                            32u8, 237u8, 102u8, 137u8, 82u8, 6u8, 215u8, 170u8, 108u8, 238u8,
                            212u8, 20u8, 138u8, 148u8, 61u8, 223u8, 216u8, 174u8, 111u8, 202u8,
                            224u8,
                        ],
                    )
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
                ) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo_unchecked_weight",
                        SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            165u8, 134u8, 50u8, 97u8, 178u8, 183u8, 157u8, 253u8, 104u8, 4u8,
                            117u8, 11u8, 174u8, 238u8, 220u8, 152u8, 97u8, 100u8, 54u8, 118u8,
                            85u8, 40u8, 108u8, 248u8, 1u8, 245u8, 22u8, 183u8, 150u8, 197u8, 21u8,
                            39u8,
                        ],
                    )
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
                    new: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetKey> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "set_key",
                        SetKey { new },
                        [
                            23u8, 224u8, 218u8, 169u8, 8u8, 28u8, 111u8, 199u8, 26u8, 88u8, 225u8,
                            105u8, 17u8, 19u8, 87u8, 156u8, 97u8, 67u8, 89u8, 173u8, 70u8, 0u8,
                            5u8, 246u8, 198u8, 135u8, 182u8, 180u8, 44u8, 9u8, 212u8, 95u8,
                        ],
                    )
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
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::node_template_runtime::Call,
                ) -> ::subxt::tx::StaticTxPayload<SudoAs> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo_as",
                        SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            6u8, 7u8, 157u8, 172u8, 7u8, 20u8, 213u8, 37u8, 114u8, 138u8, 173u8,
                            77u8, 129u8, 111u8, 121u8, 131u8, 114u8, 35u8, 230u8, 125u8, 2u8,
                            172u8, 201u8, 66u8, 69u8, 81u8, 223u8, 207u8, 239u8, 23u8, 188u8, 29u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Sudo",
                        "Key",
                        vec![],
                        [
                            244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8, 179u8, 122u8, 173u8,
                            34u8, 108u8, 137u8, 28u8, 182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8,
                            80u8, 199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8, 167u8, 20u8, 142u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod template_module {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct DoSomething {
                pub something: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CauseError;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                pub fn do_something(
                    &self,
                    something: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<DoSomething> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TemplateModule",
                        "do_something",
                        DoSomething { something },
                        [
                            170u8, 197u8, 203u8, 194u8, 254u8, 154u8, 91u8, 14u8, 208u8, 246u8,
                            241u8, 49u8, 220u8, 171u8, 104u8, 245u8, 170u8, 156u8, 18u8, 62u8,
                            61u8, 140u8, 78u8, 9u8, 8u8, 203u8, 50u8, 41u8, 88u8, 231u8, 183u8,
                            246u8,
                        ],
                    )
                }
                #[doc = "An example dispatchable that may throw a custom error."]
                pub fn cause_error(&self) -> ::subxt::tx::StaticTxPayload<CauseError> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TemplateModule",
                        "cause_error",
                        CauseError {},
                        [
                            130u8, 169u8, 2u8, 174u8, 179u8, 44u8, 35u8, 37u8, 1u8, 110u8, 218u8,
                            248u8, 55u8, 69u8, 235u8, 186u8, 204u8, 99u8, 5u8, 33u8, 20u8, 68u8,
                            138u8, 254u8, 126u8, 2u8, 199u8, 51u8, 194u8, 6u8, 53u8, 243u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_template::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Event documentation should end with an array that provides descriptive names for event"]
            #[doc = "parameters. [something, who]"]
            pub struct SomethingStored(
                pub ::core::primitive::u32,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
            );
            impl ::subxt::events::StaticEvent for SomethingStored {
                const PALLET: &'static str = "TemplateModule";
                const EVENT: &'static str = "SomethingStored";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn something(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TemplateModule",
                        "Something",
                        vec![],
                        [
                            133u8, 15u8, 68u8, 243u8, 54u8, 252u8, 134u8, 227u8, 161u8, 207u8,
                            221u8, 164u8, 33u8, 75u8, 249u8, 148u8, 184u8, 225u8, 102u8, 168u8,
                            104u8, 72u8, 191u8, 148u8, 253u8, 173u8, 33u8, 89u8, 108u8, 218u8,
                            70u8, 179u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod ibc {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Dispatchable functions allows users to interact with the pallet and invoke state changes."]
        #[doc = "These functions materialize as \"extrinsic\", which are often compared to transactions."]
        #[doc = "Dispatch able functions must be annotated with a weight and must return a DispatchResult."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Deliver {
                pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RawTransfer {
                pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateClientState {
                pub client_id: ::std::vec::Vec<::core::primitive::u8>,
                pub mmr_root: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DeleteSendPacketEvent;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DeleteAckPacketEvent;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "This function acts as an entry for all of the IBC request(except MMR root update)."]
                #[doc = "I.e., create clients, update clients, handshakes to create channels, ...etc"]
                pub fn deliver(
                    &self,
                    messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                ) -> ::subxt::tx::StaticTxPayload<Deliver> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Ibc",
                        "deliver",
                        Deliver { messages },
                        [
                            179u8, 205u8, 83u8, 66u8, 171u8, 103u8, 175u8, 57u8, 35u8, 60u8, 170u8,
                            172u8, 60u8, 57u8, 56u8, 226u8, 130u8, 222u8, 121u8, 25u8, 230u8,
                            143u8, 253u8, 77u8, 111u8, 152u8, 89u8, 150u8, 129u8, 239u8, 141u8,
                            61u8,
                        ],
                    )
                }
                pub fn raw_transfer(
                    &self,
                    messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                ) -> ::subxt::tx::StaticTxPayload<RawTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Ibc",
                        "raw_transfer",
                        RawTransfer { messages },
                        [
                            187u8, 114u8, 102u8, 236u8, 49u8, 164u8, 248u8, 116u8, 68u8, 129u8,
                            23u8, 236u8, 98u8, 248u8, 101u8, 209u8, 57u8, 218u8, 113u8, 196u8,
                            243u8, 212u8, 82u8, 45u8, 2u8, 61u8, 58u8, 76u8, 48u8, 65u8, 147u8,
                            207u8,
                        ],
                    )
                }
                #[doc = "Update the MMR root stored in client_state"]
                #[doc = "Example of invoking this function via subxt"]
                pub fn update_client_state(
                    &self,
                    client_id: ::std::vec::Vec<::core::primitive::u8>,
                    mmr_root: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<UpdateClientState> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Ibc",
                        "update_client_state",
                        UpdateClientState {
                            client_id,
                            mmr_root,
                        },
                        [
                            188u8, 208u8, 188u8, 229u8, 241u8, 170u8, 146u8, 16u8, 157u8, 10u8,
                            34u8, 136u8, 123u8, 20u8, 224u8, 58u8, 223u8, 38u8, 90u8, 238u8, 180u8,
                            211u8, 219u8, 201u8, 31u8, 119u8, 169u8, 148u8, 219u8, 216u8, 59u8,
                            110u8,
                        ],
                    )
                }
                pub fn delete_send_packet_event(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<DeleteSendPacketEvent> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Ibc",
                        "delete_send_packet_event",
                        DeleteSendPacketEvent {},
                        [
                            118u8, 248u8, 190u8, 235u8, 207u8, 82u8, 39u8, 30u8, 234u8, 197u8,
                            70u8, 76u8, 249u8, 227u8, 10u8, 48u8, 200u8, 136u8, 193u8, 248u8,
                            144u8, 104u8, 69u8, 52u8, 136u8, 217u8, 146u8, 236u8, 115u8, 107u8,
                            227u8, 111u8,
                        ],
                    )
                }
                pub fn delete_ack_packet_event(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<DeleteAckPacketEvent> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Ibc",
                        "delete_ack_packet_event",
                        DeleteAckPacketEvent {},
                        [
                            75u8, 24u8, 246u8, 1u8, 96u8, 231u8, 39u8, 112u8, 114u8, 137u8, 83u8,
                            104u8, 138u8, 161u8, 208u8, 234u8, 100u8, 210u8, 250u8, 54u8, 80u8,
                            71u8, 117u8, 43u8, 169u8, 249u8, 109u8, 181u8, 86u8, 13u8, 207u8,
                            150u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Substrate IBC event list"]
        pub type Event = runtime_types::pallet_ibc::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "New block"]
            pub struct NewBlock {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
            }
            impl ::subxt::events::StaticEvent for NewBlock {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "NewBlock";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Client Created"]
            pub struct CreateClient {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub client_type: runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                pub consensus_height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
            }
            impl ::subxt::events::StaticEvent for CreateClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CreateClient";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Client updated"]
            pub struct UpdateClient {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub client_type: runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                pub consensus_height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
            }
            impl ::subxt::events::StaticEvent for UpdateClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpdateClient";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Client upgraded"]
            pub struct UpgradeClient {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub client_type: runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                pub consensus_height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
            }
            impl ::subxt::events::StaticEvent for UpgradeClient {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpgradeClient";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Client misbehaviour"]
            pub struct ClientMisbehaviour {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub client_type: runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                pub consensus_height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
            }
            impl ::subxt::events::StaticEvent for ClientMisbehaviour {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ClientMisbehaviour";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Connection open init"]
            pub struct OpenInitConnection {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub counterparty_connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub counterparty_client_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
            }
            impl ::subxt::events::StaticEvent for OpenInitConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenInitConnection";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Connection open try"]
            pub struct OpenTryConnection {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub counterparty_connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub counterparty_client_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
            }
            impl ::subxt::events::StaticEvent for OpenTryConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenTryConnection";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Connection open acknowledgement"]
            pub struct OpenAckConnection {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub counterparty_connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub counterparty_client_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
            }
            impl ::subxt::events::StaticEvent for OpenAckConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenAckConnection";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Connection open confirm"]
            pub struct OpenConfirmConnection {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                pub counterparty_connection_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                >,
                pub counterparty_client_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
            }
            impl ::subxt::events::StaticEvent for OpenConfirmConnection {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenConfirmConnection";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Channel open init"]
            pub struct OpenInitChannel {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
                pub connection_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                pub counterparty_port_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub counterparty_channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
            }
            impl ::subxt::events::StaticEvent for OpenInitChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenInitChannel";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Channel open try"]
            pub struct OpenTryChannel {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
                pub connection_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                pub counterparty_port_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub counterparty_channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
            }
            impl ::subxt::events::StaticEvent for OpenTryChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenTryChannel";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Channel open acknowledgement"]
            pub struct OpenAckChannel {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
                pub connection_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                pub counterparty_port_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub counterparty_channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
            }
            impl ::subxt::events::StaticEvent for OpenAckChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenAckChannel";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Channel open confirm"]
            pub struct OpenConfirmChannel {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
                pub connection_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                pub counterparty_port_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub counterparty_channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
            }
            impl ::subxt::events::StaticEvent for OpenConfirmChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "OpenConfirmChannel";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Channel close init"]
            pub struct CloseInitChannel {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
                pub connection_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                pub counterparty_port_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub counterparty_channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
            }
            impl ::subxt::events::StaticEvent for CloseInitChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CloseInitChannel";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Channel close confirm"]
            pub struct CloseConfirmChannel {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
                pub connection_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                pub counterparty_port_id:
                    runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                pub counterparty_channel_id: ::core::option::Option<
                    runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                >,
            }
            impl ::subxt::events::StaticEvent for CloseConfirmChannel {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "CloseConfirmChannel";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Send packet"]
            pub struct SendPacket {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
            }
            impl ::subxt::events::StaticEvent for SendPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "SendPacket";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Receive packet"]
            pub struct ReceivePacket {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
            }
            impl ::subxt::events::StaticEvent for ReceivePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ReceivePacket";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "WriteAcknowledgement packet"]
            pub struct WriteAcknowledgement {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
                pub ack: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::events::StaticEvent for WriteAcknowledgement {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "WriteAcknowledgement";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Acknowledgements packet"]
            pub struct AcknowledgePacket {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
            }
            impl ::subxt::events::StaticEvent for AcknowledgePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "AcknowledgePacket";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Timeout packet"]
            pub struct TimeoutPacket {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
            }
            impl ::subxt::events::StaticEvent for TimeoutPacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutPacket";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "TimoutOnClose packet"]
            pub struct TimeoutOnClosePacket {
                pub height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
            }
            impl ::subxt::events::StaticEvent for TimeoutOnClosePacket {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TimeoutOnClosePacket";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Empty"]
            pub struct Empty(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::events::StaticEvent for Empty {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "Empty";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Chain Error"]
            pub struct ChainError(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::events::StaticEvent for ChainError {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "ChainError";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "App Module"]
            pub struct AppModule(pub runtime_types::pallet_ibc::events::ModuleEvent);
            impl ::subxt::events::StaticEvent for AppModule {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "AppModule";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "emit update client state event"]
            pub struct UpdateClientState(
                pub runtime_types::pallet_ibc::module::core::ics24_host::Height,
                pub runtime_types::pallet_ibc::module::clients::ics10_grandpa::ClientState,
            );
            impl ::subxt::events::StaticEvent for UpdateClientState {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "UpdateClientState";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "transfer native token"]
            pub struct TransferNativeToken(
                pub runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                pub runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                pub ::core::primitive::u128,
            );
            impl ::subxt::events::StaticEvent for TransferNativeToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TransferNativeToken";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "transfer no native token"]
            pub struct TransferNoNativeToken(
                pub runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                pub runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                pub ::core::primitive::u128,
            );
            impl ::subxt::events::StaticEvent for TransferNoNativeToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "TransferNoNativeToken";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Burn cross chain token"]
            pub struct BurnToken(
                pub ::core::primitive::u32,
                pub runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                pub ::core::primitive::u128,
            );
            impl ::subxt::events::StaticEvent for BurnToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "BurnToken";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Mint chairperson  token"]
            pub struct MintToken(
                pub ::core::primitive::u32,
                pub runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                pub ::core::primitive::u128,
            );
            impl ::subxt::events::StaticEvent for MintToken {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "MintToken";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " ClientStatePath(client_id) => ClientState"]
                pub fn client_states(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ClientStates",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            152u8, 61u8, 20u8, 185u8, 66u8, 82u8, 87u8, 198u8, 200u8, 82u8, 17u8,
                            70u8, 194u8, 161u8, 141u8, 18u8, 150u8, 161u8, 211u8, 177u8, 40u8,
                            189u8, 70u8, 104u8, 51u8, 190u8, 94u8, 171u8, 157u8, 254u8, 120u8,
                            254u8,
                        ],
                    )
                }
                #[doc = " ClientStatePath(client_id) => ClientState"]
                pub fn client_states_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ClientStates",
                        Vec::new(),
                        [
                            152u8, 61u8, 20u8, 185u8, 66u8, 82u8, 87u8, 198u8, 200u8, 82u8, 17u8,
                            70u8, 194u8, 161u8, 141u8, 18u8, 150u8, 161u8, 211u8, 177u8, 40u8,
                            189u8, 70u8, 104u8, 51u8, 190u8, 94u8, 171u8, 157u8, 254u8, 120u8,
                            254u8,
                        ],
                    )
                }
                #[doc = " (client_id, height) => timestamp"]
                pub fn client_processed_times(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ClientProcessedTimes",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            217u8, 78u8, 5u8, 80u8, 212u8, 239u8, 150u8, 48u8, 18u8, 200u8, 42u8,
                            80u8, 249u8, 199u8, 99u8, 174u8, 43u8, 226u8, 178u8, 95u8, 127u8,
                            156u8, 217u8, 23u8, 192u8, 200u8, 29u8, 235u8, 8u8, 188u8, 35u8, 179u8,
                        ],
                    )
                }
                #[doc = " (client_id, height) => timestamp"]
                pub fn client_processed_times_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ClientProcessedTimes",
                        Vec::new(),
                        [
                            217u8, 78u8, 5u8, 80u8, 212u8, 239u8, 150u8, 48u8, 18u8, 200u8, 42u8,
                            80u8, 249u8, 199u8, 99u8, 174u8, 43u8, 226u8, 178u8, 95u8, 127u8,
                            156u8, 217u8, 23u8, 192u8, 200u8, 29u8, 235u8, 8u8, 188u8, 35u8, 179u8,
                        ],
                    )
                }
                #[doc = " (client_id, height) => host_height"]
                pub fn client_processed_heights(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ClientProcessedHeights",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            132u8, 248u8, 6u8, 234u8, 122u8, 247u8, 165u8, 252u8, 28u8, 81u8, 54u8,
                            120u8, 116u8, 201u8, 65u8, 159u8, 212u8, 8u8, 64u8, 215u8, 59u8, 121u8,
                            69u8, 34u8, 59u8, 194u8, 112u8, 30u8, 238u8, 248u8, 115u8, 14u8,
                        ],
                    )
                }
                #[doc = " (client_id, height) => host_height"]
                pub fn client_processed_heights_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ClientProcessedHeights",
                        Vec::new(),
                        [
                            132u8, 248u8, 6u8, 234u8, 122u8, 247u8, 165u8, 252u8, 28u8, 81u8, 54u8,
                            120u8, 116u8, 201u8, 65u8, 159u8, 212u8, 8u8, 64u8, 215u8, 59u8, 121u8,
                            69u8, 34u8, 59u8, 194u8, 112u8, 30u8, 238u8, 248u8, 115u8, 14u8,
                        ],
                    )
                }
                #[doc = " ClientConsensusStatePath(client_id, Height) => ConsensusState"]
                pub fn consensus_states(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ConsensusStates",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            249u8, 84u8, 86u8, 26u8, 68u8, 70u8, 245u8, 236u8, 213u8, 72u8, 162u8,
                            47u8, 13u8, 158u8, 147u8, 129u8, 241u8, 182u8, 52u8, 149u8, 156u8,
                            241u8, 212u8, 252u8, 29u8, 127u8, 184u8, 60u8, 228u8, 138u8, 103u8,
                            221u8,
                        ],
                    )
                }
                #[doc = " ClientConsensusStatePath(client_id, Height) => ConsensusState"]
                pub fn consensus_states_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ConsensusStates",
                        Vec::new(),
                        [
                            249u8, 84u8, 86u8, 26u8, 68u8, 70u8, 245u8, 236u8, 213u8, 72u8, 162u8,
                            47u8, 13u8, 158u8, 147u8, 129u8, 241u8, 182u8, 52u8, 149u8, 156u8,
                            241u8, 212u8, 252u8, 29u8, 127u8, 184u8, 60u8, 228u8, 138u8, 103u8,
                            221u8,
                        ],
                    )
                }
                #[doc = " ConnectionsPath(connection_id) => ConnectionEnd"]
                pub fn connections(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Connections",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            37u8, 64u8, 201u8, 194u8, 200u8, 243u8, 164u8, 32u8, 192u8, 132u8,
                            162u8, 108u8, 130u8, 185u8, 100u8, 253u8, 190u8, 135u8, 162u8, 24u8,
                            69u8, 214u8, 50u8, 186u8, 139u8, 178u8, 132u8, 250u8, 230u8, 252u8,
                            225u8, 209u8,
                        ],
                    )
                }
                #[doc = " ConnectionsPath(connection_id) => ConnectionEnd"]
                pub fn connections_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Connections",
                        Vec::new(),
                        [
                            37u8, 64u8, 201u8, 194u8, 200u8, 243u8, 164u8, 32u8, 192u8, 132u8,
                            162u8, 108u8, 130u8, 185u8, 100u8, 253u8, 190u8, 135u8, 162u8, 24u8,
                            69u8, 214u8, 50u8, 186u8, 139u8, 178u8, 132u8, 250u8, 230u8, 252u8,
                            225u8, 209u8,
                        ],
                    )
                }
                #[doc = " ChannelEndPath(port_id, channel_id) => ChannelEnd"]
                pub fn channels(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Channels",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            155u8, 89u8, 67u8, 15u8, 71u8, 90u8, 108u8, 178u8, 66u8, 197u8, 189u8,
                            188u8, 236u8, 254u8, 221u8, 110u8, 84u8, 47u8, 173u8, 148u8, 171u8,
                            185u8, 3u8, 185u8, 35u8, 247u8, 69u8, 180u8, 196u8, 87u8, 139u8, 26u8,
                        ],
                    )
                }
                #[doc = " ChannelEndPath(port_id, channel_id) => ChannelEnd"]
                pub fn channels_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Channels",
                        Vec::new(),
                        [
                            155u8, 89u8, 67u8, 15u8, 71u8, 90u8, 108u8, 178u8, 66u8, 197u8, 189u8,
                            188u8, 236u8, 254u8, 221u8, 110u8, 84u8, 47u8, 173u8, 148u8, 171u8,
                            185u8, 3u8, 185u8, 35u8, 247u8, 69u8, 180u8, 196u8, 87u8, 139u8, 26u8,
                        ],
                    )
                }
                #[doc = " ConnectionsPath(connection_id) => Vec<ChannelEndPath(port_id, channel_id)>"]
                pub fn channels_connection(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ChannelsConnection",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            180u8, 105u8, 88u8, 231u8, 159u8, 163u8, 230u8, 246u8, 142u8, 66u8,
                            218u8, 148u8, 50u8, 239u8, 15u8, 37u8, 167u8, 204u8, 109u8, 241u8,
                            151u8, 72u8, 132u8, 7u8, 253u8, 186u8, 61u8, 180u8, 105u8, 42u8, 184u8,
                            3u8,
                        ],
                    )
                }
                #[doc = " ConnectionsPath(connection_id) => Vec<ChannelEndPath(port_id, channel_id)>"]
                pub fn channels_connection_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ChannelsConnection",
                        Vec::new(),
                        [
                            180u8, 105u8, 88u8, 231u8, 159u8, 163u8, 230u8, 246u8, 142u8, 66u8,
                            218u8, 148u8, 50u8, 239u8, 15u8, 37u8, 167u8, 204u8, 109u8, 241u8,
                            151u8, 72u8, 132u8, 7u8, 253u8, 186u8, 61u8, 180u8, 105u8, 42u8, 184u8,
                            3u8,
                        ],
                    )
                }
                #[doc = " SeqSendsPath(port_id, channel_id) => sequence"]
                pub fn next_sequence_send(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "NextSequenceSend",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            154u8, 235u8, 77u8, 36u8, 152u8, 40u8, 194u8, 82u8, 241u8, 212u8,
                            205u8, 60u8, 95u8, 168u8, 210u8, 93u8, 152u8, 32u8, 102u8, 42u8, 83u8,
                            186u8, 229u8, 93u8, 225u8, 234u8, 133u8, 52u8, 177u8, 209u8, 195u8,
                            79u8,
                        ],
                    )
                }
                #[doc = " SeqSendsPath(port_id, channel_id) => sequence"]
                pub fn next_sequence_send_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "NextSequenceSend",
                        Vec::new(),
                        [
                            154u8, 235u8, 77u8, 36u8, 152u8, 40u8, 194u8, 82u8, 241u8, 212u8,
                            205u8, 60u8, 95u8, 168u8, 210u8, 93u8, 152u8, 32u8, 102u8, 42u8, 83u8,
                            186u8, 229u8, 93u8, 225u8, 234u8, 133u8, 52u8, 177u8, 209u8, 195u8,
                            79u8,
                        ],
                    )
                }
                #[doc = " SeqRecvsPath(port_id, channel_id) => sequence"]
                pub fn next_sequence_recv(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "NextSequenceRecv",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            140u8, 106u8, 29u8, 215u8, 240u8, 11u8, 109u8, 32u8, 99u8, 100u8, 89u8,
                            172u8, 95u8, 52u8, 114u8, 27u8, 142u8, 220u8, 240u8, 103u8, 80u8, 25u8,
                            111u8, 100u8, 21u8, 196u8, 214u8, 148u8, 213u8, 56u8, 196u8, 107u8,
                        ],
                    )
                }
                #[doc = " SeqRecvsPath(port_id, channel_id) => sequence"]
                pub fn next_sequence_recv_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "NextSequenceRecv",
                        Vec::new(),
                        [
                            140u8, 106u8, 29u8, 215u8, 240u8, 11u8, 109u8, 32u8, 99u8, 100u8, 89u8,
                            172u8, 95u8, 52u8, 114u8, 27u8, 142u8, 220u8, 240u8, 103u8, 80u8, 25u8,
                            111u8, 100u8, 21u8, 196u8, 214u8, 148u8, 213u8, 56u8, 196u8, 107u8,
                        ],
                    )
                }
                #[doc = " SeqAcksPath(port_id, channel_id) => sequence"]
                pub fn next_sequence_ack(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "NextSequenceAck",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            182u8, 132u8, 89u8, 11u8, 255u8, 63u8, 131u8, 200u8, 51u8, 110u8, 95u8,
                            134u8, 26u8, 178u8, 42u8, 242u8, 93u8, 233u8, 161u8, 84u8, 140u8,
                            234u8, 90u8, 173u8, 151u8, 167u8, 59u8, 130u8, 207u8, 122u8, 121u8,
                            139u8,
                        ],
                    )
                }
                #[doc = " SeqAcksPath(port_id, channel_id) => sequence"]
                pub fn next_sequence_ack_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "NextSequenceAck",
                        Vec::new(),
                        [
                            182u8, 132u8, 89u8, 11u8, 255u8, 63u8, 131u8, 200u8, 51u8, 110u8, 95u8,
                            134u8, 26u8, 178u8, 42u8, 242u8, 93u8, 233u8, 161u8, 84u8, 140u8,
                            234u8, 90u8, 173u8, 151u8, 167u8, 59u8, 130u8, 207u8, 122u8, 121u8,
                            139u8,
                        ],
                    )
                }
                #[doc = " AcksPath(port_id, channel_id, sequence) => hash of acknowledgement"]
                pub fn acknowledgements(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Acknowledgements",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            144u8, 152u8, 242u8, 14u8, 99u8, 29u8, 132u8, 28u8, 68u8, 11u8, 11u8,
                            152u8, 147u8, 250u8, 175u8, 140u8, 165u8, 231u8, 225u8, 70u8, 160u8,
                            214u8, 240u8, 207u8, 204u8, 252u8, 110u8, 238u8, 255u8, 176u8, 109u8,
                            193u8,
                        ],
                    )
                }
                #[doc = " AcksPath(port_id, channel_id, sequence) => hash of acknowledgement"]
                pub fn acknowledgements_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Acknowledgements",
                        Vec::new(),
                        [
                            144u8, 152u8, 242u8, 14u8, 99u8, 29u8, 132u8, 28u8, 68u8, 11u8, 11u8,
                            152u8, 147u8, 250u8, 175u8, 140u8, 165u8, 231u8, 225u8, 70u8, 160u8,
                            214u8, 240u8, 207u8, 204u8, 252u8, 110u8, 238u8, 255u8, 176u8, 109u8,
                            193u8,
                        ],
                    )
                }
                #[doc = " ClientTypePath(client_id) => client_type"]
                pub fn clients(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Clients",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            239u8, 211u8, 78u8, 91u8, 26u8, 160u8, 9u8, 221u8, 209u8, 43u8, 118u8,
                            199u8, 130u8, 221u8, 246u8, 23u8, 153u8, 204u8, 137u8, 253u8, 108u8,
                            38u8, 149u8, 191u8, 248u8, 65u8, 239u8, 43u8, 133u8, 6u8, 153u8, 234u8,
                        ],
                    )
                }
                #[doc = " ClientTypePath(client_id) => client_type"]
                pub fn clients_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "Clients",
                        Vec::new(),
                        [
                            239u8, 211u8, 78u8, 91u8, 26u8, 160u8, 9u8, 221u8, 209u8, 43u8, 118u8,
                            199u8, 130u8, 221u8, 246u8, 23u8, 153u8, 204u8, 137u8, 253u8, 108u8,
                            38u8, 149u8, 191u8, 248u8, 65u8, 239u8, 43u8, 133u8, 6u8, 153u8, 234u8,
                        ],
                    )
                }
                #[doc = " client counter"]
                pub fn client_counter(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ClientCounter",
                        vec![],
                        [
                            1u8, 150u8, 194u8, 56u8, 39u8, 130u8, 126u8, 87u8, 194u8, 216u8, 27u8,
                            64u8, 125u8, 9u8, 89u8, 203u8, 105u8, 87u8, 27u8, 160u8, 235u8, 137u8,
                            164u8, 201u8, 8u8, 147u8, 164u8, 123u8, 247u8, 14u8, 190u8, 12u8,
                        ],
                    )
                }
                #[doc = " connection counter"]
                pub fn connection_counter(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ConnectionCounter",
                        vec![],
                        [
                            118u8, 133u8, 237u8, 98u8, 216u8, 34u8, 98u8, 106u8, 121u8, 19u8, 26u8,
                            46u8, 199u8, 28u8, 6u8, 100u8, 34u8, 176u8, 78u8, 81u8, 114u8, 75u8,
                            19u8, 41u8, 148u8, 84u8, 130u8, 223u8, 220u8, 211u8, 5u8, 114u8,
                        ],
                    )
                }
                #[doc = " channel counter"]
                pub fn channel_counter(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ChannelCounter",
                        vec![],
                        [
                            14u8, 251u8, 41u8, 5u8, 204u8, 203u8, 45u8, 151u8, 66u8, 199u8, 1u8,
                            166u8, 123u8, 240u8, 123u8, 121u8, 19u8, 159u8, 131u8, 59u8, 13u8,
                            12u8, 52u8, 26u8, 7u8, 110u8, 137u8, 200u8, 4u8, 234u8, 96u8, 143u8,
                        ],
                    )
                }
                #[doc = " ClientConnectionsPath(client_id) => connection_id"]
                pub fn connection_client(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ConnectionClient",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            97u8, 22u8, 25u8, 124u8, 71u8, 112u8, 42u8, 26u8, 50u8, 121u8, 187u8,
                            234u8, 234u8, 220u8, 6u8, 206u8, 83u8, 51u8, 87u8, 125u8, 65u8, 230u8,
                            61u8, 17u8, 126u8, 142u8, 200u8, 243u8, 103u8, 163u8, 105u8, 26u8,
                        ],
                    )
                }
                #[doc = " ClientConnectionsPath(client_id) => connection_id"]
                pub fn connection_client_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "ConnectionClient",
                        Vec::new(),
                        [
                            97u8, 22u8, 25u8, 124u8, 71u8, 112u8, 42u8, 26u8, 50u8, 121u8, 187u8,
                            234u8, 234u8, 220u8, 6u8, 206u8, 83u8, 51u8, 87u8, 125u8, 65u8, 230u8,
                            61u8, 17u8, 126u8, 142u8, 200u8, 243u8, 103u8, 163u8, 105u8, 26u8,
                        ],
                    )
                }
                #[doc = " ReceiptsPath(port_id, channel_id, sequence) => receipt"]
                pub fn packet_receipt(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "PacketReceipt",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            72u8, 156u8, 0u8, 186u8, 23u8, 100u8, 252u8, 193u8, 164u8, 7u8, 18u8,
                            186u8, 196u8, 0u8, 174u8, 248u8, 10u8, 46u8, 157u8, 117u8, 146u8,
                            189u8, 120u8, 180u8, 237u8, 153u8, 14u8, 9u8, 240u8, 98u8, 84u8, 86u8,
                        ],
                    )
                }
                #[doc = " ReceiptsPath(port_id, channel_id, sequence) => receipt"]
                pub fn packet_receipt_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "PacketReceipt",
                        Vec::new(),
                        [
                            72u8, 156u8, 0u8, 186u8, 23u8, 100u8, 252u8, 193u8, 164u8, 7u8, 18u8,
                            186u8, 196u8, 0u8, 174u8, 248u8, 10u8, 46u8, 157u8, 117u8, 146u8,
                            189u8, 120u8, 180u8, 237u8, 153u8, 14u8, 9u8, 240u8, 98u8, 84u8, 86u8,
                        ],
                    )
                }
                #[doc = " CommitmentsPath(port_id, channel_id, sequence) => hash of (timestamp, height, packet)"]
                pub fn packet_commitment(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "PacketCommitment",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            251u8, 193u8, 176u8, 79u8, 238u8, 1u8, 154u8, 165u8, 170u8, 194u8,
                            11u8, 101u8, 234u8, 73u8, 216u8, 131u8, 114u8, 110u8, 228u8, 24u8,
                            255u8, 80u8, 255u8, 51u8, 203u8, 75u8, 234u8, 74u8, 169u8, 0u8, 183u8,
                            115u8,
                        ],
                    )
                }
                #[doc = " CommitmentsPath(port_id, channel_id, sequence) => hash of (timestamp, height, packet)"]
                pub fn packet_commitment_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "PacketCommitment",
                        Vec::new(),
                        [
                            251u8, 193u8, 176u8, 79u8, 238u8, 1u8, 154u8, 165u8, 170u8, 194u8,
                            11u8, 101u8, 234u8, 73u8, 216u8, 131u8, 114u8, 110u8, 228u8, 24u8,
                            255u8, 80u8, 255u8, 51u8, 203u8, 75u8, 234u8, 74u8, 169u8, 0u8, 183u8,
                            115u8,
                        ],
                    )
                }
                #[doc = " (height, port_id, channel_id, sequence) => send-packet event"]
                pub fn send_packet_event(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                    _2: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "SendPacketEvent",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _2.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            88u8, 140u8, 15u8, 238u8, 88u8, 79u8, 47u8, 111u8, 163u8, 158u8, 181u8,
                            65u8, 135u8, 60u8, 13u8, 96u8, 80u8, 215u8, 62u8, 97u8, 235u8, 236u8,
                            105u8, 54u8, 201u8, 233u8, 22u8, 34u8, 168u8, 132u8, 217u8, 58u8,
                        ],
                    )
                }
                #[doc = " (height, port_id, channel_id, sequence) => send-packet event"]
                pub fn send_packet_event_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "SendPacketEvent",
                        Vec::new(),
                        [
                            88u8, 140u8, 15u8, 238u8, 88u8, 79u8, 47u8, 111u8, 163u8, 158u8, 181u8,
                            65u8, 135u8, 60u8, 13u8, 96u8, 80u8, 215u8, 62u8, 97u8, 235u8, 236u8,
                            105u8, 54u8, 201u8, 233u8, 22u8, 34u8, 168u8, 132u8, 217u8, 58u8,
                        ],
                    )
                }
                #[doc = " (port_id, channel_id, sequence) => writ ack event"]
                pub fn write_ack_packet_event(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                    _2: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "WriteAckPacketEvent",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _2.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            107u8, 75u8, 137u8, 252u8, 105u8, 139u8, 38u8, 13u8, 26u8, 206u8, 92u8,
                            82u8, 227u8, 107u8, 43u8, 222u8, 107u8, 189u8, 119u8, 142u8, 23u8,
                            95u8, 68u8, 148u8, 207u8, 174u8, 127u8, 231u8, 13u8, 0u8, 16u8, 204u8,
                        ],
                    )
                }
                #[doc = " (port_id, channel_id, sequence) => writ ack event"]
                pub fn write_ack_packet_event_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "WriteAckPacketEvent",
                        Vec::new(),
                        [
                            107u8, 75u8, 137u8, 252u8, 105u8, 139u8, 38u8, 13u8, 26u8, 206u8, 92u8,
                            82u8, 227u8, 107u8, 43u8, 222u8, 107u8, 189u8, 119u8, 142u8, 23u8,
                            95u8, 68u8, 148u8, 207u8, 174u8, 127u8, 231u8, 13u8, 0u8, 16u8, 204u8,
                        ],
                    )
                }
                #[doc = " store latest height"]
                pub fn latest_height(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "LatestHeight",
                        vec![],
                        [
                            207u8, 46u8, 16u8, 118u8, 150u8, 32u8, 84u8, 137u8, 81u8, 241u8, 168u8,
                            46u8, 77u8, 118u8, 236u8, 6u8, 146u8, 77u8, 38u8, 48u8, 238u8, 120u8,
                            17u8, 22u8, 81u8, 23u8, 142u8, 1u8, 197u8, 240u8, 1u8, 216u8,
                        ],
                    )
                }
                pub fn old_height(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "OldHeight",
                        vec![],
                        [
                            99u8, 216u8, 180u8, 203u8, 123u8, 207u8, 2u8, 234u8, 253u8, 100u8,
                            107u8, 176u8, 159u8, 168u8, 234u8, 220u8, 35u8, 202u8, 234u8, 27u8,
                            60u8, 90u8, 197u8, 146u8, 82u8, 214u8, 129u8, 56u8, 8u8, 154u8, 250u8,
                            181u8,
                        ],
                    )
                }
                #[doc = " key-value asset id with asset name"]
                pub fn asset_id_by_name(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "AssetIdByName",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ],
                    )
                }
                #[doc = " key-value asset id with asset name"]
                pub fn asset_id_by_name_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Ibc",
                        "AssetIdByName",
                        Vec::new(),
                        [
                            215u8, 34u8, 187u8, 76u8, 52u8, 161u8, 208u8, 252u8, 20u8, 73u8, 89u8,
                            86u8, 60u8, 181u8, 239u8, 83u8, 152u8, 173u8, 251u8, 138u8, 238u8,
                            156u8, 72u8, 45u8, 164u8, 36u8, 94u8, 16u8, 86u8, 155u8, 97u8, 234u8,
                        ],
                    )
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
            pub mod mmr {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BeefyNextAuthoritySet<_0> {
                    pub id: ::core::primitive::u64,
                    pub len: ::core::primitive::u32,
                    pub root: _0,
                }
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
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
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the System pallet"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Event for the System pallet."]
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
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::ext::sp_core::crypto::AccountId32,
                        hash: ::subxt::ext::sp_core::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct SessionKeys {
                    pub babe: runtime_types::sp_consensus_babe::app::Public,
                    pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                    pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    pub beefy: runtime_types::beefy_primitives::crypto::Public,
                    pub octopus: runtime_types::pallet_octopus_appchain::crypto::Public,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Runtime;
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                        admin: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        target: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        target: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        source: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        issuer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        delegate: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        delegate: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        delegate: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        destination: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Some asset class was created."]
                    Created {
                        asset_id: ::core::primitive::u32,
                        creator: ::subxt::ext::sp_core::crypto::AccountId32,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Some assets were issued."]
                    Issued {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                        total_supply: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Some assets were transferred."]
                    Transferred {
                        asset_id: ::core::primitive::u32,
                        from: ::subxt::ext::sp_core::crypto::AccountId32,
                        to: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Some assets were destroyed."]
                    Burned {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "The management team changed."]
                    TeamChanged {
                        asset_id: ::core::primitive::u32,
                        issuer: ::subxt::ext::sp_core::crypto::AccountId32,
                        admin: ::subxt::ext::sp_core::crypto::AccountId32,
                        freezer: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "The owner changed."]
                    OwnerChanged {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some account `who` was frozen."]
                    Frozen {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some account `who` was thawed."]
                    Thawed {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
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
                        source: ::subxt::ext::sp_core::crypto::AccountId32,
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 15)]
                    #[doc = "An approval for account `delegate` was cancelled by `owner`."]
                    ApprovalCancelled {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 16)]
                    #[doc = "An `amount` was transferred in its entirety from `owner` to `destination` by"]
                    #[doc = "the approved `delegate`."]
                    TransferredApproved {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                        destination: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 17)]
                    #[doc = "An asset has had its attributes changed by the `Force` origin."]
                    AssetStatusChanged { asset_id: ::core::primitive::u32 },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AssetAccount<_0, _1, _2> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0>,
                    pub extra: _2,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub accounts: ::core::primitive::u32,
                    #[codec(compact)]
                    pub sufficients: ::core::primitive::u32,
                    #[codec(compact)]
                    pub approvals: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        source: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::ext::sp_core::crypto::AccountId32,
                        to: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::ext::sp_core::crypto::AccountId32,
                        to: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_beefy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {}
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::ext::sp_core::H256,
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
                                ::subxt::ext::sp_core::H256,
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            pub mod events {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ModuleEvent {
                    pub kind: ::std::vec::Vec<::core::primitive::u8>,
                    pub module_name: runtime_types::pallet_ibc::events::ModuleId,
                    pub attributes:
                        ::std::vec::Vec<runtime_types::pallet_ibc::events::ModuleEventAttribute>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ModuleEventAttribute {
                    pub key: ::std::vec::Vec<::core::primitive::u8>,
                    pub value: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ModuleId(pub ::std::vec::Vec<::core::primitive::u8>);
            }
            pub mod module {
                use super::runtime_types;
                pub mod applications {
                    use super::runtime_types;
                    pub mod transfer {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct IbcAccount(pub ::subxt::ext::sp_core::crypto::AccountId32);
                    }
                }
                pub mod clients {
                    use super::runtime_types;
                    pub mod ics10_grandpa {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct ClientState {
                            pub chain_id: ::std::vec::Vec<::core::primitive::u8>,
                            pub latest_height: ::core::primitive::u32,
                            pub frozen_height: ::core::option::Option<
                                runtime_types::pallet_ibc::module::core::ics24_host::Height,
                            >,
                            pub latest_commitment: ::std::vec::Vec<::core::primitive::u8>,
                            pub validator_set: ::std::vec::Vec<::core::primitive::u8>,
                        }
                    }
                }
                pub mod core {
                    use super::runtime_types;
                    pub mod ics24_host {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct ChannelId(pub ::std::vec::Vec<::core::primitive::u8>);
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct ClientId(pub ::std::vec::Vec<::core::primitive::u8>);
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub enum ClientType {
                            #[codec(index = 0)]
                            Tendermint,
                            #[codec(index = 1)]
                            Grandpa,
                        }
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct ConnectionId(pub ::std::vec::Vec<::core::primitive::u8>);
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct Height {
                            pub revision_number: ::core::primitive::u64,
                            pub revision_height: ::core::primitive::u64,
                        }
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct Packet {
                            pub sequence:
                                runtime_types::pallet_ibc::module::core::ics24_host::Sequence,
                            pub source_port:
                                runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                            pub source_channel:
                                runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                            pub destination_port:
                                runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                            pub destination_channel:
                                runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                            pub data: ::std::vec::Vec<::core::primitive::u8>,
                            pub timeout_height:
                                runtime_types::pallet_ibc::module::core::ics24_host::TimeoutHeight,
                            pub timeout_timestamp:
                                runtime_types::pallet_ibc::module::core::ics24_host::Timestamp,
                        }
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct PortId(pub ::std::vec::Vec<::core::primitive::u8>);
                        #[derive(
                            :: subxt :: ext :: codec :: CompactAs,
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct Sequence(pub ::core::primitive::u64);
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub enum TimeoutHeight {
                            #[codec(index = 0)]
                            Never,
                            #[codec(index = 1)]
                            At(runtime_types::pallet_ibc::module::core::ics24_host::Height),
                        }
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub struct Timestamp {
                            pub time: ::std::vec::Vec<::core::primitive::u8>,
                        }
                    }
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Dispatchable functions allows users to interact with the pallet and invoke state changes."]
                #[doc = "These functions materialize as \"extrinsic\", which are often compared to transactions."]
                #[doc = "Dispatch able functions must be annotated with a weight and must return a DispatchResult."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "This function acts as an entry for all of the IBC request(except MMR root update)."]
                    #[doc = "I.e., create clients, update clients, handshakes to create channels, ...etc"]
                    deliver {
                        messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                    },
                    #[codec(index = 1)]
                    raw_transfer {
                        messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Update the MMR root stored in client_state"]
                    #[doc = "Example of invoking this function via subxt"]
                    update_client_state {
                        client_id: ::std::vec::Vec<::core::primitive::u8>,
                        mmr_root: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    delete_send_packet_event,
                    #[codec(index = 4)]
                    delete_ack_packet_event,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Errors in MMR verification informing users that something went wrong."]
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
                    #[doc = "invalid signed_commitment"]
                    InvalidSignedCommitment,
                    #[codec(index = 7)]
                    #[doc = "empty latest_commitment"]
                    EmptyLatestCommitment,
                    #[codec(index = 8)]
                    #[doc = "invalid token id"]
                    InvalidTokenId,
                    #[codec(index = 9)]
                    #[doc = "wrong assert id"]
                    WrongAssetId,
                    #[codec(index = 10)]
                    ParserMsgTransferError,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Substrate IBC event list"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New block"]
                    NewBlock {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                    },
                    #[codec(index = 1)]
                    #[doc = "Client Created"]
                    CreateClient {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        client_type:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                        consensus_height:
                            runtime_types::pallet_ibc::module::core::ics24_host::Height,
                    },
                    #[codec(index = 2)]
                    #[doc = "Client updated"]
                    UpdateClient {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        client_type:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                        consensus_height:
                            runtime_types::pallet_ibc::module::core::ics24_host::Height,
                    },
                    #[codec(index = 3)]
                    #[doc = "Client upgraded"]
                    UpgradeClient {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        client_type:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                        consensus_height:
                            runtime_types::pallet_ibc::module::core::ics24_host::Height,
                    },
                    #[codec(index = 4)]
                    #[doc = "Client misbehaviour"]
                    ClientMisbehaviour {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        client_type:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
                        consensus_height:
                            runtime_types::pallet_ibc::module::core::ics24_host::Height,
                    },
                    #[codec(index = 5)]
                    #[doc = "Connection open init"]
                    OpenInitConnection {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        counterparty_connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        counterparty_client_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                    },
                    #[codec(index = 6)]
                    #[doc = "Connection open try"]
                    OpenTryConnection {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        counterparty_connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        counterparty_client_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                    },
                    #[codec(index = 7)]
                    #[doc = "Connection open acknowledgement"]
                    OpenAckConnection {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        counterparty_connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        counterparty_client_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                    },
                    #[codec(index = 8)]
                    #[doc = "Connection open confirm"]
                    OpenConfirmConnection {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        client_id: runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                        counterparty_connection_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        >,
                        counterparty_client_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
                    },
                    #[codec(index = 9)]
                    #[doc = "Channel open init"]
                    OpenInitChannel {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                        connection_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        counterparty_port_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        counterparty_channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                    },
                    #[codec(index = 10)]
                    #[doc = "Channel open try"]
                    OpenTryChannel {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                        connection_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        counterparty_port_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        counterparty_channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                    },
                    #[codec(index = 11)]
                    #[doc = "Channel open acknowledgement"]
                    OpenAckChannel {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                        connection_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        counterparty_port_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        counterparty_channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                    },
                    #[codec(index = 12)]
                    #[doc = "Channel open confirm"]
                    OpenConfirmChannel {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                        connection_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        counterparty_port_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        counterparty_channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                    },
                    #[codec(index = 13)]
                    #[doc = "Channel close init"]
                    CloseInitChannel {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                        connection_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        counterparty_port_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        counterparty_channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                    },
                    #[codec(index = 14)]
                    #[doc = "Channel close confirm"]
                    CloseConfirmChannel {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        port_id: runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                        connection_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
                        counterparty_port_id:
                            runtime_types::pallet_ibc::module::core::ics24_host::PortId,
                        counterparty_channel_id: ::core::option::Option<
                            runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
                        >,
                    },
                    #[codec(index = 15)]
                    #[doc = "Send packet"]
                    SendPacket {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
                    },
                    #[codec(index = 16)]
                    #[doc = "Receive packet"]
                    ReceivePacket {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
                    },
                    #[codec(index = 17)]
                    #[doc = "WriteAcknowledgement packet"]
                    WriteAcknowledgement {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
                        ack: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 18)]
                    #[doc = "Acknowledgements packet"]
                    AcknowledgePacket {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
                    },
                    #[codec(index = 19)]
                    #[doc = "Timeout packet"]
                    TimeoutPacket {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
                    },
                    #[codec(index = 20)]
                    #[doc = "TimoutOnClose packet"]
                    TimeoutOnClosePacket {
                        height: runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        packet: runtime_types::pallet_ibc::module::core::ics24_host::Packet,
                    },
                    #[codec(index = 21)]
                    #[doc = "Empty"]
                    Empty(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 22)]
                    #[doc = "Chain Error"]
                    ChainError(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 23)]
                    #[doc = "App Module"]
                    AppModule(runtime_types::pallet_ibc::events::ModuleEvent),
                    #[codec(index = 24)]
                    #[doc = "emit update client state event"]
                    UpdateClientState(
                        runtime_types::pallet_ibc::module::core::ics24_host::Height,
                        runtime_types::pallet_ibc::module::clients::ics10_grandpa::ClientState,
                    ),
                    #[codec(index = 25)]
                    #[doc = "transfer native token"]
                    TransferNativeToken(
                        runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                        runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 26)]
                    #[doc = "transfer no native token"]
                    TransferNoNativeToken(
                        runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                        runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 27)]
                    #[doc = "Burn cross chain token"]
                    BurnToken(
                        ::core::primitive::u32,
                        runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 28)]
                    #[doc = "Mint chairperson  token"]
                    MintToken(
                        ::core::primitive::u32,
                        runtime_types::pallet_ibc::module::applications::transfer::IbcAccount,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Any {
                pub type_url: ::std::vec::Vec<::core::primitive::u8>,
                pub value: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Non existent public key."]
                    InvalidKey,
                    #[codec(index = 1)]
                    #[doc = "Duplicated heartbeat."]
                    DuplicatedHeartbeat,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Submit observations."]
                    submit_observations {
                        payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                            runtime_types::sp_runtime::MultiSigner,
                            ::core::primitive::u32,
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        receiver: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        receiver: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new set of validators is waiting to be changed."]
                    NewPlannedValidators {
                        set_id: ::core::primitive::u32,
                        validators: ::std::vec::Vec<(
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "An `amount` of native token has been locked in the appchain to indicate that"]
                    #[doc = "it will be cross-chain transferred to the mainchain."]
                    Locked {
                        sender: ::subxt::ext::sp_core::crypto::AccountId32,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                        sequence: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "An `amount` was unlocked to `receiver` from `sender`."]
                    Unlocked {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "An `amount` unlock to `receiver` from `sender` failed."]
                    UnlockFailed {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    AssetMinted {
                        asset_id: ::core::primitive::u32,
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    AssetBurned {
                        asset_id: ::core::primitive::u32,
                        sender: ::subxt::ext::sp_core::crypto::AccountId32,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                        sequence: ::core::primitive::u64,
                    },
                    #[codec(index = 6)]
                    AssetMintFailed {
                        asset_id: ::core::primitive::u32,
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    AssetIdGetFailed {
                        token_id: ::std::vec::Vec<::core::primitive::u8>,
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    TransferredFromPallet {
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    NftLocked {
                        sender: ::subxt::ext::sp_core::crypto::AccountId32,
                        receiver: ::std::vec::Vec<::core::primitive::u8>,
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        sequence: ::core::primitive::u64,
                    },
                    #[codec(index = 10)]
                    NftUnlocked {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    NftUnlockFailed {
                        sender: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::ext::sp_core::crypto::AccountId32,
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnEvent<_0> {
                pub index: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnNftEvent<_0> {
                pub index: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub class: ::core::primitive::u128,
                pub instance: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct LockAssetEvent<_0> {
                pub index: ::core::primitive::u32,
                pub token_id: ::std::vec::Vec<::core::primitive::u8>,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ObservationsPayload<_0, _1, _2> {
                pub public: _0,
                pub block_number: _1,
                pub observations:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Observation<_2>>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Validator<_0> {
                pub validator_id_in_appchain: _0,
                pub total_stake: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
                        excluded_validators:
                            ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Failed to notify the mainchain to pay the validator rewards of `era_index`."]
                    EraPayoutFailed { era_index: ::core::primitive::u32 },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub set_id: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::subxt::utils::KeyedVec<_0, ::core::primitive::u32>,
            }
        }
        pub mod pallet_octopus_support {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Nonce overflow."]
                    NonceOverflow,
                    #[codec(index = 1)]
                    #[doc = "Queue size limit reached."]
                    QueueSizeLimitReached,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {}
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the session pallet."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                        new: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
                        old_sudoer:
                            ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                    #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "An example dispatchable that may throw a custom error."]
                    cause_error,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Error names should be descriptive."]
                    NoneValue,
                    #[codec(index = 1)]
                    #[doc = "Errors should have helpful documentation associated with them."]
                    StorageOverflow,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event documentation should end with an array that provides descriptive names for event"]
                    #[doc = "parameters. [something, who]"]
                    SomethingStored(
                        ::core::primitive::u32,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
                        admin: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                            ::subxt::ext::sp_runtime::MultiAddress<
                                ::subxt::ext::sp_core::crypto::AccountId32,
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
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        issuer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                        delegate: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                            ::subxt::ext::sp_runtime::MultiAddress<
                                ::subxt::ext::sp_core::crypto::AccountId32,
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
                        owner: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An asset class was created."]
                    Created {
                        class: ::core::primitive::u128,
                        creator: ::subxt::ext::sp_core::crypto::AccountId32,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "An asset class was force-created."]
                    ForceCreated {
                        class: ::core::primitive::u128,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "An asset `class` was destroyed."]
                    Destroyed { class: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    #[doc = "An asset `instance` was issued."]
                    Issued {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An asset `instance` was transferred."]
                    Transferred {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        from: ::subxt::ext::sp_core::crypto::AccountId32,
                        to: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "An asset `instance` was destroyed."]
                    Burned {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
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
                        new_owner: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 11)]
                    #[doc = "The management team changed."]
                    TeamChanged {
                        class: ::core::primitive::u128,
                        issuer: ::subxt::ext::sp_core::crypto::AccountId32,
                        admin: ::subxt::ext::sp_core::crypto::AccountId32,
                        freezer: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 12)]
                    #[doc = "An `instance` of an asset `class` has been approved by the `owner` for transfer by a"]
                    #[doc = "`delegate`."]
                    ApprovedTransfer {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 13)]
                    #[doc = "An approval for a `delegate` account to transfer the `instance` of an asset `class` was"]
                    #[doc = "cancelled by its `owner`."]
                    ApprovalCancelled {
                        class: ::core::primitive::u128,
                        instance: ::core::primitive::u128,
                        owner: ::subxt::ext::sp_core::crypto::AccountId32,
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ClassMetadata<_0> {
                    pub deposit: _0,
                    pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub instances: ::core::primitive::u32,
                    #[codec(compact)]
                    pub instance_metadatas: ::core::primitive::u32,
                    #[codec(compact)]
                    pub attributes: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct InstanceDetails<_0, _1> {
                    pub owner: _0,
                    pub approved: ::core::option::Option<_0>,
                    pub is_frozen: ::core::primitive::bool,
                    pub deposit: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
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
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
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
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::ext::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::ext::sp_core::H256,
                        pub extrinsics_root: ::subxt::ext::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _2, _1, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlakeTwo256;
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: ::core::primitive::u8,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
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
    #[doc = r" The default error type returned when there is a runtime issue,"]
    #[doc = r" exposed here for ease of use."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn babe(&self) -> babe::constants::ConstantsApi {
            babe::constants::ConstantsApi
        }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi {
            authorship::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::constants::ConstantsApi {
            octopus_appchain::constants::ConstantsApi
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::constants::ConstantsApi {
            octopus_lpos::constants::ConstantsApi
        }
        pub fn octopus_upward_messages(&self) -> octopus_upward_messages::constants::ConstantsApi {
            octopus_upward_messages::constants::ConstantsApi
        }
        pub fn octopus_assets(&self) -> octopus_assets::constants::ConstantsApi {
            octopus_assets::constants::ConstantsApi
        }
        pub fn octopus_uniques(&self) -> octopus_uniques::constants::ConstantsApi {
            octopus_uniques::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi {
            im_online::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi {
            randomness_collective_flip::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn babe(&self) -> babe::storage::StorageApi {
            babe::storage::StorageApi
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi {
            authorship::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::storage::StorageApi {
            octopus_appchain::storage::StorageApi
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::storage::StorageApi {
            octopus_lpos::storage::StorageApi
        }
        pub fn octopus_upward_messages(&self) -> octopus_upward_messages::storage::StorageApi {
            octopus_upward_messages::storage::StorageApi
        }
        pub fn octopus_assets(&self) -> octopus_assets::storage::StorageApi {
            octopus_assets::storage::StorageApi
        }
        pub fn octopus_uniques(&self) -> octopus_uniques::storage::StorageApi {
            octopus_uniques::storage::StorageApi
        }
        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi {
            im_online::storage::StorageApi
        }
        pub fn mmr(&self) -> mmr::storage::StorageApi {
            mmr::storage::StorageApi
        }
        pub fn beefy(&self) -> beefy::storage::StorageApi {
            beefy::storage::StorageApi
        }
        pub fn mmr_leaf(&self) -> mmr_leaf::storage::StorageApi {
            mmr_leaf::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn template_module(&self) -> template_module::storage::StorageApi {
            template_module::storage::StorageApi
        }
        pub fn ibc(&self) -> ibc::storage::StorageApi {
            ibc::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn babe(&self) -> babe::calls::TransactionApi {
            babe::calls::TransactionApi
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi {
            authorship::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::calls::TransactionApi {
            octopus_appchain::calls::TransactionApi
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::calls::TransactionApi {
            octopus_lpos::calls::TransactionApi
        }
        pub fn octopus_upward_messages(&self) -> octopus_upward_messages::calls::TransactionApi {
            octopus_upward_messages::calls::TransactionApi
        }
        pub fn octopus_assets(&self) -> octopus_assets::calls::TransactionApi {
            octopus_assets::calls::TransactionApi
        }
        pub fn octopus_uniques(&self) -> octopus_uniques::calls::TransactionApi {
            octopus_uniques::calls::TransactionApi
        }
        pub fn session(&self) -> session::calls::TransactionApi {
            session::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi {
            im_online::calls::TransactionApi
        }
        pub fn beefy(&self) -> beefy::calls::TransactionApi {
            beefy::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn template_module(&self) -> template_module::calls::TransactionApi {
            template_module::calls::TransactionApi
        }
        pub fn ibc(&self) -> ibc::calls::TransactionApi {
            ibc::calls::TransactionApi
        }
    }
    #[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
    pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
        client: &C,
    ) -> Result<(), ::subxt::error::MetadataError> {
        let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
        if runtime_metadata_hash
            != [
                53u8, 175u8, 11u8, 221u8, 11u8, 175u8, 236u8, 46u8, 160u8, 73u8, 81u8, 64u8, 159u8,
                115u8, 163u8, 23u8, 60u8, 35u8, 92u8, 146u8, 105u8, 143u8, 165u8, 98u8, 240u8,
                255u8, 192u8, 107u8, 246u8, 126u8, 63u8, 118u8,
            ]
        {
            Err(::subxt::error::MetadataError::IncompatibleMetadata)
        } else {
            Ok(())
        }
    }
}
