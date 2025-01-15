#include <jni.h>
#include "jnihelper.h"


JavaVM* javaVM = nullptr;
JniClassManager * jniClassManager = nullptr;
extern "C" JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM* vm, void* reserved) {
    javaVM = vm;
    JNIEnv * jenv = JniHelper().getEnv();

    jniClassManager = new JniClassManager(jenv);

    return JNI_VERSION_1_6;
}

JniClassManager::JniClassManager(JNIEnv * jenv) {
    resultClass = jenv->FindClass("org/dashj/platform/sdk/base/Result");
}

JniHelper::JniHelper() : env(nullptr), needsDetach(false) {
    int getEnvStat = javaVM->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6);

    if (getEnvStat == JNI_EDETACHED) {
    #ifdef __ANDROID__
        if (javaVM->AttachCurrentThread(&env, nullptr) != 0) {
    #else
        if (javaVM->AttachCurrentThread(reinterpret_cast<void**>(&env), nullptr) != 0) {
    #endif
            //__android_log_print(ANDROID_LOG_ERROR, LOG_TAG, "Failed to attach current thread");
            env = nullptr;
        } else {
            needsDetach = true;
        }
    } else if (getEnvStat != JNI_OK) {
        //__android_log_print(ANDROID_LOG_ERROR, LOG_TAG, "Failed to get the environment");
        env = nullptr;
    }
}

JniHelper::~JniHelper() {
    if (needsDetach) {
        javaVM->DetachCurrentThread();
    }
}

JNIEnv* JniHelper::getEnv() {
    return env;
}