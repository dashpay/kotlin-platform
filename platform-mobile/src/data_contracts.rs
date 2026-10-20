use std::collections::BTreeMap;
use std::sync::Arc;
use dash_sdk::platform::{DocumentQuery, Fetch};
use dash_sdk::{Error, Sdk};
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use dpp::data_contract::config::v0::DataContractConfigGettersV0;
use dpp::data_contract::DataContract;
use dpp::document::Document;
use drive_proof_verifier::types::Documents;
use platform_value::{Identifier, IdentifierBytes32};
use platform_value::string_encoding::Encoding;
use rs_dapi_client::RequestSettings;
use rs_dapi_client::transport::BoxFuture;
use tokio::runtime::Builder;
use crate::config::{Config, EntryPoint};
use crate::logs::setup_logs;
use crate::sdk::{
    create_dash_sdk_using_core_mainnet,
    create_dash_sdk_using_core_testnet,
    create_dash_sdk_using_single_evonode,
    DashSdk
};

#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct DataContractFFI {
    pub id: Identifier,
    pub owner_id: Identifier,
    pub doc_types: Vec<String>,
    pub version: u32,
    pub requires_identity_encryption_bounded_key: bool,
    pub requires_identity_decryption_bounded_key: bool,
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn DataContractFFI_clone(value: DataContractFFI) -> DataContractFFI {
    value.clone()
}

impl Into<DataContractFFI> for DataContract {
    fn into(self) -> DataContractFFI {
        DataContractFFI {
            id: self.id(),
            owner_id: self.owner_id(),
            doc_types: self.document_types().keys().cloned().collect(),
            version: self.version(),
            requires_identity_encryption_bounded_key: self.config().requires_identity_encryption_bounded_key().is_some(),
            requires_identity_decryption_bounded_key: self.config().requires_identity_decryption_bounded_key().is_some(),
        }
    }
}

impl Into<DataContractFFI> for Arc<DataContract> {
    fn into(self) -> DataContractFFI {
        (*self).clone().into()
    }
}

fn fetch_data_contract_with_retry(
    sdk: Arc<Sdk>,
    data_contract_id: Identifier,
    request_settings: RequestSettings,
    retries_left: usize,
) -> BoxFuture<'static, Result<Option<DataContract>, Error>> {
    Box::pin(async move {
        match DataContract::fetch_with_settings(&sdk, data_contract_id.clone(), request_settings).await {
            Ok(Some(data_contract)) => Ok(Some(data_contract)),
            Ok(None) => {
                if retries_left > 1 {
                    if data_contract_id == Identifier::from(dpns_contract::ID_BYTES) ||
                        data_contract_id == Identifier::from(dashpay_contract::ID_BYTES) {
                        return fetch_data_contract_with_retry(sdk, data_contract_id, request_settings, retries_left - 1).await;
                    }
                }
                Ok(None)
            }
            Err(error) => {
                if retries_left > 1 {
                    if error.to_string().contains("contract not found error") {
                        if data_contract_id == Identifier::from(dpns_contract::ID_BYTES) ||
                            data_contract_id == Identifier::from(dashpay_contract::ID_BYTES) {
                            return fetch_data_contract_with_retry(sdk, data_contract_id, request_settings, retries_left - 1).await;
                        }
                    }
                }
                Err(error)
            }
        }
    })
}

#[ferment_macro::export]
pub fn fetch_data_contract(
    rust_sdk: *mut DashSdk,
    data_contract_id: Identifier
) -> Result<Option<DataContractFFI>, String> {
    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        tracing::warn!("using existing data contract id and fetching...");
        match unsafe { (*rust_sdk).get_data_contract(&data_contract_id) } {
            Some(data_contract) => Ok(Some(data_contract.into())),
            None => {
                let request_settings = unsafe { (*rust_sdk).get_request_settings() };
                match (DataContract::fetch_with_settings(&sdk, data_contract_id.clone(), request_settings).await) {
                    Ok(Some(data_contract)) => {
                        tracing::info!("{:?}", data_contract);
                        unsafe { (*rust_sdk).add_data_contract(&data_contract); };
                        let data_contract_ffi: DataContractFFI = data_contract.into();
                        tracing::info!("{:?}", data_contract_ffi);
                        Ok(Some(data_contract_ffi))
                    },
                    Ok(None) => return Ok(None),//Err("data contract not found".to_string()),
                    Err(e) => return Err(e.to_string())
                }
            }
        }
    })
}

#[test]
fn get_data_contract_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    let data_contract = fetch_data_contract(
        & mut sdk,
        Identifier::from(dpns_contract::ID_BYTES)
    ).unwrap();
    tracing::info!("dpns: {:?}", data_contract);
}

#[test]
fn get_wallet_utils_data_contract_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    let data_contract = fetch_data_contract(
        & mut sdk,
        Identifier::from(wallet_utils_contract::ID_BYTES)
    ).unwrap();
    println!("wallet-utils: {:?}", data_contract);
}

#[test]
fn get_wallet_utils_data_contract_mainnet_test() {
    let mut sdk = create_dash_sdk_using_core_mainnet();
    let data_contract = fetch_data_contract(
        & mut sdk,
        Identifier::from(wallet_utils_contract::ID_BYTES)
    );
    assert!(data_contract.is_ok(), "cannot find the wallet utils contract");
    println!("wallet-utils: {:?}", data_contract.unwrap());
}

#[test]
fn get_wallet_utils_data_contract_test_all() {
    for i in crate::config::TESTNET_ADDRESS_LIST {
        let mut sdk = create_dash_sdk_using_single_evonode(i.into(), 0, 0, true);
        let data_contract_result = fetch_data_contract(
            &mut sdk,
            Identifier::from(wallet_utils_contract::ID_BYTES)
        );
        match data_contract_result {
            Ok(Some(data_contract)) => println!("wallet-utils: {}. {:?}", i, data_contract.doc_types),
            Ok(None) => println!("wallet-utils: {}. contract not found", i),
            Err(e) => println!("wallet-utils {}. {:?}", i, e)
        };
    }
}

#[test]
fn get_wallet_utils_data_contract_mainnet_all() {
    let mut success = true;
    for i in crate::config::MAINNET_ADDRESS_LIST {
        let mut sdk = create_dash_sdk_using_single_evonode(i.into(), 0, 0, false);
        let data_contract_result = fetch_data_contract(
            &mut sdk,
            Identifier::from(wallet_utils_contract::ID_BYTES)
        );
        match data_contract_result {
            Ok(Some(data_contract)) => println!("wallet-utils: {}. {:?}", i, data_contract.doc_types),
            Ok(None) => println!("wallet-utils: {}. contract not found", i),
            Err(e) => {
                println!("wallet-utils {}. {:?}", i, e);
                success = false
            }
        };
    }
    assert!(success)
}

#[test]
fn get_missing_data_contract_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    let data_contract_result = fetch_data_contract(
        & mut sdk,
        Identifier::from_string("Fds5DDfXoLwpUZ71AAVYZP1uod8S7Ze2bR28JExBvZKR", Encoding::Base58).expect("identifier"),
    );

    assert!(data_contract_result.is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;
    use data_contracts::SystemDataContract;
    use dpp::data_contract::accessors::v0::DataContractV0Setters;
    use dpp::data_contract::config::v0::DataContractConfigSettersV0;
    use dpp::data_contract::storage_requirements::keys_for_document_type::StorageKeyRequirements;
    use dpp::system_data_contracts::load_system_data_contract;
    use platform_version::version::PlatformVersion;
    use rand::Rng;
    use platform_value::string_encoding::{encode, Encoding};

    #[test]
    fn app_connect_contract_conversion_preserves_owner_version_and_key_opt_ins() {
        let mut contract = load_system_data_contract(
            SystemDataContract::AppConnect, PlatformVersion::latest(),
        ).unwrap();
        let owner = Identifier::from([42; 32]);
        contract.set_owner_id(owner);
        contract.set_version(7);
        contract.config_mut().set_requires_identity_encryption_bounded_key(
            Some(StorageKeyRequirements::Unique),
        );
        contract.config_mut().set_requires_identity_decryption_bounded_key(None);
        let ffi: DataContractFFI = contract.into();
        assert_eq!(ffi.owner_id, owner);
        assert_ne!(ffi.owner_id, ffi.id);
        assert_eq!(ffi.version, 7);
        assert!(ffi.requires_identity_encryption_bounded_key);
        assert!(!ffi.requires_identity_decryption_bounded_key);
    }

    #[test]
    fn generate_random_32_byte_vector() {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.r#gen()).collect();
        println!("{:?}", random_bytes);
        // Encode in Base58 and print
        let base58_encoded = encode(&random_bytes, Encoding::Base58);
        println!("Base58: {}", base58_encoded);
    }
}
