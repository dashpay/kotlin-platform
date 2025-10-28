cmake_minimum_required(VERSION 3.10)
SET(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_STANDARD_REQUIRED ON)
SET(BUILD_MODE Debug)
SET(CMAKE_CXX_FLAGS "-Wall -fPIC")
SET(CMAKE_CXX_FLAGS_DEBUG "-g")
SET(CMAKE_CXX_FLAGS_RELEASE "-O3")
# set(MI_LOCAL_DYNAMIC_TLS ON CACHE BOOL "Use dlopen-compatible TLS mechanism")
if(UNIX AND NOT APPLE)
    set(MI_LOCAL_DYNAMIC_TLS ON CACHE BOOL "Use dlopen-compatible TLS mechanism")
endif()
project(sdk)

# Define the C source file
set(C_SOURCE_FILE ${CMAKE_SOURCE_DIR}/../../../build/generated/cpp/sdk.cpp
        java_map_keyid_identitypublickey.cpp
        java_list_value.cpp
        java_map_string_value.cpp
        java_map_valuemap.cpp
        context_provider.cpp
        java_vm.cpp
        wrapper.h)

# Define the pre-built Rust library
set(RUST_LIBRARY ${CMAKE_SOURCE_DIR}/../../../../dash-sdk-bindings/target/debug/libdash_sdk_bindings.a)
set(JAVA_AWT_LIBRARY NotNeeded)
set(JAVA_JVM_LIBRARY NotNeeded)
set(JAVA_AWT_INCLUDE_PATH NotNeeded)
find_package(JNI REQUIRED)
include_directories(${JNI_INCLUDE_DIRS}, ${CMAKE_SOURCE_DIR})
# Create an executable that links C and Rust code
add_library(sdklib SHARED ${C_SOURCE_FILE})

# Check if building on macOS
if(APPLE)
    if(NOT DEFINED MACOS_VERSION)
        set(MACOS_VERSION "14.5")
    endif()
    #option(MACOS_VERSION "Minimum macOS version to target" "14.5")
    set(CMAKE_OSX_DEPLOYMENT_TARGET ${MACOS_VERSION})
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mmacosx-version-min=${MACOS_VERSION}")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -mmacosx-version-min=${MACOS_VERSION}")
    # Find the Core Foundation framework
    find_library(CORE_FOUNDATION_FRAMEWORK CoreFoundation)
    if(NOT CORE_FOUNDATION_FRAMEWORK)
        message(FATAL_ERROR "CoreFoundation framework not found")
    endif()

    # Find the Security framework
    find_library(SECURITY_FRAMEWORK Security)
    if(NOT SECURITY_FRAMEWORK)
        message(FATAL_ERROR "Security framework not found")
    endif()

    # Link the frameworks only on macOS
    target_link_libraries(sdklib PRIVATE "${CORE_FOUNDATION_FRAMEWORK}" "${SECURITY_FRAMEWORK}" ${RUST_LIBRARY})
else()
    target_link_libraries(sdklib PRIVATE ${RUST_LIBRARY})
endif()

