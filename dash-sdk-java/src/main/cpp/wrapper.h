#ifndef __wrapper_h
#define __wrapper_h

#include <jni.h>


template <class T> jobject SWIG_wrap(JNIEnv * jenv, T * object, char * className, bool owner = false) {
    jclass classObject = jenv->FindClass(className);
    jmethodID constructor = jenv->GetMethodID(classObject, "<init>", "(JZ)V");
    return jenv->NewObject(classObject, constructor, (jlong) object, owner);
}

template <class T>
T * SWIG_unwrap(JNIEnv * jenv, jobject object, char * className) {
    jclass baseObjectClass = jenv->FindClass(className);
    if (baseObjectClass == nullptr) {
        printf("Cannot find %s\n", className);
        //jenv->ThrowNew(jenv->FindClass("org/"))
    }
    jmethodID getCPointerMethod = jenv->GetMethodID(baseObjectClass, "getCPointer", "()J");
    if (getCPointerMethod == nullptr) {
        printf("Cannot find getCPointer for %s\n", className);
    }
    return (T*)jenv->CallObjectMethod(object, getCPointerMethod);
}

#endif