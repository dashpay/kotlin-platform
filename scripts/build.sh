#!/bin/sh

cd dash-sdk-bindings
cargo clean
cargo build
cd ..

./gradlew generateSWIG

./gradlew generateProto

./gradlew cmakeClean cmakeConfigure cmakeBuild