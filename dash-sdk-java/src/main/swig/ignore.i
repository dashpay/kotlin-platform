%ignore dash_sdk_platform_types_identity_PublicKeyHash_ctor(Arr_u8_20 *o_0);
%ignore dash_sdk_platform_types_identity_PublicKeyHash_destroy(dash_sdk_platform_types_identity_PublicKeyHash *ffi);
%ignore platform_mobile_fetch_document_StartPoint_StartAfter_ctor(Vec_u8 *o_0);
%ignore platform_mobile_fetch_document_StartPoint_StartAt_ctor(Vec_u8 *o_0);
%ignore platform_mobile_fetch_document_StartPoint_destroy(platform_mobile_fetch_document_StartPoint *ffi);
%ignore platform_mobile_put_ChainAssetLockProofFFI_ctor(uint32_t core_chain_locked_height, platform_mobile_put_OutPointFFI *out_point);
%ignore platform_mobile_put_ChainAssetLockProofFFI_destroy(platform_mobile_put_ChainAssetLockProofFFI *ffi);
%ignore platform_mobile_put_OutPointFFI_ctor(Arr_u8_32 *txid, uint32_t vout);
%ignore platform_mobile_put_OutPointFFI_destroy(platform_mobile_put_OutPointFFI *ffi);
%ignore platform_mobile_put_AssetLockProofFFI_Instant_ctor(platform_mobile_put_InstantAssetLockProofFFI *o_0);
%ignore platform_mobile_put_AssetLockProofFFI_Chain_ctor(platform_mobile_put_ChainAssetLockProofFFI *o_0);
%ignore platform_mobile_put_AssetLockProofFFI_destroy(platform_mobile_put_AssetLockProofFFI *ffi);
%ignore platform_mobile_put_InstantAssetLockProofFFI_clone(platform_mobile_put_InstantAssetLockProofFFI *a);
%ignore platform_mobile_put_InstantAssetLockProofFFI_ctor(Vec_u8 *instant_lock, Vec_u8 *transaction, uint32_t output_index);
%ignore platform_mobile_put_InstantAssetLockProofFFI_destroy(platform_mobile_put_InstantAssetLockProofFFI *ffi);
%ignore platform_mobile_put_OutPointFFI_clone(platform_mobile_put_OutPointFFI *a);
%ignore platform_mobile_put_ChainAssetLockProofFFI_clone(platform_mobile_put_ChainAssetLockProofFFI *a);
%ignore platform_mobile_clone_Value_clone(platform_value_Value *value);
%ignore platform_mobile_clone_Vec_u8_clone(Vec_u8 *vec);
%ignore platform_mobile_clone_Arr_u8_36_clone(Arr_u8_36 *slice);
%ignore platform_mobile_clone_Revision_clone(dpp_prelude_Revision *revision);
%ignore platform_mobile_clone_std_collections_Map_keys_String_values_platform_value_Value_clone(std_collections_Map_keys_String_values_platform_value_Value *map);
%ignore platform_mobile_clone_ValueMap_clone(platform_value_value_map_ValueMap *value_map);
%ignore platform_mobile_clone_BlockHeight_clone(dpp_prelude_BlockHeight *height);
%ignore platform_mobile_clone_WhereClause_clone(drive_query_conditions_WhereClause *o);
%ignore platform_mobile_clone_WhereOperator_clone(drive_query_conditions_WhereOperator *o);
%ignore platform_mobile_clone_TimestampMillis_clone(dpp_identity_identity_public_key_TimestampMillis *time);
%ignore platform_mobile_clone_Document_clone(dpp_document_Document *document);
%ignore platform_mobile_clone_Arr_u8_32_clone(Arr_u8_32 *slice);
%ignore platform_mobile_clone_OrderClause_clone(drive_query_ordering_OrderClause *o);
%ignore platform_mobile_clone_KeyID_clone(dpp_identity_identity_public_key_KeyID *id);
%ignore platform_mobile_clone_CoreBlockHeight_clone(dpp_prelude_CoreBlockHeight *height);
%ignore platform_mobile_clone_DocumentV0_clone(dpp_document_v0_DocumentV0 *document);
%ignore platform_mobile_clone_Arr_u8_20_clone(Arr_u8_20 *slice);
%ignore platform_mobile_identity_Identity_clone(dpp_identity_identity_Identity *identity);
%ignore platform_mobile_identity_Identifier_clone(platform_value_types_identifier_Identifier *identifier);
%ignore platform_mobile_identity_IdentityPublicKey_clone(dpp_identity_identity_public_key_IdentityPublicKey *identity_public_key);
%ignore platform_mobile_identity_IdentityPublicKeyV0_clone(dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 *identity_public_key);
%ignore platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys);
%ignore platform_mobile_identity_IdentityV0_clone(dpp_identity_v0_IdentityV0 *identity);
%ignore dpp_identity_identity_public_key_KeyCount_ctor(dpp_identity_identity_public_key_KeyID *o_0);
%ignore dpp_identity_identity_public_key_KeyCount_destroy(dpp_identity_identity_public_key_KeyCount *ffi);
%ignore dpp_identity_identity_public_key_KeyID_ctor(uint32_t o_0);
%ignore dpp_identity_identity_public_key_KeyID_destroy(dpp_identity_identity_public_key_KeyID *ffi);
%ignore dpp_identity_identity_public_key_IdentityPublicKey_V0_ctor(dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 *o_0);
%ignore dpp_identity_identity_public_key_IdentityPublicKey_destroy(dpp_identity_identity_public_key_IdentityPublicKey *ffi);
%ignore dpp_identity_identity_public_key_TimestampMillis_ctor(uint64_t o_0);
%ignore dpp_identity_identity_public_key_TimestampMillis_destroy(dpp_identity_identity_public_key_TimestampMillis *ffi);
%ignore dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_ctor(dpp_identity_identity_public_key_KeyID *id, dpp_identity_identity_public_key_purpose_Purpose *purpose, dpp_identity_identity_public_key_security_level_SecurityLevel *security_level, dpp_identity_identity_public_key_contract_bounds_ContractBounds *contract_bounds, dpp_identity_identity_public_key_key_type_KeyType *key_type, bool read_only, platform_value_types_binary_data_BinaryData *data, dpp_identity_identity_public_key_TimestampMillis *disabled_at);
%ignore dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_destroy(dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 *ffi);
%ignore dpp_identity_identity_public_key_key_type_KeyType_ECDSA_SECP256K1_ctor();
%ignore dpp_identity_identity_public_key_key_type_KeyType_BLS12_381_ctor();
%ignore dpp_identity_identity_public_key_key_type_KeyType_ECDSA_HASH160_ctor();
%ignore dpp_identity_identity_public_key_key_type_KeyType_BIP13_SCRIPT_HASH_ctor();
%ignore dpp_identity_identity_public_key_key_type_KeyType_EDDSA_25519_HASH160_ctor();
%ignore dpp_identity_identity_public_key_key_type_KeyType_destroy(dpp_identity_identity_public_key_key_type_KeyType *ffi);
%ignore dpp_identity_identity_public_key_security_level_SecurityLevel_MASTER_ctor();
%ignore dpp_identity_identity_public_key_security_level_SecurityLevel_CRITICAL_ctor();
%ignore dpp_identity_identity_public_key_security_level_SecurityLevel_HIGH_ctor();
%ignore dpp_identity_identity_public_key_security_level_SecurityLevel_MEDIUM_ctor();
%ignore dpp_identity_identity_public_key_security_level_SecurityLevel_destroy(dpp_identity_identity_public_key_security_level_SecurityLevel *ffi);
%ignore dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContract_ctor(platform_value_types_identifier_Identifier *id);
%ignore dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContractDocumentType_ctor(platform_value_types_identifier_Identifier *id, char *document_type_name);
%ignore dpp_identity_identity_public_key_contract_bounds_ContractBounds_destroy(dpp_identity_identity_public_key_contract_bounds_ContractBounds *ffi);
%ignore dpp_identity_identity_public_key_purpose_Purpose_AUTHENTICATION_ctor();
%ignore dpp_identity_identity_public_key_purpose_Purpose_ENCRYPTION_ctor();
%ignore dpp_identity_identity_public_key_purpose_Purpose_DECRYPTION_ctor();
%ignore dpp_identity_identity_public_key_purpose_Purpose_TRANSFER_ctor();
%ignore dpp_identity_identity_public_key_purpose_Purpose_SYSTEM_ctor();
%ignore dpp_identity_identity_public_key_purpose_Purpose_VOTING_ctor();
%ignore dpp_identity_identity_public_key_purpose_Purpose_destroy(dpp_identity_identity_public_key_purpose_Purpose *ffi);
%ignore dpp_identity_v0_IdentityV0_ctor(platform_value_types_identifier_Identifier *id, std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys, uint64_t balance, dpp_prelude_Revision *revision);
%ignore dpp_identity_v0_IdentityV0_destroy(dpp_identity_v0_IdentityV0 *ffi);
%ignore dpp_identity_identity_Identity_V0_ctor(dpp_identity_v0_IdentityV0 *o_0);
%ignore dpp_identity_identity_Identity_destroy(dpp_identity_identity_Identity *ffi);
%ignore dpp_prelude_CoreBlockHeight_ctor(uint32_t o_0);
%ignore dpp_prelude_CoreBlockHeight_destroy(dpp_prelude_CoreBlockHeight *ffi);
%ignore dpp_prelude_BlockHeight_ctor(uint64_t o_0);
%ignore dpp_prelude_BlockHeight_destroy(dpp_prelude_BlockHeight *ffi);
%ignore dpp_prelude_Revision_ctor(uint64_t o_0);
%ignore dpp_prelude_Revision_destroy(dpp_prelude_Revision *ffi);
%ignore dpp_prelude_TimestampMillis_ctor(uint64_t o_0);
%ignore dpp_prelude_TimestampMillis_destroy(dpp_prelude_TimestampMillis *ffi);
%ignore dpp_document_Document_V0_ctor(dpp_document_v0_DocumentV0 *o_0);
%ignore dpp_document_Document_destroy(dpp_document_Document *ffi);
%ignore dpp_document_v0_DocumentV0_ctor(platform_value_types_identifier_Identifier *id, platform_value_types_identifier_Identifier *owner_id, std_collections_Map_keys_String_values_platform_value_Value *properties, dpp_prelude_Revision *revision, dpp_identity_identity_public_key_TimestampMillis *created_at, dpp_identity_identity_public_key_TimestampMillis *updated_at, dpp_identity_identity_public_key_TimestampMillis *transferred_at, dpp_prelude_BlockHeight *created_at_block_height, dpp_prelude_BlockHeight *updated_at_block_height, dpp_prelude_BlockHeight *transferred_at_block_height, dpp_prelude_CoreBlockHeight *created_at_core_block_height, dpp_prelude_CoreBlockHeight *updated_at_core_block_height, dpp_prelude_CoreBlockHeight *transferred_at_core_block_height);
%ignore dpp_document_v0_DocumentV0_destroy(dpp_document_v0_DocumentV0 *ffi);
%ignore platform_value_Hash256_ctor(Arr_u8_32 *o_0);
%ignore platform_value_Hash256_destroy(platform_value_Hash256 *ffi);
%ignore platform_value_Value_U128_ctor(u128 o_0);
%ignore platform_value_Value_I128_ctor(i128 o_0);
%ignore platform_value_Value_U64_ctor(uint64_t o_0);
%ignore platform_value_Value_I64_ctor(int64_t o_0);
%ignore platform_value_Value_U32_ctor(uint32_t o_0);
%ignore platform_value_Value_I32_ctor(int32_t o_0);
%ignore platform_value_Value_U16_ctor(uint16_t o_0);
%ignore platform_value_Value_I16_ctor(int16_t o_0);
%ignore platform_value_Value_U8_ctor(uint8_t o_0);
%ignore platform_value_Value_I8_ctor(int8_t o_0);
%ignore platform_value_Value_Bytes_ctor(Vec_u8 *o_0);
%ignore platform_value_Value_Bytes20_ctor(Arr_u8_20 *o_0);
%ignore platform_value_Value_Bytes32_ctor(Arr_u8_32 *o_0);
%ignore platform_value_Value_Bytes36_ctor(Arr_u8_36 *o_0);
%ignore platform_value_Value_EnumU8_ctor(Vec_u8 *o_0);
%ignore platform_value_Value_EnumString_ctor(Vec_String *o_0);
%ignore platform_value_Value_Identifier_ctor(platform_value_Hash256 *o_0);
%ignore platform_value_Value_Float_ctor(double o_0);
%ignore platform_value_Value_Text_ctor(char *o_0);
%ignore platform_value_Value_Bool_ctor(bool o_0);
%ignore platform_value_Value_Null_ctor();
%ignore platform_value_Value_Array_ctor(Vec_platform_value_Value *o_0);
%ignore platform_value_Value_Map_ctor(platform_value_value_map_ValueMap *o_0);
%ignore platform_value_Value_destroy(platform_value_Value *ffi);
%ignore platform_value_types_identifier_Identifier_ctor(platform_value_types_identifier_IdentifierBytes32 *o_0);
%ignore platform_value_types_identifier_Identifier_destroy(platform_value_types_identifier_Identifier *ffi);
%ignore platform_value_types_identifier_IdentifierBytes32_ctor(Arr_u8_32 *o_0);
%ignore platform_value_types_identifier_IdentifierBytes32_destroy(platform_value_types_identifier_IdentifierBytes32 *ffi);
%ignore platform_value_types_binary_data_BinaryData_ctor(Vec_u8 *o_0);
%ignore platform_value_types_binary_data_BinaryData_destroy(platform_value_types_binary_data_BinaryData *ffi);
%ignore platform_value_value_map_ValueMap_ctor(Vec_Tuple_platform_value_Value_platform_value_Value *o_0);
%ignore platform_value_value_map_ValueMap_destroy(platform_value_value_map_ValueMap *ffi);
%ignore platform_value_error_Error_Unsupported_ctor(char *o_0);
%ignore platform_value_error_Error_StructureError_ctor(char *o_0);
%ignore platform_value_error_Error_PathError_ctor(char *o_0);
%ignore platform_value_error_Error_IntegerSizeError_ctor();
%ignore platform_value_error_Error_IntegerParsingError_ctor();
%ignore platform_value_error_Error_StringDecodingError_ctor(char *o_0);
%ignore platform_value_error_Error_KeyMustBeAString_ctor();
%ignore platform_value_error_Error_ByteLengthNot20BytesError_ctor(char *o_0);
%ignore platform_value_error_Error_ByteLengthNot32BytesError_ctor(char *o_0);
%ignore platform_value_error_Error_ByteLengthNot36BytesError_ctor(char *o_0);
%ignore platform_value_error_Error_SerdeSerializationError_ctor(char *o_0);
%ignore platform_value_error_Error_SerdeDeserializationError_ctor(char *o_0);
%ignore platform_value_error_Error_CborSerializationError_ctor(char *o_0);
%ignore platform_value_error_Error_destroy(platform_value_error_Error *ffi);
%ignore drive_query_ordering_OrderClause_ctor(char *field, bool ascending);
%ignore drive_query_ordering_OrderClause_destroy(drive_query_ordering_OrderClause *ffi);
%ignore drive_query_conditions_WhereOperator_Equal_ctor();
%ignore drive_query_conditions_WhereOperator_GreaterThan_ctor();
%ignore drive_query_conditions_WhereOperator_GreaterThanOrEquals_ctor();
%ignore drive_query_conditions_WhereOperator_LessThan_ctor();
%ignore drive_query_conditions_WhereOperator_LessThanOrEquals_ctor();
%ignore drive_query_conditions_WhereOperator_Between_ctor();
%ignore drive_query_conditions_WhereOperator_BetweenExcludeBounds_ctor();
%ignore drive_query_conditions_WhereOperator_BetweenExcludeLeft_ctor();
%ignore drive_query_conditions_WhereOperator_BetweenExcludeRight_ctor();
%ignore drive_query_conditions_WhereOperator_In_ctor();
%ignore drive_query_conditions_WhereOperator_StartsWith_ctor();
%ignore drive_query_conditions_WhereOperator_destroy(drive_query_conditions_WhereOperator *ffi);
%ignore drive_query_conditions_WhereClause_ctor(char *field, drive_query_conditions_WhereOperator *operator_, platform_value_Value *value);
%ignore drive_query_conditions_WhereClause_destroy(drive_query_conditions_WhereClause *ffi);
%ignore Arr_u8_36_ctor(uintptr_t count, uint8_t *values);
%ignore Arr_u8_36_destroy(Arr_u8_36 *ffi);
%ignore Vec_dpp_document_Document_ctor(uintptr_t count, dpp_document_Document **values);
%ignore Vec_dpp_document_Document_destroy(Vec_dpp_document_Document *ffi);
%ignore Vec_drive_query_ordering_OrderClause_ctor(uintptr_t count, drive_query_ordering_OrderClause **values);
%ignore Vec_drive_query_ordering_OrderClause_destroy(Vec_drive_query_ordering_OrderClause *ffi);
%ignore Vec_u8_ctor(uintptr_t count, uint8_t *values);
%ignore Vec_u8_destroy(Vec_u8 *ffi);
%ignore Vec_platform_value_Value_ctor(uintptr_t count, platform_value_Value **values);
%ignore Vec_platform_value_Value_destroy(Vec_platform_value_Value *ffi);
%ignore Vec_String_ctor(uintptr_t count, char **values);
%ignore Vec_String_destroy(Vec_String *ffi);
%ignore Vec_drive_query_conditions_WhereClause_ctor(uintptr_t count, drive_query_conditions_WhereClause **values);
%ignore Vec_drive_query_conditions_WhereClause_destroy(Vec_drive_query_conditions_WhereClause *ffi);
%ignore Result_ok_dpp_identity_identity_Identity_err_String_ctor(dpp_identity_identity_Identity *ok, char *error);
%ignore Result_ok_dpp_identity_identity_Identity_err_String_destroy(Result_ok_dpp_identity_identity_Identity_err_String *ffi);
%ignore Result_ok_Vec_dpp_document_Document_err_String_ctor(Vec_dpp_document_Document *ok, char *error);
%ignore Result_ok_Vec_dpp_document_Document_err_String_destroy(Result_ok_Vec_dpp_document_Document_err_String *ffi);
%ignore Arr_u8_32_ctor(uintptr_t count, uint8_t *values);
%ignore Arr_u8_32_destroy(Arr_u8_32 *ffi);
%ignore std_collections_Map_keys_String_values_platform_value_Value_ctor(uintptr_t count, char **keys, platform_value_Value **values);
%ignore std_collections_Map_keys_String_values_platform_value_Value_destroy(std_collections_Map_keys_String_values_platform_value_Value *ffi);
%ignore std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_ctor(uintptr_t count, dpp_identity_identity_public_key_KeyID **keys, dpp_identity_identity_public_key_IdentityPublicKey **values);
%ignore std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_destroy(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *ffi);
%ignore Tuple_platform_value_Value_platform_value_Value_ctor(platform_value_Value *o_0, platform_value_Value *o_1);
%ignore Tuple_platform_value_Value_platform_value_Value_destroy(Tuple_platform_value_Value_platform_value_Value *ffi);
%ignore Vec_Tuple_platform_value_Value_platform_value_Value_ctor(uintptr_t count, Tuple_platform_value_Value_platform_value_Value **values);
%ignore Vec_Tuple_platform_value_Value_platform_value_Value_destroy(Vec_Tuple_platform_value_Value_platform_value_Value *ffi);
%ignore Arr_u8_20_ctor(uintptr_t count, uint8_t *values);
%ignore Arr_u8_20_destroy(Arr_u8_20 *ffi);
