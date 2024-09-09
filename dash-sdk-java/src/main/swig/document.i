//
// Document Objects
//

LIST_STRUCT_TYPEMAP(Vec_dpp_document_Document, dpp_document_Document, Document, platform_mobile_clone_Document_clone);
%ignore Vec_dpp_document_Document;

START_CLASS(Document, dpp_document_Document);
  dpp_document_Document(dpp_document_v0_DocumentV0 * docV0) {
    return dpp_document_Document_V0_ctor(clone(docV0));
  }
END_CLASS();

// dpp_document_v0_DocumentV0 *dpp_document_v0_DocumentV0_ctor(platform_value_types_identifier_Identifier *id,
//                                                             platform_value_types_identifier_Identifier *owner_id,
//                                                             std_collections_Map_keys_String_values_platform_value_Value *properties,
//                                                             dpp_prelude_Revision *revision,
//                                                             dpp_identity_identity_public_key_TimestampMillis *created_at,
//                                                             dpp_identity_identity_public_key_TimestampMillis *updated_at,
//                                                             dpp_identity_identity_public_key_TimestampMillis *transferred_at,
//                                                             dpp_prelude_BlockHeight *created_at_block_height,
//                                                             dpp_prelude_BlockHeight *updated_at_block_height,
//                                                             dpp_prelude_BlockHeight *transferred_at_block_height,
//                                                             dpp_prelude_CoreBlockHeight *created_at_core_block_height,
//                                                             dpp_prelude_CoreBlockHeight *updated_at_core_block_height,
//                                                             dpp_prelude_CoreBlockHeight *transferred_at_core_block_height);

START_CLASS(DocumentV0, dpp_document_v0_DocumentV0);
  dpp_document_v0_DocumentV0(platform_value_types_identifier_Identifier *id,
                                                            platform_value_types_identifier_Identifier *owner_id,
                                                            std_collections_Map_keys_String_values_platform_value_Value *properties,
                                                            dpp_prelude_Revision *revision,
                                                            dpp_identity_identity_public_key_TimestampMillis *created_at,
                                                            dpp_identity_identity_public_key_TimestampMillis *updated_at,
                                                            dpp_identity_identity_public_key_TimestampMillis *transferred_at,
                                                            dpp_prelude_BlockHeight *created_at_block_height,
                                                            dpp_prelude_BlockHeight *updated_at_block_height,
                                                            dpp_prelude_BlockHeight *transferred_at_block_height,
                                                            dpp_prelude_CoreBlockHeight *created_at_core_block_height,
                                                            dpp_prelude_CoreBlockHeight *updated_at_core_block_height,
                                                            dpp_prelude_CoreBlockHeight *transferred_at_core_block_height) {
    return dpp_document_v0_DocumentV0_ctor(
        clone(id),
        clone(owner_id),
        clone(properties),
        clone(revision),
        clone(created_at),
        clone(updated_at),
        clone(transferred_at),
        clone(created_at_block_height),
        clone(updated_at_block_height),
        clone(transferred_at_block_height),
        clone(created_at_core_block_height),
        clone(updated_at_core_block_height),
        clone(transferred_at_core_block_height)
    );
  }
    dpp_document_v0_DocumentV0(
        platform_value_types_identifier_Identifier *id,
        platform_value_types_identifier_Identifier *owner_id,
        std_collections_Map_keys_String_values_platform_value_Value *properties,
        dpp_prelude_Revision *revision
    ) {
        return dpp_document_v0_DocumentV0_ctor(
            clone(id),
            clone(owner_id),
            clone(properties),
            clone(revision),
            nullptr,
            nullptr,
            nullptr,
            nullptr,
            nullptr,
            nullptr,
            nullptr,
            nullptr,
            nullptr
        );
  }
END_CLASS();

MAP_STRUCT_TYPEMAP(
    std_collections_Map_keys_platform_value_types_identifier_Identifier_values_Option_dpp_document_Document,
    platform_value_types_identifier_Identifier,
    Identifier,
    dpp_document_Document,
    Document
);

//
// New Objects
//
// TODO: add document query method(s)

%newobject platform_mobile_put_replace_document_sdk(
    DashSdk *rust_sdk,
    dpp_document_Document *document,
    platform_value_types_identifier_Identifier *data_contract_id,
    char *document_type_str,
    dpp_identity_identity_public_key_IdentityPublicKey *identity_public_key,
    dpp_prelude_BlockHeight *block_height,
    dpp_prelude_CoreBlockHeight *core_block_height,
    uint64_t signer_callback);

%newobject platform_mobile_put_replace_document_sdk(
    DashSdk *rust_sdk,
    dpp_document_Document *document,
    platform_value_types_identifier_Identifier *data_contract_id,
    char *document_type_str,
    dpp_identity_identity_public_key_IdentityPublicKey *identity_public_key,
    dpp_prelude_BlockHeight *block_height,
    dpp_prelude_CoreBlockHeight *core_block_height,
    uint64_t signer_callback
);