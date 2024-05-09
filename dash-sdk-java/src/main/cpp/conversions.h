//
// Created by Eric Britten on 3/24/24.
//

#ifndef SDK_CONVERSIONS_H
#define SDK_CONVERSIONS_H

#include <jni.h>
#include "config.h"
#include "../../../../dash-sdk-bindings/target/dash_sdk_bindings.h"

typedef std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey FermentMapKeyIDIdentityPublicKey;
jobject fermented_tree_to_java_map_KeyID_IdentityPublicKey(JNIEnv * jenv, FermentMapKeyIDIdentityPublicKey * input);
FermentMapKeyIDIdentityPublicKey * java_map_KeyID_IdentityPublicKey_to_fermented_tree(JNIEnv * jenv, jobject input);

typedef Vec_Tuple_platform_value_Value_platform_value_Value FermentVectorValueMapTuple;
FermentVectorValueMapTuple * java_map_Value_Value_to_fermented_ValueMap(JNIEnv * jenv, jobject input);
jobject fermented_tree_to_java_map_Value_Value(JNIEnv * jenv, FermentVectorValueMapTuple * input);
#endif //SDK_CONVERSIONS_H
