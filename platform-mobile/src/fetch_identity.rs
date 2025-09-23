use dash_sdk::Error;
use platform_value::types::identifier::{Identifier, IdentifierBytes32};
use dpp::identity::identity::Identity;
use dpp::errors::protocol_error::ProtocolError;
use platform_version::version::PlatformVersion;
use dpp::document::{Document, DocumentV0Getters};
use dash_sdk::platform::{DocumentQuery, Fetch, FetchMany};
use dash_sdk::platform::types::identity::{NonUniquePublicKeyHashQuery, PublicKeyHash};
use dpp::data_contract::DataContract;
use serde::Deserialize;
use tokio::runtime::{Runtime, Builder};
use dpp::dashcore::PubkeyHash;
use dpp::identity::accessors::IdentityGettersV0;
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
            Err(err) => {
                let err_msg = err.to_string();
                if err_msg.contains("Identity not found") {
                    match non_uniqueIdentity_from_keyhash_sdk(rust_sdk, &PublicKeyHash(key_hash)) {
                        Ok(identity) => {
                            tracing::info!("found via non-unique: {:?}", identity);
                            Ok(identity)
                        }
                        Err(e2) => {
                            tracing::info!("not found via non-unique: {:?}", e2);
                            Err(err_msg) // preserve original “not found”
                        }
                    }
                } else {
                    Err(err_msg) // propagate non‑not‑found errors as‑is
                }
            }
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

// getNonUniqueIdentityByPublicKeyHash
unsafe fn non_uniqueIdentity_from_keyhash_sdk(rust_sdk: *mut DashSdk, pubkey_hash: &PublicKeyHash) -> Result<Identity, ProtocolError> {
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
        //let key_hash = pubkey_hash;
        let mut query = NonUniquePublicKeyHashQuery {
            key_hash: pubkey_hash.0,
            after: None,
        };

        // Now fetch identities by this non-unique public key hash
        let mut count = 0usize;
        let mut last: Option<Identity> = None;
        loop {
            match Identity::fetch_with_settings(&sdk, query, settings).await {
                Ok(Some(found)) => {
                    count += 1;
                    if count > 1 {
                        return Err(ProtocolError::IdentifierError(
                            "Multiple identities found for key hash".to_string()
                        ));
                    }
                    tracing::info!(?found, ?key_hash, ?count, "fetched identities by non-unique public key hash");
                    query = NonUniquePublicKeyHashQuery {
                        key_hash: pubkey_hash.0,
                        after: Some(*found.id().as_bytes()),
                    };
                    last = Some(found);
                }
                Ok(None) => break,
                Err(e) => {
                    return Err(ProtocolError::IdentifierError(
                        format!("Fetch by non-unique key hash failed: {}", e)
                    ));
                }
            }
        }
        last.ok_or_else(|| ProtocolError::IdentifierError("Identity not found".to_string()))
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
fn fetch_identity_by_keyhash_with_sdk_test() {
    let mut rust_sdk = create_dash_sdk_using_core_mainnet();
    let result_from_key_hash = fetch_identity_with_keyhash_sdk(
        &mut rust_sdk,
        hex::decode("a9579df520c44c8d8773887bc5c9fbec579b962a").unwrap().try_into().unwrap()
    );
    // match result {
    //     Ok(identity) => tracing::info!("success fetching identity from keyhash: {:?}", identity),
    //     Err(err) => tracing::warn!("error fetching identity: {}", err)
    // }
    let result_from_id = fetch_identity_with_sdk(
        &mut rust_sdk,
        Identifier::from_string("BRWX52QB9nshdnc2Wq7HpHbwtqhD6TziWkRMFvdBnjjF", Encoding::Base58).unwrap()
    );
    // match result {
    //     Ok(identity) => tracing::info!("success fetching identity from id: {:?}", identity),
    //     Err(err) => tracing::warn!("error fetching identity: {}", err)
    // }
    assert!(result_from_id.unwrap() == Some(result_from_key_hash.unwrap()))
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