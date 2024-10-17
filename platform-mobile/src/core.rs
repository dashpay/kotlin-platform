use dapi_grpc::core::v0::{GetTransactionRequest};
use dash_sdk::dapi_client::DapiRequestExecutor;
use dash_sdk::RequestSettings;
use tokio::runtime::Builder;
use crate::config::{Config, EntryPoint};
use crate::logs::setup_logs;
use crate::sdk::DashSdk;

#[ferment_macro::export]
pub fn get_transaction(txid: [u8; 32], quorum_public_key_callback: u64, data_contract_callback: u64) -> Result<Vec<u8>, String> {

    setup_logs();

    let rt = Builder::new_current_thread()
        .enable_all() // Enables all I/O and time drivers
        .build()
        .expect("Failed to create a runtime");

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let cfg = Config::new();
        let sdk = cfg.setup_api().await;

        let tx_info_result = sdk.execute(
                GetTransactionRequest {
                    id: hex::encode(txid),
                },
                RequestSettings::default(),
            )
            .await;
        match tx_info_result {
            Ok(tx_info) => Ok(tx_info.transaction),
            Err(error) => return Err(error.to_string())
        }
    })
}

#[ferment_macro::export]
pub fn get_transaction_sdk(rust_sdk: * mut DashSdk, txid: [u8; 32]) -> Result<Vec<u8>, String> {

    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        let settings = unsafe { (*rust_sdk).get_request_settings() };
        let tx_info_result = sdk.execute(
            GetTransactionRequest {
                id: hex::encode(txid),
            },
            settings,
        )
            .await;
        match tx_info_result {
            Ok(tx_info) => Ok(tx_info.transaction),
            Err(error) => return Err(error.to_string())
        }
    })
}