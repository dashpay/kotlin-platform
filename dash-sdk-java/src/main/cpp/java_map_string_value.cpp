//
// Created by Eric Britten on 3/24/24.
//
#include <jni.h>
#include "conversions.h"
#include <cstring>
#include "clone.h"
#include "utils.h"

// TODO: This function needs to handle all of the variants
FermentMapStringPlatformValue * java_map_String_Value_to_fermented_ValueMap(JNIEnv * jenv, jobject input) {
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
    auto **keys = (char **) malloc(
            entryCount * sizeof(dpp_identity_identity_public_key_KeyID * ));
    auto **values = (platform_value_Value **) malloc(
            entryCount * sizeof(dpp_identity_identity_public_key_IdentityPublicKey * ));
    FermentMapStringPlatformValue * result = std_collections_Map_keys_String_values_platform_value_Value_ctor(count, keys, values);

   jclass platformValueClass = jenv->FindClass("org/dashj/platform/sdk/PlatformValue");
   jmethodID platformValuePtrMethod = jenv->GetMethodID(platformValueClass, "getCPointer", "()J");

   jint i = 0;
   while (jenv->CallBooleanMethod(iterator, hasNextMethod)) {
       jobject entry = jenv->CallObjectMethod(iterator, nextMethod);

       jobject keyObject = jenv->CallObjectMethod(entry, getKeyMethod);
       jobject valueObject = jenv->CallObjectMethod(entry, getValueMethod);

        const char *keyString = jenv->GetStringUTFChars(static_cast<jstring>(keyObject), nullptr);
        result->keys[i] = strdup(keyString); 
        jenv->ReleaseStringUTFChars(static_cast<jstring>(keyObject), keyString);

       jlong nativePtr = jenv->CallLongMethod(valueObject, platformValuePtrMethod);
       auto *value = reinterpret_cast<platform_value_Value *>(nativePtr);
       result->values[i] = platform_mobile_clone_Value_clone(value);

       i++;
    }
    return result;
}
jobject fermented_tree_to_java_map_String_Value(JNIEnv * jenv, FermentMapStringPlatformValue * input) {
    jclass hashMapClass = jenv->FindClass("java/util/HashMap");
    jmethodID hashMapInit = jenv->GetMethodID(hashMapClass, "<init>", "()V");
    jobject hashMapInstance = jenv->NewObject(hashMapClass, hashMapInit);
    jmethodID putMethod = jenv->GetMethodID(hashMapClass, "put",
                                            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");

    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/PlatformValue");
    jmethodID valueConstructor = jenv->GetMethodID(valueClass, "<init>", "(JZ)V");

    LOGI("platform_value_Value size: %d, alignment %d", sizeof(platform_value_Value), alignof(platform_value_Value));

    for (uintptr_t i = 0; i < input->count; ++i) {
        jobject key = jenv->NewStringUTF(input->keys[i]);
        auto * valueClone = clone(input->values[i]);
        LOGI("converting value %d: \'%x\'[tag=%d]\n", i, input->values[i], input->values[i]->tag);
        LOGI("Value::? = %s", getHex((unsigned char *)input->values[i], sizeof(platform_value_Value)));
//        LOGI("Value::? + 4: %s", (long)(
//            ((platform_value_Value*)((uint8_t*)input->values[i]+4))->text._0)
//
//            );
        if (input->values[i]->tag == platform_value_Value::Tag::Text) {
            LOGI("  converting value[text]: %lx", (long)input->values[i]->text._0);
            if ((uint64_t)input->values[i]->text._0 > 100)
                LOGI("  converting value[text]: %s", (long)input->values[i]->text._0);
        } else if (input->values[i]->tag == platform_value_Value::Tag::Identifier) {
            LOGI("  converting value[identifier]: %lx", (long)input->values[i]->identifier._0);
        } else if (input->values[i]->tag == platform_value_Value::Tag::Map) {
            LOGI("  converting value[map]: %lx", (long)input->values[i]->map._0);
        }
        LOGI("cloned value %d: \'%x\'[tag=%d]\n", i, valueClone, valueClone->tag);
        LOGI("Value::? = %s", getHex((unsigned char *)valueClone, sizeof(platform_value_Value)));
//                LOGI("Value::? + 4: %s", (long)(
//                    ((platform_value_Value*)((uint8_t*)valueClone+4))->text._0)
//
//                    );
        if (input->values[i]->tag == platform_value_Value::Tag::Text) {
            LOGI("  cloned value[text]: %lx", (long)valueClone->text._0);
            if ((uint64_t)valueClone->text._0 > 100)
                LOGI("  cloned value[text]: %s", (long)valueClone->text._0);
        } else if (input->values[i]->tag == platform_value_Value::Tag::Identifier) {
            LOGI("  cloned value[identifier]: %lx", (long)valueClone->identifier._0);
        } else if (input->values[i]->tag == platform_value_Value::Tag::Map) {
            LOGI("  cloned value[map]: %lx", (long)valueClone->map._0);
        }
        jobject value = jenv->NewObject(valueClass, valueConstructor, (jlong) valueClone, static_cast<jboolean>(true));
        jenv->CallObjectMethod(hashMapInstance, putMethod, key, value);
    }

    return hashMapInstance;
}