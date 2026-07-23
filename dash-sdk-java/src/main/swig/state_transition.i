// Typemaps for the native state-transition deserialization API (platform-mobile/src/state_transition.rs).
// identity_update_public_keys_to_add returns Result<Vec<IdentityPublicKey>>; map it to
// Result<List<IdentityPublicKey>> (the Vec<IdentityPublicKey> item typemap and clone already exist
// via identity_public_key.i / the generated clone.h). The _ctor/_destroy %ignore directives for
// these types are generated into ignore.i by ignore.py.

DEFINE_RESULT(
    platform_mobile_state_transition_StateTransitionInfo,
    String,
    Result_ok_platform_mobile_state_transition_StateTransitionInfo_err_String,
    clone
);

DEFINE_LIST_RESULT(
    IdentityPublicKey,
    String,
    Result_ok_Vec_dpp_identity_identity_public_key_IdentityPublicKey_err_String,
    dpp_identity_identity_public_key_IdentityPublicKey
);
