#ifndef __int128_h
#define __int128_h

#if UINTPTR_MAX == 0xFFFFFFFF
#define SYNTHETIC_INT128
struct __attribute__((aligned(16))) uint128_t {
    uint64_t low;
    uint64_t high;

    uint128_t() : low(0), high(0) {}

    uint128_t(uint64_t high, uint64_t low) : low(low), high(high) {}

    uint128_t(const uint128_t & other) : low(other.low), high(other.high) {}

    uint128_t operator+(const uint128_t& other) const {
        uint128_t result;
        result.low = low + other.low;
        result.high = high + other.high + (result.low < low); // Handle carry
        return result;
    }

    uint128_t operator-(const uint128_t& other) const {
        uint128_t result;
        result.low = low - other.low;
        result.high = high - other.high - (low < other.low); // Handle borrow
        return result;
    }

    // Bitwise AND
    uint128_t operator&(const uint128_t& other) const {
        return uint128_t(high & other.high, low & other.low);
    }

    // Bitwise OR
    uint128_t operator|(const uint128_t& other) const {
        return uint128_t(high | other.high, low | other.low);
    }

    // Bitwise XOR
    uint128_t operator^(const uint128_t& other) const {
        return uint128_t(high ^ other.high, low ^ other.low);
    }

    // Bitwise NOT
    uint128_t operator~() const {
        return uint128_t(~high, ~low);
    }

    // Right shift (>>=)
    uint128_t& operator>>=(uint32_t shift) {
        if (shift >= 64) {
            low = high >> (shift - 64);
            high = 0;
        } else if (shift > 0) {
            low = (low >> shift) | (high << (64 - shift));
            high >>= shift;
        }
        return *this;
    }

    // Left shift (<<=)
    uint128_t& operator<<=(uint32_t shift) {
        if (shift >= 64) {
            high = low << (shift - 64);
            low = 0;
        } else if (shift > 0) {
            high = (high << shift) | (low >> (64 - shift));
            low <<= shift;
        }
        return *this;
    }
    operator unsigned char() {
        return (unsigned char)(low & 0xFF);
    }
    uint128_t& operator=(const uint128_t& other) {
         if (this != &other) {  // Self-assignment check
             high = other.high;
             low = other.low;
         }
         return *this;
     }

     // Bitwise OR assignment (|=)
     uint128_t& operator|=(const uint128_t& other) {
         high |= other.high;
         low |= other.low;
         return *this;
     }

};

struct __attribute__((aligned(16))) int128_t {
    uint64_t low;
    int64_t high;

    int128_t() : low(0), high(0) {}

    int128_t(int64_t high, uint64_t low) : low(low), high(high) {}

    int128_t(const int128_t & other) : low(other.low), high(other.high) {}

    int128_t(int32_t other) {
        low = static_cast<uint64_t>(other);
        high = (other < 0) ? -1 : 0;  // Sign extend based on the 32-bit value
    }

    int128_t operator+(const int128_t& other) const {
        int128_t result;
        result.low = low + other.low;
        result.high = high + other.high + (result.low < low); // Handle carry
        return result;
    }

    int128_t operator-(const int128_t& other) const {
        int128_t result;
        result.low = low - other.low;
        result.high = high - other.high - (low < other.low); // Handle borrow
        return result;
    }

    // Bitwise AND
    int128_t operator&(const int128_t& other) const {
        return int128_t(high & other.high, low & other.low);
    }

    // Bitwise OR
    int128_t operator|(const int128_t& other) const {
        return int128_t(high | other.high, low | other.low);
    }

    // Bitwise XOR
    int128_t operator^(const int128_t& other) const {
        return int128_t(high ^ other.high, low ^ other.low);
    }

    // Bitwise NOT
    int128_t operator~() const {
        return int128_t(~high, ~low);
    }

    // Right shift (>>=)
    int128_t& operator>>=(int32_t shift) {
        if (shift >= 64) {
            low = high >> (shift - 64);
            high = 0;
        } else if (shift > 0) {
            low = (low >> shift) | (high << (64 - shift));
            high >>= shift;
        }
        return *this;
    }

    // Left shift (<<=)
    int128_t& operator<<=(int32_t shift) {
        if (shift >= 64) {
            high = low << (shift - 64);
            low = 0;
        } else if (shift > 0) {
            high = (high << shift) | (low >> (64 - shift));
            low <<= shift;
        }
        return *this;
    }
    operator unsigned char() {
        return (unsigned char)(low & 0xFF);
    }
    int128_t& operator=(const int128_t& other) {
        if (this != &other) {  // Self-assignment check
            high = other.high;
            low = other.low;
        }
        return *this;
    }

    // Bitwise OR assignment (|=)
    int128_t& operator|=(const int128_t& other) {
        high |= other.high;
        low |= other.low;
        return *this;
    }
    // Bitwise OR assignment (|=)
    int128_t& operator|=(int8_t other) {
        low |= other;
        return *this;
    }
    int128_t& operator=(int32_t value) {
        low = static_cast<uint64_t>(value);
        high = (value < 0) ? -1 : 0;  // Sign extend based on the 32-bit value
        return *this;
    }
};

typedef int128_t i128;
typedef uint128_t u128;

#else

typedef __int128 i128;
typedef unsigned __int128 u128;

// are these defined?
typedef __int128 int128_t;
typedef unsigned __int128 uint128_t;
#endif

#endif // __int128_h