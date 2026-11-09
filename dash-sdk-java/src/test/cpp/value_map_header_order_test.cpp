#include <jni.h>
#include <cstdio>
#include "config.h"

// The map type must be known even when Ferment emits Value before ValueMap.
struct ValueMapBeforeGeneratedHeader {
    platform_value_value_map_ValueMap *value;
};

#include "../../../../dash-sdk-bindings/target/include/dash_sdk_bindings.h"
#include <type_traits>

static_assert(std::is_same_v<decltype(ValueMapBeforeGeneratedHeader::value),
                             decltype(platform_value_Value::Map_Body::_0)>);

int main() {}
