#include "identity_public_key_v1.h"
#include <array>
#include <cassert>

int main() {
    // Nonzero trailing bytes make an accidental four-byte enum read fail deterministically.
    std::array<uint8_t, 4> purpose {0, 0xce, 0xab, 0x0b};
    std::array<uint8_t, 4> security {2, 0x61, 0x63, 0x63};
    std::array<uint8_t, 4> type {2, 0xff, 0xee, 0xdd};
    dpp_identity_identity_public_key_v1_IdentityPublicKeyV1 key {};
    key.purpose = purpose.data();
    key.security_level = security.data();
    key.key_type = type.data();
    for (uint8_t value = 0; value <= 6; ++value) {
        purpose[0] = value;
        assert(static_cast<int>(identityPublicKeyV1Purpose(&key)) == value);
    }
    for (uint8_t value = 0; value <= 3; ++value) {
        security[0] = value;
        assert(static_cast<int>(identityPublicKeyV1SecurityLevel(&key)) == value);
    }
    for (uint8_t value = 0; value <= 4; ++value) {
        type[0] = value;
        assert(static_cast<int>(identityPublicKeyV1KeyType(&key)) == value);
    }
}
