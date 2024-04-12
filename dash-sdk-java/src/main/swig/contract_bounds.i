%rename (ContractBounds) dpp_identity_identity_public_key_contract_bounds_ContractBounds;
%rename (ContractBounds_Tag) dpp_identity_identity_public_key_contract_bounds_ContractBounds_Tag;
%rename (SingleContract) dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContract;
%rename (SingleContractDocumentType) dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContractDocumentType;

%extend dpp_identity_identity_public_key_contract_bounds_ContractBounds {
    dpp_identity_identity_public_key_contract_bounds_ContractBounds(platform_value_types_identifier_Identifier * id) {
        dpp_identity_identity_public_key_contract_bounds_ContractBounds * cb = dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContract_ctor(Identifier_clone(id));
        printf("ContractBounds: %lx->%lx\n", (unsigned long)cb, cb->single_contract.id);
        return cb;
    }
    dpp_identity_identity_public_key_contract_bounds_ContractBounds(platform_value_types_identifier_Identifier * id, char * document_type) {
        return dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContractDocumentType_ctor(Identifier_clone(id), memoryFactory.clone(document_type));
    }
    ~dpp_identity_identity_public_key_contract_bounds_ContractBounds() {
        printf("~ContractBounds: %lx->%lx\n", (unsigned long)$self, $self->single_contract.id);
        dpp_identity_identity_public_key_contract_bounds_ContractBounds_destroy($self);
    }
};

%newobject singleContract(platform_value_types_identifier_Identifier *);
%newobject singleContractDocumentType(platform_value_types_identifier_Identifier *, char *);
%ignore dpp_identity_identity_public_key_contract_bounds_ContractBounds_ctor(struct platform_value_types_identifier_Identifier *id);
%ignore dpp_identity_identity_public_key_contract_bounds_ContractBoundsDocumentType_ctor(struct platform_value_types_identifier_Identifier *id, char *document_type_name);
%rename(SingleContractStruct) dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContract_Body;
%rename(SingleContractDocumentTypeStruct) dpp_identity_identity_public_key_contract_bounds_ContractBounds_SingleContractDocumentType_Body;
