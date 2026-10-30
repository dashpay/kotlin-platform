#ifndef SDK_IDENTITY_PUBLIC_KEY_V1_H
#define SDK_IDENTITY_PUBLIC_KEY_V1_H

struct platform_value_Value;
#include "../../../../dash-sdk-bindings/target/include/dash_sdk_bindings.h"

static_assert(sizeof(KeyID) == 4, "Native Rust KeyID must be u32");
static_assert(sizeof(Purpose) == 1, "Native Rust Purpose must be u8");
static_assert(sizeof(SecurityLevel) == 1, "Native Rust SecurityLevel must be u8");
static_assert(sizeof(KeyType) == 1, "Native Rust KeyType must be u8");

inline dpp_identity_identity_public_key_purpose_Purpose identityPublicKeyV1Purpose(
    const dpp_identity_identity_public_key_v1_IdentityPublicKeyV1 *key) {
    return static_cast<dpp_identity_identity_public_key_purpose_Purpose>(*key->purpose);
}

inline dpp_identity_identity_public_key_security_level_SecurityLevel identityPublicKeyV1SecurityLevel(
    const dpp_identity_identity_public_key_v1_IdentityPublicKeyV1 *key) {
    return static_cast<dpp_identity_identity_public_key_security_level_SecurityLevel>(*key->security_level);
}

inline dpp_identity_identity_public_key_key_type_KeyType identityPublicKeyV1KeyType(
    const dpp_identity_identity_public_key_v1_IdentityPublicKeyV1 *key) {
    return static_cast<dpp_identity_identity_public_key_key_type_KeyType>(*key->key_type);
}

#endif
