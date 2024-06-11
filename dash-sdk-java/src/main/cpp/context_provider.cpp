
#include <jni.h>
#include "config.h"
#include "../../../../dash-sdk-bindings/target/dash_sdk_bindings.h"
#include <cstring>
// fetchIdentity4

//void myFetchIdentity4(platform_value_types_identifier_Identifier *identifier, jobject * callbackObject) {
//    // create lamdas for ContextProvider
//    // Call the right function in jobject
//    platform_mobile_fetch_identity_fetch_identity4(identifier);
//}

jobject contextProvider;
uint8_t invalid_key[] = {
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
};

uint8_t * get_quorum_public_key(int quorum_type, char * quorum_hash, int core_chain_locked_height, uint8_t * native_array) {
    jclass clazz = jenv->FindClass("org/dashj/platform/sdk/callbacks/ContextProvider");
    if (clazz == nullptr) {
        printf("Cannot find class\n");
    }
    jmethodID getQuorumPublicKey = jenv->GetMethodID(clazz, "getQuorumPublicKey", "(I[BI)[B");
    if (getQuorumPublicKey == nullptr) {
        printf("Cannot find getQuorumPublicKey(IB[I)[B\n");
    }
    jbyteArray quorum_hash_bytes = jenv->NewByteArray(32);
    if (quorum_hash_bytes == nullptr) {
        LOGI("Cannot allocate jbyteArray");
        return nullptr;
    }
    jenv->SetByteArrayRegion(quorum_hash_bytes, 0, 32, reinterpret_cast<jbyte *>(quorum_hash));
    if (jenv->ExceptionCheck()) {
        jenv->ExceptionDescribe();
        jenv->ExceptionClear();
        jenv->DeleteLocalRef(quorum_hash_bytes);
        return nullptr;
    }

    auto quorum_public_key = (jbyteArray) jenv->CallObjectMethod(contextProvider, getQuorumPublicKey, quorum_type, quorum_hash_bytes, core_chain_locked_height);
    if (jenv->ExceptionCheck()) {
        jenv->ExceptionDescribe();
        jenv->ExceptionClear();
        jenv->DeleteLocalRef(quorum_hash_bytes);
        return nullptr;
    }

    if (quorum_public_key == nullptr) {
        memcpy(native_array, invalid_key, 48);
    } else {
        jbyte* elements = jenv->GetByteArrayElements(quorum_public_key, nullptr);
        if (elements == nullptr) {
            jenv->DeleteLocalRef(quorum_hash_bytes);
            jenv->DeleteLocalRef(quorum_public_key);
            return nullptr;
        }
        jsize length = jenv->GetArrayLength(quorum_public_key);

        memcpy(native_array, elements, length);
        jenv->ReleaseByteArrayElements(quorum_public_key, elements, JNI_ABORT);
        jenv->DeleteLocalRef(quorum_public_key);
//        printf("C++ is reurning this:  ");
//        for(int i = 0; i < 5; i++)
//            printf("%d, ", native_array[i]);
//        printf("\n");
    }
    return native_array;
}

extern "C" JNIEXPORT jlong JNICALL Java_org_dashj_platform_sdk_callbacks_ContextProvider_getQuorumPublicKeyCallback(JNIEnv * env, jclass provider) {
    if (contextProvider != nullptr) {
        env->DeleteGlobalRef(contextProvider);
        contextProvider = nullptr;
    }

    contextProvider = env->NewGlobalRef(provider);
    if (contextProvider == nullptr) {
        printf("Failed to create global reference for ContextProvider\n");
    }

    return (long)get_quorum_public_key;
}

jobject signer;

int sign_data(u_int8_t * key_data, int key_len, uint8_t * data, int size, uint8_t * result) {
    printf("sign_data(0x%lx, %d, 0x%lx, %d, result=0x%lx)\n", key_data, key_len, data, size, result);
    JniHelper jni;
    JNIEnv * jenv = jni.getEnv();

    jclass clazz = jenv->FindClass("org/dashj/platform/sdk/callbacks/Signer");
    if (clazz == nullptr) {
        printf("Cannot find class\n");
    }
    jmethodID signMethod = jenv->GetMethodID(clazz, "sign", "([B[B)[B");
    if (signMethod == nullptr) {
        printf("Cannot find sign([B[B)[B\n");
    }

    jbyteArray keyByteArray = jenv->NewByteArray(key_len);
    jenv->SetByteArrayRegion(keyByteArray, 0, key_len, reinterpret_cast<jbyte *>(key_data));

    jbyteArray dataByteArray = jenv->NewByteArray(size);
    jenv->SetByteArrayRegion(dataByteArray, 0, size, reinterpret_cast<jbyte *>(data));
    printf("now call the function in the signer class");
    auto binaryDataObject = (jbyteArray) jenv->CallObjectMethod(signer, signMethod, keyByteArray, dataByteArray);
    printf("sign method called = 0x%lx\n", binaryDataObject);

    int length = jenv->GetArrayLength(binaryDataObject);
    jenv->GetByteArrayRegion(binaryDataObject, 0, length, (jbyte *)result);

    return length;
}

extern "C" JNIEXPORT jlong JNICALL Java_org_dashj_platform_sdk_callbacks_Signer_getSignerCallback(JNIEnv * env, jclass signerCallback) {
    if (signer != nullptr) {
        env->DeleteGlobalRef(signer);
        signer = nullptr;
    }

    signer = env->NewGlobalRef(signerCallback);
    if (signer == nullptr) {
        printf("Failed to create global reference for ContextProvider\n");
    }

    return (long)sign_data;
}