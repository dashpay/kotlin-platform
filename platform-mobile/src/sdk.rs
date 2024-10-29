use std::os::raw::c_void;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;
use dash_sdk::{RequestSettings, Sdk};
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use dpp::data_contract::DataContract;
use ferment_interfaces::{boxed, unbox_any};
use platform_value::Identifier;
use tokio::runtime::{Builder, Runtime};
use crate::config::{Config, EntryPoint};
use crate::logs::setup_logs;
use crate::provider::Cache;

#[ferment_macro::opaque]
pub struct DashSdk {
    pub config: Arc<Config>,
    pub runtime: Arc<Runtime>,
    pub sdk: Arc<Sdk>,
    pub context_provider_context: * const c_void,
    pub data_contract_cache: Arc<Cache<Identifier, DataContract>>,
    pub request_settings: RequestSettings
}

impl DashSdk {

    pub fn get_config(&self) -> Arc<Config> {
        self.config.clone()
    }
    pub fn get_data_contract_cache(&self) -> Arc<Cache<Identifier, DataContract>> {
        self.data_contract_cache.clone()
    }
}

impl EntryPoint for DashSdk {
    fn get_runtime(&self) -> Arc<Runtime> {
        self.runtime.clone()
    }
    fn get_sdk(&self) -> Arc<Sdk> {
        self.sdk.clone()
    }

    fn get_data_contract(&self, identifier: &Identifier) -> Option<Arc<DataContract>> {
        match self.data_contract_cache.get(identifier) {
            Some(T) => Some(T.clone()),
            None => None
        }
    }

    fn add_data_contract(&mut self, data_contract: &DataContract) {
        self.data_contract_cache.put(data_contract.id(), data_contract.clone());
    }

    fn get_request_settings(&self) -> RequestSettings {
        self.request_settings
    }
}

#[ferment_macro::export]
pub fn update_sdk_with_address_list(
    rust_sdk: * mut DashSdk,
    quorum_public_key_callback: u64,
    data_contract_callback: u64,
    address_list: Vec<String>
) {

    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let cfg = unsafe { (*rust_sdk).get_config() };


        let sdk = cfg.setup_api_with_callbacks_cache_list(
            unsafe { (*rust_sdk).context_provider_context },
            quorum_public_key_callback,
            data_contract_callback,
            unsafe { (*rust_sdk).get_data_contract_cache() },
            address_list
        ).await;

        tracing::info!("sdk created");

        unsafe { (*rust_sdk).sdk = sdk }

    });
}

pub fn create_dash_sdk_using_core_testnet() -> DashSdk {
    create_dash_sdk(0, 0, true)
}

pub fn create_dash_sdk_using_core_mainnet() -> DashSdk {
    create_dash_sdk(0, 0, false)
}
#[ferment_macro::export]
pub fn create_dash_sdk(
    quorum_public_key_callback: u64,
    data_contract_callback: u64,
    is_testnet: bool
) -> DashSdk {
    create_dash_sdk_with_context(
        0,
        quorum_public_key_callback,
        data_contract_callback,
        is_testnet,
        10,
        5,
        5
    )
}

#[ferment_macro::export]
pub fn create_dash_sdk_with_context(
    context_provider_context: usize,
    quorum_public_key_callback: u64,
    data_contract_callback: u64,
    is_testnet: bool,
    connect_timeout: usize,
    timeout: usize,
    retries: usize
) -> DashSdk {
    setup_logs();
    let rt = Arc::new(
        Builder::new_multi_thread()
            .enable_all() // Enables all I/O and time drivers
            .build()
            .expect("Failed to create a runtime")
    );
    tracing::info!("create_dash_sdk_with_context({}, {}, {}, is_testnet{}, ...)",
        context_provider_context,
        quorum_public_key_callback,
        data_contract_callback,
        is_testnet
    );

    let context_provider_context: *const c_void = context_provider_context as *const c_void;
    rt.block_on(async {
        let cfg = if is_testnet {
            Config::new_testnet()
        } else {
            Config::new_mainnet()
        };
        tracing::info!("configuring for testnet={} using platform port={}", cfg.is_testnet, cfg.platform_port);
        let data_contract_cache = Arc::new(Cache::new(NonZeroUsize::new(100).expect("Non Zero")));
        let sdk = if quorum_public_key_callback != 0 {
            // use the callbacks to obtain quorum public keys
            cfg.setup_api_with_callbacks_cache(
                context_provider_context.clone(),
                quorum_public_key_callback,
                data_contract_callback,
                data_contract_cache.clone()
            ).await
        } else {
            // use Dash Core for quorum public keys
            cfg.setup_api().await
        };
        DashSdk {
            config: Arc::new(cfg),
            runtime: rt.clone(),
            sdk: sdk,
            context_provider_context,
            data_contract_cache: data_contract_cache,
            request_settings: RequestSettings {
                connect_timeout: Some(Duration::from_secs(connect_timeout as u64)),
                timeout: Some(Duration::from_secs(timeout as u64)),
                retries: Some(retries),
                ban_failed_address: Some(true),
            }
        }
    })
}

#[ferment_macro::export]
pub fn create_dash_sdk_using_single_evonode(
    evonode: String,
    quorum_public_key_callback: u64,
    data_contract_callback: u64,
    is_testnet: bool
) -> DashSdk {
    setup_logs();
    let rt = Arc::new(
        Builder::new_multi_thread()
            .enable_all() // Enables all I/O and time drivers
            .build()
            .expect("Failed to create a runtime")
    );

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let cfg = if is_testnet {
            Config::new_testnet()
        } else {
            Config::new_mainnet()
        };
        let data_contract_cache = Arc::new(Cache::new(NonZeroUsize::new(100).expect("Non Zero")));
        let sdk = if quorum_public_key_callback != 0 {
            // use the callbacks to obtain quorum public keys
            cfg.setup_api_with_callbacks_cache_list(std::ptr::null(), quorum_public_key_callback, data_contract_callback, data_contract_cache.clone(), vec![evonode]).await
        } else {
            // use Dash Core for quorum public keys
            cfg.setup_api_list(vec![evonode]).await
        };
        DashSdk {
            config: Arc::new(cfg),
            runtime: rt.clone(),
            sdk: sdk,
            context_provider_context: std::ptr::null(),
            data_contract_cache: data_contract_cache,
            request_settings: RequestSettings {
                connect_timeout: Some(Duration::from_secs(5)),
                timeout: Some(Duration::from_secs(5)),
                retries: Some(0),
                ban_failed_address: Some(false),
            }
        }
    })
}

#[ferment_macro::export]
pub fn destroy_dash_sdk(rust_sdk: * mut DashSdk) {
    unsafe  { unbox_any(rust_sdk) };
}

#[test]
fn test_dash_sdk() {
    let my_sdk = create_dash_sdk_using_core_testnet();
    let my_boxed_sdk = boxed(my_sdk);
    destroy_dash_sdk(my_boxed_sdk);
}