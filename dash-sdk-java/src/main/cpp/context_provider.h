#ifndef __context_provider_h
#define __context_provider_h

#include <jni.h>

struct JavaContextProvider {
    jclass contextProviderClass;
    jmethodID getQuorumPublicKeyMethod;
    jobject contextProviderObject;
};

struct JavaSigner {
    jclass signerClass;
    jmethodID signMethod;
    jobject signerObject;
};

#endif