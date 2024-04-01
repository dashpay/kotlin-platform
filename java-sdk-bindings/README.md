
# Build
```bash
./swig.sh
./gradlew cmakeClean cmakeConfigure cmakeBuild
# with info
./gradlew cmakeClean cmakeConfigure cmakeBuild --info
```

```bash
./gradlew generateSWIG
./gradlew cmakeClean cmakeConfigure cmakeBuild
```


# TODO
1. ignore enum constructors, destructors
2. ContractBounds crash - fixed
3. equals for all objects