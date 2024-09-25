
#include <jni.h>
#include "config.h"
#include "../../../../dash-sdk-bindings/target/dash_sdk_bindings.h"
#include <cstring>
#include "jnihelper.h"
#include "context_provider.h"

uint8_t invalid_key[] = {
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
};

uint8_t * get_quorum_public_key_context(void * context, int quorum_type, char * quorum_hash, int core_chain_locked_height, uint8_t * native_array) {
    auto javaContext = reinterpret_cast<JavaContextProvider*>(context);
    LOGI("get_quorum_public_key_context(0x%08lx, %d, 0x%08lx, %d, 0x%08lx)", context, quorum_type, quorum_hash, core_chain_locked_height, native_array);
    JniHelper jni;
    JNIEnv * jenv = jni.getEnv();
    if (jenv == nullptr) {
        LOGI("Failed to get JNIEnv");
        return nullptr;
    }
    jclass clazz = javaContext->contextProviderClass;
    if (clazz == nullptr) {
        LOGI("Cannot find class\n");
        return nullptr;
    }
    jmethodID getQuorumPublicKey = javaContext->getQuorumPublicKeyMethod;
    if (getQuorumPublicKey == nullptr) {
        LOGI("Cannot find getQuorumPublicKey(IB[I)[B\n");
        return nullptr;
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
        LOGI("get_quorum_public_key_context: SetByteArrayRegion failed");
        return nullptr;
    }

    auto quorum_public_key = (jbyteArray) jenv->CallObjectMethod(javaContext->contextProviderObject, getQuorumPublicKey, quorum_type, quorum_hash_bytes, core_chain_locked_height);
    if (jenv->ExceptionCheck()) {
        jenv->ExceptionDescribe();
        jenv->ExceptionClear();
        jenv->DeleteLocalRef(quorum_hash_bytes);
        LOGI("get_quorum_public_key_context: failure calling Java Callback");
        return nullptr;
    }

    if (quorum_public_key == nullptr) {
        LOGI("get_quorum_public_key_context: key not found");
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
    jenv->DeleteLocalRef(quorum_hash_bytes);

    return native_array;
}

extern "C" JNIEXPORT jlong JNICALL Java_org_dashj_platform_sdk_callbacks_ContextProvider_getQuorumPublicKeyCallback(JNIEnv * env, jclass provider) {
    return (long)get_quorum_public_key_context;
}

extern "C" JNIEXPORT jlong JNICALL Java_org_dashj_platform_sdk_callbacks_ContextProvider_getNativeContextProvider(JNIEnv * env, jclass provider) {
    auto javaContext = new JavaContextProvider;
    auto contextProvider = env->NewGlobalRef(provider);
    if (contextProvider == nullptr) {
        LOGI("Failed to create global reference for ContextProvider\n");
    }
    javaContext->contextProviderObject = contextProvider;
    javaContext->contextProviderClass = env->FindClass("org/dashj/platform/sdk/callbacks/ContextProvider");
    if (javaContext->contextProviderClass == nullptr) {
        LOGI("Cannot find class: org/dashj/platform/sdk/callbacks/ContextProvider\n");
        return 0;
    }
    javaContext->getQuorumPublicKeyMethod = env->GetMethodID(javaContext->contextProviderClass, "getQuorumPublicKey", "(I[BI)[B");
    if (javaContext->getQuorumPublicKeyMethod == nullptr) {
        LOGI("Cannot find getQuorumPublicKey(IB[I)[B\n");
        return 0;
    }
    return (jlong) javaContext;
}

extern "C" JNIEXPORT void JNICALL Java_org_dashj_platform_sdk_callbacks_ContextProvider_freeNativeContextProvider(JNIEnv * env, jobject provider, jlong context) {
    auto javaContext = reinterpret_cast<JavaContextProvider*>(context);
    env->DeleteGlobalRef(javaContext->contextProviderObject);
    delete javaContext;
}

int sign_data_with_context(JavaSigner * context, uint8_t * key_data, int key_len, uint8_t * data, int size, uint8_t * result) {
    LOGI("sign_data(0x%lx, %d, 0x%lx, %d, result=0x%lx)\n", key_data, key_len, data, size, result);
    JniHelper jni;
    JNIEnv * jenv = jni.getEnv();

    jclass clazz = context->signerClass;
    if (clazz == nullptr) {
        LOGI("Cannot find class\n");
        return 0;
    }
    jmethodID signMethod = context->signMethod;
    if (signMethod == nullptr) {
        LOGI("Cannot find sign([B[B)[B\n");
        return 0;
    }

    jbyteArray keyByteArray = jenv->NewByteArray(key_len);
    jenv->SetByteArrayRegion(keyByteArray, 0, key_len, reinterpret_cast<jbyte *>(key_data));

    jbyteArray dataByteArray = jenv->NewByteArray(size);
    jenv->SetByteArrayRegion(dataByteArray, 0, size, reinterpret_cast<jbyte *>(data));
    LOGI("now call the function in the signer class");
    auto binaryDataObject = (jbyteArray) jenv->CallObjectMethod(context->signerObject, signMethod, keyByteArray, dataByteArray);
    LOGI("sign method called = 0x%lx\n", binaryDataObject);

    // check for pending exceptions
    if (jenv->ExceptionCheck()) {
        jenv->ExceptionClear();// Clear the exception
        LOGI("sign: java exception caught");
        return 0; // caller will need to handle this
    }

    int length = jenv->GetArrayLength(binaryDataObject);
    jenv->GetByteArrayRegion(binaryDataObject, 0, length, (jbyte *)result);

    return length;
}

extern "C" JNIEXPORT jlong JNICALL Java_org_dashj_platform_sdk_callbacks_Signer_getNativeSigner(JNIEnv * env, jclass signerCallback) {

    auto javaContext = new JavaSigner;
    auto signer = env->NewGlobalRef(signerCallback);
    if (signer == nullptr) {
        LOGI("Failed to create global reference for Signer\n");
    }
    javaContext->signerObject = signer;
    javaContext->signerClass = env->FindClass("org/dashj/platform/sdk/callbacks/Signer");
    if (javaContext->signerClass == nullptr) {
        LOGI("Cannot find class: org/dashj/platform/sdk/callbacks/Signer\n");
        return 0;
    }
    javaContext->signMethod = env->GetMethodID(javaContext->signerClass, "sign", "([B[B)[B");
    if (javaContext->signMethod == nullptr) {
        LOGI("Cannot find sign(([B[B)[B\n");
        return 0;
    }
    return (jlong) javaContext;
}

extern "C" JNIEXPORT jlong JNICALL Java_org_dashj_platform_sdk_callbacks_Signer_getSignerCallback(JNIEnv * env, jclass provider) {
    return (long)sign_data_with_context;
}

extern "C" JNIEXPORT void JNICALL Java_org_dashj_platform_sdk_callbacks_Signer_freeNativeSigner(JNIEnv * env, jobject provider, jlong context) {
    auto javaContext = reinterpret_cast<JavaSigner*>(context);
    env->DeleteGlobalRef(javaContext->signerObject);
    delete javaContext;
}