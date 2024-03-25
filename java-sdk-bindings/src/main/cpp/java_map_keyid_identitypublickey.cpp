//
// Created by Eric Britten on 3/24/24.
//
#include <jni.h>
#include "conversions.h"

FermentMapKeyIDIdentityPublicKey * java_map_KeyID_IdentityPublicKey_to_fermented_tree(JNIEnv * jenv, jobject input) {
// Get the Set of entries from the Map
    jclass mapClass = jenv->FindClass("java/util/Map");
    jmethodID entrySetMethod = jenv->GetMethodID(mapClass, "entrySet", "()Ljava/util/Set;");
    jobject setOfEntries = jenv->CallObjectMethod(input, entrySetMethod);

// Get iterator for the set
    jclass setClass = jenv->FindClass("java/util/Set");
    jmethodID iteratorMethod = jenv->GetMethodID(setClass, "iterator", "()Ljava/util/Iterator;");
    jobject iterator = jenv->CallObjectMethod(setOfEntries, iteratorMethod);

// Get Iterator class and methods
    jclass iteratorClass = jenv->FindClass("java/util/Iterator");
    jmethodID hasNextMethod = jenv->GetMethodID(iteratorClass, "hasNext", "()Z");
    jmethodID nextMethod = jenv->GetMethodID(iteratorClass, "next", "()Ljava/lang/Object;");

// Get Map.Entry class and methods
    jclass entryClass = jenv->FindClass("java/util/Map$Entry");
    jmethodID getKeyMethod = jenv->GetMethodID(entryClass, "getKey", "()Ljava/lang/Object;");
    jmethodID getValueMethod = jenv->GetMethodID(entryClass, "getValue", "()Ljava/lang/Object;");

// Count the number of entries and allocate memory
    jint entryCount = jenv->CallIntMethod(setOfEntries, jenv->GetMethodID(setClass, "size", "()I"));
    int count = (uintptr_t) entryCount;
    auto **keys = (dpp_identity_identity_public_key_KeyID **) malloc(
            entryCount * sizeof(dpp_identity_identity_public_key_KeyID * ));
    auto **values = (dpp_identity_identity_public_key_IdentityPublicKey **) malloc(
            entryCount * sizeof(dpp_identity_identity_public_key_IdentityPublicKey * ));
    FermentMapKeyIDIdentityPublicKey * result =
            std_collections_Map_keys_dpp_identity_identity_public_key_KeyID_values_dpp_identity_identity_public_key_IdentityPublicKey_ctor(
            count, keys, values);

    jclass keyIDClass = jenv->FindClass("org/dashj/platform/sdk/KeyID");
    jmethodID keyIDPtrMethod = jenv->GetMethodID(keyIDClass, "getCPointer", "()J");
    jclass identityPublicKeyClass = jenv->FindClass("org/dashj/platform/sdk/IdentityPublicKey");
    jmethodID getNativePtrMethod = jenv->GetMethodID(identityPublicKeyClass, "getCPointer", "()J");

    jint i = 0;
    while (jenv->CallBooleanMethod(iterator, hasNextMethod)) {
        jobject entry = jenv->CallObjectMethod(iterator, nextMethod);

        jobject keyObject = jenv->CallObjectMethod(entry, getKeyMethod);
        jobject valueObject = jenv->CallObjectMethod(entry, getValueMethod);

        auto *keyID = (dpp_identity_identity_public_key_KeyID *) jenv->CallLongMethod(
                keyObject, keyIDPtrMethod);
        result->keys[i] = dpp_identity_identity_public_key_KeyID_ctor(keyID->_0);


        jlong nativePtr = jenv->CallLongMethod(valueObject, getNativePtrMethod);
        auto *ipk = reinterpret_cast<dpp_identity_identity_public_key_IdentityPublicKey *>(nativePtr);
        result->values[i] = platform_mobile_identity_IdentityPublicKey_clone(ipk);

        i++;
    }
    return result;
}

jobject fermented_tree_to_java_map_KeyID_IdentityPublicKey(JNIEnv * jenv, FermentMapKeyIDIdentityPublicKey * input) {
    jclass hashMapClass = jenv->FindClass("java/util/HashMap");
    printf("hashMapClass = 0x%ld\n", (long) hashMapClass);
    jmethodID hashMapInit = jenv->GetMethodID(hashMapClass, "<init>", "()V");
    printf("hashMapInit = 0x%ld\n", (long) hashMapInit);
    jobject hashMapInstance = jenv->NewObject(hashMapClass, hashMapInit);
    printf("hashMapInstance = 0x%ld", (long) hashMapInstance);
    jmethodID putMethod = jenv->GetMethodID(hashMapClass, "put",
                                            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");

    jclass keyIDClass = jenv->FindClass("org/dashj/platform/sdk/KeyID");
    jmethodID keyIDConstructor = jenv->GetMethodID(keyIDClass, "<init>", "(JZ)V");
    jclass jclass_IdentityPublicKey = jenv->FindClass("org/dashj/platform/sdk/IdentityPublicKey");
    jmethodID jmethodID_IdentityPublicKey_constructor = jenv->GetMethodID(keyIDClass, "<init>", "(JZ)V");

    for (uintptr_t i = 0; i < input->count; ++i) {

        jobject key = jenv->NewObject(keyIDClass, keyIDConstructor, (jlong) input->keys[i], false);

        jobject value = jenv->NewObject(jclass_IdentityPublicKey, jmethodID_IdentityPublicKey_constructor,
                                        (jlong) input->values[i], false);

        jenv->CallObjectMethod(hashMapInstance, putMethod, key, value);
    }

    return hashMapInstance;
}