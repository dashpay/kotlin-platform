//
// Created by Eric Britten on 3/24/24.
//
#include <jni.h>
#include "conversions.h"

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
   printf("ready for the loop to process map: %lx\n", result);
   while (jenv->CallBooleanMethod(iterator, hasNextMethod)) {
        printf(" item %d\n", i);
       jobject entry = jenv->CallObjectMethod(iterator, nextMethod);

       jobject keyObject = jenv->CallObjectMethod(entry, getKeyMethod);
       jobject valueObject = jenv->CallObjectMethod(entry, getValueMethod);
        printf(" get objects %d\n", i);

       auto *keyID = (platform_value_Value *) jenv->CallLongMethod(
               keyObject, getNativePtrMethod);
               printf(" key %d - get ptr = %lx\n", i, keyID);
       auto keyClone = platform_mobile_clone_Value_clone(keyID);
                      printf(" key %d - clone = %lx\n", i, keyClone);

       jlong nativePtr = jenv->CallLongMethod(valueObject, getNativePtrMethod);
       auto * valuePtr = reinterpret_cast<platform_value_Value *>(nativePtr);
       auto * valueClone = platform_mobile_clone_Value_clone(valuePtr);
       result->values[i] = Tuple_platform_value_Value_platform_value_Value_ctor(keyClone, valueClone);
        printf(" value %d - original %lx(%d), clone %lx(%d)\n", i, valuePtr, valuePtr->tag, valueClone, valueClone->tag);

       i++;
   }
    return result;
}

jobject fermented_tree_to_java_map_Value_Value(JNIEnv * jenv, FermentVectorValueMapTuple * input) {
    jclass mapClass = jenv->FindClass("java/util/HashMap");
    printf("mapClass = 0x%ld\n", (long) mapClass);
    jmethodID hashMapInit = jenv->GetMethodID(mapClass, "<init>", "()V");
    printf("hashMapInit = 0x%ld\n", (long) hashMapInit);
    jobject hashMapInstance = jenv->NewObject(mapClass, hashMapInit);
    printf("hashMapInstance = 0x%ld\n", (long) hashMapInstance);
    jmethodID putMethod = jenv->GetMethodID(mapClass, "put",
                                            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");

    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/PlatformValue");
    jmethodID valueConstructor = jenv->GetMethodID(valueClass, "<init>", "(JZ)V");

    printf("ready to process map of size %d: %lx\n", input->count, input);
    for (uintptr_t i = 0; i < input->count; ++i) {
        printf("map item %d: %lx (%d)\n", i, input->values[i], input->values[i]->o_1->tag);
        jobject key = jenv->NewObject(valueClass, valueConstructor, (jlong) input->values[i]->o_0, false);
        jobject value = jenv->NewObject(valueClass, valueConstructor,
                                        (jlong) input->values[i]->o_1, false);
        jenv->CallObjectMethod(hashMapInstance, putMethod, key, value);
    }

    return hashMapInstance;
}