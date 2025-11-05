%ignore dpp_identity_v0_IdentityV0::public_keys;
%ignore dpp_identity_v0_IdentityV0::balance;
%rename(IdentityV0) dpp_identity_v0_IdentityV0;

%extend dpp_identity_v0_IdentityV0 {
    dpp_identity_v0_IdentityV0(
        platform_value_types_identifier_Identifier * identifier,
        std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * public_keys,
        uint64_t balance,
        dpp_prelude_Revision *revision
    ) {
        return dpp_identity_v0_IdentityV0_ctor(
            platform_mobile_identity_Identifier_clone(identifier),
            platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(public_keys),
            balance,
            dpp_prelude_Revision_ctor(revision->_0)
        );
    }

    ~dpp_identity_v0_IdentityV0() {
        dpp_identity_v0_IdentityV0_destroy($self);
    }

    int getPublicKeyCount() {
        return $self->public_keys->count;
    }

    struct dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 * getPublicKey(uint32_t index) {
        if (index < $self->public_keys->count) {
            return $self->public_keys->values[index]->v0._0;
        } else {
            return NULL;
        }
    }

    struct dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 * getPublicKeyById(uint32_t id) {
        for (uintptr_t i = 0; i < $self->public_keys->count; ++i) {
            if ($self->public_keys->keys[i]->_0 == id)
                return $self->public_keys->values[i]->v0._0;
        }
        return NULL;
    }

    long long getBalance() {
        return (long)$self->balance;
    }

    std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * getPublicKeys() {
        return $self->public_keys;
    }
}

%extend dpp_identity_identity_Identity {
    dpp_identity_identity_Identity(dpp_identity_v0_IdentityV0 * identityV0) {
        return dpp_identity_identity_Identity_V0_ctor(platform_mobile_identity_IdentityV0_clone(identityV0));
    }
    ~dpp_identity_identity_Identity() {
        dpp_identity_identity_Identity_destroy($self);
    }
}
%rename(Identity) dpp_identity_identity_Identity;

%newobject platform_mobile_identity_get_identity2(platform_value_types_identifier_Identifier *);
%newobject platform_mobile_identity_get_an_identity(void);
%newobject platform_mobile_identity_create_basic_identity(Arr_u8_32 *id);
%newobject platform_mobile_identity_get_identity_contract_bounds(platform_value_types_identifier_Identifier *identifier, platform_value_types_identifier_Identifier *contract_identifier);
%newobject platform_mobile_fetch_identity_fetch_identity3(platform_value_types_identifier_Identifier *identifier);
%newobject platform_mobile_fetch_identity_fetch_identity(platform_value_types_identifier_Identifier *identifier);
%newobject platform_mobile_fetch_identity_get_document(void);
%newobject platform_mobile_fetch_identity_fetch_identity2(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity) platform_mobile_fetch_identity_fetch_identity(platform_value_types_identifier_Identifier *identifier, uint64_t quorum_public_key_callback,
                                                                                                                        uint64_t data_contract_callback);
%rename (fetchIdentity2) platform_mobile_fetch_identity_fetch_identity2(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity3) platform_mobile_fetch_identity_fetch_identity3(platform_value_types_identifier_Identifier *identifier);
%rename (getDocument) platform_mobile_fetch_identity_get_document();
%rename (createBasicIdentity) platform_mobile_identity_create_basic_identity(Arr_u8_32 * identifier);
%rename (getIdentityContractBounds) platform_mobile_identity_get_identity_contract_bounds(platform_value_types_identifier_Identifier *identifier, platform_value_types_identifier_Identifier *contract_identifier);
%rename (getIdentity2) platform_mobile_identity_get_identity2(platform_value_types_identifier_Identifier *);
%rename (getIdentityByPublicKeyHash) platform_mobile_fetch_identity_fetch_identity_with_keyhash(Arr_u8_20 *key_hash, uint64_t quorum_public_key_callback, uint64_t data_contract_callback);

%ignore platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys);
struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey;

%include "stdint.i"

MAP_STRUCT_TYPEMAP(
    std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey,
    dpp_identity_identity_public_key_KeyID,
    KeyID,
    dpp_identity_identity_public_key_IdentityPublicKey,
    IdentityPublicKey
);

%newobject platform_mobile_put_put_identity_sdk(
    DashSdk *rust_sdk,
    dpp_identity_identity_Identity *identity,
    platform_mobile_put_AssetLockProofFFI *asset_lock_proof,
    Vec_u8 *asset_lock_proof_private_key,
    uint64_t signer_callback,
    bool is_testnet
);

%newobject platform_mobile_put_topup_identity_sdk(
    DashSdk *rust_sdk,
    dpp_identity_identity_Identity *identity,
    platform_mobile_put_AssetLockProofFFI *asset_lock_proof,
    Vec_u8 *asset_lock_proof_private_key,
    bool is_testnet
);