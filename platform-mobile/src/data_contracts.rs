use std::collections::BTreeMap;
use std::sync::Arc;
use dash_sdk::platform::{DocumentQuery, Fetch};
use dash_sdk::{Error, Sdk};
use dpp::data_contract::accessors::v0::DataContractV0Getters;
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
use crate::provider::Cache;
use crate::sdk::{create_dash_sdk_using_core_testnet, DashSdk};

#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct DataContractFFI {
    pub id: Identifier,
    pub owner_id: Identifier,
    pub doc_types: Vec<String>,
    pub version: u32
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn DataContractFFI_clone(value: DataContractFFI) -> DataContractFFI {
    value.clone()
}

impl Into<DataContractFFI> for DataContract {
    fn into(self) -> DataContractFFI {
        match self {
            DataContract::V0(contract) => {
                DataContractFFI {
                    id: contract.id(),
                    owner_id: contract.owner_id(),
                    doc_types: contract.document_types.keys().cloned().collect(),
                    version: contract.version()
                }
            }
            DataContract::V1(contract) => {
                DataContractFFI {
                    id: contract.id(),
                    owner_id: contract.owner_id(),
                    doc_types: contract.document_types.keys().cloned().collect(),
                    version: contract.version()
                }
            }
            _ => panic!("unknown version of DataContract")
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
                match (DataContract::fetch_with_settings(&sdk, data_contract_id.clone(), request_settings)
                    .await) {
                    Ok(Some(data_contract)) => {
                        unsafe { (*rust_sdk).add_data_contract(&data_contract); };
                        let data_contract_ffi: DataContractFFI = data_contract.into();
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
    tracing::info!("wallet-utils: {:?}", data_contract);
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
    use rand::Rng;
    use platform_value::string_encoding::{encode, Encoding};

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