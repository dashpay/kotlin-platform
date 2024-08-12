#ifndef __config_h
#define __config_h

#include <stdlib.h>

typedef __int128 i128;
typedef unsigned __int128 u128;

// are these defined?
typedef __int128 int128_t;
typedef unsigned __int128 uint128_t;

struct platform_mobile_MyValue;
struct platform_mobile_MyValueMap;
struct platform_value_Value;
struct platform_value_ValueMap;

extern JavaVM* javaVM;

#ifdef __ANDROID__
#include <android/log.h>
#define LOG_TAG "NativeLog"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__)
#else
#define LOGI(...) \
    do { \
        printf("NativeLog: "); \
        printf(__VA_ARGS__); \
        printf("\n"); \
    } while(0)
#endif

inline void logInfo(const char *message) {
    LOGI("%s", message);
}

// missing items from dash_sdk_bindings.h
struct RustSdk {};
struct RustSdk5 {};
struct Processor;
struct DashSharedCore;
struct dash_sdk_sdk_Sdk;

//uint8_t * (QuorumPublicKeyCallbackExport)();
struct Arr_u8_32;

struct InstantLock {
    void * ptr;
};

struct platform_mobile_config_EntryPoint;

#endif