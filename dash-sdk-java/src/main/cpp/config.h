#ifndef __config_h
#define __config_h

#include <stdlib.h>



struct platform_mobile_MyValue;
struct platform_mobile_MyValueMap;
struct platform_value_Value;
struct platform_value_ValueMap;
struct Vec_platform_value_Value;
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

struct Arr_u8_32;

struct InstantLock {
    void * ptr;
};

#include "int128.h"

struct platform_mobile_config_EntryPoint;

#endif