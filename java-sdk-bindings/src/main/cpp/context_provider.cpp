
#include <jni.h>
#include "../../../../rs-sdk/target/rs_sdk_bindings.h"
// fetchIdentity4

//void myFetchIdentity4(platform_value_types_identifier_Identifier *identifier, jobject * callbackObject) {
//    // create lamdas for ContextProvider
//    // Call the right function in jobject
//    platform_mobile_fetch_identity_fetch_identity4(identifier);
//}

jobject contextProvider;
JNIEnv * jenv;
uint8_t invalid_key[] = {
        0, 0, 0, 0, 0, 0,0, 0, 0,0, 0, 0,
        0, 0, 0, 0, 0, 0,0, 0, 0,0, 0, 0,
        0, 0, 0, 0, 0, 0,0, 0, 0,0, 0, 0,
        0, 0, 0, 0, 0, 0,0, 0, 0,0, 0, 0,
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
    jenv->SetByteArrayRegion(quorum_hash_bytes, 0, 32, reinterpret_cast<jbyte *>(quorum_hash));

    jbyteArray quorum_public_key = (jbyteArray) jenv->CallObjectMethod(contextProvider, getQuorumPublicKey, quorum_type, quorum_hash_bytes, core_chain_locked_height);
    if (quorum_public_key == nullptr) {
        memcpy(native_array, invalid_key, 48);
    } else {
        //auto result = (uint8_t*)malloc(48);
        //int size = jenv->GetArrayLength(quorum_public_key);
        //jenv->GetByteArrayRegion(quorum_public_key, 0, 48, reinterpret_cast<jbyte*>(result));
        jbyte* elements = jenv->GetByteArrayElements(quorum_public_key, nullptr);
        if (elements == nullptr) {
            // Handling error in case GetByteArrayElements failed.
            return nullptr;
        }
        jsize length = jenv->GetArrayLength(quorum_public_key);

        // Copy the data from the Java array to the native array.
        memcpy(native_array, elements, length);

        // Release the elements. If you made changes to `elements` and want them to reflect back in Java array, use 0 or JNI_COMMIT
        jenv->ReleaseByteArrayElements(quorum_public_key, elements, JNI_ABORT); // Use JNI_ABORT to indicate no changes are to be reflected back.

        jenv->DeleteLocalRef(quorum_public_key);
        printf("C++ is reurning this:  ");
        for(int i = 0; i < 5; i++)
            printf("%d, ", native_array[i]);
        printf("\n");
        //return native_array;
    }
}
extern "C" JNIEXPORT jlong JNICALL Java_org_dashj_platform_sdk_callbacks_ContextProvider_getQuorumPublicKeyCallback(JNIEnv * env, jclass provider) {
    jenv = env;
    if (contextProvider != nullptr) {
        env->DeleteGlobalRef(contextProvider);
        contextProvider = nullptr;
    }

    // Create a new global reference for the ContextProvider instance
    contextProvider = env->NewGlobalRef(provider);
    if (contextProvider == nullptr) {
        printf("Failed to create global reference for ContextProvider\n");
    }

    return (long)get_quorum_public_key;
}