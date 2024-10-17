# dask-sdk-bindings

## Build Instructions

### Clone Repos
```bash
# create a containing directory
mkdir dashpay
cd dashpay

git clone https://github.com/dashpay/kotlin-platform.git
git clone --branch feat/support-android https://github.com/hashengineering/bls-signatures.git # the android PR is not yet merged
git clone --branch feat/add-android-support https://github.com/hashengineering/rs-x11-hash.git # the android PR is not yet merged
git clone https://github.com/hashengineering/platform.git 
git clone --branch feat/opaque-default https://github.com/dashpay/ferment.git 
```

### Build
This creates the header file and rust static library to be used to generate the Java bindings
```bash
# clean is necessary because dependency changes are not always detected
cargo clean && cargo build
```
