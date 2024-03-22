%ignore dpp_identity_v0_IdentityV0::public_keys;
%ignore dpp_identity_v0_IdentityV0::balance;
%rename(IdentityV0) dpp_identity_v0_IdentityV0;
%extend dpp_identity_v0_IdentityV0 {
    ~dpp_identity_v0_IdentityV0() {
        printf("~IdentityV0(%lx)\n", (uint64_t)$self);
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
        for (int i = 0; i < $self->public_keys->count; ++i) {
            if ($self->public_keys->keys[i]->_0 == id)
                return $self->public_keys->values[i]->v0._0;
        }
        return NULL;
    }

    long long getBalance() {
        return (long)$self->balance;
    }
}

%extend dpp_identity_identity_Identity {
    //dpp_identity_identity_Identity() {
    //    return get_an_identity();
    //}
    ~dpp_identity_identity_Identity() {
        printf("~Identity(%lx)\n", (uint64_t)$self);
        dpp_identity_identity_Identity_destroy($self);
    }
}
%rename(Identity) dpp_identity_identity_Identity;
%rename(Identity_Tag) dpp_identity_identity_Identity_Tag;
%rename(IdentityV0Type) dpp_identity_identity_Identity_V0;

%newobject platform_mobile_identity_get_identity2(platform_value_types_identifier_Identifier *);
%newobject platform_mobile_identity_get_an_identity(void);
%newobject platform_mobile_identity_create_basic_identity(uint8_t (*)[32]);
%newobject platform_mobile_identity_get_identity_contract_bounds(platform_value_types_identifier_Identifier *identifier, platform_value_types_identifier_Identifier *contract_identifier);
%newobject platform_mobile_fetch_identity_fetch_identity3(platform_value_types_identifier_Identifier *identifier);
%newobject platform_mobile_fetch_identity_fetch_identity(platform_value_types_identifier_Identifier *identifier);
%newobject platform_mobile_fetch_identity_get_document(void);
%newobject platform_mobile_fetch_identity_fetch_identity2(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity) platform_mobile_fetch_identity_fetch_identity(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity2) platform_mobile_fetch_identity_fetch_identity2(platform_value_types_identifier_Identifier *identifier);
%rename (fetchIdentity3) platform_mobile_fetch_identity_fetch_identity3(platform_value_types_identifier_Identifier *identifier);
%rename (getDocument) platform_mobile_fetch_identity_get_document();
%rename (createBasicIdentity) platform_mobile_identity_create_basic_identity(uint8_t (*id)[32]);
%rename (getIdentityContractBounds) platform_mobile_identity_get_identity_contract_bounds(platform_value_types_identifier_Identifier *identifier, platform_value_types_identifier_Identifier *contract_identifier);
%rename (getIdentity2) platform_mobile_identity_get_identity2(platform_value_types_identifier_Identifier *);
