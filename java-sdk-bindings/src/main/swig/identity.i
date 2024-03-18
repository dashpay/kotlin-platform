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
            return $self->public_keys->values[index]->v0;
        } else {
            return NULL;
        }
    }

    struct dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 * getPublicKeyById(uint32_t id) {
        for (int i = 0; i < $self->public_keys->count; ++i) {
            if ($self->public_keys->keys[i]->_0 == id)
                return $self->public_keys->values[i]->v0;
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

%newobject get_identity2(struct platform_value_types_identifier_Identifier *);
%newobject get_an_identity(void);
%newobject create_basic_identity(uint8_t (*)[32]);
%newobject get_identity_contract_bounds(struct platform_value_types_identifier_Identifier *identifier, struct platform_value_types_identifier_Identifier *contract_identifier);
%newobject fetch_identity3(struct platform_value_types_identifier_Identifier *identifier);
%newobject fetch_identity(struct platform_value_types_identifier_Identifier *identifier);
%newobject get_document(void);
%newobject fetch_identity2(struct platform_value_types_identifier_Identifier *identifier);
