cmake_minimum_required(VERSION 3.10)
SET(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_STANDARD_REQUIRED ON)
SET(BUILD_MODE Release)
SET(CMAKE_CXX_FLAGS "-Wall -fPIC")
SET(CMAKE_CXX_FLAGS_DEBUG "-g")
SET(CMAKE_CXX_FLAGS_RELEASE "-O3")
project(sdk)

message(STATUS "ANDROID_ABI=${ANDROID_ABI}")
# get the --target triple for the static rust library
if(ANDROID_ABI STREQUAL "armeabi-v7a")
    set(RUST_TARGET_TRIPLE "armv7-linux-androideabi")
elseif(ANDROID_ABI STREQUAL "arm64-v8a")
    set(RUST_TARGET_TRIPLE "aarch64-linux-android")
elseif(ANDROID_ABI STREQUAL "x86")
    set(RUST_TARGET_TRIPLE "i686-linux-android")
elseif(ANDROID_ABI STREQUAL "x86_64")
    set(RUST_TARGET_TRIPLE "x86_64-linux-android")
endif()

# Define the C source file
set(C_SOURCE_FILE ${CMAKE_SOURCE_DIR}/../../../../dash-sdk-java/build/generated/cpp/sdk.cpp
        ${CMAKE_SOURCE_DIR}/../../../../dash-sdk-java/src/main/cpp/java_map_keyid_identitypublickey.cpp
        ${CMAKE_SOURCE_DIR}/../../../../dash-sdk-java/src/main/cpp/context_provider.cpp)

# Define the pre-built Rust library
set(RUST_LIBRARY ${CMAKE_SOURCE_DIR}/../rust/target/${RUST_TARGET_TRIPLE}/release/librssdkbindings.a)

set(JAVA_AWT_LIBRARY NotNeeded)
set(JAVA_JVM_LIBRARY NotNeeded)
set(JAVA_INCLUDE_PATH2 NotNeeded)
set(JAVA_AWT_INCLUDE_PATH NotNeeded)
find_package(JNI REQUIRED)

include_directories(${JNI_INCLUDE_DIRS}, ${CMAKE_SOURCE_DIR}/../../../../dash-sdk-java/src/main/cpp)

# Create a shared library that links C++ and Rust code
add_library(sdklib SHARED ${C_SOURCE_FILE})
target_link_libraries(sdklib PRIVATE ${RUST_LIBRARY})