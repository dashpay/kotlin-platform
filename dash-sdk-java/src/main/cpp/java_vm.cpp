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

/// @brief 
/// @param jenv 
/// @return 
bool attachThread(JNIEnv * jenv) {
#ifdef __ANDROID__
//    JavaVM* jvm;
//    jenv->GetJavaVM(&jvm);

    int getEnvStat = javaVM->GetEnv(reinterpret_cast<void**>(&jenv), JNI_VERSION_1_6);
//#else
//    int getEnvStat = jvm->GetEnv(&jenv, JNI_VERSION_1_6);
//#endif
    //bool needsDetach = false;
    if (getEnvStat == JNI_EDETACHED) {

        if (javaVM->AttachCurrentThread(&jenv, nullptr) != 0) {
            //SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "Failed to attach current thread");
            return false;
        }
        return true;
    } else if (getEnvStat != JNI_OK) {
        //SWIG_JavaThrowException(jenv, SWIG_JavaRuntimeException, "Failed to get the environment");
        return false;
    }
    return false;
#endif
}

void detachThread(JNIEnv * jenv, bool needsDetach) {
    #ifndef __ANDROID__
    if (needsDetach) {
        JavaVM* jvm;
        jenv->GetJavaVM(&jvm);

        jvm->DetachCurrentThread();
    }
    #endif
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