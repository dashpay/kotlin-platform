Critical 32-bit Compilation Issues

1. Pointer Size Issues with uintptr_t

Location: Multiple SWIG files use uintptr_t for loop counters and array indices
- identity.i:37 - for (uintptr_t i = 0; i < $self->public_keys->count; ++i)
- generics/primitive_lists.i:30 - $1 = STRUCT_TYPE##_ctor((uintptr_t)size, values);
- generics/primitive_lists.i:64 - for (uintptr_t i = 0; i < $1->count; ++i)

Problem: uintptr_t changes from 32-bit to 64-bit between architectures, affecting struct layouts and function signatures.

2. 128-bit Integer Emulation Problems

Location: int128.h:4-205

Issues:
- Line 143: int128_t& operator>>=(int32_t shift) - Right shift implementation has incorrect sign extension for negative values on line 146: high = 0 should preserve sign bit
- Line 155: Left shift doesn't handle arithmetic overflow correctly
- Line 188: int128_t& operator=(int32_t value) sign extension logic may fail for edge cases

3. SWIG Type Mapping Issues

Location: i128.i:10-33

Problems:
- Lines 17-29: The int128_t conversion assumes native __int128 operations that don't exist on 32-bit platforms
- Line 22: $1 <<= 8; $1 |= ((uint8_t)bytes[i] & 0xFF); uses native 128-bit operations
- Line 29: $1 = -($1 - (int128_t(1))); incorrect two's complement conversion

4. Function Callback Pointer Issues

Location: sdk.i:2-3, identity.i:71-72

Problem: uint64_t callback parameters become problematic on 32-bit systems where pointers are 32-bit but the code expects 64-bit values.

5. Memory Layout Issues

Location: identity.i:44-45

Problem: long long getBalance() { return (long)$self->balance; } - casting uint64_t to long truncates on 32-bit systems where long is 32-bit.

Recommended Fixes

1. Replace uintptr_t with fixed-size types where array indexing is involved
2. Fix int128.h arithmetic operations for proper 32-bit emulation
3. Update SWIG typemaps in i128.i to handle synthetic int128 types
4. Use proper size types for balance and callback parameters
5. Add 32-bit specific compilation flags in CMakeLists.txt

The most critical issues are in the 128-bit integer emulation (int128.h:143-163) and SWIG type conversion (i128.i:17-29), which will cause runtime failures on 32-bit platforms.