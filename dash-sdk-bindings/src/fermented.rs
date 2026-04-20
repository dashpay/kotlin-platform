#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unreachable_patterns,
    unused_braces,
    unused_imports,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables
)]
pub mod types {
    pub mod rs_dapi_client {
        use crate as dash_sdk_bindings;

        pub mod transport { use crate as dash_sdk_bindings; }
    }

    pub mod dpp {
        use crate as dash_sdk_bindings;

        pub mod fee {
            use crate as dash_sdk_bindings;

            pub mod epoch {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "fee-distribution")]
                pub mod distribution {
                    use crate as dash_sdk_bindings;

                    #[cfg(test)]
                    pub mod tests {
                        use crate as dash_sdk_bindings;
                    }
                }
            }

            #[cfg(feature = "fee-distribution")]
            pub mod fee_result {
                use crate as dash_sdk_bindings;

                pub mod refunds {
                    use crate as dash_sdk_bindings;

                    #[cfg(test)]
                    pub mod tests {
                        use crate as dash_sdk_bindings;
                    }
                }
            }

            pub mod default_costs { use crate as dash_sdk_bindings; }
        }

        pub mod group {
            use crate as dash_sdk_bindings;

            pub mod group_action { use crate as dash_sdk_bindings; }
        }

        #[cfg(feature = "core-types")]
        pub mod core_types {
            use crate as dash_sdk_bindings;

            pub mod validator_set {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }

            pub mod validator {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }
        }

        pub mod bls { use crate as dash_sdk_bindings; }

        pub mod asset_lock {
            use crate as dash_sdk_bindings;

            pub mod reduced_asset_lock_value { use crate as dash_sdk_bindings; }
        }

        pub mod validation {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "validation")]
            pub mod byte_array_keyword {
                use crate as dash_sdk_bindings;

                #[cfg(test)]
                pub mod tests {
                    use crate as dash_sdk_bindings;
                }
            }

            #[cfg(feature = "validation")]
            pub mod json_schema_validator {
                use crate as dash_sdk_bindings;

                pub mod methods {
                    use crate as dash_sdk_bindings;

                    pub mod new { use crate as dash_sdk_bindings; }

                    pub mod compile { use crate as dash_sdk_bindings; }

                    pub mod validate { use crate as dash_sdk_bindings; }
                }
            }
        }

        pub mod block {
            use crate as dash_sdk_bindings;

            pub mod block_info {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`BlockInfo`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_block_block_info_BlockInfo {
                    pub time_ms: *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis,
                    pub height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight,
                    pub core_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight,
                    pub epoch: *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_Epoch,
                }

                impl ferment::FFIConversionFrom<dpp::block::block_info::BlockInfo> for dpp_block_block_info_BlockInfo {
                    unsafe fn ffi_from_const(ffi: *const dpp_block_block_info_BlockInfo) -> dpp::block::block_info::BlockInfo {
                        let ffi_ref = &*ffi;
                        dpp::block::block_info::BlockInfo { time_ms: <crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis as ferment::FFIConversionFrom<dpp::prelude::TimestampMillis>>::ffi_from(ffi_ref.time_ms), height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionFrom<dpp::prelude::BlockHeight>>::ffi_from(ffi_ref.height), core_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight>>::ffi_from(ffi_ref.core_height), epoch: <crate::fermented::types::dpp::block::epoch::dpp_block_epoch_Epoch as ferment::FFIConversionFrom<dpp::block::epoch::Epoch>>::ffi_from(ffi_ref.epoch) }
                    }
                }

                impl ferment::FFIConversionTo<dpp::block::block_info::BlockInfo> for dpp_block_block_info_BlockInfo { unsafe fn ffi_to_const(obj: dpp::block::block_info::BlockInfo) -> *const dpp_block_block_info_BlockInfo { ferment::boxed(dpp_block_block_info_BlockInfo { time_ms: <crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis as ferment::FFIConversionTo<dpp::prelude::TimestampMillis>>::ffi_to(obj.time_ms), height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionTo<dpp::prelude::BlockHeight>>::ffi_to(obj.height), core_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionTo<dpp::prelude::CoreBlockHeight>>::ffi_to(obj.core_height), epoch: <crate::fermented::types::dpp::block::epoch::dpp_block_epoch_Epoch as ferment::FFIConversionTo<dpp::block::epoch::Epoch>>::ffi_to(obj.epoch) }) } }

                impl Drop for dpp_block_block_info_BlockInfo {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.time_ms);
                            ferment::unbox_any(ffi_ref.height);
                            ferment::unbox_any(ffi_ref.core_height);
                            ferment::unbox_any(ffi_ref.epoch);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_ctor<>(time_ms: *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis, height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight, core_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight, epoch: *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_Epoch) -> *mut dpp_block_block_info_BlockInfo { ferment::boxed(dpp_block_block_info_BlockInfo { time_ms, height, core_height, epoch }) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_destroy<>(ffi: *mut dpp_block_block_info_BlockInfo) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_get_time_ms<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis { (*obj).time_ms }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_get_height<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_get_core_height<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).core_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_get_epoch<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_Epoch { (*obj).epoch }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_set_time_ms<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis { (*obj).time_ms }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_set_height<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_set_core_height<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).core_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_block_info_BlockInfo_set_epoch<>(obj: *const dpp_block_block_info_BlockInfo) -> *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_Epoch { (*obj).epoch }
            }

            pub mod extended_block_info {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }

            pub mod epoch {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`EpochIndex`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_block_epoch_EpochIndex(u16);

                impl ferment::FFIConversionFrom<dpp::block::epoch::EpochIndex> for dpp_block_epoch_EpochIndex {
                    unsafe fn ffi_from_const(ffi: *const dpp_block_epoch_EpochIndex) -> dpp::block::epoch::EpochIndex {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                }

                impl ferment::FFIConversionTo<dpp::block::epoch::EpochIndex> for dpp_block_epoch_EpochIndex { unsafe fn ffi_to_const(obj: dpp::block::epoch::EpochIndex) -> *const dpp_block_epoch_EpochIndex { ferment::boxed(dpp_block_epoch_EpochIndex(obj)) } }

                impl Drop for dpp_block_epoch_EpochIndex {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ;
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_EpochIndex_ctor<>(o_0: u16) -> *mut dpp_block_epoch_EpochIndex { ferment::boxed(dpp_block_epoch_EpochIndex(o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_EpochIndex_destroy<>(ffi: *mut dpp_block_epoch_EpochIndex) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_EpochIndex_get_0<>(obj: *const dpp_block_epoch_EpochIndex) -> u16 { (*obj).0 }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_EpochIndex_set_0<>(obj: *const dpp_block_epoch_EpochIndex) -> u16 { (*obj).0 }

                #[doc = "FFI-representation of the [`Epoch`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_block_epoch_Epoch {
                    pub index: *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_EpochIndex,
                    pub key: *mut crate::fermented::generics::Arr_u8_2,
                }

                impl ferment::FFIConversionFrom<dpp::block::epoch::Epoch> for dpp_block_epoch_Epoch {
                    unsafe fn ffi_from_const(ffi: *const dpp_block_epoch_Epoch) -> dpp::block::epoch::Epoch {
                        let ffi_ref = &*ffi;
                        dpp::block::epoch::Epoch { index: <crate::fermented::types::dpp::block::epoch::dpp_block_epoch_EpochIndex as ferment::FFIConversionFrom<dpp::block::epoch::EpochIndex>>::ffi_from(ffi_ref.index), key: <crate::fermented::generics::Arr_u8_2 as ferment::FFIConversionFrom<[u8; 2]>>::ffi_from(ffi_ref.key) }
                    }
                }

                impl ferment::FFIConversionTo<dpp::block::epoch::Epoch> for dpp_block_epoch_Epoch { unsafe fn ffi_to_const(obj: dpp::block::epoch::Epoch) -> *const dpp_block_epoch_Epoch { ferment::boxed(dpp_block_epoch_Epoch { index: <crate::fermented::types::dpp::block::epoch::dpp_block_epoch_EpochIndex as ferment::FFIConversionTo<dpp::block::epoch::EpochIndex>>::ffi_to(obj.index), key: <crate::fermented::generics::Arr_u8_2 as ferment::FFIConversionTo<[u8; 2]>>::ffi_to(obj.key) }) } }

                impl Drop for dpp_block_epoch_Epoch {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.index);
                            ferment::unbox_any(ffi_ref.key);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_Epoch_ctor<>(index: *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_EpochIndex, key: *mut crate::fermented::generics::Arr_u8_2) -> *mut dpp_block_epoch_Epoch { ferment::boxed(dpp_block_epoch_Epoch { index, key }) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_Epoch_destroy<>(ffi: *mut dpp_block_epoch_Epoch) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_Epoch_get_index<>(obj: *const dpp_block_epoch_Epoch) -> *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_EpochIndex { (*obj).index }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_Epoch_get_key<>(obj: *const dpp_block_epoch_Epoch) -> *mut crate::fermented::generics::Arr_u8_2 { (*obj).key }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_Epoch_set_index<>(obj: *const dpp_block_epoch_Epoch) -> *mut crate::fermented::types::dpp::block::epoch::dpp_block_epoch_EpochIndex { (*obj).index }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_block_epoch_Epoch_set_key<>(obj: *const dpp_block_epoch_Epoch) -> *mut crate::fermented::generics::Arr_u8_2 { (*obj).key }
            }

            pub mod extended_epoch_info { use crate as dash_sdk_bindings; }

            pub mod finalized_epoch_info {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }
        }

        pub mod withdrawal {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "system_contracts")]
            pub mod document_try_into_asset_unlock_base_transaction_info {
                use crate as dash_sdk_bindings;
            }

            pub mod daily_withdrawal_limit {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }
        }

        pub mod serialization { use crate as dash_sdk_bindings; }

        pub mod errors {
            use crate as dash_sdk_bindings;

            pub mod non_consensus_error { use crate as dash_sdk_bindings; }

            pub mod consensus {
                use crate as dash_sdk_bindings;

                pub mod state {
                    use crate as dash_sdk_bindings;

                    pub mod voting { use crate as dash_sdk_bindings; }

                    pub mod data_contract { use crate as dash_sdk_bindings; }

                    pub mod document { use crate as dash_sdk_bindings; }

                    pub mod token { use crate as dash_sdk_bindings; }

                    pub mod data_trigger { use crate as dash_sdk_bindings; }

                    pub mod group { use crate as dash_sdk_bindings; }

                    pub mod prefunded_specialized_balances { use crate as dash_sdk_bindings; }

                    pub mod identity { use crate as dash_sdk_bindings; }
                }

                pub mod signature { use crate as dash_sdk_bindings; }

                pub mod fee { use crate as dash_sdk_bindings; }

                pub mod basic {
                    use crate as dash_sdk_bindings;

                    pub mod token { use crate as dash_sdk_bindings; }

                    pub mod state_transition { use crate as dash_sdk_bindings; }

                    pub mod json_schema_error { use crate as dash_sdk_bindings; }

                    pub mod group { use crate as dash_sdk_bindings; }

                    pub mod unsupported_version_error { use crate as dash_sdk_bindings; }

                    pub mod identity { use crate as dash_sdk_bindings; }

                    pub mod unsupported_protocol_version_error { use crate as dash_sdk_bindings; }

                    pub mod document { use crate as dash_sdk_bindings; }

                    pub mod decode { use crate as dash_sdk_bindings; }

                    pub mod data_contract { use crate as dash_sdk_bindings; }
                }
            }
        }

        #[cfg(feature = "state-transitions")]
        pub mod state_transition {
            use crate as dash_sdk_bindings;

            pub mod serialization { use crate as dash_sdk_bindings; }

            pub mod traits { use crate as dash_sdk_bindings; }

            pub mod abstract_state_transition { use crate as dash_sdk_bindings; }

            pub mod errors { use crate as dash_sdk_bindings; }

            pub mod state_transitions {
                use crate as dash_sdk_bindings;

                pub mod document {
                    use crate as dash_sdk_bindings;

                    pub mod batch_transition {
                        use crate as dash_sdk_bindings;

                        pub mod batched_transition {
                            use crate as dash_sdk_bindings;

                            pub mod token_config_update_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }

                                pub mod validate_structure { use crate as dash_sdk_bindings; }
                            }

                            pub mod document_transfer_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_destroy_frozen_funds_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }

                                pub mod validate_structure { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_mint_transition {
                                use crate as dash_sdk_bindings;

                                pub mod validate_structure { use crate as dash_sdk_bindings; }

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod document_purchase_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_claim_transition {
                                use crate as dash_sdk_bindings;

                                pub mod validate_structure { use crate as dash_sdk_bindings; }

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_unfreeze_transition {
                                use crate as dash_sdk_bindings;

                                pub mod validate_structure { use crate as dash_sdk_bindings; }

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_burn_transition {
                                use crate as dash_sdk_bindings;

                                pub mod validate_structure { use crate as dash_sdk_bindings; }

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod document_create_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_set_price_for_direct_purchase_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }

                                pub mod validate_structure { use crate as dash_sdk_bindings; }
                            }

                            pub mod document_delete_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_freeze_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }

                                pub mod validate_structure { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_transfer_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }

                                pub mod validate_structure { use crate as dash_sdk_bindings; }
                            }

                            pub mod document_update_price_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod document_replace_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_direct_purchase_transition {
                                use crate as dash_sdk_bindings;

                                pub mod validate_structure { use crate as dash_sdk_bindings; }

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_emergency_action_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }

                                pub mod validate_structure { use crate as dash_sdk_bindings; }
                            }

                            pub mod token_base_transition {
                                use crate as dash_sdk_bindings;

                                pub mod fields { use crate as dash_sdk_bindings; }

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod document_base_transition {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }

                                pub mod fields { use crate as dash_sdk_bindings; }

                                pub mod v1 { use crate as dash_sdk_bindings; }
                            }
                        }

                        pub mod v1 { use crate as dash_sdk_bindings; }

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        pub mod fields { use crate as dash_sdk_bindings; }

                        pub mod accessors { use crate as dash_sdk_bindings; }

                        pub mod resolvers { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }

                        #[cfg(feature = "validation")]
                        pub mod validation {
                            use crate as dash_sdk_bindings;

                            pub mod find_duplicates_by_id {
                                use crate as dash_sdk_bindings;

                                pub mod v0 { use crate as dash_sdk_bindings; }
                            }

                            pub mod validate_basic_structure { use crate as dash_sdk_bindings; }
                        }
                    }
                }

                pub mod contract {
                    use crate as dash_sdk_bindings;

                    pub mod data_contract_update_transition {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }

                        pub mod accessors { use crate as dash_sdk_bindings; }
                    }

                    pub mod data_contract_create_transition {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        #[cfg(feature = "state-transition-json-conversion")]
                        pub mod json_conversion {
                            use crate as dash_sdk_bindings;
                        }

                        pub mod accessors { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }
                    }

                    pub mod common_fields { use crate as dash_sdk_bindings; }
                }

                pub mod identity {
                    use crate as dash_sdk_bindings;

                    pub mod identity_create_transition {
                        use crate as dash_sdk_bindings;

                        pub mod accessors { use crate as dash_sdk_bindings; }

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }
                    }

                    pub mod public_key_in_creation {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        pub mod methods {
                            use crate as dash_sdk_bindings;

                            pub mod hash { use crate as dash_sdk_bindings; }

                            pub mod duplicated_key_ids_witness { use crate as dash_sdk_bindings; }

                            pub mod duplicated_keys_witness { use crate as dash_sdk_bindings; }

                            pub mod validate_identity_public_keys_structure { use crate as dash_sdk_bindings; }

                            #[cfg(feature = "state-transition-signing")]
                            pub mod from_public_key_signed_with_private_key {
                                use crate as dash_sdk_bindings;
                            }

                            #[cfg(feature = "state-transition-signing")]
                            pub mod from_public_key_signed_external {
                                use crate as dash_sdk_bindings;
                            }
                        }
                    }

                    pub mod identity_credit_withdrawal_transition {
                        use crate as dash_sdk_bindings;

                        pub mod accessors { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }

                        pub mod v1 { use crate as dash_sdk_bindings; }

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    pub mod common_fields { use crate as dash_sdk_bindings; }

                    pub mod identity_update_transition {
                        use crate as dash_sdk_bindings;

                        pub mod v0 {
                            use crate as dash_sdk_bindings;

                            #[cfg(feature = "state-transition-json-conversion")]
                            pub mod json_conversion {
                                use crate as dash_sdk_bindings;
                            }
                        }

                        pub mod accessors { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }

                        pub mod fields { use crate as dash_sdk_bindings; }
                    }

                    pub mod masternode_vote_transition {
                        use crate as dash_sdk_bindings;

                        pub mod fields { use crate as dash_sdk_bindings; }

                        pub mod accessors { use crate as dash_sdk_bindings; }

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }
                    }

                    pub mod identity_credit_transfer_transition {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        pub mod fields { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }

                        pub mod accessors { use crate as dash_sdk_bindings; }
                    }

                    pub mod identity_topup_transition {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }

                        pub mod accessors { use crate as dash_sdk_bindings; }

                        pub mod methods { use crate as dash_sdk_bindings; }
                    }
                }

                pub mod common_fields { use crate as dash_sdk_bindings; }
            }
        }

        pub mod voting {
            use crate as dash_sdk_bindings;

            pub mod vote_polls {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`VotePoll`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dpp_voting_vote_polls_VotePoll { ContestedDocumentResourceVotePoll(*mut crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) }

                impl ferment::FFIConversionFrom<dpp::voting::vote_polls::VotePoll> for dpp_voting_vote_polls_VotePoll {
                    unsafe fn ffi_from_const(ffi: *const dpp_voting_vote_polls_VotePoll) -> dpp::voting::vote_polls::VotePoll {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dpp_voting_vote_polls_VotePoll::ContestedDocumentResourceVotePoll(o_0) => dpp::voting::vote_polls::VotePoll::ContestedDocumentResourceVotePoll(<crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll as ferment::FFIConversionFrom<dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll>>::ffi_from(*o_0)) }
                    }
                }

                impl ferment::FFIConversionTo<dpp::voting::vote_polls::VotePoll> for dpp_voting_vote_polls_VotePoll {
                    unsafe fn ffi_to_const(obj: dpp::voting::vote_polls::VotePoll) -> *const dpp_voting_vote_polls_VotePoll {
                        ferment::boxed(match obj {
                            dpp::voting::vote_polls::VotePoll::ContestedDocumentResourceVotePoll(o_0) => dpp_voting_vote_polls_VotePoll::ContestedDocumentResourceVotePoll(<crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll as ferment::FFIConversionTo<dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll>>::ffi_to(o_0)),
                            _ => unreachable!("This is unreachable")
                        })
                    }
                }

                impl Drop for dpp_voting_vote_polls_VotePoll {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                dpp_voting_vote_polls_VotePoll::ContestedDocumentResourceVotePoll(o_0) => { ferment::unbox_any(*o_0); }
                                _ => unreachable!("This is unreachable")
                            };
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_voting_vote_polls_VotePoll_ContestedDocumentResourceVotePoll_ctor(o_o_0: *mut crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut dpp_voting_vote_polls_VotePoll { ferment::boxed(dpp_voting_vote_polls_VotePoll::ContestedDocumentResourceVotePoll(o_o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_voting_vote_polls_VotePoll_destroy<>(ffi: *mut dpp_voting_vote_polls_VotePoll) { ferment::unbox_any(ffi); }

                pub mod contested_document_resource_vote_poll {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`ContestedDocumentResourceVotePoll`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll {
                        pub contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
                        pub document_type_name: *mut std::os::raw::c_char,
                        pub index_name: *mut std::os::raw::c_char,
                        pub index_values: *mut crate::fermented::generics::Vec_platform_value_Value,
                    }

                    impl ferment::FFIConversionFrom<dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll> for dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll {
                        unsafe fn ffi_from_const(ffi: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll {
                            let ffi_ref = &*ffi;
                            dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll { contract_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.contract_id), document_type_name: <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(ffi_ref.document_type_name), index_name: <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(ffi_ref.index_name), index_values: <crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionFrom<Vec<platform_value::Value>>>::ffi_from(ffi_ref.index_values) }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll> for dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll {
                        unsafe fn ffi_to_const(obj: dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll) -> *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll { ferment::boxed(dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll { contract_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.contract_id), document_type_name: <std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(obj.document_type_name), index_name: <std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(obj.index_name), index_values: <crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionTo<Vec<platform_value::Value>>>::ffi_to(obj.index_values) }) }
                    }

                    impl Drop for dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.contract_id);
                                ferment::unbox_string(ffi_ref.document_type_name);
                                ferment::unbox_string(ffi_ref.index_name);
                                ferment::unbox_any(ffi_ref.index_values);
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_ctor<>(contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, document_type_name: *mut std::os::raw::c_char, index_name: *mut std::os::raw::c_char, index_values: *mut crate::fermented::generics::Vec_platform_value_Value) -> *mut dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll { ferment::boxed(dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll { contract_id, document_type_name, index_name, index_values }) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_destroy<>(ffi: *mut dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) { ferment::unbox_any(ffi); }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_get_contract_id<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).contract_id }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_get_document_type_name<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut std::os::raw::c_char { (*obj).document_type_name }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_get_index_name<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut std::os::raw::c_char { (*obj).index_name }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_get_index_values<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut crate::fermented::generics::Vec_platform_value_Value { (*obj).index_values }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_set_contract_id<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).contract_id }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_set_document_type_name<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut std::os::raw::c_char { (*obj).document_type_name }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_set_index_name<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut std::os::raw::c_char { (*obj).index_name }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll_set_index_values<>(obj: *const dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut crate::fermented::generics::Vec_platform_value_Value { (*obj).index_values }
                }
            }

            pub mod votes {
                use crate as dash_sdk_bindings;

                pub mod resource_vote {
                    use crate as dash_sdk_bindings;

                    pub mod v0 {
                        use crate as dash_sdk_bindings;

                        #[doc = "FFI-representation of the [`ResourceVoteV0`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dpp_voting_votes_resource_vote_v0_ResourceVoteV0 {
                            pub vote_poll: *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll,
                            pub resource_vote_choice: *mut crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice,
                        }

                        impl ferment::FFIConversionFrom<dpp::voting::votes::resource_vote::v0::ResourceVoteV0> for dpp_voting_votes_resource_vote_v0_ResourceVoteV0 {
                            unsafe fn ffi_from_const(ffi: *const dpp_voting_votes_resource_vote_v0_ResourceVoteV0) -> dpp::voting::votes::resource_vote::v0::ResourceVoteV0 {
                                let ffi_ref = &*ffi;
                                dpp::voting::votes::resource_vote::v0::ResourceVoteV0 { vote_poll: <crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll as ferment::FFIConversionFrom<dpp::voting::vote_polls::VotePoll>>::ffi_from(ffi_ref.vote_poll), resource_vote_choice: <crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice as ferment::FFIConversionFrom<dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice>>::ffi_from(ffi_ref.resource_vote_choice) }
                            }
                        }

                        impl ferment::FFIConversionTo<dpp::voting::votes::resource_vote::v0::ResourceVoteV0> for dpp_voting_votes_resource_vote_v0_ResourceVoteV0 { unsafe fn ffi_to_const(obj: dpp::voting::votes::resource_vote::v0::ResourceVoteV0) -> *const dpp_voting_votes_resource_vote_v0_ResourceVoteV0 { ferment::boxed(dpp_voting_votes_resource_vote_v0_ResourceVoteV0 { vote_poll: <crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll as ferment::FFIConversionTo<dpp::voting::vote_polls::VotePoll>>::ffi_to(obj.vote_poll), resource_vote_choice: <crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice as ferment::FFIConversionTo<dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice>>::ffi_to(obj.resource_vote_choice) }) } }

                        impl Drop for dpp_voting_votes_resource_vote_v0_ResourceVoteV0 {
                            fn drop(&mut self) {
                                unsafe {
                                    let ffi_ref = self;
                                    ferment::unbox_any(ffi_ref.vote_poll);
                                    ferment::unbox_any(ffi_ref.resource_vote_choice);
                                }
                            }
                        }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_votes_resource_vote_v0_ResourceVoteV0_ctor<>(vote_poll: *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll, resource_vote_choice: *mut crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice) -> *mut dpp_voting_votes_resource_vote_v0_ResourceVoteV0 { ferment::boxed(dpp_voting_votes_resource_vote_v0_ResourceVoteV0 { vote_poll, resource_vote_choice }) }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_votes_resource_vote_v0_ResourceVoteV0_destroy<>(ffi: *mut dpp_voting_votes_resource_vote_v0_ResourceVoteV0) { ferment::unbox_any(ffi); }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_votes_resource_vote_v0_ResourceVoteV0_get_vote_poll<>(obj: *const dpp_voting_votes_resource_vote_v0_ResourceVoteV0) -> *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll { (*obj).vote_poll }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_votes_resource_vote_v0_ResourceVoteV0_get_resource_vote_choice<>(obj: *const dpp_voting_votes_resource_vote_v0_ResourceVoteV0) -> *mut crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice { (*obj).resource_vote_choice }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_votes_resource_vote_v0_ResourceVoteV0_set_vote_poll<>(obj: *const dpp_voting_votes_resource_vote_v0_ResourceVoteV0) -> *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll { (*obj).vote_poll }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_votes_resource_vote_v0_ResourceVoteV0_set_resource_vote_choice<>(obj: *const dpp_voting_votes_resource_vote_v0_ResourceVoteV0) -> *mut crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice { (*obj).resource_vote_choice }
                    }

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ResourceVote`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_voting_votes_resource_vote_ResourceVote { V0(*mut crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0) }

                    impl ferment::FFIConversionFrom<dpp::voting::votes::resource_vote::ResourceVote> for dpp_voting_votes_resource_vote_ResourceVote {
                        unsafe fn ffi_from_const(ffi: *const dpp_voting_votes_resource_vote_ResourceVote) -> dpp::voting::votes::resource_vote::ResourceVote {
                            let ffi_ref = &*ffi;
                            match ffi_ref { dpp_voting_votes_resource_vote_ResourceVote::V0(o_0) => dpp::voting::votes::resource_vote::ResourceVote::V0(<crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0 as ferment::FFIConversionFrom<dpp::voting::votes::resource_vote::v0::ResourceVoteV0>>::ffi_from(*o_0)) }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::voting::votes::resource_vote::ResourceVote> for dpp_voting_votes_resource_vote_ResourceVote {
                        unsafe fn ffi_to_const(obj: dpp::voting::votes::resource_vote::ResourceVote) -> *const dpp_voting_votes_resource_vote_ResourceVote {
                            ferment::boxed(match obj {
                                dpp::voting::votes::resource_vote::ResourceVote::V0(o_0) => dpp_voting_votes_resource_vote_ResourceVote::V0(<crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0 as ferment::FFIConversionTo<dpp::voting::votes::resource_vote::v0::ResourceVoteV0>>::ffi_to(o_0)),
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_voting_votes_resource_vote_ResourceVote {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_voting_votes_resource_vote_ResourceVote::V0(o_0) => { ferment::unbox_any(*o_0); }
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_votes_resource_vote_ResourceVote_V0_ctor(o_o_0: *mut crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0) -> *mut dpp_voting_votes_resource_vote_ResourceVote { ferment::boxed(dpp_voting_votes_resource_vote_ResourceVote::V0(o_o_0)) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_votes_resource_vote_ResourceVote_destroy<>(ffi: *mut dpp_voting_votes_resource_vote_ResourceVote) { ferment::unbox_any(ffi); }

                    pub mod accessors { use crate as dash_sdk_bindings; }
                }

                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`Vote`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dpp_voting_votes_Vote { ResourceVote(*mut crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote) }

                impl ferment::FFIConversionFrom<dpp::voting::votes::Vote> for dpp_voting_votes_Vote {
                    unsafe fn ffi_from_const(ffi: *const dpp_voting_votes_Vote) -> dpp::voting::votes::Vote {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dpp_voting_votes_Vote::ResourceVote(o_0) => dpp::voting::votes::Vote::ResourceVote(<crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote as ferment::FFIConversionFrom<dpp::voting::votes::resource_vote::ResourceVote>>::ffi_from(*o_0)) }
                    }
                }

                impl ferment::FFIConversionTo<dpp::voting::votes::Vote> for dpp_voting_votes_Vote {
                    unsafe fn ffi_to_const(obj: dpp::voting::votes::Vote) -> *const dpp_voting_votes_Vote {
                        ferment::boxed(match obj {
                            dpp::voting::votes::Vote::ResourceVote(o_0) => dpp_voting_votes_Vote::ResourceVote(<crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote as ferment::FFIConversionTo<dpp::voting::votes::resource_vote::ResourceVote>>::ffi_to(o_0)),
                            _ => unreachable!("This is unreachable")
                        })
                    }
                }

                impl Drop for dpp_voting_votes_Vote {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                dpp_voting_votes_Vote::ResourceVote(o_0) => { ferment::unbox_any(*o_0); }
                                _ => unreachable!("This is unreachable")
                            };
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_voting_votes_Vote_ResourceVote_ctor(o_o_0: *mut crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote) -> *mut dpp_voting_votes_Vote { ferment::boxed(dpp_voting_votes_Vote::ResourceVote(o_o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_voting_votes_Vote_destroy<>(ffi: *mut dpp_voting_votes_Vote) { ferment::unbox_any(ffi); }
            }

            pub mod contender_structs {
                use crate as dash_sdk_bindings;

                pub mod contender {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ContenderWithSerializedDocument`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { V0(*mut crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) }

                    impl ferment::FFIConversionFrom<dpp::voting::contender_structs::contender::ContenderWithSerializedDocument> for dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
                        unsafe fn ffi_from_const(ffi: *const dpp_voting_contender_structs_contender_ContenderWithSerializedDocument) -> dpp::voting::contender_structs::contender::ContenderWithSerializedDocument {
                            let ffi_ref = &*ffi;
                            match ffi_ref { dpp_voting_contender_structs_contender_ContenderWithSerializedDocument::V0(o_0) => dpp::voting::contender_structs::contender::ContenderWithSerializedDocument::V0(<crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 as ferment::FFIConversionFrom<dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0>>::ffi_from(*o_0)) }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::voting::contender_structs::contender::ContenderWithSerializedDocument> for dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
                        unsafe fn ffi_to_const(obj: dpp::voting::contender_structs::contender::ContenderWithSerializedDocument) -> *const dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
                            ferment::boxed(match obj {
                                dpp::voting::contender_structs::contender::ContenderWithSerializedDocument::V0(o_0) => dpp_voting_contender_structs_contender_ContenderWithSerializedDocument::V0(<crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 as ferment::FFIConversionTo<dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0>>::ffi_to(o_0)),
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_voting_contender_structs_contender_ContenderWithSerializedDocument::V0(o_0) => { ferment::unbox_any(*o_0); }
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_contender_structs_contender_ContenderWithSerializedDocument_V0_ctor(o_o_0: *mut crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { ferment::boxed(dpp_voting_contender_structs_contender_ContenderWithSerializedDocument::V0(o_o_0)) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_contender_structs_contender_ContenderWithSerializedDocument_destroy<>(ffi: *mut dpp_voting_contender_structs_contender_ContenderWithSerializedDocument) { ferment::unbox_any(ffi); }

                    pub mod v0 {
                        use crate as dash_sdk_bindings;

                        #[doc = "FFI-representation of the [`ContenderWithSerializedDocumentV0`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 {
                            pub identity_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
                            pub serialized_document: *mut crate::fermented::generics::Vec_u8,
                            pub vote_tally: *mut u32,
                        }

                        impl ferment::FFIConversionFrom<dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0> for dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 {
                            unsafe fn ffi_from_const(ffi: *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0 {
                                let ffi_ref = &*ffi;
                                dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0 { identity_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.identity_id), serialized_document: <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from_opt(ffi_ref.serialized_document), vote_tally: ferment::from_opt_primitive(ffi_ref.vote_tally) }
                            }
                        }

                        impl ferment::FFIConversionTo<dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0> for dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 { unsafe fn ffi_to_const(obj: dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0) -> *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 { ferment::boxed(dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 { identity_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.identity_id), serialized_document: <crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to_opt(obj.serialized_document), vote_tally: ferment::to_opt_primitive(obj.vote_tally) }) } }

                        impl Drop for dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 {
                            fn drop(&mut self) {
                                unsafe {
                                    let ffi_ref = self;
                                    ferment::unbox_any(ffi_ref.identity_id);
                                    ferment::unbox_any_opt(ffi_ref.serialized_document);
                                    ferment::destroy_opt_primitive(ffi_ref.vote_tally);
                                }
                            }
                        }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_ctor<>(identity_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, serialized_document: *mut crate::fermented::generics::Vec_u8, vote_tally: *mut u32) -> *mut dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 { ferment::boxed(dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 { identity_id, serialized_document, vote_tally }) }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_destroy<>(ffi: *mut dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) { ferment::unbox_any(ffi); }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_get_identity_id<>(obj: *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).identity_id }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_get_serialized_document<>(obj: *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut crate::fermented::generics::Vec_u8 { (*obj).serialized_document }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_get_vote_tally<>(obj: *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut u32 { (*obj).vote_tally }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_set_identity_id<>(obj: *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).identity_id }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_set_serialized_document<>(obj: *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut crate::fermented::generics::Vec_u8 { (*obj).serialized_document }

                        #[no_mangle]
                        pub unsafe extern "C" fn dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0_set_vote_tally<>(obj: *const dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut u32 { (*obj).vote_tally }
                    }
                }
            }

            pub mod vote_info_storage {
                use crate as dash_sdk_bindings;

                pub mod contested_document_vote_poll_stored_info { use crate as dash_sdk_bindings; }

                pub mod contested_document_vote_poll_winner_info {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ContestedDocumentVotePollWinnerInfo`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo { NoWinner, WonByIdentity(*mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier), Locked }

                    impl ferment::FFIConversionFrom<dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo> for dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo {
                        unsafe fn ffi_from_const(ffi: *const dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo) -> dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo {
                            let ffi_ref = &*ffi;
                            match ffi_ref {
                                dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::NoWinner => dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo::NoWinner,
                                dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::WonByIdentity(o_0) => dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo::WonByIdentity(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(*o_0)),
                                dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::Locked => dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo::Locked
                            }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo> for dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo {
                        unsafe fn ffi_to_const(obj: dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo) -> *const dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo {
                            ferment::boxed(match obj {
                                dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo::NoWinner => dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::NoWinner,
                                dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo::WonByIdentity(o_0) => dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::WonByIdentity(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(o_0)),
                                dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo::Locked => dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::Locked,
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::NoWinner => {}
                                    dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::WonByIdentity(o_0) => { ferment::unbox_any(*o_0); }
                                    dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::Locked => {}
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_NoWinner_ctor() -> *mut dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo { ferment::boxed(dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::NoWinner {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_WonByIdentity_ctor(o_o_0: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo { ferment::boxed(dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::WonByIdentity(o_o_0)) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_Locked_ctor() -> *mut dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo { ferment::boxed(dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo::Locked {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_destroy<>(ffi: *mut dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo) { ferment::unbox_any(ffi); }
                }
            }

            pub mod vote_choices {
                use crate as dash_sdk_bindings;

                pub mod resource_vote_choice {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ResourceVoteChoice`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice { TowardsIdentity(*mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier), Abstain, Lock }

                    impl ferment::FFIConversionFrom<dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice> for dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice {
                        unsafe fn ffi_from_const(ffi: *const dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice) -> dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice {
                            let ffi_ref = &*ffi;
                            match ffi_ref {
                                dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::TowardsIdentity(o_0) => dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice::TowardsIdentity(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(*o_0)),
                                dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Abstain => dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice::Abstain,
                                dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Lock => dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice::Lock
                            }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice> for dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice {
                        unsafe fn ffi_to_const(obj: dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice) -> *const dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice {
                            ferment::boxed(match obj {
                                dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice::TowardsIdentity(o_0) => dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::TowardsIdentity(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(o_0)),
                                dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice::Abstain => dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Abstain,
                                dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice::Lock => dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Lock,
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::TowardsIdentity(o_0) => { ferment::unbox_any(*o_0); }
                                    dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Abstain => {}
                                    dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Lock => {}
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice_TowardsIdentity_ctor(o_o_0: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice { ferment::boxed(dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::TowardsIdentity(o_o_0)) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice_Abstain_ctor() -> *mut dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice { ferment::boxed(dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Abstain {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice_Lock_ctor() -> *mut dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice { ferment::boxed(dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice::Lock {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice_destroy<>(ffi: *mut dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice) { ferment::unbox_any(ffi); }
                }
            }
        }

        pub mod balances { use crate as dash_sdk_bindings; }

        pub mod tokens {
            use crate as dash_sdk_bindings;

            pub mod token_payment_info {
                use crate as dash_sdk_bindings;

                pub mod methods { use crate as dash_sdk_bindings; }

                pub mod v0 { use crate as dash_sdk_bindings; }
            }

            pub mod info { use crate as dash_sdk_bindings; }

            pub mod contract_info { use crate as dash_sdk_bindings; }

            pub mod status { use crate as dash_sdk_bindings; }
        }

        pub mod document {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "factories")]
            pub mod specialized_document_factory {
                use crate as dash_sdk_bindings;
            }

            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`Document`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum dpp_document_Document { V0(*mut crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0) }

            impl ferment::FFIConversionFrom<dpp::document::Document> for dpp_document_Document {
                unsafe fn ffi_from_const(ffi: *const dpp_document_Document) -> dpp::document::Document {
                    let ffi_ref = &*ffi;
                    match ffi_ref { dpp_document_Document::V0(o_0) => dpp::document::Document::V0(<crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0 as ferment::FFIConversionFrom<dpp::document::v0::DocumentV0>>::ffi_from(*o_0)) }
                }
            }

            impl ferment::FFIConversionTo<dpp::document::Document> for dpp_document_Document {
                unsafe fn ffi_to_const(obj: dpp::document::Document) -> *const dpp_document_Document {
                    ferment::boxed(match obj {
                        dpp::document::Document::V0(o_0) => dpp_document_Document::V0(<crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0 as ferment::FFIConversionTo<dpp::document::v0::DocumentV0>>::ffi_to(o_0)),
                        _ => unreachable!("This is unreachable")
                    })
                }
            }

            impl Drop for dpp_document_Document {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            dpp_document_Document::V0(o_0) => { ferment::unbox_any(*o_0); }
                            _ => unreachable!("This is unreachable")
                        };
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_document_Document_V0_ctor(o_o_0: *mut crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0) -> *mut dpp_document_Document { ferment::boxed(dpp_document_Document::V0(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_document_Document_destroy<>(ffi: *mut dpp_document_Document) { ferment::unbox_any(ffi); }

            #[cfg(feature = "extended-document")]
            pub mod extended_document {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }

                pub mod fields { use crate as dash_sdk_bindings; }
            }

            pub mod document_methods {
                use crate as dash_sdk_bindings;

                pub mod get_raw_for_contract { use crate as dash_sdk_bindings; }

                pub mod hash { use crate as dash_sdk_bindings; }

                pub mod get_raw_for_document_type { use crate as dash_sdk_bindings; }

                pub mod is_equal_ignoring_timestamps { use crate as dash_sdk_bindings; }
            }

            pub mod accessors { use crate as dash_sdk_bindings; }

            #[cfg(feature = "factories")]
            pub mod document_factory {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }

            pub mod serialization_traits {
                use crate as dash_sdk_bindings;

                pub mod platform_serialization_conversion {
                    use crate as dash_sdk_bindings;

                    pub mod deserialize { use crate as dash_sdk_bindings; }

                    pub mod serialize { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "document-json-conversion")]
                pub mod json_conversion {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "document-cbor-conversion")]
                pub mod cbor_conversion {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "document-value-conversion")]
                pub mod platform_value_conversion {
                    use crate as dash_sdk_bindings;
                }
            }

            pub mod v0 {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`DocumentV0`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_document_v0_DocumentV0 {
                    pub id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
                    pub owner_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
                    pub properties: *mut crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value,
                    pub revision: *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision,
                    pub created_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis,
                    pub updated_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis,
                    pub transferred_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis,
                    pub created_at_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight,
                    pub updated_at_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight,
                    pub transferred_at_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight,
                    pub created_at_core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight,
                    pub updated_at_core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight,
                    pub transferred_at_core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight,
                }

                impl ferment::FFIConversionFrom<dpp::document::v0::DocumentV0> for dpp_document_v0_DocumentV0 {
                    unsafe fn ffi_from_const(ffi: *const dpp_document_v0_DocumentV0) -> dpp::document::v0::DocumentV0 {
                        let ffi_ref = &*ffi;
                        dpp::document::v0::DocumentV0 {
                            id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.id),
                            owner_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.owner_id),
                            properties: <crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value as ferment::FFIConversionFrom<std::collections::BTreeMap<String, platform_value::Value>>>::ffi_from(ffi_ref.properties),
                            revision: <crate::fermented::types::dpp::prelude::dpp_prelude_Revision as ferment::FFIConversionFrom<dpp::prelude::Revision>>::ffi_from_opt(ffi_ref.revision),
                            created_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis>>::ffi_from_opt(ffi_ref.created_at),
                            updated_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis>>::ffi_from_opt(ffi_ref.updated_at),
                            transferred_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis>>::ffi_from_opt(ffi_ref.transferred_at),
                            created_at_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionFrom<dpp::prelude::BlockHeight>>::ffi_from_opt(ffi_ref.created_at_block_height),
                            updated_at_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionFrom<dpp::prelude::BlockHeight>>::ffi_from_opt(ffi_ref.updated_at_block_height),
                            transferred_at_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionFrom<dpp::prelude::BlockHeight>>::ffi_from_opt(ffi_ref.transferred_at_block_height),
                            created_at_core_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight>>::ffi_from_opt(ffi_ref.created_at_core_block_height),
                            updated_at_core_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight>>::ffi_from_opt(ffi_ref.updated_at_core_block_height),
                            transferred_at_core_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight>>::ffi_from_opt(ffi_ref.transferred_at_core_block_height),
                        }
                    }
                }

                impl ferment::FFIConversionTo<dpp::document::v0::DocumentV0> for dpp_document_v0_DocumentV0 {
                    unsafe fn ffi_to_const(obj: dpp::document::v0::DocumentV0) -> *const dpp_document_v0_DocumentV0 {
                        ferment::boxed(dpp_document_v0_DocumentV0 {
                            id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.id),
                            owner_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.owner_id),
                            properties: <crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value as ferment::FFIConversionTo<std::collections::BTreeMap<String, platform_value::Value>>>::ffi_to(obj.properties),
                            revision: <crate::fermented::types::dpp::prelude::dpp_prelude_Revision as ferment::FFIConversionTo<dpp::prelude::Revision>>::ffi_to_opt(obj.revision),
                            created_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionTo<dpp::identity::identity_public_key::TimestampMillis>>::ffi_to_opt(obj.created_at),
                            updated_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionTo<dpp::identity::identity_public_key::TimestampMillis>>::ffi_to_opt(obj.updated_at),
                            transferred_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionTo<dpp::identity::identity_public_key::TimestampMillis>>::ffi_to_opt(obj.transferred_at),
                            created_at_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionTo<dpp::prelude::BlockHeight>>::ffi_to_opt(obj.created_at_block_height),
                            updated_at_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionTo<dpp::prelude::BlockHeight>>::ffi_to_opt(obj.updated_at_block_height),
                            transferred_at_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionTo<dpp::prelude::BlockHeight>>::ffi_to_opt(obj.transferred_at_block_height),
                            created_at_core_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionTo<dpp::prelude::CoreBlockHeight>>::ffi_to_opt(obj.created_at_core_block_height),
                            updated_at_core_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionTo<dpp::prelude::CoreBlockHeight>>::ffi_to_opt(obj.updated_at_core_block_height),
                            transferred_at_core_block_height: <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionTo<dpp::prelude::CoreBlockHeight>>::ffi_to_opt(obj.transferred_at_core_block_height),
                        })
                    }
                }

                impl Drop for dpp_document_v0_DocumentV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.id);
                            ferment::unbox_any(ffi_ref.owner_id);
                            ferment::unbox_any(ffi_ref.properties);
                            ferment::unbox_any_opt(ffi_ref.revision);
                            ferment::unbox_any_opt(ffi_ref.created_at);
                            ferment::unbox_any_opt(ffi_ref.updated_at);
                            ferment::unbox_any_opt(ffi_ref.transferred_at);
                            ferment::unbox_any_opt(ffi_ref.created_at_block_height);
                            ferment::unbox_any_opt(ffi_ref.updated_at_block_height);
                            ferment::unbox_any_opt(ffi_ref.transferred_at_block_height);
                            ferment::unbox_any_opt(ffi_ref.created_at_core_block_height);
                            ferment::unbox_any_opt(ffi_ref.updated_at_core_block_height);
                            ferment::unbox_any_opt(ffi_ref.transferred_at_core_block_height);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_ctor<>(id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, owner_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, properties: *mut crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value, revision: *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision, created_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis, updated_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis, transferred_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis, created_at_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight, updated_at_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight, transferred_at_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight, created_at_core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight, updated_at_core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight, transferred_at_core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight) -> *mut dpp_document_v0_DocumentV0 { ferment::boxed(dpp_document_v0_DocumentV0 { id, owner_id, properties, revision, created_at, updated_at, transferred_at, created_at_block_height, updated_at_block_height, transferred_at_block_height, created_at_core_block_height, updated_at_core_block_height, transferred_at_core_block_height }) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_destroy<>(ffi: *mut dpp_document_v0_DocumentV0) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_id<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).id }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_owner_id<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).owner_id }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_properties<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value { (*obj).properties }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_revision<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision { (*obj).revision }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_created_at<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).created_at }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_updated_at<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).updated_at }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_transferred_at<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).transferred_at }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_created_at_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).created_at_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_updated_at_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).updated_at_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_transferred_at_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).transferred_at_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_created_at_core_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).created_at_core_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_updated_at_core_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).updated_at_core_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_get_transferred_at_core_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).transferred_at_core_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_id<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).id }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_owner_id<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).owner_id }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_properties<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value { (*obj).properties }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_revision<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision { (*obj).revision }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_created_at<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).created_at }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_updated_at<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).updated_at }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_transferred_at<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).transferred_at }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_created_at_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).created_at_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_updated_at_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).updated_at_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_transferred_at_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight { (*obj).transferred_at_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_created_at_core_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).created_at_core_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_updated_at_core_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).updated_at_core_block_height }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_document_v0_DocumentV0_set_transferred_at_core_block_height<>(obj: *const dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight { (*obj).transferred_at_core_block_height }
            }

            pub mod fields { use crate as dash_sdk_bindings; }
        }

        pub mod prelude {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`TimestampMillis`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct dpp_prelude_TimestampMillis(u64);

            impl ferment::FFIConversionFrom<dpp::prelude::TimestampMillis> for dpp_prelude_TimestampMillis {
                unsafe fn ffi_from_const(ffi: *const dpp_prelude_TimestampMillis) -> dpp::prelude::TimestampMillis {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
            }

            impl ferment::FFIConversionTo<dpp::prelude::TimestampMillis> for dpp_prelude_TimestampMillis { unsafe fn ffi_to_const(obj: dpp::prelude::TimestampMillis) -> *const dpp_prelude_TimestampMillis { ferment::boxed(dpp_prelude_TimestampMillis(obj)) } }

            impl Drop for dpp_prelude_TimestampMillis {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ;
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_TimestampMillis_ctor<>(o_0: u64) -> *mut dpp_prelude_TimestampMillis { ferment::boxed(dpp_prelude_TimestampMillis(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_TimestampMillis_destroy<>(ffi: *mut dpp_prelude_TimestampMillis) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_TimestampMillis_get_0<>(obj: *const dpp_prelude_TimestampMillis) -> u64 { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_TimestampMillis_set_0<>(obj: *const dpp_prelude_TimestampMillis) -> u64 { (*obj).0 }

            #[doc = "FFI-representation of the [`Revision`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct dpp_prelude_Revision(u64);

            impl ferment::FFIConversionFrom<dpp::prelude::Revision> for dpp_prelude_Revision {
                unsafe fn ffi_from_const(ffi: *const dpp_prelude_Revision) -> dpp::prelude::Revision {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
            }

            impl ferment::FFIConversionTo<dpp::prelude::Revision> for dpp_prelude_Revision { unsafe fn ffi_to_const(obj: dpp::prelude::Revision) -> *const dpp_prelude_Revision { ferment::boxed(dpp_prelude_Revision(obj)) } }

            impl Drop for dpp_prelude_Revision {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ;
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_Revision_ctor<>(o_0: u64) -> *mut dpp_prelude_Revision { ferment::boxed(dpp_prelude_Revision(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_Revision_destroy<>(ffi: *mut dpp_prelude_Revision) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_Revision_get_0<>(obj: *const dpp_prelude_Revision) -> u64 { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_Revision_set_0<>(obj: *const dpp_prelude_Revision) -> u64 { (*obj).0 }

            #[doc = "FFI-representation of the [`BlockHeight`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct dpp_prelude_BlockHeight(u64);

            impl ferment::FFIConversionFrom<dpp::prelude::BlockHeight> for dpp_prelude_BlockHeight {
                unsafe fn ffi_from_const(ffi: *const dpp_prelude_BlockHeight) -> dpp::prelude::BlockHeight {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
            }

            impl ferment::FFIConversionTo<dpp::prelude::BlockHeight> for dpp_prelude_BlockHeight { unsafe fn ffi_to_const(obj: dpp::prelude::BlockHeight) -> *const dpp_prelude_BlockHeight { ferment::boxed(dpp_prelude_BlockHeight(obj)) } }

            impl Drop for dpp_prelude_BlockHeight {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ;
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_BlockHeight_ctor<>(o_0: u64) -> *mut dpp_prelude_BlockHeight { ferment::boxed(dpp_prelude_BlockHeight(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_BlockHeight_destroy<>(ffi: *mut dpp_prelude_BlockHeight) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_BlockHeight_get_0<>(obj: *const dpp_prelude_BlockHeight) -> u64 { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_BlockHeight_set_0<>(obj: *const dpp_prelude_BlockHeight) -> u64 { (*obj).0 }

            #[doc = "FFI-representation of the [`CoreBlockHeight`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct dpp_prelude_CoreBlockHeight(u32);

            impl ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight> for dpp_prelude_CoreBlockHeight {
                unsafe fn ffi_from_const(ffi: *const dpp_prelude_CoreBlockHeight) -> dpp::prelude::CoreBlockHeight {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
            }

            impl ferment::FFIConversionTo<dpp::prelude::CoreBlockHeight> for dpp_prelude_CoreBlockHeight { unsafe fn ffi_to_const(obj: dpp::prelude::CoreBlockHeight) -> *const dpp_prelude_CoreBlockHeight { ferment::boxed(dpp_prelude_CoreBlockHeight(obj)) } }

            impl Drop for dpp_prelude_CoreBlockHeight {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ;
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_CoreBlockHeight_ctor<>(o_0: u32) -> *mut dpp_prelude_CoreBlockHeight { ferment::boxed(dpp_prelude_CoreBlockHeight(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_CoreBlockHeight_destroy<>(ffi: *mut dpp_prelude_CoreBlockHeight) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_CoreBlockHeight_get_0<>(obj: *const dpp_prelude_CoreBlockHeight) -> u32 { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn dpp_prelude_CoreBlockHeight_set_0<>(obj: *const dpp_prelude_CoreBlockHeight) -> u32 { (*obj).0 }
        }

        pub mod identity {
            use crate as dash_sdk_bindings;

            pub mod identity_nonce { use crate as dash_sdk_bindings; }

            pub mod identity_public_key {
                use crate as dash_sdk_bindings;

                pub mod key_type {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`KeyType`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_identity_identity_public_key_key_type_KeyType { ECDSA_SECP256K1 = 0, BLS12_381 = 1, ECDSA_HASH160 = 2, BIP13_SCRIPT_HASH = 3, EDDSA_25519_HASH160 = 4 }

                    impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::key_type::KeyType> for dpp_identity_identity_public_key_key_type_KeyType {
                        unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_key_type_KeyType) -> dpp::identity::identity_public_key::key_type::KeyType {
                            let ffi_ref = &*ffi;
                            match ffi_ref {
                                dpp_identity_identity_public_key_key_type_KeyType::ECDSA_SECP256K1 => dpp::identity::identity_public_key::key_type::KeyType::ECDSA_SECP256K1,
                                dpp_identity_identity_public_key_key_type_KeyType::BLS12_381 => dpp::identity::identity_public_key::key_type::KeyType::BLS12_381,
                                dpp_identity_identity_public_key_key_type_KeyType::ECDSA_HASH160 => dpp::identity::identity_public_key::key_type::KeyType::ECDSA_HASH160,
                                dpp_identity_identity_public_key_key_type_KeyType::BIP13_SCRIPT_HASH => dpp::identity::identity_public_key::key_type::KeyType::BIP13_SCRIPT_HASH,
                                dpp_identity_identity_public_key_key_type_KeyType::EDDSA_25519_HASH160 => dpp::identity::identity_public_key::key_type::KeyType::EDDSA_25519_HASH160
                            }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::identity::identity_public_key::key_type::KeyType> for dpp_identity_identity_public_key_key_type_KeyType {
                        unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::key_type::KeyType) -> *const dpp_identity_identity_public_key_key_type_KeyType {
                            ferment::boxed(match obj {
                                dpp::identity::identity_public_key::key_type::KeyType::ECDSA_SECP256K1 => dpp_identity_identity_public_key_key_type_KeyType::ECDSA_SECP256K1,
                                dpp::identity::identity_public_key::key_type::KeyType::BLS12_381 => dpp_identity_identity_public_key_key_type_KeyType::BLS12_381,
                                dpp::identity::identity_public_key::key_type::KeyType::ECDSA_HASH160 => dpp_identity_identity_public_key_key_type_KeyType::ECDSA_HASH160,
                                dpp::identity::identity_public_key::key_type::KeyType::BIP13_SCRIPT_HASH => dpp_identity_identity_public_key_key_type_KeyType::BIP13_SCRIPT_HASH,
                                dpp::identity::identity_public_key::key_type::KeyType::EDDSA_25519_HASH160 => dpp_identity_identity_public_key_key_type_KeyType::EDDSA_25519_HASH160,
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_identity_identity_public_key_key_type_KeyType {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_identity_identity_public_key_key_type_KeyType::ECDSA_SECP256K1 => {}
                                    dpp_identity_identity_public_key_key_type_KeyType::BLS12_381 => {}
                                    dpp_identity_identity_public_key_key_type_KeyType::ECDSA_HASH160 => {}
                                    dpp_identity_identity_public_key_key_type_KeyType::BIP13_SCRIPT_HASH => {}
                                    dpp_identity_identity_public_key_key_type_KeyType::EDDSA_25519_HASH160 => {}
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_key_type_KeyType_ECDSA_SECP256K1_ctor() -> *mut dpp_identity_identity_public_key_key_type_KeyType { ferment::boxed(dpp_identity_identity_public_key_key_type_KeyType::ECDSA_SECP256K1 {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_key_type_KeyType_BLS12_381_ctor() -> *mut dpp_identity_identity_public_key_key_type_KeyType { ferment::boxed(dpp_identity_identity_public_key_key_type_KeyType::BLS12_381 {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_key_type_KeyType_ECDSA_HASH160_ctor() -> *mut dpp_identity_identity_public_key_key_type_KeyType { ferment::boxed(dpp_identity_identity_public_key_key_type_KeyType::ECDSA_HASH160 {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_key_type_KeyType_BIP13_SCRIPT_HASH_ctor() -> *mut dpp_identity_identity_public_key_key_type_KeyType { ferment::boxed(dpp_identity_identity_public_key_key_type_KeyType::BIP13_SCRIPT_HASH {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_key_type_KeyType_EDDSA_25519_HASH160_ctor() -> *mut dpp_identity_identity_public_key_key_type_KeyType { ferment::boxed(dpp_identity_identity_public_key_key_type_KeyType::EDDSA_25519_HASH160 {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_key_type_KeyType_destroy<>(ffi: *mut dpp_identity_identity_public_key_key_type_KeyType) { ferment::unbox_any(ffi); }
                }

                pub mod conversion {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "platform-value")]
                    pub mod platform_value {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "json-object")]
                    pub mod json {
                        use crate as dash_sdk_bindings;
                    }
                }

                #[doc = "FFI-representation of the [`KeyID`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_identity_identity_public_key_KeyID(u32);

                impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::KeyID> for dpp_identity_identity_public_key_KeyID {
                    unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_KeyID) -> dpp::identity::identity_public_key::KeyID {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                }

                impl ferment::FFIConversionTo<dpp::identity::identity_public_key::KeyID> for dpp_identity_identity_public_key_KeyID { unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::KeyID) -> *const dpp_identity_identity_public_key_KeyID { ferment::boxed(dpp_identity_identity_public_key_KeyID(obj)) } }

                impl Drop for dpp_identity_identity_public_key_KeyID {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ;
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyID_ctor<>(o_0: u32) -> *mut dpp_identity_identity_public_key_KeyID { ferment::boxed(dpp_identity_identity_public_key_KeyID(o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyID_destroy<>(ffi: *mut dpp_identity_identity_public_key_KeyID) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyID_get_0<>(obj: *const dpp_identity_identity_public_key_KeyID) -> u32 { (*obj).0 }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyID_set_0<>(obj: *const dpp_identity_identity_public_key_KeyID) -> u32 { (*obj).0 }

                #[doc = "FFI-representation of the [`TimestampMillis`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_identity_identity_public_key_TimestampMillis(u64);

                impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis> for dpp_identity_identity_public_key_TimestampMillis {
                    unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_TimestampMillis) -> dpp::identity::identity_public_key::TimestampMillis {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                }

                impl ferment::FFIConversionTo<dpp::identity::identity_public_key::TimestampMillis> for dpp_identity_identity_public_key_TimestampMillis { unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::TimestampMillis) -> *const dpp_identity_identity_public_key_TimestampMillis { ferment::boxed(dpp_identity_identity_public_key_TimestampMillis(obj)) } }

                impl Drop for dpp_identity_identity_public_key_TimestampMillis {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ;
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_TimestampMillis_ctor<>(o_0: u64) -> *mut dpp_identity_identity_public_key_TimestampMillis { ferment::boxed(dpp_identity_identity_public_key_TimestampMillis(o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_TimestampMillis_destroy<>(ffi: *mut dpp_identity_identity_public_key_TimestampMillis) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_TimestampMillis_get_0<>(obj: *const dpp_identity_identity_public_key_TimestampMillis) -> u64 { (*obj).0 }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_TimestampMillis_set_0<>(obj: *const dpp_identity_identity_public_key_TimestampMillis) -> u64 { (*obj).0 }

                pub mod purpose {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`Purpose`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_identity_identity_public_key_purpose_Purpose { AUTHENTICATION = 0, ENCRYPTION = 1, DECRYPTION = 2, TRANSFER = 3, SYSTEM = 4, VOTING = 5, OWNER = 6 }

                    impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::purpose::Purpose> for dpp_identity_identity_public_key_purpose_Purpose {
                        unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_purpose_Purpose) -> dpp::identity::identity_public_key::purpose::Purpose {
                            let ffi_ref = &*ffi;
                            match ffi_ref {
                                dpp_identity_identity_public_key_purpose_Purpose::AUTHENTICATION => dpp::identity::identity_public_key::purpose::Purpose::AUTHENTICATION,
                                dpp_identity_identity_public_key_purpose_Purpose::ENCRYPTION => dpp::identity::identity_public_key::purpose::Purpose::ENCRYPTION,
                                dpp_identity_identity_public_key_purpose_Purpose::DECRYPTION => dpp::identity::identity_public_key::purpose::Purpose::DECRYPTION,
                                dpp_identity_identity_public_key_purpose_Purpose::TRANSFER => dpp::identity::identity_public_key::purpose::Purpose::TRANSFER,
                                dpp_identity_identity_public_key_purpose_Purpose::SYSTEM => dpp::identity::identity_public_key::purpose::Purpose::SYSTEM,
                                dpp_identity_identity_public_key_purpose_Purpose::VOTING => dpp::identity::identity_public_key::purpose::Purpose::VOTING,
                                dpp_identity_identity_public_key_purpose_Purpose::OWNER => dpp::identity::identity_public_key::purpose::Purpose::OWNER
                            }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::identity::identity_public_key::purpose::Purpose> for dpp_identity_identity_public_key_purpose_Purpose {
                        unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::purpose::Purpose) -> *const dpp_identity_identity_public_key_purpose_Purpose {
                            ferment::boxed(match obj {
                                dpp::identity::identity_public_key::purpose::Purpose::AUTHENTICATION => dpp_identity_identity_public_key_purpose_Purpose::AUTHENTICATION,
                                dpp::identity::identity_public_key::purpose::Purpose::ENCRYPTION => dpp_identity_identity_public_key_purpose_Purpose::ENCRYPTION,
                                dpp::identity::identity_public_key::purpose::Purpose::DECRYPTION => dpp_identity_identity_public_key_purpose_Purpose::DECRYPTION,
                                dpp::identity::identity_public_key::purpose::Purpose::TRANSFER => dpp_identity_identity_public_key_purpose_Purpose::TRANSFER,
                                dpp::identity::identity_public_key::purpose::Purpose::SYSTEM => dpp_identity_identity_public_key_purpose_Purpose::SYSTEM,
                                dpp::identity::identity_public_key::purpose::Purpose::VOTING => dpp_identity_identity_public_key_purpose_Purpose::VOTING,
                                dpp::identity::identity_public_key::purpose::Purpose::OWNER => dpp_identity_identity_public_key_purpose_Purpose::OWNER,
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_identity_identity_public_key_purpose_Purpose {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_identity_identity_public_key_purpose_Purpose::AUTHENTICATION => {}
                                    dpp_identity_identity_public_key_purpose_Purpose::ENCRYPTION => {}
                                    dpp_identity_identity_public_key_purpose_Purpose::DECRYPTION => {}
                                    dpp_identity_identity_public_key_purpose_Purpose::TRANSFER => {}
                                    dpp_identity_identity_public_key_purpose_Purpose::SYSTEM => {}
                                    dpp_identity_identity_public_key_purpose_Purpose::VOTING => {}
                                    dpp_identity_identity_public_key_purpose_Purpose::OWNER => {}
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_AUTHENTICATION_ctor() -> *mut dpp_identity_identity_public_key_purpose_Purpose { ferment::boxed(dpp_identity_identity_public_key_purpose_Purpose::AUTHENTICATION {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_ENCRYPTION_ctor() -> *mut dpp_identity_identity_public_key_purpose_Purpose { ferment::boxed(dpp_identity_identity_public_key_purpose_Purpose::ENCRYPTION {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_DECRYPTION_ctor() -> *mut dpp_identity_identity_public_key_purpose_Purpose { ferment::boxed(dpp_identity_identity_public_key_purpose_Purpose::DECRYPTION {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_TRANSFER_ctor() -> *mut dpp_identity_identity_public_key_purpose_Purpose { ferment::boxed(dpp_identity_identity_public_key_purpose_Purpose::TRANSFER {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_SYSTEM_ctor() -> *mut dpp_identity_identity_public_key_purpose_Purpose { ferment::boxed(dpp_identity_identity_public_key_purpose_Purpose::SYSTEM {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_VOTING_ctor() -> *mut dpp_identity_identity_public_key_purpose_Purpose { ferment::boxed(dpp_identity_identity_public_key_purpose_Purpose::VOTING {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_OWNER_ctor() -> *mut dpp_identity_identity_public_key_purpose_Purpose { ferment::boxed(dpp_identity_identity_public_key_purpose_Purpose::OWNER {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_purpose_Purpose_destroy<>(ffi: *mut dpp_identity_identity_public_key_purpose_Purpose) { ferment::unbox_any(ffi); }
                }

                pub mod v0 {
                    use crate as dash_sdk_bindings;

                    pub mod methods { use crate as dash_sdk_bindings; }

                    #[doc = "FFI-representation of the [`IdentityPublicKeyV0`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
                        pub id: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID,
                        pub purpose: *mut crate::fermented::types::dpp::identity::identity_public_key::purpose::dpp_identity_identity_public_key_purpose_Purpose,
                        pub security_level: *mut crate::fermented::types::dpp::identity::identity_public_key::security_level::dpp_identity_identity_public_key_security_level_SecurityLevel,
                        pub contract_bounds: *mut crate::fermented::types::dpp::identity::identity_public_key::contract_bounds::dpp_identity_identity_public_key_contract_bounds_ContractBounds,
                        pub key_type: *mut crate::fermented::types::dpp::identity::identity_public_key::key_type::dpp_identity_identity_public_key_key_type_KeyType,
                        pub read_only: bool,
                        pub data: *mut crate::fermented::types::platform_value::types::binary_data::platform_value_types_binary_data_BinaryData,
                        pub disabled_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis,
                    }

                    impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::v0::IdentityPublicKeyV0> for dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
                        unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> dpp::identity::identity_public_key::v0::IdentityPublicKeyV0 {
                            let ffi_ref = &*ffi;
                            dpp::identity::identity_public_key::v0::IdentityPublicKeyV0 {
                                id: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionFrom<dpp::identity::identity_public_key::KeyID>>::ffi_from(ffi_ref.id),
                                purpose: <crate::fermented::types::dpp::identity::identity_public_key::purpose::dpp_identity_identity_public_key_purpose_Purpose as ferment::FFIConversionFrom<dpp::identity::identity_public_key::purpose::Purpose>>::ffi_from(ffi_ref.purpose),
                                security_level: <crate::fermented::types::dpp::identity::identity_public_key::security_level::dpp_identity_identity_public_key_security_level_SecurityLevel as ferment::FFIConversionFrom<dpp::identity::identity_public_key::security_level::SecurityLevel>>::ffi_from(ffi_ref.security_level),
                                contract_bounds: <crate::fermented::types::dpp::identity::identity_public_key::contract_bounds::dpp_identity_identity_public_key_contract_bounds_ContractBounds as ferment::FFIConversionFrom<dpp::identity::identity_public_key::contract_bounds::ContractBounds>>::ffi_from_opt(ffi_ref.contract_bounds),
                                key_type: <crate::fermented::types::dpp::identity::identity_public_key::key_type::dpp_identity_identity_public_key_key_type_KeyType as ferment::FFIConversionFrom<dpp::identity::identity_public_key::key_type::KeyType>>::ffi_from(ffi_ref.key_type),
                                read_only: ffi_ref.read_only,
                                data: <crate::fermented::types::platform_value::types::binary_data::platform_value_types_binary_data_BinaryData as ferment::FFIConversionFrom<platform_value::types::binary_data::BinaryData>>::ffi_from(ffi_ref.data),
                                disabled_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis>>::ffi_from_opt(ffi_ref.disabled_at),
                            }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::identity::identity_public_key::v0::IdentityPublicKeyV0> for dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
                        unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::v0::IdentityPublicKeyV0) -> *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
                            ferment::boxed(dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
                                id: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionTo<dpp::identity::identity_public_key::KeyID>>::ffi_to(obj.id),
                                purpose: <crate::fermented::types::dpp::identity::identity_public_key::purpose::dpp_identity_identity_public_key_purpose_Purpose as ferment::FFIConversionTo<dpp::identity::identity_public_key::purpose::Purpose>>::ffi_to(obj.purpose),
                                security_level: <crate::fermented::types::dpp::identity::identity_public_key::security_level::dpp_identity_identity_public_key_security_level_SecurityLevel as ferment::FFIConversionTo<dpp::identity::identity_public_key::security_level::SecurityLevel>>::ffi_to(obj.security_level),
                                contract_bounds: <crate::fermented::types::dpp::identity::identity_public_key::contract_bounds::dpp_identity_identity_public_key_contract_bounds_ContractBounds as ferment::FFIConversionTo<dpp::identity::identity_public_key::contract_bounds::ContractBounds>>::ffi_to_opt(obj.contract_bounds),
                                key_type: <crate::fermented::types::dpp::identity::identity_public_key::key_type::dpp_identity_identity_public_key_key_type_KeyType as ferment::FFIConversionTo<dpp::identity::identity_public_key::key_type::KeyType>>::ffi_to(obj.key_type),
                                read_only: obj.read_only,
                                data: <crate::fermented::types::platform_value::types::binary_data::platform_value_types_binary_data_BinaryData as ferment::FFIConversionTo<platform_value::types::binary_data::BinaryData>>::ffi_to(obj.data),
                                disabled_at: <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionTo<dpp::identity::identity_public_key::TimestampMillis>>::ffi_to_opt(obj.disabled_at),
                            })
                        }
                    }

                    impl Drop for dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.id);
                                ferment::unbox_any(ffi_ref.purpose);
                                ferment::unbox_any(ffi_ref.security_level);
                                ferment::unbox_any_opt(ffi_ref.contract_bounds);
                                ferment::unbox_any(ffi_ref.key_type);
                                ;
                                ferment::unbox_any(ffi_ref.data);
                                ferment::unbox_any_opt(ffi_ref.disabled_at);
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_ctor<>(id: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID, purpose: *mut crate::fermented::types::dpp::identity::identity_public_key::purpose::dpp_identity_identity_public_key_purpose_Purpose, security_level: *mut crate::fermented::types::dpp::identity::identity_public_key::security_level::dpp_identity_identity_public_key_security_level_SecurityLevel, contract_bounds: *mut crate::fermented::types::dpp::identity::identity_public_key::contract_bounds::dpp_identity_identity_public_key_contract_bounds_ContractBounds, key_type: *mut crate::fermented::types::dpp::identity::identity_public_key::key_type::dpp_identity_identity_public_key_key_type_KeyType, read_only: bool, data: *mut crate::fermented::types::platform_value::types::binary_data::platform_value_types_binary_data_BinaryData, disabled_at: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis) -> *mut dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 { ferment::boxed(dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 { id, purpose, security_level, contract_bounds, key_type, read_only, data, disabled_at }) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_destroy<>(ffi: *mut dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) { ferment::unbox_any(ffi); }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_id<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID { (*obj).id }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_purpose<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::purpose::dpp_identity_identity_public_key_purpose_Purpose { (*obj).purpose }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_security_level<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::security_level::dpp_identity_identity_public_key_security_level_SecurityLevel { (*obj).security_level }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_contract_bounds<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::contract_bounds::dpp_identity_identity_public_key_contract_bounds_ContractBounds { (*obj).contract_bounds }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_key_type<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::key_type::dpp_identity_identity_public_key_key_type_KeyType { (*obj).key_type }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_read_only<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> bool { (*obj).read_only }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_data<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::platform_value::types::binary_data::platform_value_types_binary_data_BinaryData { (*obj).data }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_get_disabled_at<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).disabled_at }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_id<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID { (*obj).id }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_purpose<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::purpose::dpp_identity_identity_public_key_purpose_Purpose { (*obj).purpose }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_security_level<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::security_level::dpp_identity_identity_public_key_security_level_SecurityLevel { (*obj).security_level }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_contract_bounds<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::contract_bounds::dpp_identity_identity_public_key_contract_bounds_ContractBounds { (*obj).contract_bounds }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_key_type<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::key_type::dpp_identity_identity_public_key_key_type_KeyType { (*obj).key_type }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_read_only<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> bool { (*obj).read_only }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_data<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::platform_value::types::binary_data::platform_value_types_binary_data_BinaryData { (*obj).data }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_set_disabled_at<>(obj: *const dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis { (*obj).disabled_at }

                    pub mod conversion { use crate as dash_sdk_bindings; }
                }

                pub mod security_level {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`SecurityLevel`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_identity_identity_public_key_security_level_SecurityLevel { MASTER = 0, CRITICAL = 1, HIGH = 2, MEDIUM = 3 }

                    impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::security_level::SecurityLevel> for dpp_identity_identity_public_key_security_level_SecurityLevel {
                        unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_security_level_SecurityLevel) -> dpp::identity::identity_public_key::security_level::SecurityLevel {
                            let ffi_ref = &*ffi;
                            match ffi_ref {
                                dpp_identity_identity_public_key_security_level_SecurityLevel::MASTER => dpp::identity::identity_public_key::security_level::SecurityLevel::MASTER,
                                dpp_identity_identity_public_key_security_level_SecurityLevel::CRITICAL => dpp::identity::identity_public_key::security_level::SecurityLevel::CRITICAL,
                                dpp_identity_identity_public_key_security_level_SecurityLevel::HIGH => dpp::identity::identity_public_key::security_level::SecurityLevel::HIGH,
                                dpp_identity_identity_public_key_security_level_SecurityLevel::MEDIUM => dpp::identity::identity_public_key::security_level::SecurityLevel::MEDIUM
                            }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::identity::identity_public_key::security_level::SecurityLevel> for dpp_identity_identity_public_key_security_level_SecurityLevel {
                        unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::security_level::SecurityLevel) -> *const dpp_identity_identity_public_key_security_level_SecurityLevel {
                            ferment::boxed(match obj {
                                dpp::identity::identity_public_key::security_level::SecurityLevel::MASTER => dpp_identity_identity_public_key_security_level_SecurityLevel::MASTER,
                                dpp::identity::identity_public_key::security_level::SecurityLevel::CRITICAL => dpp_identity_identity_public_key_security_level_SecurityLevel::CRITICAL,
                                dpp::identity::identity_public_key::security_level::SecurityLevel::HIGH => dpp_identity_identity_public_key_security_level_SecurityLevel::HIGH,
                                dpp::identity::identity_public_key::security_level::SecurityLevel::MEDIUM => dpp_identity_identity_public_key_security_level_SecurityLevel::MEDIUM,
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_identity_identity_public_key_security_level_SecurityLevel {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_identity_identity_public_key_security_level_SecurityLevel::MASTER => {}
                                    dpp_identity_identity_public_key_security_level_SecurityLevel::CRITICAL => {}
                                    dpp_identity_identity_public_key_security_level_SecurityLevel::HIGH => {}
                                    dpp_identity_identity_public_key_security_level_SecurityLevel::MEDIUM => {}
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_security_level_SecurityLevel_MASTER_ctor() -> *mut dpp_identity_identity_public_key_security_level_SecurityLevel { ferment::boxed(dpp_identity_identity_public_key_security_level_SecurityLevel::MASTER {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_security_level_SecurityLevel_CRITICAL_ctor() -> *mut dpp_identity_identity_public_key_security_level_SecurityLevel { ferment::boxed(dpp_identity_identity_public_key_security_level_SecurityLevel::CRITICAL {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_security_level_SecurityLevel_HIGH_ctor() -> *mut dpp_identity_identity_public_key_security_level_SecurityLevel { ferment::boxed(dpp_identity_identity_public_key_security_level_SecurityLevel::HIGH {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_security_level_SecurityLevel_MEDIUM_ctor() -> *mut dpp_identity_identity_public_key_security_level_SecurityLevel { ferment::boxed(dpp_identity_identity_public_key_security_level_SecurityLevel::MEDIUM {}) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_security_level_SecurityLevel_destroy<>(ffi: *mut dpp_identity_identity_public_key_security_level_SecurityLevel) { ferment::unbox_any(ffi); }
                }

                pub mod accessors { use crate as dash_sdk_bindings; }

                pub mod contract_bounds {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ContractBounds`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dpp_identity_identity_public_key_contract_bounds_ContractBounds { SingleContract { id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier }, SingleContractDocumentType { id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, document_type_name: *mut std::os::raw::c_char } }

                    impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::contract_bounds::ContractBounds> for dpp_identity_identity_public_key_contract_bounds_ContractBounds {
                        unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_contract_bounds_ContractBounds) -> dpp::identity::identity_public_key::contract_bounds::ContractBounds {
                            let ffi_ref = &*ffi;
                            match ffi_ref {
                                dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContract { id } => dpp::identity::identity_public_key::contract_bounds::ContractBounds::SingleContract { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(*id) },
                                dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContractDocumentType { id, document_type_name } => dpp::identity::identity_public_key::contract_bounds::ContractBounds::SingleContractDocumentType { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(*id), document_type_name: <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*document_type_name) }
                            }
                        }
                    }

                    impl ferment::FFIConversionTo<dpp::identity::identity_public_key::contract_bounds::ContractBounds> for dpp_identity_identity_public_key_contract_bounds_ContractBounds {
                        unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::contract_bounds::ContractBounds) -> *const dpp_identity_identity_public_key_contract_bounds_ContractBounds {
                            ferment::boxed(match obj {
                                dpp::identity::identity_public_key::contract_bounds::ContractBounds::SingleContract { id } => dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContract { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(id) },
                                dpp::identity::identity_public_key::contract_bounds::ContractBounds::SingleContractDocumentType { id, document_type_name } => dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContractDocumentType { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(id), document_type_name: <std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(document_type_name) },
                                _ => unreachable!("This is unreachable")
                            })
                        }
                    }

                    impl Drop for dpp_identity_identity_public_key_contract_bounds_ContractBounds {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContract { id } => { ferment::unbox_any(*id); }
                                    dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContractDocumentType { id, document_type_name } => {
                                        ferment::unbox_any(*id);
                                        ;
                                        ferment::unbox_string(*document_type_name);
                                    }
                                    _ => unreachable!("This is unreachable")
                                };
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContract_ctor(id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut dpp_identity_identity_public_key_contract_bounds_ContractBounds { ferment::boxed(dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContract { id }) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContractDocumentType_ctor(id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, document_type_name: *mut std::os::raw::c_char) -> *mut dpp_identity_identity_public_key_contract_bounds_ContractBounds { ferment::boxed(dpp_identity_identity_public_key_contract_bounds_ContractBounds::SingleContractDocumentType { id, document_type_name }) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dpp_identity_identity_public_key_contract_bounds_ContractBounds_destroy<>(ffi: *mut dpp_identity_identity_public_key_contract_bounds_ContractBounds) { ferment::unbox_any(ffi); }
                }

                pub mod methods {
                    use crate as dash_sdk_bindings;

                    pub mod hash { use crate as dash_sdk_bindings; }
                }

                #[doc = "FFI-representation of the [`KeyCount`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_identity_identity_public_key_KeyCount(*mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID);

                impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::KeyCount> for dpp_identity_identity_public_key_KeyCount {
                    unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_KeyCount) -> dpp::identity::identity_public_key::KeyCount {
                        let ffi_ref = &*ffi;
                        <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionFrom<dpp::identity::identity_public_key::KeyID>>::ffi_from(ffi_ref.0)
                    }
                }

                impl ferment::FFIConversionTo<dpp::identity::identity_public_key::KeyCount> for dpp_identity_identity_public_key_KeyCount { unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::KeyCount) -> *const dpp_identity_identity_public_key_KeyCount { ferment::boxed(dpp_identity_identity_public_key_KeyCount(<crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionTo<dpp::identity::identity_public_key::KeyID>>::ffi_to(obj))) } }

                impl Drop for dpp_identity_identity_public_key_KeyCount {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyCount_ctor<>(o_0: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID) -> *mut dpp_identity_identity_public_key_KeyCount { ferment::boxed(dpp_identity_identity_public_key_KeyCount(o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyCount_destroy<>(ffi: *mut dpp_identity_identity_public_key_KeyCount) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyCount_get_0<>(obj: *const dpp_identity_identity_public_key_KeyCount) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID { (*obj).0 }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_KeyCount_set_0<>(obj: *const dpp_identity_identity_public_key_KeyCount) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID { (*obj).0 }

                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`IdentityPublicKey`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dpp_identity_identity_public_key_IdentityPublicKey { V0(*mut crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) }

                impl ferment::FFIConversionFrom<dpp::identity::identity_public_key::IdentityPublicKey> for dpp_identity_identity_public_key_IdentityPublicKey {
                    unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_public_key_IdentityPublicKey) -> dpp::identity::identity_public_key::IdentityPublicKey {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dpp_identity_identity_public_key_IdentityPublicKey::V0(o_0) => dpp::identity::identity_public_key::IdentityPublicKey::V0(<crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 as ferment::FFIConversionFrom<dpp::identity::identity_public_key::v0::IdentityPublicKeyV0>>::ffi_from(*o_0)) }
                    }
                }

                impl ferment::FFIConversionTo<dpp::identity::identity_public_key::IdentityPublicKey> for dpp_identity_identity_public_key_IdentityPublicKey {
                    unsafe fn ffi_to_const(obj: dpp::identity::identity_public_key::IdentityPublicKey) -> *const dpp_identity_identity_public_key_IdentityPublicKey {
                        ferment::boxed(match obj {
                            dpp::identity::identity_public_key::IdentityPublicKey::V0(o_0) => dpp_identity_identity_public_key_IdentityPublicKey::V0(<crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 as ferment::FFIConversionTo<dpp::identity::identity_public_key::v0::IdentityPublicKeyV0>>::ffi_to(o_0)),
                            _ => unreachable!("This is unreachable")
                        })
                    }
                }

                impl Drop for dpp_identity_identity_public_key_IdentityPublicKey {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                dpp_identity_identity_public_key_IdentityPublicKey::V0(o_0) => { ferment::unbox_any(*o_0); }
                                _ => unreachable!("This is unreachable")
                            };
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_IdentityPublicKey_V0_ctor(o_o_0: *mut crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut dpp_identity_identity_public_key_IdentityPublicKey { ferment::boxed(dpp_identity_identity_public_key_IdentityPublicKey::V0(o_o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_public_key_IdentityPublicKey_destroy<>(ffi: *mut dpp_identity_identity_public_key_IdentityPublicKey) { ferment::unbox_any(ffi); }
            }

            pub mod accessors { use crate as dash_sdk_bindings; }

            pub mod identity {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`Identity`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dpp_identity_identity_Identity { V0(*mut crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0) }

                impl ferment::FFIConversionFrom<dpp::identity::identity::Identity> for dpp_identity_identity_Identity {
                    unsafe fn ffi_from_const(ffi: *const dpp_identity_identity_Identity) -> dpp::identity::identity::Identity {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dpp_identity_identity_Identity::V0(o_0) => dpp::identity::identity::Identity::V0(<crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0 as ferment::FFIConversionFrom<dpp::identity::v0::IdentityV0>>::ffi_from(*o_0)) }
                    }
                }

                impl ferment::FFIConversionTo<dpp::identity::identity::Identity> for dpp_identity_identity_Identity {
                    unsafe fn ffi_to_const(obj: dpp::identity::identity::Identity) -> *const dpp_identity_identity_Identity {
                        ferment::boxed(match obj {
                            dpp::identity::identity::Identity::V0(o_0) => dpp_identity_identity_Identity::V0(<crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0 as ferment::FFIConversionTo<dpp::identity::v0::IdentityV0>>::ffi_to(o_0)),
                            _ => unreachable!("This is unreachable")
                        })
                    }
                }

                impl Drop for dpp_identity_identity_Identity {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                dpp_identity_identity_Identity::V0(o_0) => { ferment::unbox_any(*o_0); }
                                _ => unreachable!("This is unreachable")
                            };
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_Identity_V0_ctor(o_o_0: *mut crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0) -> *mut dpp_identity_identity_Identity { ferment::boxed(dpp_identity_identity_Identity::V0(o_o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_identity_Identity_destroy<>(ffi: *mut dpp_identity_identity_Identity) { ferment::unbox_any(ffi); }
            }

            pub mod credits_converter { use crate as dash_sdk_bindings; }

            pub mod conversion {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "identity-cbor-conversion")]
                pub mod cbor {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "identity-value-conversion")]
                pub mod platform_value {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "identity-json-conversion")]
                pub mod json {
                    use crate as dash_sdk_bindings;
                }
            }

            pub mod methods {
                use crate as dash_sdk_bindings;

                pub mod create_basic_identity { use crate as dash_sdk_bindings; }
            }

            pub mod v0 {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`IdentityV0`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dpp_identity_v0_IdentityV0 {
                    pub id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
                    pub public_keys: *mut crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey,
                    pub balance: u64,
                    pub revision: *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision,
                }

                impl ferment::FFIConversionFrom<dpp::identity::v0::IdentityV0> for dpp_identity_v0_IdentityV0 {
                    unsafe fn ffi_from_const(ffi: *const dpp_identity_v0_IdentityV0) -> dpp::identity::v0::IdentityV0 {
                        let ffi_ref = &*ffi;
                        dpp::identity::v0::IdentityV0 { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.id), public_keys: <crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey>>>::ffi_from(ffi_ref.public_keys), balance: ffi_ref.balance, revision: <crate::fermented::types::dpp::prelude::dpp_prelude_Revision as ferment::FFIConversionFrom<dpp::prelude::Revision>>::ffi_from(ffi_ref.revision) }
                    }
                }

                impl ferment::FFIConversionTo<dpp::identity::v0::IdentityV0> for dpp_identity_v0_IdentityV0 { unsafe fn ffi_to_const(obj: dpp::identity::v0::IdentityV0) -> *const dpp_identity_v0_IdentityV0 { ferment::boxed(dpp_identity_v0_IdentityV0 { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.id), public_keys: <crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionTo<std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey>>>::ffi_to(obj.public_keys), balance: obj.balance, revision: <crate::fermented::types::dpp::prelude::dpp_prelude_Revision as ferment::FFIConversionTo<dpp::prelude::Revision>>::ffi_to(obj.revision) }) } }

                impl Drop for dpp_identity_v0_IdentityV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.id);
                            ferment::unbox_any(ffi_ref.public_keys);
                            ;
                            ferment::unbox_any(ffi_ref.revision);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_ctor<>(id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, public_keys: *mut crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey, balance: u64, revision: *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision) -> *mut dpp_identity_v0_IdentityV0 { ferment::boxed(dpp_identity_v0_IdentityV0 { id, public_keys, balance, revision }) }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_destroy<>(ffi: *mut dpp_identity_v0_IdentityV0) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_get_id<>(obj: *const dpp_identity_v0_IdentityV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).id }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_get_public_keys<>(obj: *const dpp_identity_v0_IdentityV0) -> *mut crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey { (*obj).public_keys }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_get_balance<>(obj: *const dpp_identity_v0_IdentityV0) -> u64 { (*obj).balance }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_get_revision<>(obj: *const dpp_identity_v0_IdentityV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision { (*obj).revision }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_set_id<>(obj: *const dpp_identity_v0_IdentityV0) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).id }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_set_public_keys<>(obj: *const dpp_identity_v0_IdentityV0) -> *mut crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey { (*obj).public_keys }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_set_balance<>(obj: *const dpp_identity_v0_IdentityV0) -> u64 { (*obj).balance }

                #[no_mangle]
                pub unsafe extern "C" fn dpp_identity_v0_IdentityV0_set_revision<>(obj: *const dpp_identity_v0_IdentityV0) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision { (*obj).revision }

                pub mod conversion { use crate as dash_sdk_bindings; }
            }

            pub mod state_transition {
                use crate as dash_sdk_bindings;

                pub mod asset_lock_proof {
                    use crate as dash_sdk_bindings;

                    pub mod validate_asset_lock_transaction_structure { use crate as dash_sdk_bindings; }

                    pub mod chain { use crate as dash_sdk_bindings; }

                    pub mod instant {
                        use crate as dash_sdk_bindings;

                        pub mod methods {
                            use crate as dash_sdk_bindings;

                            pub mod validate_structure { use crate as dash_sdk_bindings; }
                        }
                    }
                }
            }

            pub mod errors { use crate as dash_sdk_bindings; }

            pub mod fields { use crate as dash_sdk_bindings; }
        }

        #[cfg(feature = "fixtures-and-mocks")]
        pub mod tests {
            use crate as dash_sdk_bindings;

            pub mod fixtures { use crate as dash_sdk_bindings; }
        }

        pub mod util {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "cbor")]
            pub mod cbor_value {
                use crate as dash_sdk_bindings;

                pub mod convert { use crate as dash_sdk_bindings; }

                pub mod canonical { use crate as dash_sdk_bindings; }
            }

            pub mod json_schema { use crate as dash_sdk_bindings; }

            pub mod strings {
                use crate as dash_sdk_bindings;

                #[cfg(test)]
                pub mod tests {
                    use crate as dash_sdk_bindings;
                }
            }

            pub mod json_value {
                use crate as dash_sdk_bindings;

                pub mod remove_path { use crate as dash_sdk_bindings; }

                pub mod insert_with_path { use crate as dash_sdk_bindings; }
            }

            pub mod json_path { use crate as dash_sdk_bindings; }

            pub mod deserializer { use crate as dash_sdk_bindings; }
        }

        pub mod core_subsidy {
            use crate as dash_sdk_bindings;

            pub mod epoch_core_reward_credits_for_distribution {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }
        }

        #[cfg(feature = "system_contracts")]
        pub mod system_data_contracts {
            use crate as dash_sdk_bindings;
        }

        pub mod data_contract {
            use crate as dash_sdk_bindings;

            pub mod serialized_version { use crate as dash_sdk_bindings; }

            #[cfg(any(feature = "state-transitions", feature = "factories"))]
            pub mod created_data_contract {
                use crate as dash_sdk_bindings;

                pub mod fields { use crate as dash_sdk_bindings; }
            }

            pub mod accessors { use crate as dash_sdk_bindings; }

            pub mod group {
                use crate as dash_sdk_bindings;

                pub mod accessors { use crate as dash_sdk_bindings; }

                pub mod v0 {
                    use crate as dash_sdk_bindings;

                    #[cfg(test)]
                    pub mod tests {
                        use crate as dash_sdk_bindings;
                    }
                }

                pub mod methods { use crate as dash_sdk_bindings; }
            }

            pub mod v1 {
                use crate as dash_sdk_bindings;

                pub mod serialization { use crate as dash_sdk_bindings; }

                pub mod methods {
                    use crate as dash_sdk_bindings;

                    pub mod schema { use crate as dash_sdk_bindings; }
                }

                pub mod conversion { use crate as dash_sdk_bindings; }
            }

            pub mod document_type {
                use crate as dash_sdk_bindings;

                pub mod token_costs { use crate as dash_sdk_bindings; }

                pub mod v0 { use crate as dash_sdk_bindings; }

                pub mod class_methods {
                    use crate as dash_sdk_bindings;

                    pub mod try_from_schema {
                        use crate as dash_sdk_bindings;

                        pub mod v0 {
                            use crate as dash_sdk_bindings;

                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }

                        pub mod v1 {
                            use crate as dash_sdk_bindings;

                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }
                    }

                    pub mod create_document_types_from_document_schemas {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }
                }

                pub mod schema {
                    use crate as dash_sdk_bindings;

                    pub mod enrich_with_base_schema { use crate as dash_sdk_bindings; }

                    #[cfg(feature = "validation")]
                    pub mod validate_schema_compatibility {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "validation")]
                    pub mod recursive_schema_validator {
                        use crate as dash_sdk_bindings;

                        pub mod traversal_validator { use crate as dash_sdk_bindings; }
                    }

                    pub mod find_identifier_and_binary_paths { use crate as dash_sdk_bindings; }

                    #[cfg(feature = "validation")]
                    pub mod validate_max_depth {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }
                }

                pub mod methods {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "validation")]
                    pub mod validate_update {
                        use crate as dash_sdk_bindings;

                        pub mod v0 {
                            use crate as dash_sdk_bindings;

                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }
                    }
                }

                pub mod property { use crate as dash_sdk_bindings; }

                pub mod index_level { use crate as dash_sdk_bindings; }

                pub mod v1 { use crate as dash_sdk_bindings; }

                pub mod index { use crate as dash_sdk_bindings; }

                pub mod accessors { use crate as dash_sdk_bindings; }
            }

            pub mod change_control_rules { use crate as dash_sdk_bindings; }

            pub mod associated_token {
                use crate as dash_sdk_bindings;

                pub mod token_marketplace_rules {
                    use crate as dash_sdk_bindings;

                    pub mod accessors { use crate as dash_sdk_bindings; }

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod token_distribution_rules {
                    use crate as dash_sdk_bindings;

                    pub mod accessors { use crate as dash_sdk_bindings; }

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod token_perpetual_distribution {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }

                    pub mod reward_distribution_moment { use crate as dash_sdk_bindings; }

                    pub mod reward_distribution_type {
                        use crate as dash_sdk_bindings;

                        pub mod validation { use crate as dash_sdk_bindings; }
                    }

                    pub mod methods { use crate as dash_sdk_bindings; }

                    pub mod distribution_function {
                        use crate as dash_sdk_bindings;

                        pub mod evaluate_interval {
                            use crate as dash_sdk_bindings;

                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }

                        pub mod validation {
                            use crate as dash_sdk_bindings;

                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }

                        pub mod evaluate {
                            use crate as dash_sdk_bindings;

                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }
                    }
                }

                pub mod token_pre_programmed_distribution {
                    use crate as dash_sdk_bindings;

                    pub mod accessors { use crate as dash_sdk_bindings; }
                }

                pub mod token_configuration_convention {
                    use crate as dash_sdk_bindings;

                    pub mod accessors { use crate as dash_sdk_bindings; }

                    pub mod methods {
                        use crate as dash_sdk_bindings;

                        pub mod validate_localizations { use crate as dash_sdk_bindings; }
                    }
                }

                pub mod token_keeps_history_rules {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }

                    pub mod accessors { use crate as dash_sdk_bindings; }
                }

                pub mod token_configuration {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }

                    pub mod methods {
                        use crate as dash_sdk_bindings;

                        pub mod validate_token_configuration_update { use crate as dash_sdk_bindings; }

                        pub mod apply_token_configuration_item { use crate as dash_sdk_bindings; }

                        pub mod can_apply_token_configuration_item { use crate as dash_sdk_bindings; }

                        pub mod validate_token_configuration_groups_exist { use crate as dash_sdk_bindings; }

                        pub mod authorized_action_takers_for_configuration_item { use crate as dash_sdk_bindings; }
                    }

                    pub mod accessors { use crate as dash_sdk_bindings; }
                }

                pub mod token_configuration_localization {
                    use crate as dash_sdk_bindings;

                    pub mod accessors { use crate as dash_sdk_bindings; }

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(any(
                feature = "data-contract-value-conversion",
                feature = "data-contract-cbor-conversion",
                feature = "data-contract-json-conversion"
            ))]
            pub mod conversion {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "data-contract-cbor-conversion")]
                pub mod cbor {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "data-contract-value-conversion")]
                pub mod value {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "data-contract-json-conversion")]
                pub mod json {
                    use crate as dash_sdk_bindings;
                }
            }

            pub mod extra {
                use crate as dash_sdk_bindings;

                #[cfg(test)]
                pub mod drive_api_tests {
                    use crate as dash_sdk_bindings;
                }
            }

            pub mod storage_requirements { use crate as dash_sdk_bindings; }

            pub mod methods {
                use crate as dash_sdk_bindings;

                pub mod registration_cost { use crate as dash_sdk_bindings; }

                pub mod schema { use crate as dash_sdk_bindings; }

                #[cfg(feature = "validation")]
                pub mod validate_document {
                    use crate as dash_sdk_bindings;
                }

                pub mod equal_ignoring_time_based_fields { use crate as dash_sdk_bindings; }

                #[cfg(feature = "validation")]
                pub mod validate_update {
                    use crate as dash_sdk_bindings;

                    pub mod v0 {
                        use crate as dash_sdk_bindings;

                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }
                    }
                }

                #[cfg(feature = "validation")]
                pub mod validate_groups {
                    use crate as dash_sdk_bindings;
                }
            }

            pub mod v0 {
                use crate as dash_sdk_bindings;

                pub mod data_contract { use crate as dash_sdk_bindings; }

                pub mod conversion { use crate as dash_sdk_bindings; }

                pub mod serialization { use crate as dash_sdk_bindings; }

                pub mod methods {
                    use crate as dash_sdk_bindings;

                    pub mod schema { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(feature = "factories")]
            pub mod factory {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }
            }

            pub mod config {
                use crate as dash_sdk_bindings;

                pub mod fields { use crate as dash_sdk_bindings; }

                pub mod methods {
                    use crate as dash_sdk_bindings;

                    pub mod validate_update { use crate as dash_sdk_bindings; }
                }
            }

            pub mod errors { use crate as dash_sdk_bindings; }
        }
    }

    pub mod drive_proof_verifier {
        use crate as dash_sdk_bindings;

        pub mod types {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`VotePollsGroupedByTimestamp`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct drive_proof_verifier_types_VotePollsGroupedByTimestamp(*mut crate::fermented::generics::Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll);

            impl ferment::FFIConversionFrom<drive_proof_verifier::types::VotePollsGroupedByTimestamp> for drive_proof_verifier_types_VotePollsGroupedByTimestamp {
                unsafe fn ffi_from_const(ffi: *const drive_proof_verifier_types_VotePollsGroupedByTimestamp) -> drive_proof_verifier::types::VotePollsGroupedByTimestamp {
                    let ffi_ref = &*ffi;
                    drive_proof_verifier::types::VotePollsGroupedByTimestamp(<crate::fermented::generics::Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll as ferment::FFIConversionFrom<Vec<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)>>>::ffi_from(ffi_ref.0))
                }
            }

            impl ferment::FFIConversionTo<drive_proof_verifier::types::VotePollsGroupedByTimestamp> for drive_proof_verifier_types_VotePollsGroupedByTimestamp { unsafe fn ffi_to_const(obj: drive_proof_verifier::types::VotePollsGroupedByTimestamp) -> *const drive_proof_verifier_types_VotePollsGroupedByTimestamp { ferment::boxed(drive_proof_verifier_types_VotePollsGroupedByTimestamp(<crate::fermented::generics::Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll as ferment::FFIConversionTo<Vec<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)>>>::ffi_to(obj.0))) } }

            impl Drop for drive_proof_verifier_types_VotePollsGroupedByTimestamp {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_VotePollsGroupedByTimestamp_ctor<>(o_0: *mut crate::fermented::generics::Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll) -> *mut drive_proof_verifier_types_VotePollsGroupedByTimestamp { ferment::boxed(drive_proof_verifier_types_VotePollsGroupedByTimestamp(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_VotePollsGroupedByTimestamp_destroy<>(ffi: *mut drive_proof_verifier_types_VotePollsGroupedByTimestamp) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_VotePollsGroupedByTimestamp_get_0<>(obj: *const drive_proof_verifier_types_VotePollsGroupedByTimestamp) -> *mut crate::fermented::generics::Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_VotePollsGroupedByTimestamp_set_0<>(obj: *const drive_proof_verifier_types_VotePollsGroupedByTimestamp) -> *mut crate::fermented::generics::Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { (*obj).0 }

            #[doc = "FFI-representation of the [`ContestedResources`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct drive_proof_verifier_types_ContestedResources(*mut crate::fermented::generics::Vec_drive_proof_verifier_types_ContestedResource);

            impl ferment::FFIConversionFrom<drive_proof_verifier::types::ContestedResources> for drive_proof_verifier_types_ContestedResources {
                unsafe fn ffi_from_const(ffi: *const drive_proof_verifier_types_ContestedResources) -> drive_proof_verifier::types::ContestedResources {
                    let ffi_ref = &*ffi;
                    drive_proof_verifier::types::ContestedResources(<crate::fermented::generics::Vec_drive_proof_verifier_types_ContestedResource as ferment::FFIConversionFrom<Vec<drive_proof_verifier::types::ContestedResource>>>::ffi_from(ffi_ref.0))
                }
            }

            impl ferment::FFIConversionTo<drive_proof_verifier::types::ContestedResources> for drive_proof_verifier_types_ContestedResources { unsafe fn ffi_to_const(obj: drive_proof_verifier::types::ContestedResources) -> *const drive_proof_verifier_types_ContestedResources { ferment::boxed(drive_proof_verifier_types_ContestedResources(<crate::fermented::generics::Vec_drive_proof_verifier_types_ContestedResource as ferment::FFIConversionTo<Vec<drive_proof_verifier::types::ContestedResource>>>::ffi_to(obj.0))) } }

            impl Drop for drive_proof_verifier_types_ContestedResources {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResources_ctor<>(o_0: *mut crate::fermented::generics::Vec_drive_proof_verifier_types_ContestedResource) -> *mut drive_proof_verifier_types_ContestedResources { ferment::boxed(drive_proof_verifier_types_ContestedResources(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResources_destroy<>(ffi: *mut drive_proof_verifier_types_ContestedResources) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResources_get_0<>(obj: *const drive_proof_verifier_types_ContestedResources) -> *mut crate::fermented::generics::Vec_drive_proof_verifier_types_ContestedResource { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResources_set_0<>(obj: *const drive_proof_verifier_types_ContestedResources) -> *mut crate::fermented::generics::Vec_drive_proof_verifier_types_ContestedResource { (*obj).0 }

            #[doc = "FFI-representation of the [`Contenders`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct drive_proof_verifier_types_Contenders {
                pub winner: *mut crate::fermented::generics::Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo,
                pub contenders: *mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument,
                pub abstain_vote_tally: *mut u32,
                pub lock_vote_tally: *mut u32,
            }

            impl ferment::FFIConversionFrom<drive_proof_verifier::types::Contenders> for drive_proof_verifier_types_Contenders {
                unsafe fn ffi_from_const(ffi: *const drive_proof_verifier_types_Contenders) -> drive_proof_verifier::types::Contenders {
                    let ffi_ref = &*ffi;
                    drive_proof_verifier::types::Contenders {
                        winner: <crate::fermented::generics::Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo as ferment::FFIConversionFrom<(dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo, dpp::block::block_info::BlockInfo)>>::ffi_from_opt(ffi_ref.winner),
                        contenders: <crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument as ferment::FFIConversionFrom<std::collections::BTreeMap<platform_value::types::identifier::Identifier, dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>>>::ffi_from(ffi_ref.contenders),
                        abstain_vote_tally: ferment::from_opt_primitive(ffi_ref.abstain_vote_tally),
                        lock_vote_tally: ferment::from_opt_primitive(ffi_ref.lock_vote_tally),
                    }
                }
            }

            impl ferment::FFIConversionTo<drive_proof_verifier::types::Contenders> for drive_proof_verifier_types_Contenders {
                unsafe fn ffi_to_const(obj: drive_proof_verifier::types::Contenders) -> *const drive_proof_verifier_types_Contenders {
                    ferment::boxed(drive_proof_verifier_types_Contenders { winner: <crate::fermented::generics::Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo as ferment::FFIConversionTo<(dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo, dpp::block::block_info::BlockInfo)>>::ffi_to_opt(obj.winner), contenders: <crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument as ferment::FFIConversionTo<std::collections::BTreeMap<platform_value::types::identifier::Identifier, dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>>>::ffi_to(obj.contenders), abstain_vote_tally: ferment::to_opt_primitive(obj.abstain_vote_tally), lock_vote_tally: ferment::to_opt_primitive(obj.lock_vote_tally) })
                }
            }

            impl Drop for drive_proof_verifier_types_Contenders {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any_opt(ffi_ref.winner);
                        ferment::unbox_any(ffi_ref.contenders);
                        ferment::destroy_opt_primitive(ffi_ref.abstain_vote_tally);
                        ferment::destroy_opt_primitive(ffi_ref.lock_vote_tally);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_ctor<>(winner: *mut crate::fermented::generics::Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo, contenders: *mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument, abstain_vote_tally: *mut u32, lock_vote_tally: *mut u32) -> *mut drive_proof_verifier_types_Contenders { ferment::boxed(drive_proof_verifier_types_Contenders { winner, contenders, abstain_vote_tally, lock_vote_tally }) }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_destroy<>(ffi: *mut drive_proof_verifier_types_Contenders) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_get_winner<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut crate::fermented::generics::Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo { (*obj).winner }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_get_contenders<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { (*obj).contenders }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_get_abstain_vote_tally<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut u32 { (*obj).abstain_vote_tally }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_get_lock_vote_tally<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut u32 { (*obj).lock_vote_tally }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_set_winner<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut crate::fermented::generics::Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo { (*obj).winner }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_set_contenders<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { (*obj).contenders }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_set_abstain_vote_tally<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut u32 { (*obj).abstain_vote_tally }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Contenders_set_lock_vote_tally<>(obj: *const drive_proof_verifier_types_Contenders) -> *mut u32 { (*obj).lock_vote_tally }

            #[doc = "FFI-representation of the [`Voters`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct drive_proof_verifier_types_Voters(*mut crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter);

            impl ferment::FFIConversionFrom<drive_proof_verifier::types::Voters> for drive_proof_verifier_types_Voters {
                unsafe fn ffi_from_const(ffi: *const drive_proof_verifier_types_Voters) -> drive_proof_verifier::types::Voters {
                    let ffi_ref = &*ffi;
                    drive_proof_verifier::types::Voters(<crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter as ferment::FFIConversionFrom<std::collections::BTreeSet<drive_proof_verifier::types::Voter>>>::ffi_from(ffi_ref.0))
                }
            }

            impl ferment::FFIConversionTo<drive_proof_verifier::types::Voters> for drive_proof_verifier_types_Voters { unsafe fn ffi_to_const(obj: drive_proof_verifier::types::Voters) -> *const drive_proof_verifier_types_Voters { ferment::boxed(drive_proof_verifier_types_Voters(<crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter as ferment::FFIConversionTo<std::collections::BTreeSet<drive_proof_verifier::types::Voter>>>::ffi_to(obj.0))) } }

            impl Drop for drive_proof_verifier_types_Voters {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voters_ctor<>(o_0: *mut crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter) -> *mut drive_proof_verifier_types_Voters { ferment::boxed(drive_proof_verifier_types_Voters(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voters_destroy<>(ffi: *mut drive_proof_verifier_types_Voters) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voters_get_0<>(obj: *const drive_proof_verifier_types_Voters) -> *mut crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voters_set_0<>(obj: *const drive_proof_verifier_types_Voters) -> *mut crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter { (*obj).0 }

            #[doc = "FFI-representation of the [`ResourceVotesByIdentity`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct drive_proof_verifier_types_ResourceVotesByIdentity(*mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote);

            impl ferment::FFIConversionFrom<drive_proof_verifier::types::ResourceVotesByIdentity> for drive_proof_verifier_types_ResourceVotesByIdentity {
                unsafe fn ffi_from_const(ffi: *const drive_proof_verifier_types_ResourceVotesByIdentity) -> drive_proof_verifier::types::ResourceVotesByIdentity {
                    let ffi_ref = &*ffi;
                    <crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote as ferment::FFIConversionFrom<std::collections::BTreeMap<platform_value::types::identifier::Identifier, Option<dpp::voting::votes::resource_vote::ResourceVote>>>>::ffi_from(ffi_ref.0)
                }
            }

            impl ferment::FFIConversionTo<drive_proof_verifier::types::ResourceVotesByIdentity> for drive_proof_verifier_types_ResourceVotesByIdentity { unsafe fn ffi_to_const(obj: drive_proof_verifier::types::ResourceVotesByIdentity) -> *const drive_proof_verifier_types_ResourceVotesByIdentity { ferment::boxed(drive_proof_verifier_types_ResourceVotesByIdentity(<crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote as ferment::FFIConversionTo<std::collections::BTreeMap<platform_value::types::identifier::Identifier, Option<dpp::voting::votes::resource_vote::ResourceVote>>>>::ffi_to(obj))) } }

            impl Drop for drive_proof_verifier_types_ResourceVotesByIdentity {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ResourceVotesByIdentity_ctor<>(o_0: *mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote) -> *mut drive_proof_verifier_types_ResourceVotesByIdentity { ferment::boxed(drive_proof_verifier_types_ResourceVotesByIdentity(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ResourceVotesByIdentity_destroy<>(ffi: *mut drive_proof_verifier_types_ResourceVotesByIdentity) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ResourceVotesByIdentity_get_0<>(obj: *const drive_proof_verifier_types_ResourceVotesByIdentity) -> *mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ResourceVotesByIdentity_set_0<>(obj: *const drive_proof_verifier_types_ResourceVotesByIdentity) -> *mut crate::fermented::generics::std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote { (*obj).0 }

            #[doc = "FFI-representation of the [`ContestedResource`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct drive_proof_verifier_types_ContestedResource(*mut crate::fermented::types::platform_value::platform_value_Value);

            impl ferment::FFIConversionFrom<drive_proof_verifier::types::ContestedResource> for drive_proof_verifier_types_ContestedResource {
                unsafe fn ffi_from_const(ffi: *const drive_proof_verifier_types_ContestedResource) -> drive_proof_verifier::types::ContestedResource {
                    let ffi_ref = &*ffi;
                    drive_proof_verifier::types::ContestedResource(<crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from(ffi_ref.0))
                }
            }

            impl ferment::FFIConversionTo<drive_proof_verifier::types::ContestedResource> for drive_proof_verifier_types_ContestedResource { unsafe fn ffi_to_const(obj: drive_proof_verifier::types::ContestedResource) -> *const drive_proof_verifier_types_ContestedResource { ferment::boxed(drive_proof_verifier_types_ContestedResource(<crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionTo<platform_value::Value>>::ffi_to(obj.0))) } }

            impl Drop for drive_proof_verifier_types_ContestedResource {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResource_ctor<>(o_0: *mut crate::fermented::types::platform_value::platform_value_Value) -> *mut drive_proof_verifier_types_ContestedResource { ferment::boxed(drive_proof_verifier_types_ContestedResource(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResource_destroy<>(ffi: *mut drive_proof_verifier_types_ContestedResource) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResource_get_0<>(obj: *const drive_proof_verifier_types_ContestedResource) -> *mut crate::fermented::types::platform_value::platform_value_Value { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_ContestedResource_set_0<>(obj: *const drive_proof_verifier_types_ContestedResource) -> *mut crate::fermented::types::platform_value::platform_value_Value { (*obj).0 }

            #[doc = "FFI-representation of the [`Voter`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct drive_proof_verifier_types_Voter(*mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier);

            impl ferment::FFIConversionFrom<drive_proof_verifier::types::Voter> for drive_proof_verifier_types_Voter {
                unsafe fn ffi_from_const(ffi: *const drive_proof_verifier_types_Voter) -> drive_proof_verifier::types::Voter {
                    let ffi_ref = &*ffi;
                    drive_proof_verifier::types::Voter(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.0))
                }
            }

            impl ferment::FFIConversionTo<drive_proof_verifier::types::Voter> for drive_proof_verifier_types_Voter { unsafe fn ffi_to_const(obj: drive_proof_verifier::types::Voter) -> *const drive_proof_verifier_types_Voter { ferment::boxed(drive_proof_verifier_types_Voter(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.0))) } }

            impl Drop for drive_proof_verifier_types_Voter {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voter_ctor<>(o_0: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut drive_proof_verifier_types_Voter { ferment::boxed(drive_proof_verifier_types_Voter(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voter_destroy<>(ffi: *mut drive_proof_verifier_types_Voter) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voter_get_0<>(obj: *const drive_proof_verifier_types_Voter) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn drive_proof_verifier_types_Voter_set_0<>(obj: *const drive_proof_verifier_types_Voter) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).0 }
        }

        pub mod proof { use crate as dash_sdk_bindings; }
    }

    pub mod dash_sdk {
        use crate as dash_sdk_bindings;

        pub mod error {
            use crate as dash_sdk_bindings;

            #[cfg(test)]
            pub mod tests {
                use crate as dash_sdk_bindings;
            }
        }

        pub mod mock {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "mocks")]
            pub mod sdk {
                use crate as dash_sdk_bindings;
            }
        }

        pub mod sync { use crate as dash_sdk_bindings; }

        pub mod core { use crate as dash_sdk_bindings; }

        pub mod platform {
            use crate as dash_sdk_bindings;

            pub mod transition { use crate as dash_sdk_bindings; }

            pub mod tokens {
                use crate as dash_sdk_bindings;

                pub mod transitions { use crate as dash_sdk_bindings; }

                pub mod builders { use crate as dash_sdk_bindings; }
            }

            pub mod documents {
                use crate as dash_sdk_bindings;

                pub mod transitions { use crate as dash_sdk_bindings; }
            }

            pub mod types {
                use crate as dash_sdk_bindings;

                pub mod identity {
                    use crate as dash_sdk_bindings;

                    #[doc = "FFI-representation of the [`PublicKeyHash`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dash_sdk_platform_types_identity_PublicKeyHash(*mut crate::fermented::generics::Arr_u8_20);

                    impl ferment::FFIConversionFrom<dash_sdk::platform::types::identity::PublicKeyHash> for dash_sdk_platform_types_identity_PublicKeyHash {
                        unsafe fn ffi_from_const(ffi: *const dash_sdk_platform_types_identity_PublicKeyHash) -> dash_sdk::platform::types::identity::PublicKeyHash {
                            let ffi_ref = &*ffi;
                            dash_sdk::platform::types::identity::PublicKeyHash(<crate::fermented::generics::Arr_u8_20 as ferment::FFIConversionFrom<[u8; 20]>>::ffi_from(ffi_ref.0))
                        }
                    }

                    impl ferment::FFIConversionTo<dash_sdk::platform::types::identity::PublicKeyHash> for dash_sdk_platform_types_identity_PublicKeyHash { unsafe fn ffi_to_const(obj: dash_sdk::platform::types::identity::PublicKeyHash) -> *const dash_sdk_platform_types_identity_PublicKeyHash { ferment::boxed(dash_sdk_platform_types_identity_PublicKeyHash(<crate::fermented::generics::Arr_u8_20 as ferment::FFIConversionTo<[u8; 20]>>::ffi_to(obj.0))) } }

                    impl Drop for dash_sdk_platform_types_identity_PublicKeyHash {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.0);
                            }
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn dash_sdk_platform_types_identity_PublicKeyHash_ctor<>(o_0: *mut crate::fermented::generics::Arr_u8_20) -> *mut dash_sdk_platform_types_identity_PublicKeyHash { ferment::boxed(dash_sdk_platform_types_identity_PublicKeyHash(o_0)) }

                    #[no_mangle]
                    pub unsafe extern "C" fn dash_sdk_platform_types_identity_PublicKeyHash_destroy<>(ffi: *mut dash_sdk_platform_types_identity_PublicKeyHash) { ferment::unbox_any(ffi); }

                    #[no_mangle]
                    pub unsafe extern "C" fn dash_sdk_platform_types_identity_PublicKeyHash_get_0<>(obj: *const dash_sdk_platform_types_identity_PublicKeyHash) -> *mut crate::fermented::generics::Arr_u8_20 { (*obj).0 }

                    #[no_mangle]
                    pub unsafe extern "C" fn dash_sdk_platform_types_identity_PublicKeyHash_set_0<>(obj: *const dash_sdk_platform_types_identity_PublicKeyHash) -> *mut crate::fermented::generics::Arr_u8_20 { (*obj).0 }
                }
            }
        }

        pub mod sdk { use crate as dash_sdk_bindings; }
    }

    pub mod drive {
        use crate as dash_sdk_bindings;

        #[cfg(feature = "server")]
        pub mod state_transition_action {
            use crate as dash_sdk_bindings;

            pub mod batch {
                use crate as dash_sdk_bindings;

                pub mod batched_transition {
                    use crate as dash_sdk_bindings;

                    pub mod token_transition {
                        use crate as dash_sdk_bindings;

                        pub mod token_claim_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_unfreeze_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_freeze_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_mint_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_base_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_set_price_for_direct_purchase_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_direct_purchase_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_transfer_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_emergency_action_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_destroy_frozen_funds_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_burn_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod token_config_update_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }
                    }

                    pub mod document_transition {
                        use crate as dash_sdk_bindings;

                        pub mod document_purchase_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod document_replace_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod document_transfer_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod document_create_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod document_base_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod document_delete_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod document_update_price_transition_action {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }
                    }
                }
            }

            pub mod action_convert_to_operations {
                use crate as dash_sdk_bindings;

                pub mod system { use crate as dash_sdk_bindings; }

                pub mod batch {
                    use crate as dash_sdk_bindings;

                    pub mod document { use crate as dash_sdk_bindings; }

                    pub mod token { use crate as dash_sdk_bindings; }
                }

                pub mod contract { use crate as dash_sdk_bindings; }

                pub mod identity { use crate as dash_sdk_bindings; }
            }

            pub mod identity {
                use crate as dash_sdk_bindings;

                pub mod identity_update {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod identity_create {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod identity_credit_transfer {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod identity_topup {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod masternode_vote {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod identity_credit_withdrawal {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }

                    pub mod transformer { use crate as dash_sdk_bindings; }
                }
            }

            pub mod system {
                use crate as dash_sdk_bindings;

                pub mod bump_identity_nonce_action {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod partially_use_asset_lock_action {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod bump_identity_data_contract_nonce_action {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }
            }

            pub mod contract {
                use crate as dash_sdk_bindings;

                pub mod data_contract_create {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod data_contract_update {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }
            }
        }

        #[cfg(any(feature = "server", feature = "verify"))]
        pub mod drive {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "server")]
            pub mod platform_state {
                use crate as dash_sdk_bindings;

                pub mod fetch_platform_state_bytes { use crate as dash_sdk_bindings; }

                pub mod store_platform_state_bytes { use crate as dash_sdk_bindings; }
            }

            #[cfg(feature = "server")]
            pub mod initialization {
                use crate as dash_sdk_bindings;

                pub mod v0 { use crate as dash_sdk_bindings; }

                pub mod genesis_core_height { use crate as dash_sdk_bindings; }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod votes {
                use crate as dash_sdk_bindings;

                #[cfg(any(feature = "server", feature = "verify"))]
                pub mod resolved {
                    use crate as dash_sdk_bindings;

                    pub mod votes {
                        use crate as dash_sdk_bindings;

                        pub mod resolved_resource_vote {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }

                            pub mod accessors { use crate as dash_sdk_bindings; }
                        }
                    }

                    pub mod vote_polls {
                        use crate as dash_sdk_bindings;

                        pub mod contested_document_resource_vote_poll { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(feature = "server")]
                pub mod cleanup {
                    use crate as dash_sdk_bindings;

                    pub mod remove_all_votes_given_by_identities { use crate as dash_sdk_bindings; }

                    pub mod remove_contested_resource_vote_poll_top_level_index_operations { use crate as dash_sdk_bindings; }

                    pub mod remove_contested_resource_vote_poll_votes_operations { use crate as dash_sdk_bindings; }

                    pub mod remove_specific_votes_given_by_identity { use crate as dash_sdk_bindings; }

                    pub mod remove_contested_resource_vote_poll_end_date_query_operations { use crate as dash_sdk_bindings; }

                    pub mod remove_contested_resource_vote_poll_info_operations { use crate as dash_sdk_bindings; }

                    pub mod remove_contested_resource_vote_poll_contenders_operations { use crate as dash_sdk_bindings; }

                    pub mod remove_contested_resource_vote_poll_documents_operations { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod fetch {
                    use crate as dash_sdk_bindings;

                    pub mod fetch_identities_voting_for_contenders { use crate as dash_sdk_bindings; }

                    pub mod fetch_contested_document_vote_poll_stored_info { use crate as dash_sdk_bindings; }

                    pub mod fetch_identity_contested_resource_vote { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod insert {
                    use crate as dash_sdk_bindings;

                    pub mod register_identity_vote { use crate as dash_sdk_bindings; }

                    pub mod vote_poll {
                        use crate as dash_sdk_bindings;

                        pub mod add_vote_poll_end_date_query_operations { use crate as dash_sdk_bindings; }
                    }

                    pub mod contested_resource {
                        use crate as dash_sdk_bindings;

                        pub mod individual_vote {
                            use crate as dash_sdk_bindings;

                            pub mod register_contested_resource_identity_vote { use crate as dash_sdk_bindings; }
                        }

                        pub mod insert_stored_info_for_contested_resource_vote_poll { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(any(feature = "server", feature = "verify"))]
                pub mod storage_form {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod setup {
                    use crate as dash_sdk_bindings;

                    pub mod setup_initial_vote_tree_main_structure { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod identity {
                use crate as dash_sdk_bindings;

                #[cfg(any(feature = "server", feature = "verify"))]
                pub mod contract_info {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod identity_contract_nonce {
                        use crate as dash_sdk_bindings;

                        pub mod merge_identity_contract_nonce {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        #[cfg(feature = "server")]
                        pub mod fetch_identity_contract_nonce {
                            use crate as dash_sdk_bindings;
                        }

                        #[cfg(feature = "server")]
                        pub mod prove_identity_contract_nonce {
                            use crate as dash_sdk_bindings;
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod keys {
                        use crate as dash_sdk_bindings;

                        pub mod add_potential_contract_info_for_contract_bounded_key { use crate as dash_sdk_bindings; }

                        pub mod refresh_potential_contract_info_key_references { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(feature = "server")]
                pub mod withdrawals {
                    use crate as dash_sdk_bindings;

                    pub mod transaction {
                        use crate as dash_sdk_bindings;

                        pub mod index {
                            use crate as dash_sdk_bindings;

                            pub mod add_update_next_withdrawal_transaction_index_operation { use crate as dash_sdk_bindings; }

                            pub mod fetch_next_withdrawal_transaction_index { use crate as dash_sdk_bindings; }
                        }

                        pub mod queue {
                            use crate as dash_sdk_bindings;

                            pub mod add_enqueue_untied_withdrawal_transaction_operations { use crate as dash_sdk_bindings; }

                            pub mod dequeue_untied_withdrawal_transactions { use crate as dash_sdk_bindings; }

                            pub mod remove_broadcasted_withdrawal_transactions_after_completion { use crate as dash_sdk_bindings; }

                            pub mod move_broadcasted_withdrawal_transactions_back_to_queue_operations { use crate as dash_sdk_bindings; }
                        }
                    }

                    pub mod document {
                        use crate as dash_sdk_bindings;

                        pub mod fetch_oldest_withdrawal_documents_by_status {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod find_withdrawal_documents_by_status_and_transaction_indices {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }
                    }

                    pub mod calculate_current_withdrawal_limit { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod update {
                    use crate as dash_sdk_bindings;

                    pub mod operations {
                        use crate as dash_sdk_bindings;

                        pub mod update_identity_balance_operation { use crate as dash_sdk_bindings; }

                        pub mod insert_identity_balance_operation { use crate as dash_sdk_bindings; }

                        pub mod initialize_negative_identity_balance_operation { use crate as dash_sdk_bindings; }

                        pub mod merge_identity_nonce_operations {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod update_identity_revision_operation { use crate as dash_sdk_bindings; }

                        pub mod initialize_identity_revision_operation { use crate as dash_sdk_bindings; }

                        pub mod initialize_identity_nonce_operation { use crate as dash_sdk_bindings; }

                        pub mod update_identity_negative_credit_operation { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(test)]
                    pub mod tests {
                        use crate as dash_sdk_bindings;
                    }

                    pub mod methods {
                        use crate as dash_sdk_bindings;

                        pub mod add_to_identity_balance { use crate as dash_sdk_bindings; }

                        pub mod add_to_previous_balance { use crate as dash_sdk_bindings; }

                        pub mod apply_balance_change_from_fee_to_identity { use crate as dash_sdk_bindings; }

                        pub mod re_enable_identity_keys { use crate as dash_sdk_bindings; }

                        pub mod add_new_unique_keys_to_identity { use crate as dash_sdk_bindings; }

                        pub mod add_new_non_unique_keys_to_identity { use crate as dash_sdk_bindings; }

                        pub mod update_identity_revision { use crate as dash_sdk_bindings; }

                        pub mod refresh_identity_key_reference_operations { use crate as dash_sdk_bindings; }

                        pub mod remove_from_identity_balance { use crate as dash_sdk_bindings; }

                        pub mod merge_identity_nonce { use crate as dash_sdk_bindings; }

                        pub mod disable_identity_keys { use crate as dash_sdk_bindings; }

                        pub mod add_new_keys_to_identity { use crate as dash_sdk_bindings; }
                    }

                    pub mod structs {
                        use crate as dash_sdk_bindings;

                        pub mod add_to_previous_balance_outcome { use crate as dash_sdk_bindings; }

                        pub mod apply_balance_change_outcome { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(any(feature = "server", feature = "verify"))]
                pub mod fetch {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod contract_keys {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_by_public_key_hashes {
                        use crate as dash_sdk_bindings;

                        pub mod fetch_full_identity_by_non_unique_public_key_hash { use crate as dash_sdk_bindings; }

                        pub mod fetch_full_identities_by_unique_public_key_hashes { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_id_by_unique_public_key_hash { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_ids_by_non_unique_public_key_hash { use crate as dash_sdk_bindings; }

                        pub mod fetch_full_identity_by_unique_public_key_hash { use crate as dash_sdk_bindings; }

                        pub mod has_any_of_unique_public_key_hashes { use crate as dash_sdk_bindings; }

                        pub mod has_non_unique_public_key_hash_already_for_identity { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_ids_by_unique_public_key_hashes { use crate as dash_sdk_bindings; }

                        pub mod has_non_unique_public_key_hash { use crate as dash_sdk_bindings; }

                        pub mod has_unique_public_key_hash { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod revision {
                        use crate as dash_sdk_bindings;

                        pub mod fetch_identity_revision { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod balance {
                        use crate as dash_sdk_bindings;

                        pub mod fetch_identity_balance { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_balance_include_debt { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_negative_balance { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod nonce {
                        use crate as dash_sdk_bindings;

                        pub mod prove_identity_nonce { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_nonce { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod full_identity {
                        use crate as dash_sdk_bindings;

                        #[cfg(feature = "server")]
                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }

                        pub mod fetch_full_identities { use crate as dash_sdk_bindings; }

                        pub mod fetch_full_identity { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod partial_identity {
                        use crate as dash_sdk_bindings;

                        pub mod fetch_identity_keys { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_balance_with_keys { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_revision_with_keys { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_with_balance { use crate as dash_sdk_bindings; }

                        pub mod fetch_identity_balance_with_keys_and_revision { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod prove {
                        use crate as dash_sdk_bindings;

                        pub mod prove_full_identity_by_non_unique_public_key_hash {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod prove_full_identities {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod prove_identity_ids_by_unique_public_key_hashes {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod prove_full_identities_by_unique_public_key_hashes {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod prove_identity_id_by_unique_public_key_hash {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod prove_full_identity {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod prove_identities_contract_keys { use crate as dash_sdk_bindings; }

                        pub mod prove_full_identity_by_unique_public_key_hash {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }
                    }
                }

                #[cfg(feature = "server")]
                pub mod estimation_costs {
                    use crate as dash_sdk_bindings;

                    pub mod for_keys_for_identity_id { use crate as dash_sdk_bindings; }

                    pub mod for_balances { use crate as dash_sdk_bindings; }

                    pub mod for_update_nonce { use crate as dash_sdk_bindings; }

                    pub mod for_identity_contract_info_group { use crate as dash_sdk_bindings; }

                    pub mod for_identity_contract_info_group_keys { use crate as dash_sdk_bindings; }

                    pub mod for_purpose_in_key_reference_tree { use crate as dash_sdk_bindings; }

                    pub mod for_identity_contract_info_group_key_purpose { use crate as dash_sdk_bindings; }

                    pub mod for_identity_contract_info { use crate as dash_sdk_bindings; }

                    pub mod for_root_key_reference_tree { use crate as dash_sdk_bindings; }

                    pub mod for_update_revision { use crate as dash_sdk_bindings; }

                    pub mod for_negative_credit { use crate as dash_sdk_bindings; }

                    pub mod for_authentication_keys_security_level_in_key_reference_tree { use crate as dash_sdk_bindings; }
                }

                #[cfg(any(feature = "server", feature = "verify"))]
                pub mod key {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod insert_key_hash_identity_reference {
                        use crate as dash_sdk_bindings;

                        pub mod insert_reference_to_non_unique_key { use crate as dash_sdk_bindings; }

                        pub mod insert_reference_to_unique_key { use crate as dash_sdk_bindings; }

                        pub mod insert_non_unique_public_key_hash_reference_to_identity { use crate as dash_sdk_bindings; }

                        pub mod insert_unique_public_key_hash_reference_to_identity { use crate as dash_sdk_bindings; }

                        pub mod estimation_costs {
                            use crate as dash_sdk_bindings;

                            pub mod add_estimation_costs_for_insert_unique_public_key_hash_reference { use crate as dash_sdk_bindings; }

                            pub mod add_estimation_costs_for_insert_non_unique_public_key_hash_reference { use crate as dash_sdk_bindings; }
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod insert {
                        use crate as dash_sdk_bindings;

                        pub mod create_key_tree_with_keys { use crate as dash_sdk_bindings; }

                        pub mod insert_new_non_unique_key { use crate as dash_sdk_bindings; }

                        pub mod create_new_identity_key_query_trees { use crate as dash_sdk_bindings; }

                        pub mod replace_key_in_storage { use crate as dash_sdk_bindings; }

                        pub mod insert_key_to_storage { use crate as dash_sdk_bindings; }

                        pub mod insert_new_unique_key { use crate as dash_sdk_bindings; }

                        pub mod insert_key_searchable_references { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(any(feature = "server", feature = "verify"))]
                    pub mod fetch {
                        use crate as dash_sdk_bindings;

                        #[cfg(feature = "server")]
                        pub mod fetch_all_identity_keys {
                            use crate as dash_sdk_bindings;
                        }

                        #[cfg(feature = "server")]
                        pub mod fetch_all_current_identity_keys {
                            use crate as dash_sdk_bindings;
                        }

                        #[cfg(feature = "server")]
                        pub mod fetch_identity_keys {
                            use crate as dash_sdk_bindings;
                        }

                        #[cfg(feature = "server")]
                        pub mod fetch_identities_all_keys {
                            use crate as dash_sdk_bindings;
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod prove {
                        use crate as dash_sdk_bindings;

                        pub mod prove_identity_keys { use crate as dash_sdk_bindings; }

                        pub mod prove_identities_all_keys { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(feature = "server")]
                pub mod insert {
                    use crate as dash_sdk_bindings;

                    pub mod add_new_identity { use crate as dash_sdk_bindings; }
                }

                #[cfg(any(feature = "server", feature = "verify"))]
                pub mod balance {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod prove {
                        use crate as dash_sdk_bindings;

                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod update {
                        use crate as dash_sdk_bindings;

                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }
                    }
                }
            }

            #[cfg(feature = "server")]
            pub mod shared {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "server")]
                pub mod shared_estimation_costs {
                    use crate as dash_sdk_bindings;

                    pub mod add_estimation_costs_for_levels_up_to_contract_document_type_excluded { use crate as dash_sdk_bindings; }

                    pub mod add_estimation_costs_for_levels_up_to_contract { use crate as dash_sdk_bindings; }

                    pub mod add_estimation_costs_for_contested_document_tree_levels_up_to_contract { use crate as dash_sdk_bindings; }

                    pub mod add_estimation_costs_for_contested_document_tree_levels_up_to_contract_document_type_excluded { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod credit_pools {
                use crate as dash_sdk_bindings;

                #[cfg(any(feature = "server", feature = "verify"))]
                pub mod epochs {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod get_finalized_epoch_info {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod get_epochs_protocol_versions {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod start_block {
                        use crate as dash_sdk_bindings;

                        pub mod get_epoch_start_block_core_height { use crate as dash_sdk_bindings; }

                        #[cfg(feature = "server")]
                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }

                        pub mod get_first_epoch_start_block_info_between_epochs { use crate as dash_sdk_bindings; }

                        pub mod get_epoch_start_block_height { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod get_epoch_protocol_version {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod credit_distribution_pools {
                        use crate as dash_sdk_bindings;

                        pub mod add_epoch_processing_credits_for_distribution_operation { use crate as dash_sdk_bindings; }

                        pub mod add_epoch_final_info_operation { use crate as dash_sdk_bindings; }

                        pub mod get_epoch_fee_multiplier {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod get_epoch_total_credits_for_distribution {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod get_epoch_processing_credits_for_distribution {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod get_storage_credits_for_distribution_for_epochs_in_range { use crate as dash_sdk_bindings; }

                        pub mod get_epoch_storage_credits_for_distribution {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod get_epochs_infos {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_epochs_infos {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod proposers {
                        use crate as dash_sdk_bindings;

                        pub mod prove_epoch_proposers {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod get_epochs_proposer_block_count {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod fetch_epoch_proposers {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod is_epochs_proposers_tree_empty { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_finalized_epoch_infos {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod operations_factory {
                        use crate as dash_sdk_bindings;

                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod start_time {
                        use crate as dash_sdk_bindings;

                        pub mod get_epoch_start_time {
                            use crate as dash_sdk_bindings;

                            pub mod v0 {
                                use crate as dash_sdk_bindings;

                                #[cfg(feature = "server")]
                                #[cfg(test)]
                                pub mod tests {
                                    use crate as dash_sdk_bindings;
                                }
                            }
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod has_epoch_tree_exists {
                        use crate as dash_sdk_bindings;

                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }
                    }
                }

                #[cfg(feature = "server")]
                pub mod storage_fee_distribution_pool {
                    use crate as dash_sdk_bindings;

                    pub mod get_storage_fees_from_distribution_pool {
                        use crate as dash_sdk_bindings;

                        pub mod v0 {
                            use crate as dash_sdk_bindings;

                            #[cfg(feature = "server")]
                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }
                    }
                }

                #[cfg(feature = "server")]
                pub mod operations {
                    use crate as dash_sdk_bindings;

                    #[cfg(test)]
                    pub mod tests {
                        use crate as dash_sdk_bindings;
                    }
                }

                #[cfg(feature = "server")]
                #[cfg(test)]
                pub mod tests {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod unpaid_epoch {
                    use crate as dash_sdk_bindings;

                    pub mod get_unpaid_epoch_index {
                        use crate as dash_sdk_bindings;

                        pub mod v0 {
                            use crate as dash_sdk_bindings;

                            #[cfg(feature = "server")]
                            #[cfg(test)]
                            pub mod tests {
                                use crate as dash_sdk_bindings;
                            }
                        }
                    }
                }

                #[cfg(feature = "server")]
                pub mod pending_epoch_refunds {
                    use crate as dash_sdk_bindings;

                    pub mod methods {
                        use crate as dash_sdk_bindings;

                        pub mod add_delete_pending_epoch_refunds_except_specified {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod fetch_and_add_pending_epoch_refunds_to_collection {
                            use crate as dash_sdk_bindings;

                            pub mod v0 { use crate as dash_sdk_bindings; }
                        }

                        pub mod fetch_pending_epoch_refunds { use crate as dash_sdk_bindings; }
                    }

                    pub mod operations {
                        use crate as dash_sdk_bindings;

                        pub mod add_update_pending_epoch_refunds_operations { use crate as dash_sdk_bindings; }
                    }
                }
            }

            #[cfg(feature = "server")]
            pub mod asset_lock {
                use crate as dash_sdk_bindings;

                pub mod fetch_asset_lock_outpoint_info { use crate as dash_sdk_bindings; }

                pub mod add_asset_lock_outpoint_operations { use crate as dash_sdk_bindings; }

                pub mod estimation_costs {
                    use crate as dash_sdk_bindings;

                    pub mod add_estimation_costs_for_adding_asset_lock { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod balances {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "server")]
                pub mod remove_from_system_credits {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod add_to_system_credits {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod calculate_total_credits_balance {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod add_to_system_credits_operations {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod remove_from_system_credits_operations {
                    use crate as dash_sdk_bindings;
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod prefunded_specialized_balances {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "server")]
                pub mod estimation_costs {
                    use crate as dash_sdk_bindings;

                    pub mod for_prefunded_specialized_balance_update { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod prove {
                    use crate as dash_sdk_bindings;

                    pub mod single_balance { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod deduct_from_prefunded_specialized_balance {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod empty_prefunded_specialized_balance_operations {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod deduct_from_prefunded_specialized_balance_operations {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod add_prefunded_specialized_balance {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod fetch {
                    use crate as dash_sdk_bindings;

                    pub mod single_balance { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod add_prefunded_specialized_balance_operations {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod empty_prefunded_specialized_balance {
                    use crate as dash_sdk_bindings;
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod group {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "server")]
                pub mod insert {
                    use crate as dash_sdk_bindings;

                    pub mod add_new_groups { use crate as dash_sdk_bindings; }

                    pub mod add_group_action { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod prove {
                    use crate as dash_sdk_bindings;

                    pub mod prove_group_infos {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    pub mod prove_group_info {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    pub mod prove_action_infos {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    pub mod prove_action_signers {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(feature = "server")]
                pub mod fetch {
                    use crate as dash_sdk_bindings;

                    pub mod fetch_action_signers { use crate as dash_sdk_bindings; }

                    pub mod fetch_active_action_info { use crate as dash_sdk_bindings; }

                    pub mod fetch_group_info { use crate as dash_sdk_bindings; }

                    pub mod fetch_group_infos { use crate as dash_sdk_bindings; }

                    pub mod fetch_action_infos { use crate as dash_sdk_bindings; }

                    pub mod fetch_action_id_signers_power { use crate as dash_sdk_bindings; }

                    pub mod fetch_action_is_closed { use crate as dash_sdk_bindings; }

                    pub mod fetch_action_id_info_keep_serialized { use crate as dash_sdk_bindings; }

                    pub mod fetch_action_id_has_signer { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod estimated_costs {
                    use crate as dash_sdk_bindings;

                    pub mod for_add_groups { use crate as dash_sdk_bindings; }

                    pub mod for_add_group_action { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod protocol_upgrade {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "server")]
                pub mod update_validator_proposed_app_version {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod fetch_proved_validator_version_votes {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod remove_validators_proposed_app_versions {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod fetch_versions_with_counter {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod fetch_proved_versions_with_counter {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod fetch_validator_version_votes {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod clear_version_information {
                    use crate as dash_sdk_bindings;
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod tokens {
                use crate as dash_sdk_bindings;

                pub mod status {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod fetch_token_statuses {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_token_status {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_token_statuses {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(feature = "server")]
                pub mod apply_status {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod freeze {
                    use crate as dash_sdk_bindings;
                }

                pub mod balance {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod remove_from_identity_token_balance {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_identities_token_balances {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_identity_token_balances {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_identities_token_balances {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_identity_token_balances {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod add_to_previous_token_balance {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod update {
                        use crate as dash_sdk_bindings;

                        #[cfg(test)]
                        pub mod tests {
                            use crate as dash_sdk_bindings;
                        }
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_identity_token_balance {
                        use crate as dash_sdk_bindings;
                    }
                }

                #[cfg(feature = "server")]
                pub mod calculate_total_tokens_balance {
                    use crate as dash_sdk_bindings;
                }

                pub mod contract_info {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod prove_token_contract_info {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_token_contract_info {
                        use crate as dash_sdk_bindings;
                    }
                }

                #[cfg(feature = "server")]
                pub mod mint_many {
                    use crate as dash_sdk_bindings;
                }

                pub mod distribution {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod mark_pre_programmed_release_as_distributed {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod mark_perpetual_release_as_distributed {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch {
                        use crate as dash_sdk_bindings;

                        pub mod pre_programmed_distribution_last_paid_time_ms { use crate as dash_sdk_bindings; }

                        pub mod pre_programmed_distributions { use crate as dash_sdk_bindings; }

                        pub mod perpetual_distribution_last_paid_moment { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod add_perpetual_distribution {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod add_pre_programmed_distribution {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod prove {
                        use crate as dash_sdk_bindings;

                        pub mod perpetual_distribution_last_paid_moment { use crate as dash_sdk_bindings; }

                        pub mod pre_programmed_distributions { use crate as dash_sdk_bindings; }
                    }
                }

                pub mod info {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod fetch_identities_token_infos {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_identity_token_infos {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_identity_token_infos {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_identities_token_infos {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_identity_token_info {
                        use crate as dash_sdk_bindings;
                    }
                }

                #[cfg(feature = "server")]
                pub mod burn {
                    use crate as dash_sdk_bindings;
                }

                pub mod direct_purchase {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod set_direct_purchase_price {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod prove_tokens_direct_purchase_price {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_token_direct_purchase_price {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod fetch_tokens_direct_purchase_price {
                        use crate as dash_sdk_bindings;
                    }
                }

                #[cfg(feature = "server")]
                pub mod unfreeze {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod transfer {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod estimated_costs {
                    use crate as dash_sdk_bindings;

                    pub mod for_token_status_infos { use crate as dash_sdk_bindings; }

                    pub mod for_token_total_supply { use crate as dash_sdk_bindings; }

                    pub mod for_token_balances { use crate as dash_sdk_bindings; }

                    pub mod for_root_token_ms_interval_distribution { use crate as dash_sdk_bindings; }

                    pub mod for_token_identity_infos { use crate as dash_sdk_bindings; }

                    pub mod for_token_direct_selling_prices { use crate as dash_sdk_bindings; }

                    pub mod for_token_contract_infos { use crate as dash_sdk_bindings; }

                    pub mod for_token_perpetual_distribution { use crate as dash_sdk_bindings; }

                    pub mod for_token_pre_programmed_distribution { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod mint {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod add_transaction_history_operations {
                    use crate as dash_sdk_bindings;
                }

                #[cfg(feature = "server")]
                pub mod system {
                    use crate as dash_sdk_bindings;

                    pub mod add_to_token_total_supply { use crate as dash_sdk_bindings; }

                    pub mod fetch_token_total_aggregated_identity_balances { use crate as dash_sdk_bindings; }

                    pub mod create_token_trees { use crate as dash_sdk_bindings; }

                    pub mod fetch_token_total_supply { use crate as dash_sdk_bindings; }

                    pub mod prove_token_total_supply_and_aggregated_identity_balances {
                        use crate as dash_sdk_bindings;

                        pub mod v0 { use crate as dash_sdk_bindings; }
                    }

                    pub mod remove_from_token_total_supply { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(any(feature = "server", feature = "verify", feature = "fixtures-and-mocks"))]
            pub mod contract {
                use crate as dash_sdk_bindings;

                #[cfg(feature = "server")]
                pub mod prove {
                    use crate as dash_sdk_bindings;

                    pub mod prove_contracts { use crate as dash_sdk_bindings; }

                    pub mod prove_contract { use crate as dash_sdk_bindings; }

                    pub mod prove_contract_history { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod get_fetch {
                    use crate as dash_sdk_bindings;

                    pub mod fetch_contract { use crate as dash_sdk_bindings; }

                    pub mod get_contract_with_fetch_info { use crate as dash_sdk_bindings; }

                    pub mod get_cached_contract_with_fetch_info { use crate as dash_sdk_bindings; }

                    pub mod get_contracts_with_fetch_info { use crate as dash_sdk_bindings; }

                    pub mod fetch_contract_with_history { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod insert {
                    use crate as dash_sdk_bindings;

                    pub mod add_description { use crate as dash_sdk_bindings; }

                    pub mod add_contract_to_storage { use crate as dash_sdk_bindings; }

                    pub mod add_new_keywords { use crate as dash_sdk_bindings; }

                    pub mod insert_contract { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod update {
                    use crate as dash_sdk_bindings;

                    pub mod update_contract { use crate as dash_sdk_bindings; }

                    pub mod update_keywords { use crate as dash_sdk_bindings; }

                    pub mod update_description { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod apply {
                    use crate as dash_sdk_bindings;

                    pub mod apply_contract_with_serialization { use crate as dash_sdk_bindings; }

                    pub mod apply_contract { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod estimation_costs {
                    use crate as dash_sdk_bindings;

                    pub mod add_estimation_costs_for_contract_insertion { use crate as dash_sdk_bindings; }
                }
            }

            #[cfg(feature = "server")]
            pub mod system {
                use crate as dash_sdk_bindings;

                pub mod fetch_elements { use crate as dash_sdk_bindings; }

                pub mod estimation_costs {
                    use crate as dash_sdk_bindings;

                    pub mod for_total_system_credits_update { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod genesis_time {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    #[cfg(test)]
                    pub mod tests {
                        use crate as dash_sdk_bindings;
                    }
                }

                pub mod protocol_version { use crate as dash_sdk_bindings; }
            }

            #[cfg(any(feature = "server", feature = "verify", feature = "fixtures-and-mocks"))]
            pub mod document {
                use crate as dash_sdk_bindings;

                #[cfg(any(feature = "server", feature = "fixtures-and-mocks"))]
                pub mod update {
                    use crate as dash_sdk_bindings;

                    #[cfg(feature = "server")]
                    pub mod update_document_for_contract {
                        use crate as dash_sdk_bindings;
                    }

                    #[cfg(feature = "server")]
                    pub mod add_update_multiple_documents_operations {
                        use crate as dash_sdk_bindings;
                    }

                    pub mod update_document_with_serialization_for_contract { use crate as dash_sdk_bindings; }

                    #[cfg(feature = "server")]
                    pub mod update_document_for_contract_id {
                        use crate as dash_sdk_bindings;
                    }

                    pub mod internal {
                        use crate as dash_sdk_bindings;

                        pub mod update_document_for_contract_operations { use crate as dash_sdk_bindings; }

                        pub mod update_document_for_contract_apply_and_add_to_operations { use crate as dash_sdk_bindings; }
                    }
                }

                #[cfg(any(feature = "server", feature = "fixtures-and-mocks"))]
                pub mod insert {
                    use crate as dash_sdk_bindings;

                    pub mod add_document { use crate as dash_sdk_bindings; }

                    pub mod add_reference_for_index_level_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod add_document_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod add_indices_for_index_level_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod add_document_to_primary_storage { use crate as dash_sdk_bindings; }

                    pub mod add_indices_for_top_index_level_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod add_document_for_contract_apply_and_add_to_operations { use crate as dash_sdk_bindings; }

                    pub mod add_document_for_contract { use crate as dash_sdk_bindings; }
                }

                #[cfg(any(feature = "server", feature = "fixtures-and-mocks"))]
                pub mod insert_contested {
                    use crate as dash_sdk_bindings;

                    pub mod add_contested_document_to_primary_storage { use crate as dash_sdk_bindings; }

                    pub mod add_contested_document { use crate as dash_sdk_bindings; }

                    pub mod add_contested_document_for_contract_apply_and_add_to_operations { use crate as dash_sdk_bindings; }

                    pub mod add_contested_vote_subtrees_for_non_identities_operations { use crate as dash_sdk_bindings; }

                    pub mod add_contested_document_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod add_contested_document_for_contract { use crate as dash_sdk_bindings; }

                    pub mod add_contested_indices_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod add_contested_reference_and_vote_subtree_to_document_operations { use crate as dash_sdk_bindings; }
                }

                #[cfg(any(feature = "server", feature = "fixtures-and-mocks"))]
                pub mod query {
                    use crate as dash_sdk_bindings;

                    pub mod query_documents { use crate as dash_sdk_bindings; }

                    pub mod query_contested_documents_storage { use crate as dash_sdk_bindings; }

                    pub mod query_contested_documents_vote_state { use crate as dash_sdk_bindings; }

                    pub mod query_documents_with_flags { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod estimation_costs {
                    use crate as dash_sdk_bindings;

                    pub mod add_estimation_costs_for_add_document_to_primary_storage { use crate as dash_sdk_bindings; }

                    pub mod stateless_delete_of_non_tree_for_costs { use crate as dash_sdk_bindings; }

                    pub mod add_estimation_costs_for_add_contested_document_to_primary_storage { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod index_uniqueness {
                    use crate as dash_sdk_bindings;

                    pub mod validate_document_create_transition_action_uniqueness { use crate as dash_sdk_bindings; }

                    pub mod validate_document_replace_transition_action_uniqueness { use crate as dash_sdk_bindings; }

                    pub mod internal {
                        use crate as dash_sdk_bindings;

                        pub mod validate_uniqueness_of_data { use crate as dash_sdk_bindings; }
                    }

                    pub mod validate_document_purchase_transition_action_uniqueness { use crate as dash_sdk_bindings; }

                    pub mod validate_document_update_price_transition_action_uniqueness { use crate as dash_sdk_bindings; }

                    pub mod validate_document_uniqueness { use crate as dash_sdk_bindings; }

                    pub mod validate_document_transfer_transition_action_uniqueness { use crate as dash_sdk_bindings; }
                }

                #[cfg(feature = "server")]
                pub mod delete {
                    use crate as dash_sdk_bindings;

                    pub mod remove_indices_for_top_index_level_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod remove_document_from_primary_storage { use crate as dash_sdk_bindings; }

                    pub mod delete_document_for_contract_with_named_type_operations { use crate as dash_sdk_bindings; }

                    pub mod internal {
                        use crate as dash_sdk_bindings;

                        pub mod add_estimation_costs_for_remove_document_to_primary_storage { use crate as dash_sdk_bindings; }
                    }

                    pub mod delete_document_for_contract_id_with_named_type_operations { use crate as dash_sdk_bindings; }

                    pub mod delete_document_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod delete_document_for_contract { use crate as dash_sdk_bindings; }

                    pub mod delete_document_for_contract_apply_and_add_to_operations { use crate as dash_sdk_bindings; }

                    pub mod delete_document_for_contract_id { use crate as dash_sdk_bindings; }

                    pub mod remove_reference_for_index_level_for_contract_operations { use crate as dash_sdk_bindings; }

                    pub mod remove_indices_for_index_level_for_contract_operations { use crate as dash_sdk_bindings; }
                }
            }
        }

        #[cfg(any(feature = "server", feature = "verify"))]
        pub mod verify {
            use crate as dash_sdk_bindings;

            pub mod group {
                use crate as dash_sdk_bindings;

                pub mod verify_action_signers_total_power { use crate as dash_sdk_bindings; }

                pub mod verify_active_action_infos { use crate as dash_sdk_bindings; }

                pub mod verify_group_info { use crate as dash_sdk_bindings; }

                pub mod verify_group_infos_in_contract { use crate as dash_sdk_bindings; }

                pub mod verify_action_signers { use crate as dash_sdk_bindings; }
            }

            pub mod state_transition {
                use crate as dash_sdk_bindings;

                pub mod verify_state_transition_was_executed_with_proof { use crate as dash_sdk_bindings; }

                pub mod state_transition_execution_path_queries { use crate as dash_sdk_bindings; }
            }

            pub mod single_document {
                use crate as dash_sdk_bindings;

                pub mod verify_proof_keep_serialized { use crate as dash_sdk_bindings; }

                pub mod verify_proof { use crate as dash_sdk_bindings; }
            }

            pub mod tokens {
                use crate as dash_sdk_bindings;

                pub mod verify_token_direct_selling_prices { use crate as dash_sdk_bindings; }

                pub mod verify_token_infos_for_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_token_perpetual_distribution_last_paid_time { use crate as dash_sdk_bindings; }

                pub mod verify_token_balances_for_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_token_direct_selling_price { use crate as dash_sdk_bindings; }

                pub mod verify_token_balance_for_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_token_status { use crate as dash_sdk_bindings; }

                pub mod verify_token_balances_for_identity_ids { use crate as dash_sdk_bindings; }

                pub mod verify_token_info_for_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_token_total_supply_and_aggregated_identity_balance { use crate as dash_sdk_bindings; }

                pub mod verify_token_pre_programmed_distributions { use crate as dash_sdk_bindings; }

                pub mod verify_token_statuses { use crate as dash_sdk_bindings; }

                pub mod verify_token_infos_for_identity_ids { use crate as dash_sdk_bindings; }

                pub mod verify_token_contract_info { use crate as dash_sdk_bindings; }
            }

            pub mod contract {
                use crate as dash_sdk_bindings;

                pub mod verify_contract { use crate as dash_sdk_bindings; }

                pub mod verify_contract_history { use crate as dash_sdk_bindings; }
            }

            pub mod document {
                use crate as dash_sdk_bindings;

                pub mod verify_proof_keep_serialized { use crate as dash_sdk_bindings; }

                pub mod verify_proof { use crate as dash_sdk_bindings; }

                pub mod verify_start_at_document_in_proof { use crate as dash_sdk_bindings; }
            }

            pub mod system {
                use crate as dash_sdk_bindings;

                pub mod verify_elements { use crate as dash_sdk_bindings; }

                pub mod verify_finalized_epoch_infos { use crate as dash_sdk_bindings; }

                pub mod verify_epoch_infos { use crate as dash_sdk_bindings; }

                pub mod verify_epoch_proposers { use crate as dash_sdk_bindings; }

                pub mod verify_upgrade_state { use crate as dash_sdk_bindings; }

                pub mod verify_upgrade_vote_status { use crate as dash_sdk_bindings; }

                pub mod verify_total_credits_in_system { use crate as dash_sdk_bindings; }
            }

            pub mod voting {
                use crate as dash_sdk_bindings;

                pub mod verify_vote_poll_vote_state_proof { use crate as dash_sdk_bindings; }

                pub mod verify_vote_polls_end_date_query { use crate as dash_sdk_bindings; }

                pub mod verify_vote_poll_votes_proof { use crate as dash_sdk_bindings; }

                pub mod verify_masternode_vote { use crate as dash_sdk_bindings; }

                pub mod verify_contests_proof { use crate as dash_sdk_bindings; }

                pub mod verify_identity_votes_given_proof { use crate as dash_sdk_bindings; }

                pub mod verify_specialized_balance { use crate as dash_sdk_bindings; }
            }

            pub mod identity {
                use crate as dash_sdk_bindings;

                pub mod verify_identity_balance_for_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_identity_revision_for_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_full_identity_by_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_identity_id_by_unique_public_key_hash { use crate as dash_sdk_bindings; }

                pub mod verify_identity_ids_by_unique_public_key_hashes { use crate as dash_sdk_bindings; }

                pub mod verify_identity_nonce { use crate as dash_sdk_bindings; }

                pub mod verify_identity_balance_and_revision_for_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_full_identities_by_public_key_hashes { use crate as dash_sdk_bindings; }

                pub mod verify_identity_keys_by_identity_id { use crate as dash_sdk_bindings; }

                pub mod verify_full_identity_by_unique_public_key_hash { use crate as dash_sdk_bindings; }

                pub mod verify_full_identity_by_non_unique_public_key_hash { use crate as dash_sdk_bindings; }

                pub mod verify_identity_id_by_non_unique_public_key_hash { use crate as dash_sdk_bindings; }

                pub mod verify_identity_balances_for_identity_ids { use crate as dash_sdk_bindings; }

                pub mod verify_identities_contract_keys { use crate as dash_sdk_bindings; }

                pub mod verify_identity_contract_nonce { use crate as dash_sdk_bindings; }
            }
        }

        #[cfg(any(feature = "server", feature = "verify"))]
        pub mod query {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "server")]
            pub mod test_index {
                use crate as dash_sdk_bindings;
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod ordering {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`OrderClause`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct drive_query_ordering_OrderClause {
                    pub field: *mut std::os::raw::c_char,
                    pub ascending: bool,
                }

                impl ferment::FFIConversionFrom<drive::query::ordering::OrderClause> for drive_query_ordering_OrderClause {
                    unsafe fn ffi_from_const(ffi: *const drive_query_ordering_OrderClause) -> drive::query::ordering::OrderClause {
                        let ffi_ref = &*ffi;
                        drive::query::ordering::OrderClause { field: <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(ffi_ref.field), ascending: ffi_ref.ascending }
                    }
                }

                impl ferment::FFIConversionTo<drive::query::ordering::OrderClause> for drive_query_ordering_OrderClause { unsafe fn ffi_to_const(obj: drive::query::ordering::OrderClause) -> *const drive_query_ordering_OrderClause { ferment::boxed(drive_query_ordering_OrderClause { field: <std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(obj.field), ascending: obj.ascending }) } }

                impl Drop for drive_query_ordering_OrderClause {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_string(ffi_ref.field);
                            ;
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_ordering_OrderClause_ctor<>(field: *mut std::os::raw::c_char, ascending: bool) -> *mut drive_query_ordering_OrderClause { ferment::boxed(drive_query_ordering_OrderClause { field, ascending }) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_ordering_OrderClause_destroy<>(ffi: *mut drive_query_ordering_OrderClause) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_ordering_OrderClause_get_field<>(obj: *const drive_query_ordering_OrderClause) -> *mut std::os::raw::c_char { (*obj).field }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_ordering_OrderClause_get_ascending<>(obj: *const drive_query_ordering_OrderClause) -> bool { (*obj).ascending }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_ordering_OrderClause_set_field<>(obj: *const drive_query_ordering_OrderClause) -> *mut std::os::raw::c_char { (*obj).field }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_ordering_OrderClause_set_ascending<>(obj: *const drive_query_ordering_OrderClause) -> bool { (*obj).ascending }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod conditions {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`WhereOperator`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum drive_query_conditions_WhereOperator { Equal, GreaterThan, GreaterThanOrEquals, LessThan, LessThanOrEquals, Between, BetweenExcludeBounds, BetweenExcludeLeft, BetweenExcludeRight, In, StartsWith }

                impl ferment::FFIConversionFrom<drive::query::conditions::WhereOperator> for drive_query_conditions_WhereOperator {
                    unsafe fn ffi_from_const(ffi: *const drive_query_conditions_WhereOperator) -> drive::query::conditions::WhereOperator {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            drive_query_conditions_WhereOperator::Equal => drive::query::conditions::WhereOperator::Equal,
                            drive_query_conditions_WhereOperator::GreaterThan => drive::query::conditions::WhereOperator::GreaterThan,
                            drive_query_conditions_WhereOperator::GreaterThanOrEquals => drive::query::conditions::WhereOperator::GreaterThanOrEquals,
                            drive_query_conditions_WhereOperator::LessThan => drive::query::conditions::WhereOperator::LessThan,
                            drive_query_conditions_WhereOperator::LessThanOrEquals => drive::query::conditions::WhereOperator::LessThanOrEquals,
                            drive_query_conditions_WhereOperator::Between => drive::query::conditions::WhereOperator::Between,
                            drive_query_conditions_WhereOperator::BetweenExcludeBounds => drive::query::conditions::WhereOperator::BetweenExcludeBounds,
                            drive_query_conditions_WhereOperator::BetweenExcludeLeft => drive::query::conditions::WhereOperator::BetweenExcludeLeft,
                            drive_query_conditions_WhereOperator::BetweenExcludeRight => drive::query::conditions::WhereOperator::BetweenExcludeRight,
                            drive_query_conditions_WhereOperator::In => drive::query::conditions::WhereOperator::In,
                            drive_query_conditions_WhereOperator::StartsWith => drive::query::conditions::WhereOperator::StartsWith
                        }
                    }
                }

                impl ferment::FFIConversionTo<drive::query::conditions::WhereOperator> for drive_query_conditions_WhereOperator {
                    unsafe fn ffi_to_const(obj: drive::query::conditions::WhereOperator) -> *const drive_query_conditions_WhereOperator {
                        ferment::boxed(match obj {
                            drive::query::conditions::WhereOperator::Equal => drive_query_conditions_WhereOperator::Equal,
                            drive::query::conditions::WhereOperator::GreaterThan => drive_query_conditions_WhereOperator::GreaterThan,
                            drive::query::conditions::WhereOperator::GreaterThanOrEquals => drive_query_conditions_WhereOperator::GreaterThanOrEquals,
                            drive::query::conditions::WhereOperator::LessThan => drive_query_conditions_WhereOperator::LessThan,
                            drive::query::conditions::WhereOperator::LessThanOrEquals => drive_query_conditions_WhereOperator::LessThanOrEquals,
                            drive::query::conditions::WhereOperator::Between => drive_query_conditions_WhereOperator::Between,
                            drive::query::conditions::WhereOperator::BetweenExcludeBounds => drive_query_conditions_WhereOperator::BetweenExcludeBounds,
                            drive::query::conditions::WhereOperator::BetweenExcludeLeft => drive_query_conditions_WhereOperator::BetweenExcludeLeft,
                            drive::query::conditions::WhereOperator::BetweenExcludeRight => drive_query_conditions_WhereOperator::BetweenExcludeRight,
                            drive::query::conditions::WhereOperator::In => drive_query_conditions_WhereOperator::In,
                            drive::query::conditions::WhereOperator::StartsWith => drive_query_conditions_WhereOperator::StartsWith,
                            _ => unreachable!("This is unreachable")
                        })
                    }
                }

                impl Drop for drive_query_conditions_WhereOperator {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                drive_query_conditions_WhereOperator::Equal => {}
                                drive_query_conditions_WhereOperator::GreaterThan => {}
                                drive_query_conditions_WhereOperator::GreaterThanOrEquals => {}
                                drive_query_conditions_WhereOperator::LessThan => {}
                                drive_query_conditions_WhereOperator::LessThanOrEquals => {}
                                drive_query_conditions_WhereOperator::Between => {}
                                drive_query_conditions_WhereOperator::BetweenExcludeBounds => {}
                                drive_query_conditions_WhereOperator::BetweenExcludeLeft => {}
                                drive_query_conditions_WhereOperator::BetweenExcludeRight => {}
                                drive_query_conditions_WhereOperator::In => {}
                                drive_query_conditions_WhereOperator::StartsWith => {}
                                _ => unreachable!("This is unreachable")
                            };
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_Equal_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::Equal {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_GreaterThan_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::GreaterThan {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_GreaterThanOrEquals_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::GreaterThanOrEquals {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_LessThan_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::LessThan {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_LessThanOrEquals_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::LessThanOrEquals {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_Between_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::Between {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_BetweenExcludeBounds_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::BetweenExcludeBounds {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_BetweenExcludeLeft_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::BetweenExcludeLeft {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_BetweenExcludeRight_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::BetweenExcludeRight {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_In_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::In {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_StartsWith_ctor() -> *mut drive_query_conditions_WhereOperator { ferment::boxed(drive_query_conditions_WhereOperator::StartsWith {}) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereOperator_destroy<>(ffi: *mut drive_query_conditions_WhereOperator) { ferment::unbox_any(ffi); }

                #[doc = "FFI-representation of the [`WhereClause`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct drive_query_conditions_WhereClause {
                    pub field: *mut std::os::raw::c_char,
                    pub operator: *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator,
                    pub value: *mut crate::fermented::types::platform_value::platform_value_Value,
                }

                impl ferment::FFIConversionFrom<drive::query::conditions::WhereClause> for drive_query_conditions_WhereClause {
                    unsafe fn ffi_from_const(ffi: *const drive_query_conditions_WhereClause) -> drive::query::conditions::WhereClause {
                        let ffi_ref = &*ffi;
                        drive::query::conditions::WhereClause { field: <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(ffi_ref.field), operator: <crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator as ferment::FFIConversionFrom<drive::query::conditions::WhereOperator>>::ffi_from(ffi_ref.operator), value: <crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from(ffi_ref.value) }
                    }
                }

                impl ferment::FFIConversionTo<drive::query::conditions::WhereClause> for drive_query_conditions_WhereClause { unsafe fn ffi_to_const(obj: drive::query::conditions::WhereClause) -> *const drive_query_conditions_WhereClause { ferment::boxed(drive_query_conditions_WhereClause { field: <std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(obj.field), operator: <crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator as ferment::FFIConversionTo<drive::query::conditions::WhereOperator>>::ffi_to(obj.operator), value: <crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionTo<platform_value::Value>>::ffi_to(obj.value) }) } }

                impl Drop for drive_query_conditions_WhereClause {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_string(ffi_ref.field);
                            ferment::unbox_any(ffi_ref.operator);
                            ferment::unbox_any(ffi_ref.value);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_ctor<>(field: *mut std::os::raw::c_char, operator: *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator, value: *mut crate::fermented::types::platform_value::platform_value_Value) -> *mut drive_query_conditions_WhereClause { ferment::boxed(drive_query_conditions_WhereClause { field, operator, value }) }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_destroy<>(ffi: *mut drive_query_conditions_WhereClause) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_get_field<>(obj: *const drive_query_conditions_WhereClause) -> *mut std::os::raw::c_char { (*obj).field }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_get_operator<>(obj: *const drive_query_conditions_WhereClause) -> *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator { (*obj).operator }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_get_value<>(obj: *const drive_query_conditions_WhereClause) -> *mut crate::fermented::types::platform_value::platform_value_Value { (*obj).value }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_set_field<>(obj: *const drive_query_conditions_WhereClause) -> *mut std::os::raw::c_char { (*obj).field }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_set_operator<>(obj: *const drive_query_conditions_WhereClause) -> *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator { (*obj).operator }

                #[no_mangle]
                pub unsafe extern "C" fn drive_query_conditions_WhereClause_set_value<>(obj: *const drive_query_conditions_WhereClause) -> *mut crate::fermented::types::platform_value::platform_value_Value { (*obj).value }
            }
        }

        #[cfg(feature = "server")]
        pub mod prove {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "server")]
            pub mod prove_state_transition {
                use crate as dash_sdk_bindings;
            }

            pub mod prove_elements { use crate as dash_sdk_bindings; }

            pub mod prove_multiple_state_transition_results { use crate as dash_sdk_bindings; }
        }

        #[cfg(feature = "server")]
        pub mod fees {
            use crate as dash_sdk_bindings;

            pub mod calculate_fee { use crate as dash_sdk_bindings; }
        }

        #[cfg(any(feature = "server", feature = "verify"))]
        pub mod error {
            use crate as dash_sdk_bindings;
        }

        #[cfg(feature = "server")]
        pub mod cache {
            use crate as dash_sdk_bindings;

            pub mod data_contract {
                use crate as dash_sdk_bindings;

                #[cfg(test)]
                pub mod tests {
                    use crate as dash_sdk_bindings;
                }
            }
        }

        #[cfg(any(feature = "server", feature = "verify", feature = "fixtures-and-mocks"))]
        pub mod util {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "server")]
            pub mod batch {
                use crate as dash_sdk_bindings;

                pub mod drive_op_batch {
                    use crate as dash_sdk_bindings;

                    pub mod drive_methods {
                        use crate as dash_sdk_bindings;

                        pub mod convert_drive_operations_to_grove_operations { use crate as dash_sdk_bindings; }

                        pub mod apply_drive_operations { use crate as dash_sdk_bindings; }
                    }
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod object_size_info {
                use crate as dash_sdk_bindings;
            }

            #[cfg(feature = "server")]
            pub mod grove_operations {
                use crate as dash_sdk_bindings;

                pub mod batch_remove_raw { use crate as dash_sdk_bindings; }

                pub mod grove_get_path_query_serialized_results { use crate as dash_sdk_bindings; }

                pub mod batch_insert_sum_item_if_not_exists { use crate as dash_sdk_bindings; }

                pub mod batch_insert_if_changed_value { use crate as dash_sdk_bindings; }

                pub mod batch_insert_empty_tree_if_not_exists_check_existing_operations { use crate as dash_sdk_bindings; }

                pub mod grove_get_raw { use crate as dash_sdk_bindings; }

                pub mod batch_insert_empty_sum_tree { use crate as dash_sdk_bindings; }

                pub mod grove_get_path_query_with_optional { use crate as dash_sdk_bindings; }

                pub mod batch_insert { use crate as dash_sdk_bindings; }

                pub mod grove_get_raw_value_u64_from_encoded_var_vec { use crate as dash_sdk_bindings; }

                pub mod grove_get_proved_path_query { use crate as dash_sdk_bindings; }

                pub mod grove_get_big_sum_tree_total_value { use crate as dash_sdk_bindings; }

                pub mod grove_insert_if_not_exists { use crate as dash_sdk_bindings; }

                pub mod batch_insert_sum_item_or_add_to_if_already_exists { use crate as dash_sdk_bindings; }

                pub mod grove_get_sum_tree_total_value { use crate as dash_sdk_bindings; }

                pub mod grove_batch_operations_costs { use crate as dash_sdk_bindings; }

                pub mod grove_get_raw_optional_item { use crate as dash_sdk_bindings; }

                pub mod batch_insert_empty_tree { use crate as dash_sdk_bindings; }

                pub mod batch_insert_if_not_exists_return_existing_element { use crate as dash_sdk_bindings; }

                pub mod batch_delete_up_tree_while_empty { use crate as dash_sdk_bindings; }

                pub mod grove_delete { use crate as dash_sdk_bindings; }

                pub mod grove_clear { use crate as dash_sdk_bindings; }

                pub mod batch_delete { use crate as dash_sdk_bindings; }

                pub mod batch_move_items_in_path_query {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod grove_insert_if_not_exists_return_existing_element { use crate as dash_sdk_bindings; }

                pub mod grove_insert { use crate as dash_sdk_bindings; }

                pub mod batch_refresh_reference { use crate as dash_sdk_bindings; }

                pub mod grove_get_path_query_serialized_or_sum_results { use crate as dash_sdk_bindings; }

                pub mod grove_get_path_query { use crate as dash_sdk_bindings; }

                pub mod batch_replace { use crate as dash_sdk_bindings; }

                pub mod grove_get_raw_item { use crate as dash_sdk_bindings; }

                pub mod grove_get_optional_sum_tree_total_value { use crate as dash_sdk_bindings; }

                pub mod grove_has_raw { use crate as dash_sdk_bindings; }

                pub mod batch_move {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod grove_apply_batch_with_add_costs { use crate as dash_sdk_bindings; }

                pub mod grove_insert_empty_tree { use crate as dash_sdk_bindings; }

                pub mod grove_get_raw_optional { use crate as dash_sdk_bindings; }

                pub mod batch_insert_empty_tree_if_not_exists { use crate as dash_sdk_bindings; }

                pub mod grove_get_raw_path_query { use crate as dash_sdk_bindings; }

                pub mod grove_apply_partial_batch_with_add_costs { use crate as dash_sdk_bindings; }

                pub mod grove_get { use crate as dash_sdk_bindings; }

                pub mod grove_get_proved_path_query_with_conditional { use crate as dash_sdk_bindings; }

                pub mod batch_insert_if_not_exists { use crate as dash_sdk_bindings; }

                pub mod grove_apply_operation { use crate as dash_sdk_bindings; }

                pub mod batch_delete_items_in_path_query {
                    use crate as dash_sdk_bindings;

                    pub mod v0 { use crate as dash_sdk_bindings; }
                }

                pub mod grove_get_raw_path_query_with_optional { use crate as dash_sdk_bindings; }

                pub mod grove_insert_empty_sum_tree { use crate as dash_sdk_bindings; }
            }

            #[cfg(any(test, feature = "server", feature = "fixtures-and-mocks"))]
            pub mod test_helpers {
                use crate as dash_sdk_bindings;

                #[cfg(any(test, feature = "fixtures-and-mocks"))]
                pub mod test_utils {
                    use crate as dash_sdk_bindings;
                }
            }

            #[cfg(any(feature = "server", feature = "verify"))]
            pub mod common {
                use crate as dash_sdk_bindings;
            }

            #[cfg(feature = "server")]
            pub mod operations {
                use crate as dash_sdk_bindings;

                pub mod apply_batch_grovedb_operations { use crate as dash_sdk_bindings; }

                pub mod rollback_transaction { use crate as dash_sdk_bindings; }

                pub mod apply_partial_batch_grovedb_operations { use crate as dash_sdk_bindings; }

                pub mod drop_cache { use crate as dash_sdk_bindings; }

                pub mod apply_batch_low_level_drive_operations { use crate as dash_sdk_bindings; }

                pub mod apply_partial_batch_low_level_drive_operations { use crate as dash_sdk_bindings; }

                pub mod commit_transaction { use crate as dash_sdk_bindings; }
            }
        }
    }

    pub mod platform_value {
        use crate as dash_sdk_bindings;

        pub mod value_map {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`ValueMap`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct platform_value_value_map_ValueMap(*mut crate::fermented::generics::Vec_Tuple_platform_value_Value_platform_value_Value);

            impl ferment::FFIConversionFrom<platform_value::value_map::ValueMap> for platform_value_value_map_ValueMap {
                unsafe fn ffi_from_const(ffi: *const platform_value_value_map_ValueMap) -> platform_value::value_map::ValueMap {
                    let ffi_ref = &*ffi;
                    <crate::fermented::generics::Vec_Tuple_platform_value_Value_platform_value_Value as ferment::FFIConversionFrom<Vec<(platform_value::Value, platform_value::Value)>>>::ffi_from(ffi_ref.0)
                }
            }

            impl ferment::FFIConversionTo<platform_value::value_map::ValueMap> for platform_value_value_map_ValueMap { unsafe fn ffi_to_const(obj: platform_value::value_map::ValueMap) -> *const platform_value_value_map_ValueMap { ferment::boxed(platform_value_value_map_ValueMap(<crate::fermented::generics::Vec_Tuple_platform_value_Value_platform_value_Value as ferment::FFIConversionTo<Vec<(platform_value::Value, platform_value::Value)>>>::ffi_to(obj))) } }

            impl Drop for platform_value_value_map_ValueMap {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_value_map_ValueMap_ctor<>(o_0: *mut crate::fermented::generics::Vec_Tuple_platform_value_Value_platform_value_Value) -> *mut platform_value_value_map_ValueMap { ferment::boxed(platform_value_value_map_ValueMap(o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_value_map_ValueMap_destroy<>(ffi: *mut platform_value_value_map_ValueMap) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_value_map_ValueMap_get_0<>(obj: *const platform_value_value_map_ValueMap) -> *mut crate::fermented::generics::Vec_Tuple_platform_value_Value_platform_value_Value { (*obj).0 }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_value_map_ValueMap_set_0<>(obj: *const platform_value_value_map_ValueMap) -> *mut crate::fermented::generics::Vec_Tuple_platform_value_Value_platform_value_Value { (*obj).0 }
        }

        #[doc = "FFI-representation of the [`Hash256`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct platform_value_Hash256(*mut crate::fermented::generics::Arr_u8_32);

        impl ferment::FFIConversionFrom<platform_value::Hash256> for platform_value_Hash256 {
            unsafe fn ffi_from_const(ffi: *const platform_value_Hash256) -> platform_value::Hash256 {
                let ffi_ref = &*ffi;
                <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(ffi_ref.0)
            }
        }

        impl ferment::FFIConversionTo<platform_value::Hash256> for platform_value_Hash256 { unsafe fn ffi_to_const(obj: platform_value::Hash256) -> *const platform_value_Hash256 { ferment::boxed(platform_value_Hash256(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionTo<[u8; 32]>>::ffi_to(obj))) } }

        impl Drop for platform_value_Hash256 {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment::unbox_any(ffi_ref.0);
                }
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Hash256_ctor<>(o_0: *mut crate::fermented::generics::Arr_u8_32) -> *mut platform_value_Hash256 { ferment::boxed(platform_value_Hash256(o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Hash256_destroy<>(ffi: *mut platform_value_Hash256) { ferment::unbox_any(ffi); }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Hash256_get_0<>(obj: *const platform_value_Hash256) -> *mut crate::fermented::generics::Arr_u8_32 { (*obj).0 }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Hash256_set_0<>(obj: *const platform_value_Hash256) -> *mut crate::fermented::generics::Arr_u8_32 { (*obj).0 }

        #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`Value`]\"`]"]
        #[repr(C)]
        #[derive(Clone)]
        #[non_exhaustive]
        pub enum platform_value_Value { U128(*mut [u8; 16]), I128(*mut [u8; 16]), U64(u64), I64(i64), U32(u32), I32(i32), U16(u16), I16(i16), U8(u8), I8(i8), Bytes(*mut crate::fermented::generics::Vec_u8), Bytes20(*mut crate::fermented::generics::Arr_u8_20), Bytes32(*mut crate::fermented::generics::Arr_u8_32), Bytes36(*mut crate::fermented::generics::Arr_u8_36), EnumU8(*mut crate::fermented::generics::Vec_u8), EnumString(*mut crate::fermented::generics::Vec_String), Identifier(*mut crate::fermented::types::platform_value::platform_value_Hash256), Float(f64), Text(*mut std::os::raw::c_char), Bool(bool), Null, Array(*mut crate::fermented::generics::Vec_platform_value_Value), Map(*mut crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap) }

        impl ferment::FFIConversionFrom<platform_value::Value> for platform_value_Value {
            unsafe fn ffi_from_const(ffi: *const platform_value_Value) -> platform_value::Value {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    platform_value_Value::U128(o_0) => platform_value::Value::U128(<[u8; 16] as ferment::FFIConversionFrom<u128>>::ffi_from(*o_0)),
                    platform_value_Value::I128(o_0) => platform_value::Value::I128(<[u8; 16] as ferment::FFIConversionFrom<i128>>::ffi_from(*o_0)),
                    platform_value_Value::U64(o_0) => platform_value::Value::U64(*o_0),
                    platform_value_Value::I64(o_0) => platform_value::Value::I64(*o_0),
                    platform_value_Value::U32(o_0) => platform_value::Value::U32(*o_0),
                    platform_value_Value::I32(o_0) => platform_value::Value::I32(*o_0),
                    platform_value_Value::U16(o_0) => platform_value::Value::U16(*o_0),
                    platform_value_Value::I16(o_0) => platform_value::Value::I16(*o_0),
                    platform_value_Value::U8(o_0) => platform_value::Value::U8(*o_0),
                    platform_value_Value::I8(o_0) => platform_value::Value::I8(*o_0),
                    platform_value_Value::Bytes(o_0) => platform_value::Value::Bytes(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(*o_0)),
                    platform_value_Value::Bytes20(o_0) => platform_value::Value::Bytes20(<crate::fermented::generics::Arr_u8_20 as ferment::FFIConversionFrom<[u8; 20]>>::ffi_from(*o_0)),
                    platform_value_Value::Bytes32(o_0) => platform_value::Value::Bytes32(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(*o_0)),
                    platform_value_Value::Bytes36(o_0) => platform_value::Value::Bytes36(<crate::fermented::generics::Arr_u8_36 as ferment::FFIConversionFrom<[u8; 36]>>::ffi_from(*o_0)),
                    platform_value_Value::EnumU8(o_0) => platform_value::Value::EnumU8(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(*o_0)),
                    platform_value_Value::EnumString(o_0) => platform_value::Value::EnumString(<crate::fermented::generics::Vec_String as ferment::FFIConversionFrom<Vec<String>>>::ffi_from(*o_0)),
                    platform_value_Value::Identifier(o_0) => platform_value::Value::Identifier(<crate::fermented::types::platform_value::platform_value_Hash256 as ferment::FFIConversionFrom<platform_value::Hash256>>::ffi_from(*o_0)),
                    platform_value_Value::Float(o_0) => platform_value::Value::Float(*o_0),
                    platform_value_Value::Text(o_0) => platform_value::Value::Text(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                    platform_value_Value::Bool(o_0) => platform_value::Value::Bool(*o_0),
                    platform_value_Value::Null => platform_value::Value::Null,
                    platform_value_Value::Array(o_0) => platform_value::Value::Array(<crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionFrom<Vec<platform_value::Value>>>::ffi_from(*o_0)),
                    platform_value_Value::Map(o_0) => platform_value::Value::Map(<crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap as ferment::FFIConversionFrom<platform_value::value_map::ValueMap>>::ffi_from(*o_0))
                }
            }
        }

        impl ferment::FFIConversionTo<platform_value::Value> for platform_value_Value {
            unsafe fn ffi_to_const(obj: platform_value::Value) -> *const platform_value_Value {
                ferment::boxed(match obj {
                    platform_value::Value::U128(o_0) => platform_value_Value::U128(<[u8; 16] as ferment::FFIConversionTo<u128>>::ffi_to(o_0)),
                    platform_value::Value::I128(o_0) => platform_value_Value::I128(<[u8; 16] as ferment::FFIConversionTo<i128>>::ffi_to(o_0)),
                    platform_value::Value::U64(o_0) => platform_value_Value::U64(o_0),
                    platform_value::Value::I64(o_0) => platform_value_Value::I64(o_0),
                    platform_value::Value::U32(o_0) => platform_value_Value::U32(o_0),
                    platform_value::Value::I32(o_0) => platform_value_Value::I32(o_0),
                    platform_value::Value::U16(o_0) => platform_value_Value::U16(o_0),
                    platform_value::Value::I16(o_0) => platform_value_Value::I16(o_0),
                    platform_value::Value::U8(o_0) => platform_value_Value::U8(o_0),
                    platform_value::Value::I8(o_0) => platform_value_Value::I8(o_0),
                    platform_value::Value::Bytes(o_0) => platform_value_Value::Bytes(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(o_0)),
                    platform_value::Value::Bytes20(o_0) => platform_value_Value::Bytes20(<crate::fermented::generics::Arr_u8_20 as ferment::FFIConversionTo<[u8; 20]>>::ffi_to(o_0)),
                    platform_value::Value::Bytes32(o_0) => platform_value_Value::Bytes32(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionTo<[u8; 32]>>::ffi_to(o_0)),
                    platform_value::Value::Bytes36(o_0) => platform_value_Value::Bytes36(<crate::fermented::generics::Arr_u8_36 as ferment::FFIConversionTo<[u8; 36]>>::ffi_to(o_0)),
                    platform_value::Value::EnumU8(o_0) => platform_value_Value::EnumU8(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(o_0)),
                    platform_value::Value::EnumString(o_0) => platform_value_Value::EnumString(<crate::fermented::generics::Vec_String as ferment::FFIConversionTo<Vec<String>>>::ffi_to(o_0)),
                    platform_value::Value::Identifier(o_0) => platform_value_Value::Identifier(<crate::fermented::types::platform_value::platform_value_Hash256 as ferment::FFIConversionTo<platform_value::Hash256>>::ffi_to(o_0)),
                    platform_value::Value::Float(o_0) => platform_value_Value::Float(o_0),
                    platform_value::Value::Text(o_0) => platform_value_Value::Text(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                    platform_value::Value::Bool(o_0) => platform_value_Value::Bool(o_0),
                    platform_value::Value::Null => platform_value_Value::Null,
                    platform_value::Value::Array(o_0) => platform_value_Value::Array(<crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionTo<Vec<platform_value::Value>>>::ffi_to(o_0)),
                    platform_value::Value::Map(o_0) => platform_value_Value::Map(<crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap as ferment::FFIConversionTo<platform_value::value_map::ValueMap>>::ffi_to(o_0)),
                    _ => unreachable!("This is unreachable")
                })
            }
        }

        impl Drop for platform_value_Value {
            fn drop(&mut self) {
                unsafe {
                    match self {
                        platform_value_Value::U128(o_0) => { ferment::unbox_any_opt(*o_0); }
                        platform_value_Value::I128(o_0) => { ferment::unbox_any_opt(*o_0); }
                        platform_value_Value::U64(o_0) => { ; }
                        platform_value_Value::I64(o_0) => { ; }
                        platform_value_Value::U32(o_0) => { ; }
                        platform_value_Value::I32(o_0) => { ; }
                        platform_value_Value::U16(o_0) => { ; }
                        platform_value_Value::I16(o_0) => { ; }
                        platform_value_Value::U8(o_0) => { ; }
                        platform_value_Value::I8(o_0) => { ; }
                        platform_value_Value::Bytes(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::Bytes20(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::Bytes32(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::Bytes36(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::EnumU8(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::EnumString(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::Identifier(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::Float(o_0) => { ; }
                        platform_value_Value::Text(o_0) => { ferment::unbox_string(*o_0); }
                        platform_value_Value::Bool(o_0) => { ; }
                        platform_value_Value::Null => {}
                        platform_value_Value::Array(o_0) => { ferment::unbox_any(*o_0); }
                        platform_value_Value::Map(o_0) => { ferment::unbox_any(*o_0); }
                        _ => unreachable!("This is unreachable")
                    };
                }
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_U128_ctor(o_o_0: *mut [u8; 16]) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::U128(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_I128_ctor(o_o_0: *mut [u8; 16]) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::I128(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_U64_ctor(o_o_0: u64) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::U64(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_I64_ctor(o_o_0: i64) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::I64(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_U32_ctor(o_o_0: u32) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::U32(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_I32_ctor(o_o_0: i32) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::I32(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_U16_ctor(o_o_0: u16) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::U16(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_I16_ctor(o_o_0: i16) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::I16(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_U8_ctor(o_o_0: u8) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::U8(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_I8_ctor(o_o_0: i8) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::I8(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Bytes_ctor(o_o_0: *mut crate::fermented::generics::Vec_u8) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Bytes(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Bytes20_ctor(o_o_0: *mut crate::fermented::generics::Arr_u8_20) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Bytes20(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Bytes32_ctor(o_o_0: *mut crate::fermented::generics::Arr_u8_32) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Bytes32(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Bytes36_ctor(o_o_0: *mut crate::fermented::generics::Arr_u8_36) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Bytes36(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_EnumU8_ctor(o_o_0: *mut crate::fermented::generics::Vec_u8) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::EnumU8(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_EnumString_ctor(o_o_0: *mut crate::fermented::generics::Vec_String) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::EnumString(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Identifier_ctor(o_o_0: *mut crate::fermented::types::platform_value::platform_value_Hash256) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Identifier(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Float_ctor(o_o_0: f64) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Float(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Text_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Text(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Bool_ctor(o_o_0: bool) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Bool(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Null_ctor() -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Null {}) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Array_ctor(o_o_0: *mut crate::fermented::generics::Vec_platform_value_Value) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Array(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_Map_ctor(o_o_0: *mut crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap) -> *mut platform_value_Value { ferment::boxed(platform_value_Value::Map(o_o_0)) }

        #[no_mangle]
        pub unsafe extern "C" fn platform_value_Value_destroy<>(ffi: *mut platform_value_Value) { ferment::unbox_any(ffi); }

        pub mod index { use crate as dash_sdk_bindings; }

        pub mod value_serialization { use crate as dash_sdk_bindings; }

        pub mod converter {
            use crate as dash_sdk_bindings;

            #[cfg(feature = "json")]
            pub mod serde_json {
                use crate as dash_sdk_bindings;
            }
        }

        pub mod btreemap_extensions { use crate as dash_sdk_bindings; }

        pub mod macros { use crate as dash_sdk_bindings; }

        pub mod patch {
            use crate as dash_sdk_bindings;

            pub mod diff { use crate as dash_sdk_bindings; }
        }

        pub mod inner_value_at_path {
            use crate as dash_sdk_bindings;

            #[cfg(test)]
            pub mod tests {
                use crate as dash_sdk_bindings;
            }
        }

        pub mod error {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`Error`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum platform_value_error_Error { Unsupported(*mut std::os::raw::c_char), StructureError(*mut std::os::raw::c_char), PathError(*mut std::os::raw::c_char), IntegerSizeError, IntegerParsingError, StringDecodingError(*mut std::os::raw::c_char), KeyMustBeAString, ByteLengthNot20BytesError(*mut std::os::raw::c_char), ByteLengthNot32BytesError(*mut std::os::raw::c_char), ByteLengthNot36BytesError(*mut std::os::raw::c_char), SerdeSerializationError(*mut std::os::raw::c_char), SerdeDeserializationError(*mut std::os::raw::c_char), CborSerializationError(*mut std::os::raw::c_char) }

            impl ferment::FFIConversionFrom<platform_value::error::Error> for platform_value_error_Error {
                unsafe fn ffi_from_const(ffi: *const platform_value_error_Error) -> platform_value::error::Error {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        platform_value_error_Error::Unsupported(o_0) => platform_value::error::Error::Unsupported(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::StructureError(o_0) => platform_value::error::Error::StructureError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::PathError(o_0) => platform_value::error::Error::PathError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::IntegerSizeError => platform_value::error::Error::IntegerSizeError,
                        platform_value_error_Error::IntegerParsingError => platform_value::error::Error::IntegerParsingError,
                        platform_value_error_Error::StringDecodingError(o_0) => platform_value::error::Error::StringDecodingError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::KeyMustBeAString => platform_value::error::Error::KeyMustBeAString,
                        platform_value_error_Error::ByteLengthNot20BytesError(o_0) => platform_value::error::Error::ByteLengthNot20BytesError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::ByteLengthNot32BytesError(o_0) => platform_value::error::Error::ByteLengthNot32BytesError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::ByteLengthNot36BytesError(o_0) => platform_value::error::Error::ByteLengthNot36BytesError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::SerdeSerializationError(o_0) => platform_value::error::Error::SerdeSerializationError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::SerdeDeserializationError(o_0) => platform_value::error::Error::SerdeDeserializationError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0)),
                        platform_value_error_Error::CborSerializationError(o_0) => platform_value::error::Error::CborSerializationError(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(*o_0))
                    }
                }
            }

            impl ferment::FFIConversionTo<platform_value::error::Error> for platform_value_error_Error {
                unsafe fn ffi_to_const(obj: platform_value::error::Error) -> *const platform_value_error_Error {
                    ferment::boxed(match obj {
                        platform_value::error::Error::Unsupported(o_0) => platform_value_error_Error::Unsupported(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::StructureError(o_0) => platform_value_error_Error::StructureError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::PathError(o_0) => platform_value_error_Error::PathError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::IntegerSizeError => platform_value_error_Error::IntegerSizeError,
                        platform_value::error::Error::IntegerParsingError => platform_value_error_Error::IntegerParsingError,
                        platform_value::error::Error::StringDecodingError(o_0) => platform_value_error_Error::StringDecodingError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::KeyMustBeAString => platform_value_error_Error::KeyMustBeAString,
                        platform_value::error::Error::ByteLengthNot20BytesError(o_0) => platform_value_error_Error::ByteLengthNot20BytesError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::ByteLengthNot32BytesError(o_0) => platform_value_error_Error::ByteLengthNot32BytesError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::ByteLengthNot36BytesError(o_0) => platform_value_error_Error::ByteLengthNot36BytesError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::SerdeSerializationError(o_0) => platform_value_error_Error::SerdeSerializationError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::SerdeDeserializationError(o_0) => platform_value_error_Error::SerdeDeserializationError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        platform_value::error::Error::CborSerializationError(o_0) => platform_value_error_Error::CborSerializationError(<std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_0)),
                        _ => unreachable!("This is unreachable")
                    })
                }
            }

            impl Drop for platform_value_error_Error {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            platform_value_error_Error::Unsupported(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::StructureError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::PathError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::IntegerSizeError => {}
                            platform_value_error_Error::IntegerParsingError => {}
                            platform_value_error_Error::StringDecodingError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::KeyMustBeAString => {}
                            platform_value_error_Error::ByteLengthNot20BytesError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::ByteLengthNot32BytesError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::ByteLengthNot36BytesError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::SerdeSerializationError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::SerdeDeserializationError(o_0) => { ferment::unbox_string(*o_0); }
                            platform_value_error_Error::CborSerializationError(o_0) => { ferment::unbox_string(*o_0); }
                            _ => unreachable!("This is unreachable")
                        };
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_Unsupported_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::Unsupported(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_StructureError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::StructureError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_PathError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::PathError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_IntegerSizeError_ctor() -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::IntegerSizeError {}) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_IntegerParsingError_ctor() -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::IntegerParsingError {}) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_StringDecodingError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::StringDecodingError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_KeyMustBeAString_ctor() -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::KeyMustBeAString {}) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_ByteLengthNot20BytesError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::ByteLengthNot20BytesError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_ByteLengthNot32BytesError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::ByteLengthNot32BytesError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_ByteLengthNot36BytesError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::ByteLengthNot36BytesError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_SerdeSerializationError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::SerdeSerializationError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_SerdeDeserializationError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::SerdeDeserializationError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_CborSerializationError_ctor(o_o_0: *mut std::os::raw::c_char) -> *mut platform_value_error_Error { ferment::boxed(platform_value_error_Error::CborSerializationError(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_value_error_Error_destroy<>(ffi: *mut platform_value_error_Error) { ferment::unbox_any(ffi); }
        }

        pub mod types {
            use crate as dash_sdk_bindings;

            pub mod identifier {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`IdentifierBytes32`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct platform_value_types_identifier_IdentifierBytes32(*mut crate::fermented::generics::Arr_u8_32);

                impl ferment::FFIConversionFrom<platform_value::types::identifier::IdentifierBytes32> for platform_value_types_identifier_IdentifierBytes32 {
                    unsafe fn ffi_from_const(ffi: *const platform_value_types_identifier_IdentifierBytes32) -> platform_value::types::identifier::IdentifierBytes32 {
                        let ffi_ref = &*ffi;
                        platform_value::types::identifier::IdentifierBytes32(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(ffi_ref.0))
                    }
                }

                impl ferment::FFIConversionTo<platform_value::types::identifier::IdentifierBytes32> for platform_value_types_identifier_IdentifierBytes32 { unsafe fn ffi_to_const(obj: platform_value::types::identifier::IdentifierBytes32) -> *const platform_value_types_identifier_IdentifierBytes32 { ferment::boxed(platform_value_types_identifier_IdentifierBytes32(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionTo<[u8; 32]>>::ffi_to(obj.0))) } }

                impl Drop for platform_value_types_identifier_IdentifierBytes32 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_ctor<>(o_0: *mut crate::fermented::generics::Arr_u8_32) -> *mut platform_value_types_identifier_IdentifierBytes32 { ferment::boxed(platform_value_types_identifier_IdentifierBytes32(o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_destroy<>(ffi: *mut platform_value_types_identifier_IdentifierBytes32) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_get_0<>(obj: *const platform_value_types_identifier_IdentifierBytes32) -> *mut crate::fermented::generics::Arr_u8_32 { (*obj).0 }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_set_0<>(obj: *const platform_value_types_identifier_IdentifierBytes32) -> *mut crate::fermented::generics::Arr_u8_32 { (*obj).0 }

                #[doc = "FFI-representation of the [`Identifier`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct platform_value_types_identifier_Identifier(*mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_IdentifierBytes32);

                impl ferment::FFIConversionFrom<platform_value::types::identifier::Identifier> for platform_value_types_identifier_Identifier {
                    unsafe fn ffi_from_const(ffi: *const platform_value_types_identifier_Identifier) -> platform_value::types::identifier::Identifier {
                        let ffi_ref = &*ffi;
                        platform_value::types::identifier::Identifier(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_IdentifierBytes32 as ferment::FFIConversionFrom<platform_value::types::identifier::IdentifierBytes32>>::ffi_from(ffi_ref.0))
                    }
                }

                impl ferment::FFIConversionTo<platform_value::types::identifier::Identifier> for platform_value_types_identifier_Identifier { unsafe fn ffi_to_const(obj: platform_value::types::identifier::Identifier) -> *const platform_value_types_identifier_Identifier { ferment::boxed(platform_value_types_identifier_Identifier(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_IdentifierBytes32 as ferment::FFIConversionTo<platform_value::types::identifier::IdentifierBytes32>>::ffi_to(obj.0))) } }

                impl Drop for platform_value_types_identifier_Identifier {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_ctor<>(o_0: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_IdentifierBytes32) -> *mut platform_value_types_identifier_Identifier { ferment::boxed(platform_value_types_identifier_Identifier(o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_destroy<>(ffi: *mut platform_value_types_identifier_Identifier) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_get_0<>(obj: *const platform_value_types_identifier_Identifier) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_IdentifierBytes32 { (*obj).0 }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_set_0<>(obj: *const platform_value_types_identifier_Identifier) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_IdentifierBytes32 { (*obj).0 }
            }

            pub mod binary_data {
                use crate as dash_sdk_bindings;

                #[doc = "FFI-representation of the [`BinaryData`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct platform_value_types_binary_data_BinaryData(*mut crate::fermented::generics::Vec_u8);

                impl ferment::FFIConversionFrom<platform_value::types::binary_data::BinaryData> for platform_value_types_binary_data_BinaryData {
                    unsafe fn ffi_from_const(ffi: *const platform_value_types_binary_data_BinaryData) -> platform_value::types::binary_data::BinaryData {
                        let ffi_ref = &*ffi;
                        platform_value::types::binary_data::BinaryData(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(ffi_ref.0))
                    }
                }

                impl ferment::FFIConversionTo<platform_value::types::binary_data::BinaryData> for platform_value_types_binary_data_BinaryData { unsafe fn ffi_to_const(obj: platform_value::types::binary_data::BinaryData) -> *const platform_value_types_binary_data_BinaryData { ferment::boxed(platform_value_types_binary_data_BinaryData(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(obj.0))) } }

                impl Drop for platform_value_types_binary_data_BinaryData {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.0);
                        }
                    }
                }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_ctor<>(o_0: *mut crate::fermented::generics::Vec_u8) -> *mut platform_value_types_binary_data_BinaryData { ferment::boxed(platform_value_types_binary_data_BinaryData(o_0)) }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_destroy<>(ffi: *mut platform_value_types_binary_data_BinaryData) { ferment::unbox_any(ffi); }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_get_0<>(obj: *const platform_value_types_binary_data_BinaryData) -> *mut crate::fermented::generics::Vec_u8 { (*obj).0 }

                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_set_0<>(obj: *const platform_value_types_binary_data_BinaryData) -> *mut crate::fermented::generics::Vec_u8 { (*obj).0 }
            }
        }
    }

    pub mod platform_mobile {
        use crate as dash_sdk_bindings;

        pub mod core {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::core::get_transaction_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_core_get_transaction_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, txid: *mut crate::fermented::generics::Arr_u8_32) -> *mut crate::fermented::generics::Result_ok_Vec_u8_err_String {
                let obj = platform_mobile::core::get_transaction_sdk(rust_sdk, <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(txid));
                <crate::fermented::generics::Result_ok_Vec_u8_err_String as ferment::FFIConversionTo<Result<Vec<u8>, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::core::get_transaction`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_core_get_transaction(txid: *mut crate::fermented::generics::Arr_u8_32, quorum_public_key_callback: u64, data_contract_callback: u64) -> *mut crate::fermented::generics::Result_ok_Vec_u8_err_String {
                let obj = platform_mobile::core::get_transaction(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(txid), quorum_public_key_callback, data_contract_callback);
                <crate::fermented::generics::Result_ok_Vec_u8_err_String as ferment::FFIConversionTo<Result<Vec<u8>, String>>>::ffi_to(obj)
            }
        }

        pub mod data_contracts {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::data_contracts::DataContractFFI_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_clone(value: *mut crate::fermented::types::platform_mobile::data_contracts::platform_mobile_data_contracts_DataContractFFI) -> *mut crate::fermented::types::platform_mobile::data_contracts::platform_mobile_data_contracts_DataContractFFI {
                let obj = platform_mobile::data_contracts::DataContractFFI_clone(<crate::fermented::types::platform_mobile::data_contracts::platform_mobile_data_contracts_DataContractFFI as ferment::FFIConversionFrom<platform_mobile::data_contracts::DataContractFFI>>::ffi_from(value));
                <crate::fermented::types::platform_mobile::data_contracts::platform_mobile_data_contracts_DataContractFFI as ferment::FFIConversionTo<platform_mobile::data_contracts::DataContractFFI>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`DataContractFFI`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct platform_mobile_data_contracts_DataContractFFI {
                pub id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
                pub owner_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
                pub doc_types: *mut crate::fermented::generics::Vec_String,
                pub version: u32,
            }

            impl ferment::FFIConversionFrom<platform_mobile::data_contracts::DataContractFFI> for platform_mobile_data_contracts_DataContractFFI {
                unsafe fn ffi_from_const(ffi: *const platform_mobile_data_contracts_DataContractFFI) -> platform_mobile::data_contracts::DataContractFFI {
                    let ffi_ref = &*ffi;
                    platform_mobile::data_contracts::DataContractFFI { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.id), owner_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(ffi_ref.owner_id), doc_types: <crate::fermented::generics::Vec_String as ferment::FFIConversionFrom<Vec<String>>>::ffi_from(ffi_ref.doc_types), version: ffi_ref.version }
                }
            }

            impl ferment::FFIConversionTo<platform_mobile::data_contracts::DataContractFFI> for platform_mobile_data_contracts_DataContractFFI { unsafe fn ffi_to_const(obj: platform_mobile::data_contracts::DataContractFFI) -> *const platform_mobile_data_contracts_DataContractFFI { ferment::boxed(platform_mobile_data_contracts_DataContractFFI { id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.id), owner_id: <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj.owner_id), doc_types: <crate::fermented::generics::Vec_String as ferment::FFIConversionTo<Vec<String>>>::ffi_to(obj.doc_types), version: obj.version }) } }

            impl Drop for platform_mobile_data_contracts_DataContractFFI {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.id);
                        ferment::unbox_any(ffi_ref.owner_id);
                        ferment::unbox_any(ffi_ref.doc_types);
                        ;
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_ctor<>(id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, owner_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, doc_types: *mut crate::fermented::generics::Vec_String, version: u32) -> *mut platform_mobile_data_contracts_DataContractFFI { ferment::boxed(platform_mobile_data_contracts_DataContractFFI { id, owner_id, doc_types, version }) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_destroy<>(ffi: *mut platform_mobile_data_contracts_DataContractFFI) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_get_id<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).id }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_get_owner_id<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).owner_id }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_get_doc_types<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> *mut crate::fermented::generics::Vec_String { (*obj).doc_types }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_get_version<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> u32 { (*obj).version }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_set_id<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).id }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_set_owner_id<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier { (*obj).owner_id }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_set_doc_types<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> *mut crate::fermented::generics::Vec_String { (*obj).doc_types }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_DataContractFFI_set_version<>(obj: *const platform_mobile_data_contracts_DataContractFFI) -> u32 { (*obj).version }

            #[doc = "FFI-representation of the [`platform_mobile::data_contracts::fetch_data_contract`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_data_contracts_fetch_data_contract(rust_sdk: *mut platform_mobile::sdk::DashSdk, data_contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::generics::Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String {
                let obj = platform_mobile::data_contracts::fetch_data_contract(rust_sdk, <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(data_contract_id));
                <crate::fermented::generics::Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String as ferment::FFIConversionTo<Result<Option<platform_mobile::data_contracts::DataContractFFI>, String>>>::ffi_to(obj)
            }
        }

        pub mod clone {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::clone::ContestedResources_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ContestedResources_clone(o: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResources) -> *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResources {
                let obj = platform_mobile::clone::ContestedResources_clone(<crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResources as ferment::FFIConversionFrom<drive_proof_verifier::types::ContestedResources>>::ffi_from(o));
                <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResources as ferment::FFIConversionTo<drive_proof_verifier::types::ContestedResources>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Vote_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Vote_clone(o: *mut crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote) -> *mut crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote {
                let obj = platform_mobile::clone::Vote_clone(<crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote as ferment::FFIConversionFrom<dpp::voting::votes::Vote>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote as ferment::FFIConversionTo<dpp::voting::votes::Vote>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_clone(o: *mut crate::fermented::generics::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll) -> *mut crate::fermented::generics::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll {
                let obj = platform_mobile::clone::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_clone(<crate::fermented::generics::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll as ferment::FFIConversionFrom<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)>>::ffi_from(o));
                <crate::fermented::generics::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll as ferment::FFIConversionTo<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Revision_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Revision_clone(revision: *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_Revision {
                let obj = platform_mobile::clone::Revision_clone(<crate::fermented::types::dpp::prelude::dpp_prelude_Revision as ferment::FFIConversionFrom<dpp::prelude::Revision>>::ffi_from(revision));
                <crate::fermented::types::dpp::prelude::dpp_prelude_Revision as ferment::FFIConversionTo<dpp::prelude::Revision>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ResourceVotesByIdentity_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ResourceVotesByIdentity_clone(o: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ResourceVotesByIdentity) -> *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ResourceVotesByIdentity {
                let obj = platform_mobile::clone::ResourceVotesByIdentity_clone(<crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ResourceVotesByIdentity as ferment::FFIConversionFrom<drive_proof_verifier::types::ResourceVotesByIdentity>>::ffi_from(o));
                <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ResourceVotesByIdentity as ferment::FFIConversionTo<drive_proof_verifier::types::ResourceVotesByIdentity>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::TimestampMillis_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_TimestampMillis_clone(time: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis {
                let obj = platform_mobile::clone::TimestampMillis_clone(<crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis>>::ffi_from(time));
                <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionTo<dpp::identity::identity_public_key::TimestampMillis>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ResourceVoteChoice_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ResourceVoteChoice_clone(o: *mut crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice) -> *mut crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice {
                let obj = platform_mobile::clone::ResourceVoteChoice_clone(<crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice as ferment::FFIConversionFrom<dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::vote_choices::resource_vote_choice::dpp_voting_vote_choices_resource_vote_choice_ResourceVoteChoice as ferment::FFIConversionTo<dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Arr_u8_32_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Arr_u8_32_clone(slice: *mut crate::fermented::generics::Arr_u8_32) -> *mut crate::fermented::generics::Arr_u8_32 {
                let obj = platform_mobile::clone::Arr_u8_32_clone(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(slice));
                <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionTo<[u8; 32]>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Voters_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Voters_clone(o: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voters) -> *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voters {
                let obj = platform_mobile::clone::Voters_clone(<crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voters as ferment::FFIConversionFrom<drive_proof_verifier::types::Voters>>::ffi_from(o));
                <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voters as ferment::FFIConversionTo<drive_proof_verifier::types::Voters>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ContestedDocumentResourceVotePoll_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ContestedDocumentResourceVotePoll_clone(o: *mut crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll) -> *mut crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll {
                let obj = platform_mobile::clone::ContestedDocumentResourceVotePoll_clone(<crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll as ferment::FFIConversionFrom<dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::vote_polls::contested_document_resource_vote_poll::dpp_voting_vote_polls_contested_document_resource_vote_poll_ContestedDocumentResourceVotePoll as ferment::FFIConversionTo<dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::std_collections_Map_keys_String_values_platform_value_Value_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_std_collections_Map_keys_String_values_platform_value_Value_clone(map: *mut crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value) -> *mut crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value {
                let obj = platform_mobile::clone::std_collections_Map_keys_String_values_platform_value_Value_clone(<crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value as ferment::FFIConversionFrom<std::collections::BTreeMap<String, platform_value::Value>>>::ffi_from(map));
                <crate::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value as ferment::FFIConversionTo<std::collections::BTreeMap<String, platform_value::Value>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Value_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Value_clone(value: *mut crate::fermented::types::platform_value::platform_value_Value) -> *mut crate::fermented::types::platform_value::platform_value_Value {
                let obj = platform_mobile::clone::Value_clone(<crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from(value));
                <crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionTo<platform_value::Value>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ContenderWithSerializedDocument_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ContenderWithSerializedDocument_clone(o: *mut crate::fermented::types::dpp::voting::contender_structs::contender::dpp_voting_contender_structs_contender_ContenderWithSerializedDocument) -> *mut crate::fermented::types::dpp::voting::contender_structs::contender::dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
                let obj = platform_mobile::clone::ContenderWithSerializedDocument_clone(<crate::fermented::types::dpp::voting::contender_structs::contender::dpp_voting_contender_structs_contender_ContenderWithSerializedDocument as ferment::FFIConversionFrom<dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::contender_structs::contender::dpp_voting_contender_structs_contender_ContenderWithSerializedDocument as ferment::FFIConversionTo<dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ResourceVote_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ResourceVote_clone(o: *mut crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote) -> *mut crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote {
                let obj = platform_mobile::clone::ResourceVote_clone(<crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote as ferment::FFIConversionFrom<dpp::voting::votes::resource_vote::ResourceVote>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote as ferment::FFIConversionTo<dpp::voting::votes::resource_vote::ResourceVote>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::prelude_TimestampMillis_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_prelude_TimestampMillis_clone(time: *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis {
                let obj = platform_mobile::clone::prelude_TimestampMillis_clone(<crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis as ferment::FFIConversionFrom<dpp::prelude::TimestampMillis>>::ffi_from(time));
                <crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis as ferment::FFIConversionTo<dpp::prelude::TimestampMillis>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::WhereOperator_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_WhereOperator_clone(o: *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator) -> *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator {
                let obj = platform_mobile::clone::WhereOperator_clone(<crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator as ferment::FFIConversionFrom<drive::query::conditions::WhereOperator>>::ffi_from(o));
                <crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereOperator as ferment::FFIConversionTo<drive::query::conditions::WhereOperator>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::std_collections_BTreeSet_drive_proof_verifier_types_Voter_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_std_collections_BTreeSet_drive_proof_verifier_types_Voter_clone(o: *mut crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter) -> *mut crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter {
                let obj = platform_mobile::clone::std_collections_BTreeSet_drive_proof_verifier_types_Voter_clone(<crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter as ferment::FFIConversionFrom<std::collections::BTreeSet<drive_proof_verifier::types::Voter>>>::ffi_from(o));
                <crate::fermented::generics::std_collections_BTreeSet_drive_proof_verifier_types_Voter as ferment::FFIConversionTo<std::collections::BTreeSet<drive_proof_verifier::types::Voter>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::VotePollsGroupedByTimestamp_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_VotePollsGroupedByTimestamp_clone(o: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_VotePollsGroupedByTimestamp) -> *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_VotePollsGroupedByTimestamp {
                let obj = platform_mobile::clone::VotePollsGroupedByTimestamp_clone(<crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_VotePollsGroupedByTimestamp as ferment::FFIConversionFrom<drive_proof_verifier::types::VotePollsGroupedByTimestamp>>::ffi_from(o));
                <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_VotePollsGroupedByTimestamp as ferment::FFIConversionTo<drive_proof_verifier::types::VotePollsGroupedByTimestamp>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::CoreBlockHeight_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_CoreBlockHeight_clone(height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight {
                let obj = platform_mobile::clone::CoreBlockHeight_clone(<crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight>>::ffi_from(height));
                <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionTo<dpp::prelude::CoreBlockHeight>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Document_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Document_clone(document: *mut crate::fermented::types::dpp::document::dpp_document_Document) -> *mut crate::fermented::types::dpp::document::dpp_document_Document {
                let obj = platform_mobile::clone::Document_clone(<crate::fermented::types::dpp::document::dpp_document_Document as ferment::FFIConversionFrom<dpp::document::Document>>::ffi_from(document));
                <crate::fermented::types::dpp::document::dpp_document_Document as ferment::FFIConversionTo<dpp::document::Document>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ValueMap_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ValueMap_clone(value_map: *mut crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap) -> *mut crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap {
                let obj = platform_mobile::clone::ValueMap_clone(<crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap as ferment::FFIConversionFrom<platform_value::value_map::ValueMap>>::ffi_from(value_map));
                <crate::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap as ferment::FFIConversionTo<platform_value::value_map::ValueMap>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::DocumentV0_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_DocumentV0_clone(document: *mut crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0) -> *mut crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0 {
                let obj = platform_mobile::clone::DocumentV0_clone(<crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0 as ferment::FFIConversionFrom<dpp::document::v0::DocumentV0>>::ffi_from(document));
                <crate::fermented::types::dpp::document::v0::dpp_document_v0_DocumentV0 as ferment::FFIConversionTo<dpp::document::v0::DocumentV0>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::KeyID_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_KeyID_clone(id: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID {
                let obj = platform_mobile::clone::KeyID_clone(<crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionFrom<dpp::identity::identity_public_key::KeyID>>::ffi_from(id));
                <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionTo<dpp::identity::identity_public_key::KeyID>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Arr_u8_36_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Arr_u8_36_clone(slice: *mut crate::fermented::generics::Arr_u8_36) -> *mut crate::fermented::generics::Arr_u8_36 {
                let obj = platform_mobile::clone::Arr_u8_36_clone(<crate::fermented::generics::Arr_u8_36 as ferment::FFIConversionFrom<[u8; 36]>>::ffi_from(slice));
                <crate::fermented::generics::Arr_u8_36 as ferment::FFIConversionTo<[u8; 36]>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Arr_u8_20_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Arr_u8_20_clone(slice: *mut crate::fermented::generics::Arr_u8_20) -> *mut crate::fermented::generics::Arr_u8_20 {
                let obj = platform_mobile::clone::Arr_u8_20_clone(<crate::fermented::generics::Arr_u8_20 as ferment::FFIConversionFrom<[u8; 20]>>::ffi_from(slice));
                <crate::fermented::generics::Arr_u8_20 as ferment::FFIConversionTo<[u8; 20]>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Contenders_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Contenders_clone(o: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Contenders) -> *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Contenders {
                let obj = platform_mobile::clone::Contenders_clone(<crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Contenders as ferment::FFIConversionFrom<drive_proof_verifier::types::Contenders>>::ffi_from(o));
                <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Contenders as ferment::FFIConversionTo<drive_proof_verifier::types::Contenders>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Vec_u8_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Vec_u8_clone(vec: *mut crate::fermented::generics::Vec_u8) -> *mut crate::fermented::generics::Vec_u8 {
                let obj = platform_mobile::clone::Vec_u8_clone(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(vec));
                <crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::BlockHeight_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_BlockHeight_clone(height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight) -> *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight {
                let obj = platform_mobile::clone::BlockHeight_clone(<crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionFrom<dpp::prelude::BlockHeight>>::ffi_from(height));
                <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionTo<dpp::prelude::BlockHeight>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ContenderWithSerializedDocumentV0_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ContenderWithSerializedDocumentV0_clone(o: *mut crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0) -> *mut crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 {
                let obj = platform_mobile::clone::ContenderWithSerializedDocumentV0_clone(<crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 as ferment::FFIConversionFrom<dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::contender_structs::contender::v0::dpp_voting_contender_structs_contender_v0_ContenderWithSerializedDocumentV0 as ferment::FFIConversionTo<dpp::voting::contender_structs::contender::v0::ContenderWithSerializedDocumentV0>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ResourceVoteV0_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ResourceVoteV0_clone(o: *mut crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0) -> *mut crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0 {
                let obj = platform_mobile::clone::ResourceVoteV0_clone(<crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0 as ferment::FFIConversionFrom<dpp::voting::votes::resource_vote::v0::ResourceVoteV0>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::votes::resource_vote::v0::dpp_voting_votes_resource_vote_v0_ResourceVoteV0 as ferment::FFIConversionTo<dpp::voting::votes::resource_vote::v0::ResourceVoteV0>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Vec_Value_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Vec_Value_clone(value_vec: *mut crate::fermented::generics::Vec_platform_value_Value) -> *mut crate::fermented::generics::Vec_platform_value_Value {
                let obj = platform_mobile::clone::Vec_Value_clone(<crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionFrom<Vec<platform_value::Value>>>::ffi_from(value_vec));
                <crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionTo<Vec<platform_value::Value>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::VotePoll_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_VotePoll_clone(o: *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll) -> *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll {
                let obj = platform_mobile::clone::VotePoll_clone(<crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll as ferment::FFIConversionFrom<dpp::voting::vote_polls::VotePoll>>::ffi_from(o));
                <crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll as ferment::FFIConversionTo<dpp::voting::vote_polls::VotePoll>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Voter_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Voter_clone(o: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voter) -> *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voter {
                let obj = platform_mobile::clone::Voter_clone(<crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voter as ferment::FFIConversionFrom<drive_proof_verifier::types::Voter>>::ffi_from(o));
                <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voter as ferment::FFIConversionTo<drive_proof_verifier::types::Voter>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::OrderClause_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_OrderClause_clone(o: *mut crate::fermented::types::drive::query::ordering::drive_query_ordering_OrderClause) -> *mut crate::fermented::types::drive::query::ordering::drive_query_ordering_OrderClause {
                let obj = platform_mobile::clone::OrderClause_clone(<crate::fermented::types::drive::query::ordering::drive_query_ordering_OrderClause as ferment::FFIConversionFrom<drive::query::ordering::OrderClause>>::ffi_from(o));
                <crate::fermented::types::drive::query::ordering::drive_query_ordering_OrderClause as ferment::FFIConversionTo<drive::query::ordering::OrderClause>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::ContestedResource_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_ContestedResource_clone(o: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResource) -> *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResource {
                let obj = platform_mobile::clone::ContestedResource_clone(<crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResource as ferment::FFIConversionFrom<drive_proof_verifier::types::ContestedResource>>::ffi_from(o));
                <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResource as ferment::FFIConversionTo<drive_proof_verifier::types::ContestedResource>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::WhereClause_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_WhereClause_clone(o: *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereClause) -> *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereClause {
                let obj = platform_mobile::clone::WhereClause_clone(<crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereClause as ferment::FFIConversionFrom<drive::query::conditions::WhereClause>>::ffi_from(o));
                <crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereClause as ferment::FFIConversionTo<drive::query::conditions::WhereClause>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::clone::Hash256_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_clone_Hash256_clone(o: *mut crate::fermented::types::platform_value::platform_value_Hash256) -> *mut crate::fermented::types::platform_value::platform_value_Hash256 {
                let obj = platform_mobile::clone::Hash256_clone(<crate::fermented::types::platform_value::platform_value_Hash256 as ferment::FFIConversionFrom<platform_value::Hash256>>::ffi_from(o));
                <crate::fermented::types::platform_value::platform_value_Hash256 as ferment::FFIConversionTo<platform_value::Hash256>>::ffi_to(obj)
            }
        }

        pub mod operators {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::operators::Value_hash`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_operators_Value_hash(a: *mut crate::fermented::types::platform_value::platform_value_Value) -> i32 {
                let obj = platform_mobile::operators::Value_hash(<crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from(a));
                obj
            }

            #[doc = "FFI-representation of the [`platform_mobile::operators::Value_eq`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_operators_Value_eq(a: *mut crate::fermented::types::platform_value::platform_value_Value, b: *mut crate::fermented::types::platform_value::platform_value_Value) -> bool {
                let obj = platform_mobile::operators::Value_eq(<crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from(a), <crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from(b));
                obj
            }
        }

        pub mod identity {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::identity::IdentityPublicKey_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_IdentityPublicKey_clone(identity_public_key: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey) -> *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey {
                let obj = platform_mobile::identity::IdentityPublicKey_clone(<crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<dpp::identity::identity_public_key::IdentityPublicKey>>::ffi_from(identity_public_key));
                <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionTo<dpp::identity::identity_public_key::IdentityPublicKey>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(public_keys: *mut crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey) -> *mut crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey {
                let obj = platform_mobile::identity::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(<crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey>>>::ffi_from(public_keys));
                <crate::fermented::generics::std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionTo<std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::Identity_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_Identity_clone(identity: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity) -> *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity {
                let obj = platform_mobile::identity::Identity_clone(<crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionFrom<dpp::identity::identity::Identity>>::ffi_from(identity));
                <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionTo<dpp::identity::identity::Identity>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::create_basic_identity`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_create_basic_identity(id: *mut crate::fermented::generics::Arr_u8_32) -> *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity {
                let obj = platform_mobile::identity::create_basic_identity(<crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(id));
                <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionTo<dpp::identity::identity::Identity>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::Identifier_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_Identifier_clone(identifier: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier {
                let obj = platform_mobile::identity::Identifier_clone(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(identifier));
                <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionTo<platform_value::types::identifier::Identifier>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::IdentityV0_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_IdentityV0_clone(identity: *mut crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0) -> *mut crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0 {
                let obj = platform_mobile::identity::IdentityV0_clone(<crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0 as ferment::FFIConversionFrom<dpp::identity::v0::IdentityV0>>::ffi_from(identity));
                <crate::fermented::types::dpp::identity::v0::dpp_identity_v0_IdentityV0 as ferment::FFIConversionTo<dpp::identity::v0::IdentityV0>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::IdentityPublicKeyV0_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_IdentityPublicKeyV0_clone(identity_public_key: *mut crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0) -> *mut crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
                let obj = platform_mobile::identity::IdentityPublicKeyV0_clone(<crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 as ferment::FFIConversionFrom<dpp::identity::identity_public_key::v0::IdentityPublicKeyV0>>::ffi_from(identity_public_key));
                <crate::fermented::types::dpp::identity::identity_public_key::v0::dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 as ferment::FFIConversionTo<dpp::identity::identity_public_key::v0::IdentityPublicKeyV0>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::get_identity_contract_bounds`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_get_identity_contract_bounds(identifier: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, contract_identifier: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity {
                let obj = platform_mobile::identity::get_identity_contract_bounds(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(identifier), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from_opt(contract_identifier));
                <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionTo<dpp::identity::identity::Identity>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::identity::get_identity2`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_identity_get_identity2(identifier: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity {
                let obj = platform_mobile::identity::get_identity2(<crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(identifier));
                <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionTo<dpp::identity::identity::Identity>>::ffi_to(obj)
            }
        }

        pub mod sdk {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::sdk::destroy_dash_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_sdk_destroy_dash_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk) {
                let obj = platform_mobile::sdk::destroy_dash_sdk(rust_sdk);
                ;
            }

            #[doc = "FFI-representation of the [`platform_mobile::sdk::create_dash_sdk_with_context`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_sdk_create_dash_sdk_with_context(context_provider_context: usize, quorum_public_key_callback: u64, data_contract_callback: u64, is_testnet: bool, connect_timeout: usize, timeout: usize, retries: usize) -> *mut platform_mobile::sdk::DashSdk {
                let obj = platform_mobile::sdk::create_dash_sdk_with_context(context_provider_context, quorum_public_key_callback, data_contract_callback, is_testnet, connect_timeout, timeout, retries);
                ferment::boxed(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::sdk::create_dash_sdk_using_single_evonode`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_sdk_create_dash_sdk_using_single_evonode(evonode: *mut std::os::raw::c_char, quorum_public_key_callback: u64, data_contract_callback: u64, is_testnet: bool) -> *mut platform_mobile::sdk::DashSdk {
                let obj = platform_mobile::sdk::create_dash_sdk_using_single_evonode(<std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(evonode), quorum_public_key_callback, data_contract_callback, is_testnet);
                ferment::boxed(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::sdk::create_dash_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_sdk_create_dash_sdk(quorum_public_key_callback: u64, data_contract_callback: u64, is_testnet: bool) -> *mut platform_mobile::sdk::DashSdk {
                let obj = platform_mobile::sdk::create_dash_sdk(quorum_public_key_callback, data_contract_callback, is_testnet);
                ferment::boxed(obj)
            }
        }

        pub mod voting {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::voting::get_vote_contenders`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_voting_get_vote_contenders(rust_sdk: *mut platform_mobile::sdk::DashSdk, index_name: *mut std::os::raw::c_char, index_values: *mut crate::fermented::generics::Vec_platform_value_Value, document_type_name: *mut std::os::raw::c_char, contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::generics::Result_ok_drive_proof_verifier_types_Contenders_err_String {
                let obj = platform_mobile::voting::get_vote_contenders(rust_sdk, <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(index_name), <crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionFrom<Vec<platform_value::Value>>>::ffi_from(index_values), <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(document_type_name), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(contract_id));
                <crate::fermented::generics::Result_ok_drive_proof_verifier_types_Contenders_err_String as ferment::FFIConversionTo<Result<drive_proof_verifier::types::Contenders, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::voting::put_vote_to_platform`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_voting_put_vote_to_platform(rust_sdk: *mut platform_mobile::sdk::DashSdk, vote: *mut crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote, voter_pro_tx_hash: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, voting_public_key: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey, signer_context: usize, signer_callback: u64) -> *mut crate::fermented::generics::Result_ok_dpp_voting_votes_Vote_err_String {
                let obj = platform_mobile::voting::put_vote_to_platform(rust_sdk, <crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote as ferment::FFIConversionFrom<dpp::voting::votes::Vote>>::ffi_from(vote), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(voter_pro_tx_hash), <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<dpp::identity::identity_public_key::IdentityPublicKey>>::ffi_from(voting_public_key), signer_context, signer_callback);
                <crate::fermented::generics::Result_ok_dpp_voting_votes_Vote_err_String as ferment::FFIConversionTo<Result<dpp::voting::votes::Vote, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::voting::get_votes`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_voting_get_votes(rust_sdk: *mut platform_mobile::sdk::DashSdk, data_contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::generics::Result_ok_Option_dpp_voting_votes_Vote_err_String {
                let obj = platform_mobile::voting::get_votes(rust_sdk, <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(data_contract_id));
                <crate::fermented::generics::Result_ok_Option_dpp_voting_votes_Vote_err_String as ferment::FFIConversionTo<Result<Option<dpp::voting::votes::Vote>, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::voting::get_vote_polls`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_voting_get_vote_polls(rust_sdk: *mut platform_mobile::sdk::DashSdk, start_time: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis, start_time_included: bool, end_time: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis, end_time_included: bool, limit: u16, offset: u16, order_ascending: bool) -> *mut crate::fermented::generics::Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String {
                let obj = platform_mobile::voting::get_vote_polls(rust_sdk, <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis>>::ffi_from(start_time), start_time_included, <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_TimestampMillis as ferment::FFIConversionFrom<dpp::identity::identity_public_key::TimestampMillis>>::ffi_from(end_time), end_time_included, limit, offset, order_ascending);
                <crate::fermented::generics::Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String as ferment::FFIConversionTo<Result<drive_proof_verifier::types::VotePollsGroupedByTimestamp, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::voting::get_last_vote_from_masternode`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_voting_get_last_vote_from_masternode(rust_sdk: *mut platform_mobile::sdk::DashSdk, masternode_protxhash: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, index_name: *mut std::os::raw::c_char, index_values: *mut crate::fermented::generics::Vec_platform_value_Value, document_type_name: *mut std::os::raw::c_char, contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::generics::Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String {
                let obj = platform_mobile::voting::get_last_vote_from_masternode(rust_sdk, <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(masternode_protxhash), <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(index_name), <crate::fermented::generics::Vec_platform_value_Value as ferment::FFIConversionFrom<Vec<platform_value::Value>>>::ffi_from(index_values), <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(document_type_name), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(contract_id));
                <crate::fermented::generics::Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String as ferment::FFIConversionTo<Result<drive_proof_verifier::types::ResourceVotesByIdentity, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::voting::get_contested_resources`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_voting_get_contested_resources(rust_sdk: *mut platform_mobile::sdk::DashSdk, document_type_name: *mut std::os::raw::c_char, data_contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, limit: u16, start_at: *mut crate::fermented::types::platform_value::platform_value_Value, start_at_include: bool) -> *mut crate::fermented::generics::Result_ok_drive_proof_verifier_types_ContestedResources_err_String {
                let obj = platform_mobile::voting::get_contested_resources(rust_sdk, <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(document_type_name), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(data_contract_id), limit, <crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from_opt(start_at), start_at_include);
                <crate::fermented::generics::Result_ok_drive_proof_verifier_types_ContestedResources_err_String as ferment::FFIConversionTo<Result<drive_proof_verifier::types::ContestedResources, String>>>::ffi_to(obj)
            }
        }

        pub mod put {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`AssetLockProofFFI`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum platform_mobile_put_AssetLockProofFFI { Instant(*mut crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI), Chain(*mut crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI) }

            impl ferment::FFIConversionFrom<platform_mobile::put::AssetLockProofFFI> for platform_mobile_put_AssetLockProofFFI {
                unsafe fn ffi_from_const(ffi: *const platform_mobile_put_AssetLockProofFFI) -> platform_mobile::put::AssetLockProofFFI {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        platform_mobile_put_AssetLockProofFFI::Instant(o_0) => platform_mobile::put::AssetLockProofFFI::Instant(<crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI as ferment::FFIConversionFrom<platform_mobile::put::InstantAssetLockProofFFI>>::ffi_from(*o_0)),
                        platform_mobile_put_AssetLockProofFFI::Chain(o_0) => platform_mobile::put::AssetLockProofFFI::Chain(<crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI as ferment::FFIConversionFrom<platform_mobile::put::ChainAssetLockProofFFI>>::ffi_from(*o_0))
                    }
                }
            }

            impl ferment::FFIConversionTo<platform_mobile::put::AssetLockProofFFI> for platform_mobile_put_AssetLockProofFFI {
                unsafe fn ffi_to_const(obj: platform_mobile::put::AssetLockProofFFI) -> *const platform_mobile_put_AssetLockProofFFI {
                    ferment::boxed(match obj {
                        platform_mobile::put::AssetLockProofFFI::Instant(o_0) => platform_mobile_put_AssetLockProofFFI::Instant(<crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI as ferment::FFIConversionTo<platform_mobile::put::InstantAssetLockProofFFI>>::ffi_to(o_0)),
                        platform_mobile::put::AssetLockProofFFI::Chain(o_0) => platform_mobile_put_AssetLockProofFFI::Chain(<crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI as ferment::FFIConversionTo<platform_mobile::put::ChainAssetLockProofFFI>>::ffi_to(o_0)),
                        _ => unreachable!("This is unreachable")
                    })
                }
            }

            impl Drop for platform_mobile_put_AssetLockProofFFI {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            platform_mobile_put_AssetLockProofFFI::Instant(o_0) => { ferment::unbox_any(*o_0); }
                            platform_mobile_put_AssetLockProofFFI::Chain(o_0) => { ferment::unbox_any(*o_0); }
                            _ => unreachable!("This is unreachable")
                        };
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_AssetLockProofFFI_Instant_ctor(o_o_0: *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI) -> *mut platform_mobile_put_AssetLockProofFFI { ferment::boxed(platform_mobile_put_AssetLockProofFFI::Instant(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_AssetLockProofFFI_Chain_ctor(o_o_0: *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI) -> *mut platform_mobile_put_AssetLockProofFFI { ferment::boxed(platform_mobile_put_AssetLockProofFFI::Chain(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_AssetLockProofFFI_destroy<>(ffi: *mut platform_mobile_put_AssetLockProofFFI) { ferment::unbox_any(ffi); }

            #[doc = "FFI-representation of the [`platform_mobile::put::put_document_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_put_document_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, document: *mut crate::fermented::types::dpp::document::dpp_document_Document, data_contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, document_type_str: *mut std::os::raw::c_char, identity_public_key: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey, block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight, core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight, signer_context: usize, signer_callback: u64) -> *mut crate::fermented::generics::Result_ok_dpp_document_Document_err_String {
                let obj = platform_mobile::put::put_document_sdk(rust_sdk, <crate::fermented::types::dpp::document::dpp_document_Document as ferment::FFIConversionFrom<dpp::document::Document>>::ffi_from(document), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(data_contract_id), <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(document_type_str), <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<dpp::identity::identity_public_key::IdentityPublicKey>>::ffi_from(identity_public_key), <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionFrom<dpp::prelude::BlockHeight>>::ffi_from(block_height), <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight>>::ffi_from(core_block_height), signer_context, signer_callback);
                <crate::fermented::generics::Result_ok_dpp_document_Document_err_String as ferment::FFIConversionTo<Result<dpp::document::Document, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::put::replace_document_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_replace_document_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, document: *mut crate::fermented::types::dpp::document::dpp_document_Document, data_contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, document_type_str: *mut std::os::raw::c_char, identity_public_key: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey, block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight, core_block_height: *mut crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight, signer_context: usize, signer_callback: u64) -> *mut crate::fermented::generics::Result_ok_dpp_document_Document_err_String {
                let obj = platform_mobile::put::replace_document_sdk(rust_sdk, <crate::fermented::types::dpp::document::dpp_document_Document as ferment::FFIConversionFrom<dpp::document::Document>>::ffi_from(document), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(data_contract_id), <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(document_type_str), <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<dpp::identity::identity_public_key::IdentityPublicKey>>::ffi_from(identity_public_key), <crate::fermented::types::dpp::prelude::dpp_prelude_BlockHeight as ferment::FFIConversionFrom<dpp::prelude::BlockHeight>>::ffi_from(block_height), <crate::fermented::types::dpp::prelude::dpp_prelude_CoreBlockHeight as ferment::FFIConversionFrom<dpp::prelude::CoreBlockHeight>>::ffi_from(core_block_height), signer_context, signer_callback);
                <crate::fermented::generics::Result_ok_dpp_document_Document_err_String as ferment::FFIConversionTo<Result<dpp::document::Document, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::put::InstantAssetLockProofFFI_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_clone(a: *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI) -> *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI {
                let obj = platform_mobile::put::InstantAssetLockProofFFI_clone(<crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI as ferment::FFIConversionFrom<platform_mobile::put::InstantAssetLockProofFFI>>::ffi_from(a));
                <crate::fermented::types::platform_mobile::put::platform_mobile_put_InstantAssetLockProofFFI as ferment::FFIConversionTo<platform_mobile::put::InstantAssetLockProofFFI>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::put::topup_identity_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_topup_identity_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, identity: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity, asset_lock_proof: *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_AssetLockProofFFI, asset_lock_proof_private_key: *mut crate::fermented::generics::Vec_u8, is_testnet: bool) -> *mut crate::fermented::generics::Result_ok_u64_err_String {
                let obj = platform_mobile::put::topup_identity_sdk(rust_sdk, <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionFrom<dpp::identity::identity::Identity>>::ffi_from(identity), <crate::fermented::types::platform_mobile::put::platform_mobile_put_AssetLockProofFFI as ferment::FFIConversionFrom<platform_mobile::put::AssetLockProofFFI>>::ffi_from(asset_lock_proof), <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(asset_lock_proof_private_key), is_testnet);
                <crate::fermented::generics::Result_ok_u64_err_String as ferment::FFIConversionTo<Result<u64, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::put::put_identity_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_put_identity_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, identity: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity, asset_lock_proof: *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_AssetLockProofFFI, asset_lock_proof_private_key: *mut crate::fermented::generics::Vec_u8, signer_context: usize, signer_callback: u64, is_testnet: bool) -> *mut crate::fermented::generics::Result_ok_dpp_identity_identity_Identity_err_String {
                let obj = platform_mobile::put::put_identity_sdk(rust_sdk, <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionFrom<dpp::identity::identity::Identity>>::ffi_from(identity), <crate::fermented::types::platform_mobile::put::platform_mobile_put_AssetLockProofFFI as ferment::FFIConversionFrom<platform_mobile::put::AssetLockProofFFI>>::ffi_from(asset_lock_proof), <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(asset_lock_proof_private_key), signer_context, signer_callback, is_testnet);
                <crate::fermented::generics::Result_ok_dpp_identity_identity_Identity_err_String as ferment::FFIConversionTo<Result<dpp::identity::identity::Identity, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`ChainAssetLockProofFFI`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct platform_mobile_put_ChainAssetLockProofFFI {
                pub core_chain_locked_height: u32,
                pub out_point: *mut platform_mobile::custom::OutPoint,
            }

            impl ferment::FFIConversionFrom<platform_mobile::put::ChainAssetLockProofFFI> for platform_mobile_put_ChainAssetLockProofFFI {
                unsafe fn ffi_from_const(ffi: *const platform_mobile_put_ChainAssetLockProofFFI) -> platform_mobile::put::ChainAssetLockProofFFI {
                    let ffi_ref = &*ffi;
                    platform_mobile::put::ChainAssetLockProofFFI { core_chain_locked_height: ffi_ref.core_chain_locked_height, out_point: <platform_mobile::custom::OutPoint as ferment::FFIConversionFrom<dashcore::blockdata::transaction::outpoint::OutPoint>>::ffi_from(ffi_ref.out_point) }
                }
            }

            impl ferment::FFIConversionTo<platform_mobile::put::ChainAssetLockProofFFI> for platform_mobile_put_ChainAssetLockProofFFI { unsafe fn ffi_to_const(obj: platform_mobile::put::ChainAssetLockProofFFI) -> *const platform_mobile_put_ChainAssetLockProofFFI { ferment::boxed(platform_mobile_put_ChainAssetLockProofFFI { core_chain_locked_height: obj.core_chain_locked_height, out_point: <platform_mobile::custom::OutPoint as ferment::FFIConversionTo<dashcore::blockdata::transaction::outpoint::OutPoint>>::ffi_to(obj.out_point) }) } }

            impl Drop for platform_mobile_put_ChainAssetLockProofFFI {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ;
                        ferment::unbox_any(ffi_ref.out_point);
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_ChainAssetLockProofFFI_ctor<>(core_chain_locked_height: u32, out_point: *mut platform_mobile::custom::OutPoint) -> *mut platform_mobile_put_ChainAssetLockProofFFI { ferment::boxed(platform_mobile_put_ChainAssetLockProofFFI { core_chain_locked_height, out_point }) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_ChainAssetLockProofFFI_destroy<>(ffi: *mut platform_mobile_put_ChainAssetLockProofFFI) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_ChainAssetLockProofFFI_get_core_chain_locked_height<>(obj: *const platform_mobile_put_ChainAssetLockProofFFI) -> u32 { (*obj).core_chain_locked_height }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_ChainAssetLockProofFFI_get_out_point<>(obj: *const platform_mobile_put_ChainAssetLockProofFFI) -> *mut platform_mobile::custom::OutPoint { (*obj).out_point }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_ChainAssetLockProofFFI_set_core_chain_locked_height<>(obj: *const platform_mobile_put_ChainAssetLockProofFFI) -> u32 { (*obj).core_chain_locked_height }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_ChainAssetLockProofFFI_set_out_point<>(obj: *const platform_mobile_put_ChainAssetLockProofFFI) -> *mut platform_mobile::custom::OutPoint { (*obj).out_point }

            #[doc = "FFI-representation of the [`platform_mobile::put::put_identity_update_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_put_identity_update_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, identity: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity, master_public_key_id: *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID, add_public_keys: *mut crate::fermented::generics::Vec_dpp_identity_identity_public_key_IdentityPublicKey, disable_public_keys: *mut crate::fermented::generics::Vec_dpp_identity_identity_public_key_KeyID, signer_context: usize, signer_callback: u64) -> *mut crate::fermented::generics::Result_ok_dpp_identity_identity_Identity_err_String {
                let obj = platform_mobile::put::put_identity_update_sdk(rust_sdk, <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionFrom<dpp::identity::identity::Identity>>::ffi_from(identity), <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionFrom<dpp::identity::identity_public_key::KeyID>>::ffi_from(master_public_key_id), <crate::fermented::generics::Vec_dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<Vec<dpp::identity::identity_public_key::IdentityPublicKey>>>::ffi_from(add_public_keys), <crate::fermented::generics::Vec_dpp_identity_identity_public_key_KeyID as ferment::FFIConversionFrom<Vec<dpp::identity::identity_public_key::KeyID>>>::ffi_from(disable_public_keys), signer_context, signer_callback);
                <crate::fermented::generics::Result_ok_dpp_identity_identity_Identity_err_String as ferment::FFIConversionTo<Result<dpp::identity::identity::Identity, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::put::ChainAssetLockProofFFI_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_ChainAssetLockProofFFI_clone(a: *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI) -> *mut crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI {
                let obj = platform_mobile::put::ChainAssetLockProofFFI_clone(<crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI as ferment::FFIConversionFrom<platform_mobile::put::ChainAssetLockProofFFI>>::ffi_from(a));
                <crate::fermented::types::platform_mobile::put::platform_mobile_put_ChainAssetLockProofFFI as ferment::FFIConversionTo<platform_mobile::put::ChainAssetLockProofFFI>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::put::OutPointFFI_clone`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_OutPointFFI_clone(a: *mut platform_mobile::custom::OutPoint) -> *mut platform_mobile::custom::OutPoint {
                let obj = platform_mobile::put::OutPointFFI_clone(<platform_mobile::custom::OutPoint as ferment::FFIConversionFrom<dashcore::blockdata::transaction::outpoint::OutPoint>>::ffi_from(a));
                <platform_mobile::custom::OutPoint as ferment::FFIConversionTo<dashcore::blockdata::transaction::outpoint::OutPoint>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`OutPointFFI`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct platform_mobile_put_OutPointFFI {
                pub txid: *mut crate::fermented::generics::Arr_u8_32,
                pub vout: u32,
            }

            impl ferment::FFIConversionFrom<platform_mobile::put::OutPointFFI> for platform_mobile_put_OutPointFFI {
                unsafe fn ffi_from_const(ffi: *const platform_mobile_put_OutPointFFI) -> platform_mobile::put::OutPointFFI {
                    let ffi_ref = &*ffi;
                    platform_mobile::put::OutPointFFI { txid: <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<[u8; 32]>>::ffi_from(ffi_ref.txid), vout: ffi_ref.vout }
                }
            }

            impl ferment::FFIConversionTo<platform_mobile::put::OutPointFFI> for platform_mobile_put_OutPointFFI { unsafe fn ffi_to_const(obj: platform_mobile::put::OutPointFFI) -> *const platform_mobile_put_OutPointFFI { ferment::boxed(platform_mobile_put_OutPointFFI { txid: <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionTo<[u8; 32]>>::ffi_to(obj.txid), vout: obj.vout }) } }

            impl Drop for platform_mobile_put_OutPointFFI {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.txid);
                        ;
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_OutPointFFI_ctor<>(txid: *mut crate::fermented::generics::Arr_u8_32, vout: u32) -> *mut platform_mobile_put_OutPointFFI { ferment::boxed(platform_mobile_put_OutPointFFI { txid, vout }) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_OutPointFFI_destroy<>(ffi: *mut platform_mobile_put_OutPointFFI) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_OutPointFFI_get_txid<>(obj: *const platform_mobile_put_OutPointFFI) -> *mut crate::fermented::generics::Arr_u8_32 { (*obj).txid }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_OutPointFFI_get_vout<>(obj: *const platform_mobile_put_OutPointFFI) -> u32 { (*obj).vout }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_OutPointFFI_set_txid<>(obj: *const platform_mobile_put_OutPointFFI) -> *mut crate::fermented::generics::Arr_u8_32 { (*obj).txid }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_OutPointFFI_set_vout<>(obj: *const platform_mobile_put_OutPointFFI) -> u32 { (*obj).vout }

            #[doc = "FFI-representation of the [`InstantAssetLockProofFFI`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct platform_mobile_put_InstantAssetLockProofFFI {
                pub instant_lock: *mut crate::fermented::generics::Vec_u8,
                pub transaction: *mut crate::fermented::generics::Vec_u8,
                pub output_index: u32,
            }

            impl ferment::FFIConversionFrom<platform_mobile::put::InstantAssetLockProofFFI> for platform_mobile_put_InstantAssetLockProofFFI {
                unsafe fn ffi_from_const(ffi: *const platform_mobile_put_InstantAssetLockProofFFI) -> platform_mobile::put::InstantAssetLockProofFFI {
                    let ffi_ref = &*ffi;
                    platform_mobile::put::InstantAssetLockProofFFI { instant_lock: <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(ffi_ref.instant_lock), transaction: <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(ffi_ref.transaction), output_index: ffi_ref.output_index }
                }
            }

            impl ferment::FFIConversionTo<platform_mobile::put::InstantAssetLockProofFFI> for platform_mobile_put_InstantAssetLockProofFFI { unsafe fn ffi_to_const(obj: platform_mobile::put::InstantAssetLockProofFFI) -> *const platform_mobile_put_InstantAssetLockProofFFI { ferment::boxed(platform_mobile_put_InstantAssetLockProofFFI { instant_lock: <crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(obj.instant_lock), transaction: <crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(obj.transaction), output_index: obj.output_index }) } }

            impl Drop for platform_mobile_put_InstantAssetLockProofFFI {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.instant_lock);
                        ferment::unbox_any(ffi_ref.transaction);
                        ;
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_ctor<>(instant_lock: *mut crate::fermented::generics::Vec_u8, transaction: *mut crate::fermented::generics::Vec_u8, output_index: u32) -> *mut platform_mobile_put_InstantAssetLockProofFFI { ferment::boxed(platform_mobile_put_InstantAssetLockProofFFI { instant_lock, transaction, output_index }) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_destroy<>(ffi: *mut platform_mobile_put_InstantAssetLockProofFFI) { ferment::unbox_any(ffi); }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_get_instant_lock<>(obj: *const platform_mobile_put_InstantAssetLockProofFFI) -> *mut crate::fermented::generics::Vec_u8 { (*obj).instant_lock }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_get_transaction<>(obj: *const platform_mobile_put_InstantAssetLockProofFFI) -> *mut crate::fermented::generics::Vec_u8 { (*obj).transaction }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_get_output_index<>(obj: *const platform_mobile_put_InstantAssetLockProofFFI) -> u32 { (*obj).output_index }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_set_instant_lock<>(obj: *const platform_mobile_put_InstantAssetLockProofFFI) -> *mut crate::fermented::generics::Vec_u8 { (*obj).instant_lock }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_set_transaction<>(obj: *const platform_mobile_put_InstantAssetLockProofFFI) -> *mut crate::fermented::generics::Vec_u8 { (*obj).transaction }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_put_InstantAssetLockProofFFI_set_output_index<>(obj: *const platform_mobile_put_InstantAssetLockProofFFI) -> u32 { (*obj).output_index }
        }

        pub mod fetch_document {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`StartPoint`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum platform_mobile_fetch_document_StartPoint { StartAfter(*mut crate::fermented::generics::Vec_u8), StartAt(*mut crate::fermented::generics::Vec_u8) }

            impl ferment::FFIConversionFrom<platform_mobile::fetch_document::StartPoint> for platform_mobile_fetch_document_StartPoint {
                unsafe fn ffi_from_const(ffi: *const platform_mobile_fetch_document_StartPoint) -> platform_mobile::fetch_document::StartPoint {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        platform_mobile_fetch_document_StartPoint::StartAfter(o_0) => platform_mobile::fetch_document::StartPoint::StartAfter(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(*o_0)),
                        platform_mobile_fetch_document_StartPoint::StartAt(o_0) => platform_mobile::fetch_document::StartPoint::StartAt(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(*o_0))
                    }
                }
            }

            impl ferment::FFIConversionTo<platform_mobile::fetch_document::StartPoint> for platform_mobile_fetch_document_StartPoint {
                unsafe fn ffi_to_const(obj: platform_mobile::fetch_document::StartPoint) -> *const platform_mobile_fetch_document_StartPoint {
                    ferment::boxed(match obj {
                        platform_mobile::fetch_document::StartPoint::StartAfter(o_0) => platform_mobile_fetch_document_StartPoint::StartAfter(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(o_0)),
                        platform_mobile::fetch_document::StartPoint::StartAt(o_0) => platform_mobile_fetch_document_StartPoint::StartAt(<crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(o_0)),
                        _ => unreachable!("This is unreachable")
                    })
                }
            }

            impl Drop for platform_mobile_fetch_document_StartPoint {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            platform_mobile_fetch_document_StartPoint::StartAfter(o_0) => { ferment::unbox_any(*o_0); }
                            platform_mobile_fetch_document_StartPoint::StartAt(o_0) => { ferment::unbox_any(*o_0); }
                            _ => unreachable!("This is unreachable")
                        };
                    }
                }
            }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_document_StartPoint_StartAfter_ctor(o_o_0: *mut crate::fermented::generics::Vec_u8) -> *mut platform_mobile_fetch_document_StartPoint { ferment::boxed(platform_mobile_fetch_document_StartPoint::StartAfter(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_document_StartPoint_StartAt_ctor(o_o_0: *mut crate::fermented::generics::Vec_u8) -> *mut platform_mobile_fetch_document_StartPoint { ferment::boxed(platform_mobile_fetch_document_StartPoint::StartAt(o_o_0)) }

            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_document_StartPoint_destroy<>(ffi: *mut platform_mobile_fetch_document_StartPoint) { ferment::unbox_any(ffi); }

            #[doc = "FFI-representation of the [`platform_mobile::fetch_document::document_to_string`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_document_document_to_string(document: *mut crate::fermented::types::dpp::document::dpp_document_Document) -> *mut std::os::raw::c_char {
                let obj = platform_mobile::fetch_document::document_to_string(<crate::fermented::types::dpp::document::dpp_document_Document as ferment::FFIConversionFrom<dpp::document::Document>>::ffi_from(document));
                <std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::fetch_document::fetch_documents_with_query_and_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_document_fetch_documents_with_query_and_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, data_contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, document_type: *mut std::os::raw::c_char, where_clauses: *mut crate::fermented::generics::Vec_drive_query_conditions_WhereClause, order_clauses: *mut crate::fermented::generics::Vec_drive_query_ordering_OrderClause, limit: u32, start: *mut crate::fermented::types::platform_mobile::fetch_document::platform_mobile_fetch_document_StartPoint) -> *mut crate::fermented::generics::Result_ok_Vec_dpp_document_Document_err_String {
                let obj = platform_mobile::fetch_document::fetch_documents_with_query_and_sdk(rust_sdk, <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(data_contract_id), <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(document_type), <crate::fermented::generics::Vec_drive_query_conditions_WhereClause as ferment::FFIConversionFrom<Vec<drive::query::conditions::WhereClause>>>::ffi_from(where_clauses), <crate::fermented::generics::Vec_drive_query_ordering_OrderClause as ferment::FFIConversionFrom<Vec<drive::query::ordering::OrderClause>>>::ffi_from(order_clauses), limit, <crate::fermented::types::platform_mobile::fetch_document::platform_mobile_fetch_document_StartPoint as ferment::FFIConversionFrom<platform_mobile::fetch_document::StartPoint>>::ffi_from_opt(start));
                <crate::fermented::generics::Result_ok_Vec_dpp_document_Document_err_String as ferment::FFIConversionTo<Result<Vec<dpp::document::Document>, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::fetch_document::deserialize_document_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_document_deserialize_document_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, bytes: *mut crate::fermented::generics::Vec_u8, data_contract_id: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, document_type: *mut std::os::raw::c_char) -> *mut crate::fermented::generics::Result_ok_dpp_document_Document_err_String {
                let obj = platform_mobile::fetch_document::deserialize_document_sdk(rust_sdk, <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(bytes), <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(data_contract_id), <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(document_type));
                <crate::fermented::generics::Result_ok_dpp_document_Document_err_String as ferment::FFIConversionTo<Result<dpp::document::Document, String>>>::ffi_to(obj)
            }
        }

        pub mod config {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::config::testnet_address_list`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_config_testnet_address_list() -> *mut crate::fermented::generics::Vec_String {
                let obj = platform_mobile::config::testnet_address_list();
                <crate::fermented::generics::Vec_String as ferment::FFIConversionTo<Vec<String>>>::ffi_to(obj)
            }
        }

        pub mod fetch_identity {
            use crate as dash_sdk_bindings;

            #[doc = "FFI-representation of the [`platform_mobile::fetch_identity::fetch_identity_with_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_identity_fetch_identity_with_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, identifier: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::generics::Result_ok_Option_dpp_identity_identity_Identity_err_String {
                let obj = platform_mobile::fetch_identity::fetch_identity_with_sdk(rust_sdk, <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(identifier));
                <crate::fermented::generics::Result_ok_Option_dpp_identity_identity_Identity_err_String as ferment::FFIConversionTo<Result<Option<dpp::identity::identity::Identity>, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::fetch_identity::fetch_identity_with_keyhash_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_identity_fetch_identity_with_keyhash_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, key_hash: *mut crate::fermented::generics::Arr_u8_20) -> *mut crate::fermented::generics::Result_ok_dpp_identity_identity_Identity_err_String {
                let obj = platform_mobile::fetch_identity::fetch_identity_with_keyhash_sdk(rust_sdk, <crate::fermented::generics::Arr_u8_20 as ferment::FFIConversionFrom<[u8; 20]>>::ffi_from(key_hash));
                <crate::fermented::generics::Result_ok_dpp_identity_identity_Identity_err_String as ferment::FFIConversionTo<Result<dpp::identity::identity::Identity, String>>>::ffi_to(obj)
            }

            #[doc = "FFI-representation of the [`platform_mobile::fetch_identity::fetch_identity_balance_with_sdk`]"]
            #[no_mangle]
            pub unsafe extern "C" fn platform_mobile_fetch_identity_fetch_identity_balance_with_sdk(rust_sdk: *mut platform_mobile::sdk::DashSdk, identifier: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut crate::fermented::generics::Result_ok_u64_err_String {
                let obj = platform_mobile::fetch_identity::fetch_identity_balance_with_sdk(rust_sdk, <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(identifier));
                <crate::fermented::generics::Result_ok_u64_err_String as ferment::FFIConversionTo<Result<u64, String>>>::ffi_to(obj)
            }
        }
    }

    pub mod dapi_grpc {
        use crate as dash_sdk_bindings;

        #[cfg(feature = "core")]
        pub mod core {
            use crate as dash_sdk_bindings;
        }

        pub mod mock { use crate as dash_sdk_bindings; }

        #[cfg(feature = "serde")]
        pub mod deserialization {
            use crate as dash_sdk_bindings;
        }

        #[cfg(feature = "drive")]
        pub mod drive {
            use crate as dash_sdk_bindings;
        }

        #[cfg(feature = "platform")]
        pub mod platform {
            use crate as dash_sdk_bindings;
        }
    }
}

#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unreachable_patterns,
    unused_braces,
    unused_imports,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables
)]
pub mod generics {
    use crate as dash_sdk_bindings;

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String {
        pub ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_VotePollsGroupedByTimestamp,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<drive_proof_verifier::types::VotePollsGroupedByTimestamp, String>> for Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String) -> Result<drive_proof_verifier::types::VotePollsGroupedByTimestamp, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_VotePollsGroupedByTimestamp as ferment::FFIConversionFrom<drive_proof_verifier::types::VotePollsGroupedByTimestamp>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<drive_proof_verifier::types::VotePollsGroupedByTimestamp, String>> for Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String {
        unsafe fn ffi_to_const(obj: Result<drive_proof_verifier::types::VotePollsGroupedByTimestamp, String>) -> *const Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String_ctor(ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_VotePollsGroupedByTimestamp, error: *mut std::os::raw::c_char) -> *mut Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String { ferment::boxed(Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String_destroy(ffi: *mut Result_ok_drive_proof_verifier_types_VotePollsGroupedByTimestamp_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_dpp_voting_votes_Vote_err_String {
        pub ok: *mut crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<dpp::voting::votes::Vote, String>> for Result_ok_dpp_voting_votes_Vote_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_dpp_voting_votes_Vote_err_String) -> Result<dpp::voting::votes::Vote, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote as ferment::FFIConversionFrom<dpp::voting::votes::Vote>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<dpp::voting::votes::Vote, String>> for Result_ok_dpp_voting_votes_Vote_err_String {
        unsafe fn ffi_to_const(obj: Result<dpp::voting::votes::Vote, String>) -> *const Result_ok_dpp_voting_votes_Vote_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_dpp_voting_votes_Vote_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_voting_votes_Vote_err_String_ctor(ok: *mut crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote, error: *mut std::os::raw::c_char) -> *mut Result_ok_dpp_voting_votes_Vote_err_String { ferment::boxed(Result_ok_dpp_voting_votes_Vote_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_voting_votes_Vote_err_String_destroy(ffi: *mut Result_ok_dpp_voting_votes_Vote_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Tuple_platform_value_Value_platform_value_Value {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Tuple_platform_value_Value_platform_value_Value,
    }

    impl ferment::FFIConversionFrom<Vec<(platform_value::Value, platform_value::Value)>> for Vec_Tuple_platform_value_Value_platform_value_Value {
        unsafe fn ffi_from_const(ffi: *const Vec_Tuple_platform_value_Value_platform_value_Value) -> Vec<(platform_value::Value, platform_value::Value)> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<(platform_value::Value, platform_value::Value)>> for Vec_Tuple_platform_value_Value_platform_value_Value { unsafe fn ffi_to_const(obj: Vec<(platform_value::Value, platform_value::Value)>) -> *const Vec_Tuple_platform_value_Value_platform_value_Value { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_Tuple_platform_value_Value_platform_value_Value { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_Tuple_platform_value_Value_platform_value_Value_ctor(count: usize, values: *mut *mut crate::fermented::generics::Tuple_platform_value_Value_platform_value_Value) -> *mut Vec_Tuple_platform_value_Value_platform_value_Value { ferment::boxed(Vec_Tuple_platform_value_Value_platform_value_Value { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_Tuple_platform_value_Value_platform_value_Value_destroy(ffi: *mut Vec_Tuple_platform_value_Value_platform_value_Value) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_dpp_document_Document_err_String {
        pub ok: *mut crate::fermented::types::dpp::document::dpp_document_Document,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<dpp::document::Document, String>> for Result_ok_dpp_document_Document_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_dpp_document_Document_err_String) -> Result<dpp::document::Document, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::dpp::document::dpp_document_Document as ferment::FFIConversionFrom<dpp::document::Document>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<dpp::document::Document, String>> for Result_ok_dpp_document_Document_err_String {
        unsafe fn ffi_to_const(obj: Result<dpp::document::Document, String>) -> *const Result_ok_dpp_document_Document_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_dpp_document_Document_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_document_Document_err_String_ctor(ok: *mut crate::fermented::types::dpp::document::dpp_document_Document, error: *mut std::os::raw::c_char) -> *mut Result_ok_dpp_document_Document_err_String { ferment::boxed(Result_ok_dpp_document_Document_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_document_Document_err_String_destroy(ffi: *mut Result_ok_dpp_document_Document_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo {
        pub o_0: *mut crate::fermented::types::dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo,
        pub o_1: *mut crate::fermented::types::dpp::block::block_info::dpp_block_block_info_BlockInfo,
    }

    impl ferment::FFIConversionFrom<(dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo, dpp::block::block_info::BlockInfo)> for Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo {
        unsafe fn ffi_from_const(ffi: *const Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo) -> (dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo, dpp::block::block_info::BlockInfo) {
            let ffi_ref = &*ffi;
            (ferment::FFIConversionFrom::ffi_from(ffi_ref.o_0), ferment::FFIConversionFrom::ffi_from(ffi_ref.o_1))
        }
    }

    impl ferment::FFIConversionTo<(dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo, dpp::block::block_info::BlockInfo)> for Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo { unsafe fn ffi_to_const(obj: (dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::ContestedDocumentVotePollWinnerInfo, dpp::block::block_info::BlockInfo)) -> *const Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo { ferment::boxed(Self { o_0: ferment::FFIConversionTo::ffi_to(obj.0), o_1: ferment::FFIConversionTo::ffi_to(obj.1) }) } }

    impl Drop for Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.o_0);
                ferment::unbox_any(self.o_1);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo_ctor(o_0: *mut crate::fermented::types::dpp::voting::vote_info_storage::contested_document_vote_poll_winner_info::dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo, o_1: *mut crate::fermented::types::dpp::block::block_info::dpp_block_block_info_BlockInfo) -> *mut Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo { ferment::boxed(Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo { o_0, o_1 }) }

    #[no_mangle]
    pub unsafe extern "C" fn Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo_destroy(ffi: *mut Tuple_dpp_voting_vote_info_storage_contested_document_vote_poll_winner_info_ContestedDocumentVotePollWinnerInfo_dpp_block_block_info_BlockInfo) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dpp_document_Document {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::dpp::document::dpp_document_Document,
    }

    impl ferment::FFIConversionFrom<Vec<dpp::document::Document>> for Vec_dpp_document_Document {
        unsafe fn ffi_from_const(ffi: *const Vec_dpp_document_Document) -> Vec<dpp::document::Document> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<dpp::document::Document>> for Vec_dpp_document_Document { unsafe fn ffi_to_const(obj: Vec<dpp::document::Document>) -> *const Vec_dpp_document_Document { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_dpp_document_Document { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_document_Document_ctor(count: usize, values: *mut *mut crate::fermented::types::dpp::document::dpp_document_Document) -> *mut Vec_dpp_document_Document { ferment::boxed(Vec_dpp_document_Document { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_document_Document_destroy(ffi: *mut Vec_dpp_document_Document) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_dpp_identity_identity_Identity_err_String {
        pub ok: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<Option<dpp::identity::identity::Identity>, String>> for Result_ok_Option_dpp_identity_identity_Identity_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_Option_dpp_identity_identity_Identity_err_String) -> Result<Option<dpp::identity::identity::Identity>, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionFrom<dpp::identity::identity::Identity>>::ffi_from_opt(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<Option<dpp::identity::identity::Identity>, String>> for Result_ok_Option_dpp_identity_identity_Identity_err_String {
        unsafe fn ffi_to_const(obj: Result<Option<dpp::identity::identity::Identity>, String>) -> *const Result_ok_Option_dpp_identity_identity_Identity_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to_opt(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_Option_dpp_identity_identity_Identity_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_dpp_identity_identity_Identity_err_String_ctor(ok: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity, error: *mut std::os::raw::c_char) -> *mut Result_ok_Option_dpp_identity_identity_Identity_err_String { ferment::boxed(Result_ok_Option_dpp_identity_identity_Identity_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_dpp_identity_identity_Identity_err_String_destroy(ffi: *mut Result_ok_Option_dpp_identity_identity_Identity_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_String_values_platform_value_Value {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values: *mut *mut crate::fermented::types::platform_value::platform_value_Value,
    }

    impl ferment::FFIConversionFrom<std::collections::BTreeMap<String, platform_value::Value>> for std_collections_Map_keys_String_values_platform_value_Value {
        unsafe fn ffi_from_const(ffi: *const std_collections_Map_keys_String_values_platform_value_Value) -> std::collections::BTreeMap<String, platform_value::Value> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o), |o| <crate::fermented::types::platform_value::platform_value_Value as ferment::FFIConversionFrom<platform_value::Value>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<std::collections::BTreeMap<String, platform_value::Value>> for std_collections_Map_keys_String_values_platform_value_Value { unsafe fn ffi_to_const(obj: std::collections::BTreeMap<String, platform_value::Value>) -> *const std_collections_Map_keys_String_values_platform_value_Value { ferment::boxed(Self { count: obj.len(), keys: ferment::to_complex_group(obj.keys().cloned()), values: ferment::to_complex_group(obj.values().cloned()) }) } }

    impl Drop for std_collections_Map_keys_String_values_platform_value_Value {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr_composer(self.keys, self.count, ferment::unbox_string);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_platform_value_Value_ctor(count: usize, keys: *mut *mut std::os::raw::c_char, values: *mut *mut crate::fermented::types::platform_value::platform_value_Value) -> *mut std_collections_Map_keys_String_values_platform_value_Value { ferment::boxed(std_collections_Map_keys_String_values_platform_value_Value { count, keys, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_platform_value_Value_destroy(ffi: *mut std_collections_Map_keys_String_values_platform_value_Value) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll,
    }

    impl ferment::FFIConversionFrom<Vec<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)>> for Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll {
        unsafe fn ffi_from_const(ffi: *const Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll) -> Vec<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)>> for Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { unsafe fn ffi_to_const(obj: Vec<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)>) -> *const Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_ctor(count: usize, values: *mut *mut crate::fermented::generics::Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll) -> *mut Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { ferment::boxed(Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_destroy(ffi: *mut Vec_Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll {
        pub o_0: *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis,
        pub o_1: *mut crate::fermented::generics::Vec_dpp_voting_vote_polls_VotePoll,
    }

    impl ferment::FFIConversionFrom<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)> for Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll {
        unsafe fn ffi_from_const(ffi: *const Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll) -> (dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>) {
            let ffi_ref = &*ffi;
            (ferment::FFIConversionFrom::ffi_from(ffi_ref.o_0), ferment::FFIConversionFrom::ffi_from(ffi_ref.o_1))
        }
    }

    impl ferment::FFIConversionTo<(dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)> for Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { unsafe fn ffi_to_const(obj: (dpp::prelude::TimestampMillis, Vec<dpp::voting::vote_polls::VotePoll>)) -> *const Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { ferment::boxed(Self { o_0: ferment::FFIConversionTo::ffi_to(obj.0), o_1: ferment::FFIConversionTo::ffi_to(obj.1) }) } }

    impl Drop for Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.o_0);
                ferment::unbox_any(self.o_1);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_ctor(o_0: *mut crate::fermented::types::dpp::prelude::dpp_prelude_TimestampMillis, o_1: *mut crate::fermented::generics::Vec_dpp_voting_vote_polls_VotePoll) -> *mut Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { ferment::boxed(Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll { o_0, o_1 }) }

    #[no_mangle]
    pub unsafe extern "C" fn Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll_destroy(ffi: *mut Tuple_dpp_prelude_TimestampMillis_Vec_dpp_voting_vote_polls_VotePoll) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Vec_u8_err_String {
        pub ok: *mut crate::fermented::generics::Vec_u8,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<Vec<u8>, String>> for Result_ok_Vec_u8_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_Vec_u8_err_String) -> Result<Vec<u8>, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<Vec<u8>, String>> for Result_ok_Vec_u8_err_String {
        unsafe fn ffi_to_const(obj: Result<Vec<u8>, String>) -> *const Result_ok_Vec_u8_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_Vec_u8_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Vec_u8_err_String_ctor(ok: *mut crate::fermented::generics::Vec_u8, error: *mut std::os::raw::c_char) -> *mut Result_ok_Vec_u8_err_String { ferment::boxed(Result_ok_Vec_u8_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Vec_u8_err_String_destroy(ffi: *mut Result_ok_Vec_u8_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_dpp_voting_votes_Vote_err_String {
        pub ok: *mut crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<Option<dpp::voting::votes::Vote>, String>> for Result_ok_Option_dpp_voting_votes_Vote_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_Option_dpp_voting_votes_Vote_err_String) -> Result<Option<dpp::voting::votes::Vote>, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote as ferment::FFIConversionFrom<dpp::voting::votes::Vote>>::ffi_from_opt(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<Option<dpp::voting::votes::Vote>, String>> for Result_ok_Option_dpp_voting_votes_Vote_err_String {
        unsafe fn ffi_to_const(obj: Result<Option<dpp::voting::votes::Vote>, String>) -> *const Result_ok_Option_dpp_voting_votes_Vote_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to_opt(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_Option_dpp_voting_votes_Vote_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_dpp_voting_votes_Vote_err_String_ctor(ok: *mut crate::fermented::types::dpp::voting::votes::dpp_voting_votes_Vote, error: *mut std::os::raw::c_char) -> *mut Result_ok_Option_dpp_voting_votes_Vote_err_String { ferment::boxed(Result_ok_Option_dpp_voting_votes_Vote_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_dpp_voting_votes_Vote_err_String_destroy(ffi: *mut Result_ok_Option_dpp_voting_votes_Vote_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Vec_dpp_document_Document_err_String {
        pub ok: *mut crate::fermented::generics::Vec_dpp_document_Document,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<Vec<dpp::document::Document>, String>> for Result_ok_Vec_dpp_document_Document_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_Vec_dpp_document_Document_err_String) -> Result<Vec<dpp::document::Document>, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::generics::Vec_dpp_document_Document as ferment::FFIConversionFrom<Vec<dpp::document::Document>>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<Vec<dpp::document::Document>, String>> for Result_ok_Vec_dpp_document_Document_err_String {
        unsafe fn ffi_to_const(obj: Result<Vec<dpp::document::Document>, String>) -> *const Result_ok_Vec_dpp_document_Document_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_Vec_dpp_document_Document_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Vec_dpp_document_Document_err_String_ctor(ok: *mut crate::fermented::generics::Vec_dpp_document_Document, error: *mut std::os::raw::c_char) -> *mut Result_ok_Vec_dpp_document_Document_err_String { ferment::boxed(Result_ok_Vec_dpp_document_Document_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Vec_dpp_document_Document_err_String_destroy(ffi: *mut Result_ok_Vec_dpp_document_Document_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut u8,
    }

    impl ferment::FFIConversionFrom<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
            let ffi_ref = &*ffi;
            ferment::from_primitive_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<u8>> for Vec_u8 { unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8 { ferment::boxed(Self { count: obj.len(), values: ferment::to_primitive_group(obj.into_iter()) }) } }

    impl Drop for Vec_u8 { fn drop(&mut self) { unsafe { ferment::unbox_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(count: usize, values: *mut u8) -> *mut Vec_u8 { ferment::boxed(Vec_u8 { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_platform_value_Value {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::platform_value::platform_value_Value,
    }

    impl ferment::FFIConversionFrom<Vec<platform_value::Value>> for Vec_platform_value_Value {
        unsafe fn ffi_from_const(ffi: *const Vec_platform_value_Value) -> Vec<platform_value::Value> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<platform_value::Value>> for Vec_platform_value_Value { unsafe fn ffi_to_const(obj: Vec<platform_value::Value>) -> *const Vec_platform_value_Value { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_platform_value_Value { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_platform_value_Value_ctor(count: usize, values: *mut *mut crate::fermented::types::platform_value::platform_value_Value) -> *mut Vec_platform_value_Value { ferment::boxed(Vec_platform_value_Value { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_platform_value_Value_destroy(ffi: *mut Vec_platform_value_Value) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_dpp_identity_identity_Identity_err_String {
        pub ok: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<dpp::identity::identity::Identity, String>> for Result_ok_dpp_identity_identity_Identity_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_dpp_identity_identity_Identity_err_String) -> Result<dpp::identity::identity::Identity, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity as ferment::FFIConversionFrom<dpp::identity::identity::Identity>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<dpp::identity::identity::Identity, String>> for Result_ok_dpp_identity_identity_Identity_err_String {
        unsafe fn ffi_to_const(obj: Result<dpp::identity::identity::Identity, String>) -> *const Result_ok_dpp_identity_identity_Identity_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_dpp_identity_identity_Identity_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_identity_identity_Identity_err_String_ctor(ok: *mut crate::fermented::types::dpp::identity::identity::dpp_identity_identity_Identity, error: *mut std::os::raw::c_char) -> *mut Result_ok_dpp_identity_identity_Identity_err_String { ferment::boxed(Result_ok_dpp_identity_identity_Identity_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_identity_identity_Identity_err_String_destroy(ffi: *mut Result_ok_dpp_identity_identity_Identity_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dpp_identity_identity_public_key_IdentityPublicKey {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey,
    }

    impl ferment::FFIConversionFrom<Vec<dpp::identity::identity_public_key::IdentityPublicKey>> for Vec_dpp_identity_identity_public_key_IdentityPublicKey {
        unsafe fn ffi_from_const(ffi: *const Vec_dpp_identity_identity_public_key_IdentityPublicKey) -> Vec<dpp::identity::identity_public_key::IdentityPublicKey> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<dpp::identity::identity_public_key::IdentityPublicKey>> for Vec_dpp_identity_identity_public_key_IdentityPublicKey { unsafe fn ffi_to_const(obj: Vec<dpp::identity::identity_public_key::IdentityPublicKey>) -> *const Vec_dpp_identity_identity_public_key_IdentityPublicKey { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_dpp_identity_identity_public_key_IdentityPublicKey { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_identity_identity_public_key_IdentityPublicKey_ctor(count: usize, values: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey) -> *mut Vec_dpp_identity_identity_public_key_IdentityPublicKey { ferment::boxed(Vec_dpp_identity_identity_public_key_IdentityPublicKey { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_identity_identity_public_key_IdentityPublicKey_destroy(ffi: *mut Vec_dpp_identity_identity_public_key_IdentityPublicKey) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_drive_query_ordering_OrderClause {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::drive::query::ordering::drive_query_ordering_OrderClause,
    }

    impl ferment::FFIConversionFrom<Vec<drive::query::ordering::OrderClause>> for Vec_drive_query_ordering_OrderClause {
        unsafe fn ffi_from_const(ffi: *const Vec_drive_query_ordering_OrderClause) -> Vec<drive::query::ordering::OrderClause> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<drive::query::ordering::OrderClause>> for Vec_drive_query_ordering_OrderClause { unsafe fn ffi_to_const(obj: Vec<drive::query::ordering::OrderClause>) -> *const Vec_drive_query_ordering_OrderClause { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_drive_query_ordering_OrderClause { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_drive_query_ordering_OrderClause_ctor(count: usize, values: *mut *mut crate::fermented::types::drive::query::ordering::drive_query_ordering_OrderClause) -> *mut Vec_drive_query_ordering_OrderClause { ferment::boxed(Vec_drive_query_ordering_OrderClause { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_drive_query_ordering_OrderClause_destroy(ffi: *mut Vec_drive_query_ordering_OrderClause) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String {
        pub ok: *mut crate::fermented::types::platform_mobile::data_contracts::platform_mobile_data_contracts_DataContractFFI,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<Option<platform_mobile::data_contracts::DataContractFFI>, String>> for Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String) -> Result<Option<platform_mobile::data_contracts::DataContractFFI>, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::platform_mobile::data_contracts::platform_mobile_data_contracts_DataContractFFI as ferment::FFIConversionFrom<platform_mobile::data_contracts::DataContractFFI>>::ffi_from_opt(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<Option<platform_mobile::data_contracts::DataContractFFI>, String>> for Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String {
        unsafe fn ffi_to_const(obj: Result<Option<platform_mobile::data_contracts::DataContractFFI>, String>) -> *const Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to_opt(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String_ctor(ok: *mut crate::fermented::types::platform_mobile::data_contracts::platform_mobile_data_contracts_DataContractFFI, error: *mut std::os::raw::c_char) -> *mut Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String { ferment::boxed(Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String_destroy(ffi: *mut Result_ok_Option_platform_mobile_data_contracts_DataContractFFI_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
        pub values: *mut *mut crate::fermented::types::dpp::voting::contender_structs::contender::dpp_voting_contender_structs_contender_ContenderWithSerializedDocument,
    }

    impl ferment::FFIConversionFrom<std::collections::BTreeMap<platform_value::types::identifier::Identifier, dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>> for std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
        unsafe fn ffi_from_const(ffi: *const std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument) -> std::collections::BTreeMap<platform_value::types::identifier::Identifier, dpp::voting::contender_structs::contender::ContenderWithSerializedDocument> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, |o| <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(o), |o| <crate::fermented::types::dpp::voting::contender_structs::contender::dpp_voting_contender_structs_contender_ContenderWithSerializedDocument as ferment::FFIConversionFrom<dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<std::collections::BTreeMap<platform_value::types::identifier::Identifier, dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>> for std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { unsafe fn ffi_to_const(obj: std::collections::BTreeMap<platform_value::types::identifier::Identifier, dpp::voting::contender_structs::contender::ContenderWithSerializedDocument>) -> *const std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { ferment::boxed(Self { count: obj.len(), keys: ferment::to_complex_group(obj.keys().cloned()), values: ferment::to_complex_group(obj.values().cloned()) }) } }

    impl Drop for std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument_ctor(count: usize, keys: *mut *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, values: *mut *mut crate::fermented::types::dpp::voting::contender_structs::contender::dpp_voting_contender_structs_contender_ContenderWithSerializedDocument) -> *mut std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { ferment::boxed(std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument { count, keys, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument_destroy(ffi: *mut std_collections_Map_keys_platform_value_types_identifier_Identifier_values_dpp_voting_contender_structs_contender_ContenderWithSerializedDocument) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String {
        pub ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ResourceVotesByIdentity,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<drive_proof_verifier::types::ResourceVotesByIdentity, String>> for Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String) -> Result<drive_proof_verifier::types::ResourceVotesByIdentity, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ResourceVotesByIdentity as ferment::FFIConversionFrom<drive_proof_verifier::types::ResourceVotesByIdentity>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<drive_proof_verifier::types::ResourceVotesByIdentity, String>> for Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String {
        unsafe fn ffi_to_const(obj: Result<drive_proof_verifier::types::ResourceVotesByIdentity, String>) -> *const Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String_ctor(ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ResourceVotesByIdentity, error: *mut std::os::raw::c_char) -> *mut Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String { ferment::boxed(Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String_destroy(ffi: *mut Result_ok_drive_proof_verifier_types_ResourceVotesByIdentity_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Vec<String>> for Vec_String {
        unsafe fn ffi_from_const(ffi: *const Vec_String) -> Vec<String> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<String>> for Vec_String { unsafe fn ffi_to_const(obj: Vec<String>) -> *const Vec_String { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_String { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr_composer(self.values, self.count, ferment::unbox_string); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_String_ctor(count: usize, values: *mut *mut std::os::raw::c_char) -> *mut Vec_String { ferment::boxed(Vec_String { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_String_destroy(ffi: *mut Vec_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_2 {
        pub count: usize,
        pub values: *mut u8,
    }

    impl ferment::FFIConversionFrom<[u8; 2]> for Arr_u8_2 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_2) -> [u8; 2] {
            let ffi_ref = &*ffi;
            let vec: Vec<u8> = ferment::from_primitive_group(ffi_ref.values, ffi_ref.count);
            vec.try_into().unwrap()
        }
    }

    impl ferment::FFIConversionTo<[u8; 2]> for Arr_u8_2 { unsafe fn ffi_to_const(obj: [u8; 2]) -> *const Arr_u8_2 { ferment::boxed(Self { count: obj.len(), values: ferment::to_primitive_group(obj.into_iter()) }) } }

    impl Drop for Arr_u8_2 { fn drop(&mut self) { unsafe { ferment::unbox_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_2_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_2 { ferment::boxed(Arr_u8_2 { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_2_destroy(ffi: *mut Arr_u8_2) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_drive_proof_verifier_types_ContestedResources_err_String {
        pub ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResources,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<drive_proof_verifier::types::ContestedResources, String>> for Result_ok_drive_proof_verifier_types_ContestedResources_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_drive_proof_verifier_types_ContestedResources_err_String) -> Result<drive_proof_verifier::types::ContestedResources, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResources as ferment::FFIConversionFrom<drive_proof_verifier::types::ContestedResources>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<drive_proof_verifier::types::ContestedResources, String>> for Result_ok_drive_proof_verifier_types_ContestedResources_err_String {
        unsafe fn ffi_to_const(obj: Result<drive_proof_verifier::types::ContestedResources, String>) -> *const Result_ok_drive_proof_verifier_types_ContestedResources_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_drive_proof_verifier_types_ContestedResources_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_ContestedResources_err_String_ctor(ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResources, error: *mut std::os::raw::c_char) -> *mut Result_ok_drive_proof_verifier_types_ContestedResources_err_String { ferment::boxed(Result_ok_drive_proof_verifier_types_ContestedResources_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_ContestedResources_err_String_destroy(ffi: *mut Result_ok_drive_proof_verifier_types_ContestedResources_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_20 {
        pub count: usize,
        pub values: *mut u8,
    }

    impl ferment::FFIConversionFrom<[u8; 20]> for Arr_u8_20 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_20) -> [u8; 20] {
            let ffi_ref = &*ffi;
            let vec: Vec<u8> = ferment::from_primitive_group(ffi_ref.values, ffi_ref.count);
            vec.try_into().unwrap()
        }
    }

    impl ferment::FFIConversionTo<[u8; 20]> for Arr_u8_20 { unsafe fn ffi_to_const(obj: [u8; 20]) -> *const Arr_u8_20 { ferment::boxed(Self { count: obj.len(), values: ferment::to_primitive_group(obj.into_iter()) }) } }

    impl Drop for Arr_u8_20 { fn drop(&mut self) { unsafe { ferment::unbox_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_20_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_20 { ferment::boxed(Arr_u8_20 { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_20_destroy(ffi: *mut Arr_u8_20) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dpp_voting_vote_polls_VotePoll {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll,
    }

    impl ferment::FFIConversionFrom<Vec<dpp::voting::vote_polls::VotePoll>> for Vec_dpp_voting_vote_polls_VotePoll {
        unsafe fn ffi_from_const(ffi: *const Vec_dpp_voting_vote_polls_VotePoll) -> Vec<dpp::voting::vote_polls::VotePoll> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<dpp::voting::vote_polls::VotePoll>> for Vec_dpp_voting_vote_polls_VotePoll { unsafe fn ffi_to_const(obj: Vec<dpp::voting::vote_polls::VotePoll>) -> *const Vec_dpp_voting_vote_polls_VotePoll { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_dpp_voting_vote_polls_VotePoll { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_voting_vote_polls_VotePoll_ctor(count: usize, values: *mut *mut crate::fermented::types::dpp::voting::vote_polls::dpp_voting_vote_polls_VotePoll) -> *mut Vec_dpp_voting_vote_polls_VotePoll { ferment::boxed(Vec_dpp_voting_vote_polls_VotePoll { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_voting_vote_polls_VotePoll_destroy(ffi: *mut Vec_dpp_voting_vote_polls_VotePoll) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_platform_value_Value_platform_value_Value {
        pub o_0: *mut crate::fermented::types::platform_value::platform_value_Value,
        pub o_1: *mut crate::fermented::types::platform_value::platform_value_Value,
    }

    impl ferment::FFIConversionFrom<(platform_value::Value, platform_value::Value)> for Tuple_platform_value_Value_platform_value_Value {
        unsafe fn ffi_from_const(ffi: *const Tuple_platform_value_Value_platform_value_Value) -> (platform_value::Value, platform_value::Value) {
            let ffi_ref = &*ffi;
            (ferment::FFIConversionFrom::ffi_from(ffi_ref.o_0), ferment::FFIConversionFrom::ffi_from(ffi_ref.o_1))
        }
    }

    impl ferment::FFIConversionTo<(platform_value::Value, platform_value::Value)> for Tuple_platform_value_Value_platform_value_Value { unsafe fn ffi_to_const(obj: (platform_value::Value, platform_value::Value)) -> *const Tuple_platform_value_Value_platform_value_Value { ferment::boxed(Self { o_0: ferment::FFIConversionTo::ffi_to(obj.0), o_1: ferment::FFIConversionTo::ffi_to(obj.1) }) } }

    impl Drop for Tuple_platform_value_Value_platform_value_Value {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.o_0);
                ferment::unbox_any(self.o_1);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Tuple_platform_value_Value_platform_value_Value_ctor(o_0: *mut crate::fermented::types::platform_value::platform_value_Value, o_1: *mut crate::fermented::types::platform_value::platform_value_Value) -> *mut Tuple_platform_value_Value_platform_value_Value { ferment::boxed(Tuple_platform_value_Value_platform_value_Value { o_0, o_1 }) }

    #[no_mangle]
    pub unsafe extern "C" fn Tuple_platform_value_Value_platform_value_Value_destroy(ffi: *mut Tuple_platform_value_Value_platform_value_Value) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
        pub values: *mut *mut crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote,
    }

    impl ferment::FFIConversionFrom<std::collections::BTreeMap<platform_value::types::identifier::Identifier, Option<dpp::voting::votes::resource_vote::ResourceVote>>> for std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote {
        unsafe fn ffi_from_const(ffi: *const std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote) -> std::collections::BTreeMap<platform_value::types::identifier::Identifier, Option<dpp::voting::votes::resource_vote::ResourceVote>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, |o| <crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier as ferment::FFIConversionFrom<platform_value::types::identifier::Identifier>>::ffi_from(o), |o| <crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote as ferment::FFIConversionFrom<dpp::voting::votes::resource_vote::ResourceVote>>::ffi_from_opt(o))
        }
    }

    impl ferment::FFIConversionTo<std::collections::BTreeMap<platform_value::types::identifier::Identifier, Option<dpp::voting::votes::resource_vote::ResourceVote>>> for std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote { unsafe fn ffi_to_const(obj: std::collections::BTreeMap<platform_value::types::identifier::Identifier, Option<dpp::voting::votes::resource_vote::ResourceVote>>) -> *const std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote { ferment::boxed(Self { count: obj.len(), keys: ferment::to_complex_group(obj.keys().cloned()), values: ferment::to_opt_complex_group(obj.values().cloned()) }) } }

    impl Drop for std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote_ctor(count: usize, keys: *mut *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier, values: *mut *mut crate::fermented::types::dpp::voting::votes::resource_vote::dpp_voting_votes_resource_vote_ResourceVote) -> *mut std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote { ferment::boxed(std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote { count, keys, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote_destroy(ffi: *mut std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_voting_votes_resource_vote_ResourceVote) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_drive_query_conditions_WhereClause {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereClause,
    }

    impl ferment::FFIConversionFrom<Vec<drive::query::conditions::WhereClause>> for Vec_drive_query_conditions_WhereClause {
        unsafe fn ffi_from_const(ffi: *const Vec_drive_query_conditions_WhereClause) -> Vec<drive::query::conditions::WhereClause> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<drive::query::conditions::WhereClause>> for Vec_drive_query_conditions_WhereClause { unsafe fn ffi_to_const(obj: Vec<drive::query::conditions::WhereClause>) -> *const Vec_drive_query_conditions_WhereClause { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_drive_query_conditions_WhereClause { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_drive_query_conditions_WhereClause_ctor(count: usize, values: *mut *mut crate::fermented::types::drive::query::conditions::drive_query_conditions_WhereClause) -> *mut Vec_drive_query_conditions_WhereClause { ferment::boxed(Vec_drive_query_conditions_WhereClause { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_drive_query_conditions_WhereClause_destroy(ffi: *mut Vec_drive_query_conditions_WhereClause) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_32 {
        pub count: usize,
        pub values: *mut u8,
    }

    impl ferment::FFIConversionFrom<[u8; 32]> for Arr_u8_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_32) -> [u8; 32] {
            let ffi_ref = &*ffi;
            let vec: Vec<u8> = ferment::from_primitive_group(ffi_ref.values, ffi_ref.count);
            vec.try_into().unwrap()
        }
    }

    impl ferment::FFIConversionTo<[u8; 32]> for Arr_u8_32 { unsafe fn ffi_to_const(obj: [u8; 32]) -> *const Arr_u8_32 { ferment::boxed(Self { count: obj.len(), values: ferment::to_primitive_group(obj.into_iter()) }) } }

    impl Drop for Arr_u8_32 { fn drop(&mut self) { unsafe { ferment::unbox_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_32 { ferment::boxed(Arr_u8_32 { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_destroy(ffi: *mut Arr_u8_32) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_drive_proof_verifier_types_ContestedResource {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResource,
    }

    impl ferment::FFIConversionFrom<Vec<drive_proof_verifier::types::ContestedResource>> for Vec_drive_proof_verifier_types_ContestedResource {
        unsafe fn ffi_from_const(ffi: *const Vec_drive_proof_verifier_types_ContestedResource) -> Vec<drive_proof_verifier::types::ContestedResource> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<drive_proof_verifier::types::ContestedResource>> for Vec_drive_proof_verifier_types_ContestedResource { unsafe fn ffi_to_const(obj: Vec<drive_proof_verifier::types::ContestedResource>) -> *const Vec_drive_proof_verifier_types_ContestedResource { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_drive_proof_verifier_types_ContestedResource { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_drive_proof_verifier_types_ContestedResource_ctor(count: usize, values: *mut *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_ContestedResource) -> *mut Vec_drive_proof_verifier_types_ContestedResource { ferment::boxed(Vec_drive_proof_verifier_types_ContestedResource { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_drive_proof_verifier_types_ContestedResource_destroy(ffi: *mut Vec_drive_proof_verifier_types_ContestedResource) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct platform_mobile_provider_Cache_platform_value_types_identifier_Identifier_dpp_data_contract_DataContract {
        pub obj: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier,
    }

    impl Drop for platform_mobile_provider_Cache_platform_value_types_identifier_Identifier_dpp_data_contract_DataContract { fn drop(&mut self) { unsafe { ferment::unbox_any(self.obj); } } }

    #[no_mangle]
    pub unsafe extern "C" fn platform_mobile_provider_Cache_platform_value_types_identifier_Identifier_dpp_data_contract_DataContract_ctor(obj: *mut crate::fermented::types::platform_value::types::identifier::platform_value_types_identifier_Identifier) -> *mut platform_mobile_provider_Cache_platform_value_types_identifier_Identifier_dpp_data_contract_DataContract { ferment::boxed(platform_mobile_provider_Cache_platform_value_types_identifier_Identifier_dpp_data_contract_DataContract { obj }) }

    #[no_mangle]
    pub unsafe extern "C" fn platform_mobile_provider_Cache_platform_value_types_identifier_Identifier_dpp_data_contract_DataContract_destroy(ffi: *mut platform_mobile_provider_Cache_platform_value_types_identifier_Identifier_dpp_data_contract_DataContract) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_drive_proof_verifier_types_Voter {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voter,
    }

    impl ferment::FFIConversionFrom<std::collections::BTreeSet<drive_proof_verifier::types::Voter>> for std_collections_BTreeSet_drive_proof_verifier_types_Voter {
        unsafe fn ffi_from_const(ffi: *const std_collections_BTreeSet_drive_proof_verifier_types_Voter) -> std::collections::BTreeSet<drive_proof_verifier::types::Voter> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<std::collections::BTreeSet<drive_proof_verifier::types::Voter>> for std_collections_BTreeSet_drive_proof_verifier_types_Voter { unsafe fn ffi_to_const(obj: std::collections::BTreeSet<drive_proof_verifier::types::Voter>) -> *const std_collections_BTreeSet_drive_proof_verifier_types_Voter { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for std_collections_BTreeSet_drive_proof_verifier_types_Voter { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_drive_proof_verifier_types_Voter_ctor(count: usize, values: *mut *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Voter) -> *mut std_collections_BTreeSet_drive_proof_verifier_types_Voter { ferment::boxed(std_collections_BTreeSet_drive_proof_verifier_types_Voter { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_drive_proof_verifier_types_Voter_destroy(ffi: *mut std_collections_BTreeSet_drive_proof_verifier_types_Voter) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_drive_proof_verifier_types_Contenders_err_String {
        pub ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Contenders,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<drive_proof_verifier::types::Contenders, String>> for Result_ok_drive_proof_verifier_types_Contenders_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_drive_proof_verifier_types_Contenders_err_String) -> Result<drive_proof_verifier::types::Contenders, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| <crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Contenders as ferment::FFIConversionFrom<drive_proof_verifier::types::Contenders>>::ffi_from(o), ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<drive_proof_verifier::types::Contenders, String>> for Result_ok_drive_proof_verifier_types_Contenders_err_String {
        unsafe fn ffi_to_const(obj: Result<drive_proof_verifier::types::Contenders, String>) -> *const Result_ok_drive_proof_verifier_types_Contenders_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::FFIConversionTo::ffi_to(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_drive_proof_verifier_types_Contenders_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_Contenders_err_String_ctor(ok: *mut crate::fermented::types::drive_proof_verifier::types::drive_proof_verifier_types_Contenders, error: *mut std::os::raw::c_char) -> *mut Result_ok_drive_proof_verifier_types_Contenders_err_String { ferment::boxed(Result_ok_drive_proof_verifier_types_Contenders_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_drive_proof_verifier_types_Contenders_err_String_destroy(ffi: *mut Result_ok_drive_proof_verifier_types_Contenders_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID,
        pub values: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey,
    }

    impl ferment::FFIConversionFrom<std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey>> for std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey {
        unsafe fn ffi_from_const(ffi: *const std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey) -> std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, |o| <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID as ferment::FFIConversionFrom<dpp::identity::identity_public_key::KeyID>>::ffi_from(o), |o| <crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey as ferment::FFIConversionFrom<dpp::identity::identity_public_key::IdentityPublicKey>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey>> for std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey { unsafe fn ffi_to_const(obj: std::collections::BTreeMap<dpp::identity::identity_public_key::KeyID, dpp::identity::identity_public_key::IdentityPublicKey>) -> *const std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey { ferment::boxed(Self { count: obj.len(), keys: ferment::to_complex_group(obj.keys().cloned()), values: ferment::to_complex_group(obj.values().cloned()) }) } }

    impl Drop for std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_ctor(count: usize, keys: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID, values: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_IdentityPublicKey) -> *mut std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey { ferment::boxed(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey { count, keys, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_destroy(ffi: *mut std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_36 {
        pub count: usize,
        pub values: *mut u8,
    }

    impl ferment::FFIConversionFrom<[u8; 36]> for Arr_u8_36 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_36) -> [u8; 36] {
            let ffi_ref = &*ffi;
            let vec: Vec<u8> = ferment::from_primitive_group(ffi_ref.values, ffi_ref.count);
            vec.try_into().unwrap()
        }
    }

    impl ferment::FFIConversionTo<[u8; 36]> for Arr_u8_36 { unsafe fn ffi_to_const(obj: [u8; 36]) -> *const Arr_u8_36 { ferment::boxed(Self { count: obj.len(), values: ferment::to_primitive_group(obj.into_iter()) }) } }

    impl Drop for Arr_u8_36 { fn drop(&mut self) { unsafe { ferment::unbox_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_36_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_36 { ferment::boxed(Arr_u8_36 { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_36_destroy(ffi: *mut Arr_u8_36) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_u64_err_String {
        pub ok: *mut u64,
        pub error: *mut std::os::raw::c_char,
    }

    impl ferment::FFIConversionFrom<Result<u64, String>> for Result_ok_u64_err_String {
        unsafe fn ffi_from_const(ffi: *const Result_ok_u64_err_String) -> Result<u64, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| *o, ffi_ref.error, |o| <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(o))
        }
    }

    impl ferment::FFIConversionTo<Result<u64, String>> for Result_ok_u64_err_String {
        unsafe fn ffi_to_const(obj: Result<u64, String>) -> *const Result_ok_u64_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(obj, |o| ferment::boxed(o), |o| ferment::FFIConversionTo::ffi_to(o));
                Self { ok, error }
            })
        }
    }

    impl Drop for Result_ok_u64_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::destroy_opt_primitive(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u64_err_String_ctor(ok: *mut u64, error: *mut std::os::raw::c_char) -> *mut Result_ok_u64_err_String { ferment::boxed(Result_ok_u64_err_String { ok, error }) }

    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u64_err_String_destroy(ffi: *mut Result_ok_u64_err_String) { ferment::unbox_any(ffi); }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dpp_identity_identity_public_key_KeyID {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID,
    }

    impl ferment::FFIConversionFrom<Vec<dpp::identity::identity_public_key::KeyID>> for Vec_dpp_identity_identity_public_key_KeyID {
        unsafe fn ffi_from_const(ffi: *const Vec_dpp_identity_identity_public_key_KeyID) -> Vec<dpp::identity::identity_public_key::KeyID> {
            let ffi_ref = &*ffi;
            ferment::from_complex_group(ffi_ref.values, ffi_ref.count)
        }
    }

    impl ferment::FFIConversionTo<Vec<dpp::identity::identity_public_key::KeyID>> for Vec_dpp_identity_identity_public_key_KeyID { unsafe fn ffi_to_const(obj: Vec<dpp::identity::identity_public_key::KeyID>) -> *const Vec_dpp_identity_identity_public_key_KeyID { ferment::boxed(Self { count: obj.len(), values: ferment::to_complex_group(obj.into_iter()) }) } }

    impl Drop for Vec_dpp_identity_identity_public_key_KeyID { fn drop(&mut self) { unsafe { ferment::unbox_any_vec_ptr(self.values, self.count); } } }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_identity_identity_public_key_KeyID_ctor(count: usize, values: *mut *mut crate::fermented::types::dpp::identity::identity_public_key::dpp_identity_identity_public_key_KeyID) -> *mut Vec_dpp_identity_identity_public_key_KeyID { ferment::boxed(Vec_dpp_identity_identity_public_key_KeyID { count, values }) }

    #[no_mangle]
    pub unsafe extern "C" fn Vec_dpp_identity_identity_public_key_KeyID_destroy(ffi: *mut Vec_dpp_identity_identity_public_key_KeyID) { ferment::unbox_any(ffi); }
}