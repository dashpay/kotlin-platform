// Typemaps for the native state-transition deserialization API (platform-mobile/src/state_transition.rs).
// identity_update_public_keys_to_add returns Result<Vec<IdentityPublicKey>>; the Vec<IdentityPublicKey>
// list typemap and clone already exist (identity_public_key.i / clone.h). Here we map the Result to
// Result<List<IdentityPublicKey>> and suppress the raw ctor/destroy wrappers.

DEFINE_LIST_RESULT(
    IdentityPublicKey,
    String,
    Result_ok_Vec_dpp_identity_identity_public_key_IdentityPublicKey_err_String,
    dpp_identity_identity_public_key_IdentityPublicKey
);

%ignore Result_ok_Vec_dpp_identity_identity_public_key_IdentityPublicKey_err_String_ctor(Vec_dpp_identity_identity_public_key_IdentityPublicKey *ok, char *error);
%ignore Result_ok_Vec_dpp_identity_identity_public_key_IdentityPublicKey_err_String_destroy(Result_ok_Vec_dpp_identity_identity_public_key_IdentityPublicKey_err_String *ffi);
