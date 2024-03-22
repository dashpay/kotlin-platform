%ignore dpp_identity_identity_public_key_v0_IdentityPublicKeyV0::key_type;
%ignore dpp_identity_identity_public_key_v0_IdentityPublicKeyV0::purpose;
%ignore dpp_identity_identity_public_key_v0_IdentityPublicKeyV0::security_level;
%rename(IdentityPublicKeyV0) dpp_identity_identity_public_key_v0_IdentityPublicKeyV0;
%extend dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 {
    dpp_identity_identity_public_key_v0_IdentityPublicKeyV0(dpp_identity_identity_public_key_KeyID * keyId,
        dpp_identity_identity_public_key_purpose_Purpose purpose,
        dpp_identity_identity_public_key_security_level_SecurityLevel securityLevel,
        dpp_identity_identity_public_key_contract_bounds_ContractBounds * contract_bounds,
        dpp_identity_identity_public_key_key_type_KeyType key_type, bool read_only, platform_value_types_binary_data_BinaryData * data,
        dpp_identity_identity_public_key_TimestampMillis * disabled_at) {

        // enums
        dpp_identity_identity_public_key_purpose_Purpose * purposeObject = intToPurpose(purpose);
        dpp_identity_identity_public_key_key_type_KeyType * keyTypeObject = intToKeyType(key_type);
        dpp_identity_identity_public_key_security_level_SecurityLevel * securityLevelObject = intToSecurityLevel(securityLevel);

        uint8_t * byteArray = (uint8_t*)memoryFactory.alloc(data->_0->count);
        memcpy(byteArray, data->_0->values, data->_0->count);
        Vec_u8 * vec_u8 = Vec_u8_ctor(data->_0->count, byteArray);
        platform_value_types_binary_data_BinaryData * binaryData = platform_value_types_binary_data_BinaryData_ctor(vec_u8);
        printf("  ->data(%lx)\n", (long)binaryData);
        printf("  ->data->_0(%lx)\n", (long)binaryData->_0);
        printf("  ->data->_0->values(%lx)\n", (long)binaryData->_0->values);
        dpp_identity_identity_public_key_contract_bounds_ContractBounds * contract_bounds_copy = nullptr;
        if (contract_bounds != nullptr) {
            if (contract_bounds->tag == dpp_identity_identity_public_key_contract_bounds_ContractBounds::Tag::SingleContract) {
                contract_bounds_copy = dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContract_ctor(Identifier_clone(contract_bounds->single_contract.id));
            } else if (contract_bounds->tag == dpp_identity_identity_public_key_contract_bounds_ContractBounds::Tag::SingleContractDocumentType) {
                char * typeCopy = memoryFactory.clone(contract_bounds->single_contract_document_type.document_type_name);
                contract_bounds_copy = dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContractDocumentType_ctor(Identifier_clone(contract_bounds->single_contract_document_type.id), typeCopy);
            }
        }
        //printf("  ->contract_bounds(%lx): %d\n", (long)contract_bounds, contract_bounds != nullptr ? contract_bounds->tag : -1);
        printf("  ->contract_bounds_copy(%lx)\n", (long)contract_bounds_copy);
        dpp_identity_identity_public_key_KeyID * keyIdObject = dpp_identity_identity_public_key_KeyID_ctor(keyId->_0);
        dpp_identity_identity_public_key_TimestampMillis * disabled_at_copy = disabled_at != nullptr ? dpp_identity_identity_public_key_TimestampMillis_ctor(disabled_at->_0) : nullptr;
        dpp_identity_identity_public_key_v0_IdentityPublicKeyV0 * ipkv0 = dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_ctor(keyIdObject, purposeObject, securityLevelObject,
            contract_bounds_copy,
            keyTypeObject, read_only, binaryData, disabled_at_copy);
        printf("IdentityPublicKeyV0(%lx\n", (long)ipkv0);
        return ipkv0;
    }
    ~dpp_identity_identity_public_key_v0_IdentityPublicKeyV0() {
//         printf("~IdentityPublicKeyV0(%lx)\n", (unsigned long)$self);
//         printf("  ->purpose(%lx)\n", (uint64_t)$self->purpose);
//         printf("  ->data->_0(%lx)\n", (uint64_t)$self->data->_0);
//         printf("  ->data->_0->values(%lx)\n", (uint64_t)$self->data->_0->values);
//         printf("  ->data(%lx)\n", (uint64_t)$self->data);
        dpp_identity_identity_public_key_v0_IdentityPublicKeyV0_destroy($self); //crash
    }
    enum dpp_identity_identity_public_key_key_type_KeyType getKeyType() {
        return *$self->key_type;
    }
    enum dpp_identity_identity_public_key_purpose_Purpose getPurpose() {
        return *$self->purpose;
    }
    enum dpp_identity_identity_public_key_security_level_SecurityLevel getSecurityLevel() {
        return *self->security_level;
    }
}
%rename(IdentityPublicKey) dpp_identity_identity_public_key_IdentityPublicKey;
%rename(IdentityPublicKey_Tag) dpp_identity_identity_public_key_IdentityPublicKey_Tag;

%newobject random_key;
%newobject random_key_args;