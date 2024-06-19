#ifndef JNI_HELPER_H
#define JNI_HELPER_H

#include <jni.h>
#include <config.h>

#define LOG_TAG "NativeLog"

extern JavaVM* gJvm; // Declare the global JavaVM pointer

class JniHelper {
public:
    JniHelper();
    ~JniHelper();

    JNIEnv* getEnv();

private:
    JNIEnv* env;
    bool needsDetach;
};

class JniClassManager {
public:
    jclass resultClass;

    JniClassManager(JNIEnv * jenv);
};

extern JniClassManager * jniClassManager;

#endif // JNI_HELPER_H
