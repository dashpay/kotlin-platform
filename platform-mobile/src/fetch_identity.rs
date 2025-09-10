use platform_value::types::identifier::{Identifier, IdentifierBytes32};
use dpp::identity::identity::Identity;
use dpp::errors::protocol_error::ProtocolError;
use platform_version::version::PlatformVersion;
use dpp::document::{Document, DocumentV0Getters};
use dash_sdk::platform::{DocumentQuery, Fetch, FetchMany};
use dash_sdk::platform::types::identity::PublicKeyHash;
use dpp::data_contract::DataContract;
use serde::Deserialize;
use tokio::runtime::{Runtime, Builder};
use dpp::dashcore::PubkeyHash;
use drive_proof_verifier::types::IdentityBalance;
use platform_value::string_encoding::Encoding;
use crate::config::{Config, EntryPoint};
use crate::logs::setup_logs;
use crate::sdk::{create_dash_sdk, create_dash_sdk_using_core_mainnet, create_dash_sdk_using_core_testnet, DashSdk};

pub fn test_identifier() -> Identifier {
    Identifier::from_string("7Yowk46VwwHqmD5yZyyygggh937aP6h2UW7aQWBdWpM5", Encoding::Base58).unwrap()
}

pub fn test_not_exist_identifier() -> Identifier {
    Identifier::from_string("7Yowk46VwwHqmD5yZyyyhijh937aP6h2UW7aQWBdWpM5", Encoding::Base58).unwrap()
}

#[ferment_macro::export]
pub fn fetch_identity_with_sdk(
    rust_sdk: *mut DashSdk,
    identifier: Identifier
) -> Result<Option<Identity>, String> {
    tracing::info!("fetch_identity_with_sdk");
    unsafe {
        match identity_read_with_sdk(rust_sdk, &identifier) {
            Ok(Some(identity)) => Ok(Some(identity)),
            Ok(None) => Ok(None),
            Err(err) => Err(err.to_string())
        }
    }
}

#[ferment_macro::export]
pub fn fetch_identity_balance_with_sdk(
    rust_sdk: *mut DashSdk,
    identifier: Identifier
) -> Result<u64, String> {
    tracing::info!("fetch_identity_balance_with_sdk");
    unsafe {
        match identity_read_balance_with_sdk(rust_sdk, &identifier) {
            Ok(balance) => Ok(balance),
            Err(err) => Err(err.to_string())
        }
    }
}

#[ferment_macro::export]
pub fn fetch_identity_with_keyhash_sdk(
    rust_sdk: *mut DashSdk,
    key_hash: [u8; 20]
) -> Result<Identity, String> {
    tracing::info!("fetch_identity_with_keyhash_sdk");
    unsafe {
        match identity_from_keyhash_sdk(rust_sdk, &PublicKeyHash(key_hash)) {
            Ok(identity) => Ok(identity),
            Err(err) => Err(err.to_string())
        }
    }
}

// TODO: this should return Result<Option<Identity>, String>
unsafe fn identity_read_with_sdk(rust_sdk: *mut DashSdk, id: &Identifier) -> Result<Option<Identity>, ProtocolError> {

    let rt = unsafe { (*rust_sdk).get_runtime() }.clone();

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let id: Identifier = id.clone();
        tracing::info!("Setting up SDK");
        let sdk = unsafe { (*rust_sdk).get_sdk() };
        tracing::info!("Finished SDK, {:?}", sdk);
        tracing::info!("Call fetch");
        let settings = unsafe { (*rust_sdk).get_request_settings() };
        let identity_result = Identity::fetch_with_settings(&sdk, id, settings).await;

        match identity_result {
            Ok(Some(identity)) => Ok(Some(identity)),
            Ok(None) => Ok(None), // Placeholder for actual error handling
            Err(e) => Err(ProtocolError::IdentifierError(
                format!("Identifier not found: failure: {})", e))
            )
        }
    })
}


unsafe fn identity_read_balance_with_sdk(rust_sdk: *mut DashSdk, id: &Identifier) -> Result<u64, ProtocolError> {

    let rt = unsafe { (*rust_sdk).get_runtime() }.clone();

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        // Your async code here
        let id: Identifier = id.clone();
        let sdk = unsafe { (*rust_sdk).get_sdk() };
        let settings = unsafe { (*rust_sdk).get_request_settings() };
        let identity_result = IdentityBalance::fetch_with_settings(&sdk, id, settings).await;

        match identity_result {
            Ok(Some(identity)) => Ok(identity),
            Ok(None) => Err(ProtocolError::IdentifierError("Identity not found".to_string())), // Placeholder for actual error handling
            Err(e) => Err(ProtocolError::IdentifierError(
                format!("Identifier not found: failure: {})", e))
            )
        }
    })
}

unsafe fn identity_from_keyhash_sdk(rust_sdk: *mut DashSdk, pubkey_hash: &PublicKeyHash) -> Result<Identity, ProtocolError> {
    // Create a new Tokio runtime
    //let rt = tokio::runtime::Runtime::new().expect("Failed to create a runtime");
    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        // Your async code here
        let key_hash = pubkey_hash.clone();
        tracing::info!("Setting up SDK");
        let sdk = unsafe { (*rust_sdk).get_sdk() };
        tracing::info!("Finished SDK, {:?}", sdk);
        tracing::info!("Call fetch");
        let settings = unsafe { (*rust_sdk).get_request_settings() };
        let identity_result = Identity::fetch_with_settings(&sdk, key_hash, settings).await;

        match identity_result {
            Ok(Some(identity)) => Ok(identity),
            Ok(None) => Err(ProtocolError::IdentifierError("Identity not found".to_string())), // Placeholder for actual error handling
            Err(e) => Err(ProtocolError::IdentifierError(
                format!("Identifier not found: failure: {})", e))
            )
        }
    })
}

#[test]
fn fetch_identity_with_sdk_test() {
    let mut rust_sdk = create_dash_sdk_using_core_testnet();
    let result = fetch_identity_with_sdk(
        &mut rust_sdk,
        test_identifier()
    );
    match result {
        Ok(identity) => tracing::info!("success fetching identity: {:?}", identity),
        Err(err) => panic!("error fetching identity: {}", err)
    }
}

#[test]
fn fetch_identity_with_sdk_error_test() {
    let mut rust_sdk = create_dash_sdk_using_core_testnet();
    let result = fetch_identity_with_sdk(
        &mut rust_sdk,
        test_not_exist_identifier()
    );
    match result {
        Ok(identity) => tracing::info!("success fetching identity: {:?}", identity),
        Err(err) => panic!("error fetching identity: {}", err)
    }
}



#[test]
fn fetch_identity_balance_with_sdk_test() {
    let mut rust_sdk = create_dash_sdk_using_core_testnet();
    let result = fetch_identity_balance_with_sdk(
        &mut rust_sdk,
        test_identifier()
    );
    match result {
        Ok(balance) => tracing::info!("success fetching identity: {:?}", balance),
        Err(err) => panic!("error fetching identity: {}", err)
    }
}