# dask-sdk-bindings

## Build Instructions

### Clone Repos
```bash
# create a containing directory such as `dashpay`
mkdir dashpay
cd dashpay

git clone https://github.com/dashpay/kotlin-platform.git
git clone --branch ferment-2.0 https://github.com/hashengineering/platform.git
```

### Build
This creates the header file and rust static library to be used to generate the Java bindings
```bash
# clean is necessary because dependency changes are not always detected
cargo clean && cargo build
```
