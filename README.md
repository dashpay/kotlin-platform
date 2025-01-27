# kotlin-platform
kotlin-platform is the library that supports Platform functionality for the Java VM on desktop and Android devices.

## Directory Layout
* _dash-sdk-android_ - This is for the Dash SDK that supports platform for all Android architectures
* _dash-sdk-bindings_ - This generates the primary Dash SDK header and the shared library for Java bindings for the local machine
* _dash-sdk-java_ - This is for the Dash SDK that supports platform for the local environment
* _dpp_ - This is the Kotlin Dash SDK for Platfrom
  * This is a more simple interface than `dash-sdk-java` that supports the needs of the DashPay app which uses `dpns` and `dashpay` data contracts.
  * It depends on `dash-sdk-java` and `dash-sdk-android` if targeting android
* _examples_ - Contains many example programs that perform some platform operations
* _platform-mobile_ - A rust crate used by `dash-sdk-android` and `dash-sdk-java` that supports many Platform DAPI operations
* _tools_ - a module that contains the WalletTool program

## Build Instructions
See [dash-sdk-bindings readme](dash-sdk-bindings/README.md) for build instructions of the basic bindings system. 
Perform these steps first.

### Java Bindings for Desktop
See [dash-sdk-java readme](dash-sdk-java/README.md) for build instructions of the Java bindings.
Perform these steps second.
* This will generate the Java classes for the Java bindings and the shared library that
supports the current environment
* There are currently problems when running on Linux 64-bit systems.  Mac M1 systems work well.  No other systems have been tested.

### Java Bindings for Android

Build the android libraries for the local machine (only works on Mac)
```bash
./gradlew publishToMavenLocal
```
Build the android libraries to publish to Maven Central
```bash
./gradlew publish
```

### Use in other projects
```groovy
dppVersion = "1.7.2-SNAPSHOT"
dependencies {
    implementation "org.dashj.platform:dash-sdk-java:$dppVersion"
    implementation "org.dashj.platform:dash-sdk-kotlin:$dppVersion" // dpp
    implementation "org.dashj.platform:dash-sdk-android:$dppVersion" // for android only
}
```

### Operating Systems
Currently only building for Android on a Mac is supported.

| **build system**     | **Android (arm32)**       | **Android (arm64)** | **Android (x86)**         | **Android (x86_64)** | **Windows (x86_64)** | **Mac (arm64)** | **Mac (x86_64)** | **Linux (arm64)** | **Linux (x86_64)** |
|----------------------|---------------------------|---------------------|---------------------------|----------------------|----------------------|-----------------|------------------|-------------------|--------------------|
| **Mac (arm64)**      | No 32-bit support Crashes |                     | No 32-bit support Crashes | Yes                  | N/A                  | Yes             | N/A              | N/A               | N/A                |
| **Mac (x86_64)**     | N/A                       | N/A                 | N/A                       | N/A                  | N/A                  | N/A             | N/A              | N/A               | N/A                |
| **Linux (arm64)**    | N/A                       | N/A                 | N/A                       | N/A                  | N/A                  | N/A             | N/A              | N/A               | N/A                |
| **Linux (x86_64)**   | No 32-bit support Crashes | No Crashes          | No 32-bit support Crashes | No Crashes           | N/A                  | N/A             | N/A              | N/A               | N/A                |
| **Windows (x86_64)** | Not supported             | Not supported       | Not supported             | Not supported        | Not Tested           | N/A             | N/A              | N/A               | N/A                |