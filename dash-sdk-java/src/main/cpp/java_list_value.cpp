//
// Created by Eric Britten on 3/24/24.
//
#include <jni.h>
#include "conversions.h"

// TODO: This function needs to handle all of the variants
Vec_platform_value_Value * java_list_Value_to_fermented_vec_Value(JNIEnv * jenv, jobject input) {
    jclass listClass = jenv->FindClass("java/util/List");

    // Get iterator for the set
    jmethodID iteratorMethod = jenv->GetMethodID(listClass, "iterator", "()Ljava/util/Iterator;");
    jobject iterator = jenv->CallObjectMethod(input, iteratorMethod);

// Get Iterator class and methods
    jclass iteratorClass = jenv->FindClass("java/util/Iterator");
    jmethodID hasNextMethod = jenv->GetMethodID(iteratorClass, "hasNext", "()Z");
    jmethodID nextMethod = jenv->GetMethodID(iteratorClass, "next", "()Ljava/lang/Object;");

// Count the number of entries and allocate memory
    jint entryCount = jenv->CallIntMethod(input, jenv->GetMethodID(listClass, "size", "()I"));
    int count = (uintptr_t) entryCount;
    auto **values = (platform_value_Value **) malloc(
            entryCount * sizeof(platform_value_Value * ));
    Vec_platform_value_Value * result = Vec_platform_value_Value_ctor(count, values);

    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/PlatformValue");
   jmethodID getNativePtrMethod = jenv->GetMethodID(valueClass, "getCPointer", "()J");

   jint i = 0;
   //printf("ready for the loop to process List: 0x%lx, count: %d\n", result, result->count);
   while (jenv->CallBooleanMethod(iterator, hasNextMethod)) {
        //printf(" item %d\n", i);
       jobject valueObject = jenv->CallObjectMethod(iterator, nextMethod);

        //printf(" get objects %d\n", i);

       jlong nativePtr = jenv->CallLongMethod(valueObject, getNativePtrMethod);
       auto * valuePtr = reinterpret_cast<platform_value_Value *>(nativePtr);
       auto * valueClone = platform_mobile_clone_Value_clone(valuePtr);
       result->values[i] = valueClone;
       //printf(" value %d - %lx\n", i, result->values[i]);

       i++;
   }
    return result;
}

jobject fermented_vec_to_java_list_Value(JNIEnv * jenv, Vec_platform_value_Value * input) {
    jclass listClass = jenv->FindClass("java/util/ArrayList");
    //printf("listClass = 0x%ld\n", (long) listClass);
    jmethodID hashMapInit = jenv->GetMethodID(listClass, "<init>", "()V");
    //printf("listInit = 0x%ld\n", (long) hashMapInit);
    jobject hashMapInstance = jenv->NewObject(listClass, hashMapInit);
    //printf("listInstance = 0x%ld\n", (long) hashMapInstance);
    jmethodID addMethod = jenv->GetMethodID(listClass, "add",
                                            "(Ljava/lang/Object;)Z");
    //printf("addMethod = 0x%ld\n", (long) addMethod);

    jclass valueClass = jenv->FindClass("org/dashj/platform/sdk/PlatformValue");
    jmethodID valueConstructor = jenv->GetMethodID(valueClass, "<init>", "(JZ)V");
    //printf("PlatformValue: %lx, %lx\n", valueClass, valueConstructor);
    printf("processing list input = 0x%lx, count: %d\n", input, input->count);
    for (uintptr_t i = 0; i < input->count; ++i) {
        printf("%d, %lx, %lx\n", i, input, input->values);
        jobject value = jenv->NewObject(valueClass, valueConstructor,
                                        (jlong) input->values[i], false);
        printf("new object: %lx", value);
        jenv->CallObjectMethod(hashMapInstance, addMethod, value);
    }

    return hashMapInstance;
}