//
// Created by Eric Britten on 3/24/24.
//
#include <jni.h>
#include "conversions.h"
#include "clone.h"

// TODO: This function needs to handle all of the variants
// TODO: there is a bug with handling some types such as lists
FermentVectorValueMapTuple * java_map_Value_Value_to_fermented_ValueMap(JNIEnv * jenv, jobject input) {
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
    auto **values = (Tuple_platform_value_Value_platform_value_Value **) malloc(
            entryCount * sizeof(Vec_Tuple_platform_value_Value_platform_value_Value * ));
    FermentVectorValueMapTuple * result = Vec_Tuple_platform_value_Value_platform_value_Value_ctor(count, values);

    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/PlatformValue");
   jmethodID getNativePtrMethod = jenv->GetMethodID(valueClass, "getCPointer", "()J");

   jint i = 0;
   //printf("ready for the loop to process map: %lx\n", result);
   while (jenv->CallBooleanMethod(iterator, hasNextMethod)) {
       // printf(" item %d\n", i);
       jobject entry = jenv->CallObjectMethod(iterator, nextMethod);

       jobject keyObject = jenv->CallObjectMethod(entry, getKeyMethod);
       jobject valueObject = jenv->CallObjectMethod(entry, getValueMethod);
       // printf(" get objects %d\n", i);

       auto *keyID = (platform_value_Value *) jenv->CallLongMethod(
               keyObject, getNativePtrMethod);
      //         printf(" key %d - get ptr = %lx\n", i, keyID);
       auto keyClone = platform_mobile_clone_Value_clone(keyID);
       //               printf(" key %d - clone = %lx\n", i, keyClone);

       jlong nativePtr = jenv->CallLongMethod(valueObject, getNativePtrMethod);
       auto * valuePtr = reinterpret_cast<platform_value_Value *>(nativePtr);
       auto * valueClone = platform_mobile_clone_Value_clone(valuePtr);
       result->values[i] = Tuple_platform_value_Value_platform_value_Value_ctor(keyClone, valueClone);

       i++;
   }
    return result;
}

jobject fermented_tree_to_java_map_Value_Value(JNIEnv * jenv, FermentVectorValueMapTuple * input) {
    jclass mapClass = jenv->FindClass("java/util/HashMap");
    jmethodID hashMapInit = jenv->GetMethodID(mapClass, "<init>", "()V");
    jobject hashMapInstance = jenv->NewObject(mapClass, hashMapInit);
    jmethodID putMethod = jenv->GetMethodID(mapClass, "put",
                                            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");

    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/PlatformValue");
    jmethodID valueConstructor = jenv->GetMethodID(valueClass, "<init>", "(JZ)V");
    if (input == nullptr) {
        LOGI("  null pointer input");
        return nullptr;
    }

    for (uintptr_t i = 0; i < input->count; ++i) {
        LOGI("fermented_tree_to_java_map_Value_Value");
            if (input->values[0] == nullptr) {
                    LOGI("  null pointer input value[i]");
                    return nullptr;
                }
                            if (input->values[0]->o_0 == nullptr) {
                                    LOGI("  null pointer input value[i].first");
                                    return nullptr;
                                }

                                if (input->values[0]->o_1 == nullptr) {
                                                                    LOGI("  null pointer input value[i].second");
                                                                    return nullptr;
                                                                }
        LOGI("converting key %d: \'%x\'[tag=%d]\n", i, input->values[i]->o_0, input->values[i]->o_0->tag);
        if (input->values[i]->o_0->tag == platform_value_Value::Tag::Text) {
            LOGI("  converting key[text]: %lx", (long)input->values[i]->o_0->text._0);
        } else if (input->values[i]->o_0->tag == platform_value_Value::Tag::Identifier) {
            LOGI("  converting key[identifier]: %lx", (long)input->values[i]->o_0->identifier._0);
        } else if (input->values[i]->o_0->tag == platform_value_Value::Tag::Map) {
            LOGI("  converting key[map]: %lx", (long)input->values[i]->o_0->map._0);
        }
        LOGI("converting key %d: \'%x\'[tag=%d]\n", i, input->values[i]->o_1, input->values[i]->o_1->tag);
        if (input->values[i]->o_1->tag == platform_value_Value::Tag::Text) {
            LOGI("  converting value[text]: %lx", (long)input->values[i]->o_1->text._0);
        } else if (input->values[i]->o_1->tag == platform_value_Value::Tag::Identifier) {
            LOGI("  converting value[identifier]: %lx", (long)input->values[i]->o_1->identifier._0);
        } else if (input->values[i]->o_1->tag == platform_value_Value::Tag::Map) {
            LOGI("  converting value[map]: %lx", (long)input->values[i]->o_1->map._0);
        }
        auto clonedKey = clone(input->values[i]->o_0);
        auto clonedValue = clone(input->values[i]->o_1);
        jobject key = jenv->NewObject(valueClass, valueConstructor, (jlong) clonedKey, static_cast<jboolean>(true));
        jobject value = jenv->NewObject(valueClass, valueConstructor, (jlong) clonedValue, static_cast<jboolean>(true));
        jenv->CallObjectMethod(hashMapInstance, putMethod, key, value);
    }

    return hashMapInstance;
}