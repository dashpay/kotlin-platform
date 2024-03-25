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
            Identifier_clone(identifier),
            platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(public_keys),
            balance,
            dpp_prelude_Revision_ctor(revision->_0)
        );
    }

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

    std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * getPublicKeys() {
        return $self->public_keys;
    }
}

%extend dpp_identity_identity_Identity {
    dpp_identity_identity_Identity(dpp_identity_v0_IdentityV0 * identityV0) {
        return dpp_identity_identity_Identity_V0_ctor(platform_mobile_identity_IdentityV0_clone(identityV0));
    }
    ~dpp_identity_identity_Identity() {
        printf("~Identity(%lx)\n", (uint64_t)$self);
        dpp_identity_identity_Identity_destroy($self);
    }
}
%rename(Identity) dpp_identity_identity_Identity;
//%rename(Identity_Tag) dpp_identity_identity_Identity_Tag;
//%rename(IdentityV0Type) dpp_identity_identity_Identity_V0;

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

%rename (IdentityResult) Result_ok_dpp_identity_identity_Identity_err_String;
%extend Result_ok_dpp_identity_identity_Identity_err_String {
    ~Result_ok_dpp_identity_identity_Identity_err_String() {
        Result_ok_dpp_identity_identity_Identity_err_String_destroy($self);
    }
}
%ignore platform_mobile_identity_std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_clone(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys);
%ignore dpp_identity_v0_IdentityV0_ctor(platform_value_types_identifier_Identifier *id,
                                                            std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys,
                                                            uint64_t balance,
                                                            dpp_prelude_Revision *revision);
struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey;

%include "stdint.i"

%typemap(javaclassname) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(javatype) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(jtype) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(jstype) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "java.util.Map<KeyID, IdentityPublicKey>"
%typemap(jni) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* "jobject"

%typemap(out) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* {
    $result = fermented_tree_to_java_map_KeyID_IdentityPublicKey(jenv, $1);
}

%typemap(in) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* {
    $1 = java_map_KeyID_IdentityPublicKey_to_fermented_tree(jenv, $input);
}

%typemap(freearg) struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey* {
    free($1.keys);
    free($1.values);
}

%typemap(javain) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * "$javainput"

%typemap(javaout) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey * {
    return $jnicall;
  }


%typemap(throws) std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *
%{ SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "null std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey");
   return $null; %}

%apply struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey {struct std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey};
%ignore std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey;
%ignore dpp_identity_v0_IdentityV0_ctor(platform_value_types_identifier_Identifier *id,
                                                                    std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *public_keys,
                                                                    uint64_t balance,
                                                                    dpp_prelude_Revision *revision);
%ignore dpp_identity_v0_IdentityV0_get_public_keys(const dpp_identity_v0_IdentityV0 *obj);
%ignore dpp_identity_v0_IdentityV0_set_public_keys(dpp_identity_v0_IdentityV0 *obj,
                                                std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *value);
%ignore std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_destroy(std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey *ffi);