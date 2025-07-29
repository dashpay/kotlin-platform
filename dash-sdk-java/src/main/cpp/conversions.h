//
// Created by Eric Britten on 3/24/24.
//

#ifndef SDK_CONVERSIONS_H
#define SDK_CONVERSIONS_H

#include <jni.h>
#include "config.h"
#include "../../../../dash-sdk-bindings/target/include/dash_sdk_bindings.h"

typedef std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey FermentMapKeyIDIdentityPublicKey;
jobject fermented_tree_to_java_map_KeyID_IdentityPublicKey(JNIEnv * jenv, FermentMapKeyIDIdentityPublicKey * input);
FermentMapKeyIDIdentityPublicKey * java_map_KeyID_IdentityPublicKey_to_fermented_tree(JNIEnv * jenv, jobject input);

typedef Vec_Tuple_platform_value_Value_platform_value_Value FermentVectorValueMapTuple;
FermentVectorValueMapTuple * java_map_Value_Value_to_fermented_ValueMap(JNIEnv * jenv, jobject input);
jobject fermented_tree_to_java_map_Value_Value(JNIEnv * jenv, FermentVectorValueMapTuple * input);

typedef std_collections_Map_keys_String_values_platform_value_Value FermentMapStringPlatformValue;
FermentMapStringPlatformValue * java_map_String_Value_to_fermented_ValueMap(JNIEnv * jenv, jobject input);
jobject fermented_tree_to_java_map_String_Value(JNIEnv * jenv, FermentMapStringPlatformValue * input);

typedef Vec_platform_value_Value FermentArrayPlatformValue;
FermentArrayPlatformValue * java_list_Value_to_fermented_vec_Value(JNIEnv * jenv, jobject input);
jobject fermented_vec_to_java_list_Value(JNIEnv * jenv, FermentArrayPlatformValue * input);

#endif //SDK_CONVERSIONS_H
