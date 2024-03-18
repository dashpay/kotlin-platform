%ignore Purpose_AUTHENTICATION_ctor(void);
%ignore Purpose_ENCRYPTION_ctor(void);
%ignore Purpose_DECRYPTION_ctor(void);
%ignore Purpose_WITHDRAW_ctor(void);
%ignore Purpose_SYSTEM_ctor(void);
%ignore Purpose_VOTING_ctor(void);
%ignore Purpose_destroy(enum Purpose *ffi);
%ignore intToPurpose(int);
%ignore dpp_identity_identity_Identity_V0_ctor(struct KeyID *id,
     enum Purpose *purpose,
     enum SecurityLevel* security_level,
     struct ferment_example_identity_identity_ContractBounds *contract_bounds,
     enum KeyType *key_type,
     bool read_only,
     struct BinaryData *data,
     struct TimestampMillis *disabled_at);
     
%ignore SecurityLevel_MASTER_ctor(void);
%ignore SecurityLevel_CRITICAL_ctor(void);
%ignore SecurityLevel_HIGH_ctor(void);
%ignore SecurityLevel_MEDIUM_ctor(void);
%ignore SecurityLevel_destroy(enum SecurityLevel *ffi);
%ignore intToSecurityLevel(int);
%ignore intToKeyType(int);
%ignore KeyType_ECDSA_SECP256K1_ctor(void);
%ignore KeyType_BLS12_381_ctor(void);
%ignore KeyType_ECDSA_HASH160_ctor(void);
%ignore KeyType_BIP13_SCRIPT_HASH_ctor(void);
%ignore KeyType_EDDSA_25519_HASH160_ctor(void);
%ignore KeyType_destroy(enum KeyType *ffi);