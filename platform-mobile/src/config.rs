use std::os::raw::c_void;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use platform_value::{Identifier, IdentifierBytes32};
use dash_sdk::dapi_client::AddressList;
use std::sync::Arc;
use std::str::FromStr;
use dpp::data_contract::DataContract;
use drive_proof_verifier::ContextProvider;
use drive_proof_verifier::error::Error::ContextProviderError;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::time::Duration;
use rs_dapi_client::Address;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use dash_sdk::mock::provider::GrpcContextProvider;
use dash_sdk::{RequestSettings, Sdk};
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use ferment::{boxed, unbox_any};
use dash_sdk::sdk::Uri;
use platform_version::version::PlatformVersion;
use platform_version::version::v7::PLATFORM_V7;
use tokio::runtime::{Builder, Runtime};
use crate::logs::setup_logs;
use crate::provider::{Cache, CallbackContextProvider};
use crate::sdk::DashSdk;

pub const TESTNET_ADDRESS_LIST: [&str; 29] = [
    "68.67.122.1",
    "68.67.122.2",
    "68.67.122.3",
    "68.67.122.4",
    "68.67.122.5",
    "68.67.122.6",
    "68.67.122.7",
    "68.67.122.8",
    "68.67.122.9",
    "68.67.122.10",
    "68.67.122.11",
    "68.67.122.12",
    "68.67.122.13",
    "68.67.122.14",
    "68.67.122.15",
    "68.67.122.16",
    "68.67.122.17",
    "68.67.122.18",
    "68.67.122.19",
    "68.67.122.20",
    "68.67.122.21",
    "68.67.122.22",
    "68.67.122.23",
    "68.67.122.24",
    "68.67.122.25",
    "68.67.122.26",
    "68.67.122.27",
    "68.67.122.28",
    "68.67.122.29"
];


pub const MAINNET_ADDRESS_LIST: [&str; 127] = [
    "216.238.75.46", "194.163.172.206", "89.125.209.110", "84.247.180.201", "134.255.182.186", "93.115.172.39", "5.189.164.253", "161.97.165.115", "38.242.197.189", "95.111.241.20", "66.245.196.52", "173.212.245.118", "202.71.14.79", "185.252.234.238", "207.180.233.6", "89.125.209.69", "136.244.99.17", "173.212.232.90", "37.60.244.253", "157.90.238.161", "213.199.54.171", "84.247.180.198", "132.243.197.252", "161.97.75.36", "23.88.63.58", "207.244.247.40", "45.32.70.131", "31.220.91.43", "52.33.9.172", "161.97.88.199", "109.199.124.30", "185.198.234.17", "139.84.236.208", "38.242.218.26", "207.180.224.96", "62.171.170.222", "103.214.68.30", "75.119.153.10", "31.220.91.60", "132.243.197.251", "95.179.159.65", "44.240.99.214", "5.75.133.148", "213.199.54.37", "161.97.88.219", "89.35.131.149", "192.248.178.237", "161.97.153.122", "37.60.235.218", "207.180.241.242", "195.26.254.228", "45.76.36.241", "45.77.11.194", "139.84.232.129", "161.97.175.233", "104.156.154.179", "89.125.209.195", "161.97.176.38", "91.198.108.35", "161.97.180.105", "109.199.105.14", "213.199.54.34", "65.108.246.145", "64.176.10.71", "158.247.247.241", "37.60.254.213", "149.102.140.101", "139.180.143.115", "37.60.235.205", "213.199.44.112", "213.199.53.161", "162.212.35.100", "185.239.209.6", "173.212.196.214", "37.60.254.202", "134.255.182.185", "206.245.167.76", "31.220.77.84", "139.84.137.143", "144.91.87.82", "161.97.120.156", "161.97.91.217", "132.243.197.249", "62.171.168.44", "194.163.159.171", "80.240.19.200", "144.126.141.62", "173.249.21.12", "161.97.159.172", "194.163.156.190", "139.84.170.10", "164.68.118.37", "62.171.171.253", "143.198.145.184", "84.247.180.200", "37.60.234.205", "43.133.171.101", "192.248.175.198", "161.97.142.210", "43.167.244.109", "146.59.45.235", "172.236.244.81", "62.171.144.192", "193.70.43.172", "84.247.180.190", "185.215.164.84", "75.119.138.9", "172.238.7.25", "157.173.122.20", "82.208.20.153", "95.111.239.54", "85.190.243.3", "51.195.235.166", "156.67.29.45", "132.243.197.38", "93.115.172.37", "89.35.131.39", "93.115.172.38", "161.97.104.37", "75.119.128.71", "161.97.117.125", "49.13.28.255", "95.111.242.220", "52.36.102.91", "139.99.201.103", "109.199.120.79", "161.97.74.173"
];

#[ferment_macro::export]
pub fn testnet_address_list() -> Vec<String> {
    TESTNET_ADDRESS_LIST.iter().map(|&s| s.to_string()).collect()
}

pub fn mainnet_address_list() -> Vec<String> {
    MAINNET_ADDRESS_LIST.iter().map(|&s| s.to_string()).collect()
}


/// Configuration for dash-platform-sdk.
///
/// Content of this configuration is loaded from environment variables or `${CARGO_MANIFEST_DIR}/.env` file
/// when the [Config::new()] is called.
/// Variable names in the enviroment and `.env` file must be prefixed with [RS_SDK_](Config::CONFIG_PREFIX)
/// and written as SCREAMING_SNAKE_CASE (e.g. `RS_SDK_PLATFORM_HOST`).
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Hostname of the Dash Platform node to connect to
    #[serde(default)]
    pub platform_host: String,
    /// Port of the Dash Platform node grpc interface
    #[serde(default)]
    pub platform_port: u16,

    /// Hostname of the Dash Core node to connect to
    #[serde(default)]
    pub core_ip: String,
    /// Port of the Dash Core RPC interface running on the Dash Platform node
    #[serde(default)]
    pub core_port: u16,
    /// Username for Dash Core RPC interface
    #[serde(default)]
    pub core_user: String,
    /// Password for Dash Core RPC interface
    #[serde(default)]
    pub core_password: String,
    /// When true, use SSL for the Dash Platform node grpc interface
    #[serde(default)]
    pub platform_ssl: bool,

    /// Directory where all generated test vectors will be saved.
    ///
    /// See [SdkBuilder::with_dump_dir()](crate::SdkBuilder::with_dump_dir()) for more details.
    #[serde(default = "Config::default_dump_dir")]
    pub dump_dir: PathBuf,
    #[serde(default = "Config::default_is_testnet")]
    pub is_testnet: bool
}

pub trait EntryPoint {
    fn get_runtime(&self) -> Arc<Runtime>;
    fn get_sdk(&self) -> Arc<Sdk>;
    fn get_data_contract(&self, identifier: &Identifier) -> Option<Arc<DataContract>>;
    fn add_data_contract(&mut self, data_contract: &DataContract);
    fn get_request_settings(&self) -> RequestSettings;
}

impl Config {
    /// Prefix of configuration options in the environment variables and `.env` file.
    pub const CONFIG_PREFIX: &'static str = "RS_SDK_";
    /// Load configuration from operating system environment variables and `.env` file.
    ///
    /// Create new [Config] with data from environment variables and `${CARGO_MANIFEST_DIR}/tests/.env` file.
    /// Variable names in the environment and `.env` file must be converted to SCREAMING_SNAKE_CASE and
    /// prefixed with [RS_SDK_](Config::CONFIG_PREFIX).
    pub fn new() -> Self {
        // load config from .env file, ignore errors

        let path: String = env!("CARGO_MANIFEST_DIR").to_owned() + "/.env";
        if let Err(err) = dotenvy::from_path(&path) {
            tracing::warn!(path, ?err, "failed to load config file");
        }

        let mut config: Self = envy::prefixed(Self::CONFIG_PREFIX)
             .from_env()
             .expect("configuration error");

        if config.is_empty() {
            tracing::warn!(path, ?config, "some config fields are empty");
            config.platform_host = "54.213.204.85".to_string();
            config.platform_port = 1443;
            config.core_port = 19998;
            config.platform_ssl = true
        }

        config
    }

    pub fn new_testnet() -> Self {
        // load config from .env file, ignore errors

        let path: String = env!("CARGO_MANIFEST_DIR").to_owned() + "/testnet.env";
        if let Err(err) = dotenvy::from_path(&path) {
            tracing::warn!(path, ?err, "failed to load config file");
        }

        let mut config: Self = envy::prefixed(Self::CONFIG_PREFIX)
            .from_env()
            .expect("configuration error");

        if config.is_empty() {
            tracing::warn!(path, ?config, "some config fields are empty");
            config.platform_host = "54.213.204.85".to_string();
            config.platform_port = 1443;
            config.core_port = 19998;
            config.platform_ssl = true;
            config.is_testnet = true;
        }

        config
    }

    pub fn new_mainnet() -> Self {
        // load config from .env file, ignore errors

        let path: String = env!("CARGO_MANIFEST_DIR").to_owned() + "/mainnet.env";
        if let Err(err) = dotenvy::from_path(&path) {
            tracing::warn!(path, ?err, "failed to load config file");
        }

        let mut config: Self = envy::prefixed(Self::CONFIG_PREFIX)
            .from_env()
            .expect("configuration error");

        if config.is_empty() {
            tracing::warn!(path, ?config, "some config fields are empty");
            config.platform_host = "44.239.39.153".to_string();
            config.platform_port = 443;
            config.core_port = 9998;
            config.platform_ssl = true
        }
        config.is_testnet = false;
        config
    }

    /// Check if credentials of the config are empty.
    ///
    /// Checks if fields [platform_host](Config::platform_host), [platform_port](Config::platform_port),
    /// [core_port](Config::core_port), [core_user](Config::core_user) and [core_password](Config::core_password)
    /// are not empty.
    ///
    /// Other fields are ignored.
    pub fn is_empty(&self) -> bool {
        self.core_user.is_empty()
            || self.core_password.is_empty()
            || self.platform_host.is_empty()
            || self.platform_port == 0
            || self.core_port == 0
    }

    #[allow(unused)]
    /// Create list of Platform addresses from the configuration
    pub fn address_list(&self) -> AddressList {
        let scheme = if self.platform_ssl { "https" } else { "http" };

        let white_list = if self.is_testnet {
            TESTNET_ADDRESS_LIST.as_slice()
        } else {
            MAINNET_ADDRESS_LIST.as_slice()
        };
        tracing::info!("white_list {:?}", white_list);

        // Step 1: Create an iterator of formatted URI strings
        let uris_as_strings = white_list.iter().map(|host| {
            format!("{}://{}:{}", scheme, host, self.platform_port)
        });
        tracing::info!("uris_as_strings {:?}", uris_as_strings);

        let mut uris: Vec<Address> = Vec::new();
        for uri_str in uris_as_strings {
            match Uri::from_str(&uri_str) {
                Ok(uri) => {
                    uris.push(uri.try_into().unwrap());
                },
                Err(e) => tracing::warn!("error parsing address: {}", e),  // Return early if any URI fails to parse
            }
        }

        let address_list = AddressList::from_iter(uris);
        tracing::info!("address_list: {:?}", address_list);
        address_list
    }

    pub fn new_address_list(&self, address_list: Vec<String>) -> AddressList {
        tracing::info!("new_address_list: {}", address_list.len());
        let scheme = if self.platform_ssl { "https" } else { "http" };
        let uris: Vec<Address> = address_list.into_iter().map(|host| {
            let uri = format!("{}://{}:{}", scheme, host, self.platform_port);
            Uri::from_str(&uri).expect("valid address list").try_into().unwrap()
        }).collect();

        AddressList::from_iter(uris)
    }

    /// Create new SDK instance
    ///
    /// Depending on the feature flags, it will connect to the configured platform node or mock API.
    ///
    /// ## Feature flags
    ///
    /// * `offline-testing` is not set - connect to the platform and generate
    /// new test vectors during execution
    /// * `offline-testing` is set - use mock implementation and
    /// load existing test vectors from disk
    pub async fn setup_api(&self, version: &'static PlatformVersion) -> Arc<Sdk> {
        let sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.address_list())
                .with_version(version)
                .with_core(
                &self.core_ip.as_str(),
                self.core_port,
                &self.core_user.as_str(),
                &self.core_password.as_str(),
            );

            builder
                .with_version(version)
                .build().expect("cannot initialize api")
        };

        sdk.into()
    }

    pub async fn setup_api_list(&self, address_list: Vec<String>,
                                version: &'static PlatformVersion) -> Arc<Sdk> {
        let sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.new_address_list(address_list)).with_core(
                &self.core_ip.as_str(),
                self.core_port,
                &self.core_user.as_str(),
                &self.core_password.as_str(),
            );

            builder
                .with_version(version)
                .build().expect("cannot initialize api")
        };

        sdk.into()
    }

    pub async fn setup_api_with_callbacks(&self, q: u64, d: u64) -> Arc<Sdk> {
        let mut context_provider = CallbackContextProvider::new(
            std::ptr::null(),
            q,
            d,
            None,
            Arc::new(Cache::new(NonZeroUsize::new(100).expect("Non Zero"))),
            NonZeroUsize::new(100).expect("Non Zero"),
        ).expect("context provider");
        let mut sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.address_list());
            builder.build().expect("cannot initialize api")
        };
        // not ideal because context provider has a clone of the sdk
        context_provider.set_sdk(Some(Arc::new(sdk.clone())));
        sdk.set_context_provider(context_provider);
        sdk.into()
    }

    pub async fn setup_api_with_callbacks_cache(
        &self,
        context_provider_context: * const c_void,
        q: u64,
        d: u64,
        data_contract_cache: Arc<Cache<Identifier, DataContract>>,
        connect_timeout: usize,
        timeout: usize,
        retries: usize,
        version: &'static PlatformVersion
    ) -> Arc<Sdk> {
        let mut context_provider = CallbackContextProvider::new(
            context_provider_context,
            q,
            d,
            None,
            data_contract_cache.clone(),
            NonZeroUsize::new(100).expect("Non Zero")
        ).expect("context provider");

        let context_provider_clone = CallbackContextProvider::new(
            context_provider_context,
            q,
            d,
            None,
            data_contract_cache,
            NonZeroUsize::new(100).expect("Non Zero")
        ).expect("context provider");

        let mut sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.address_list())
                .with_settings(
                    RequestSettings {
                        connect_timeout: Some(Duration::from_secs(connect_timeout as u64)),
                        timeout: Some(Duration::from_secs(timeout as u64)),
                        retries: Some(retries),
                        ban_failed_address: Some(true),
                        max_decoding_message_size: Some(16 * 1024 * 1024)
                    }
                )
                .with_version(&version)
                .with_context_provider(context_provider_clone);
            builder.build().expect("cannot initialize api")
        };
        // not ideal because context provider has a clone of the sdk
        context_provider.set_sdk(Some(Arc::new(sdk.clone())));
        sdk.set_context_provider(context_provider);
        sdk.into()
    }

    pub async fn setup_api_with_callbacks_cache_list(
        &self,
        context: * const c_void,
        q: u64,
        d: u64,
        data_contract_cache: Arc<Cache<Identifier, DataContract>>,
        address_list: Vec<String>,
        version: &'static PlatformVersion
    ) -> Arc<Sdk> {
        let mut context_provider = CallbackContextProvider::new(
            context,
            q,
            d,
            None,
            data_contract_cache,
            NonZeroUsize::new(100).expect("Non Zero")
        ).expect("context provider");
        let mut sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.new_address_list(address_list));
            builder
                .with_version(version)
                .build()
                .expect("cannot initialize api")
        };
        // not ideal because context provider has a clone of the sdk
        context_provider.set_sdk(Some(Arc::new(sdk.clone())));
        sdk.set_context_provider(context_provider);
        sdk.into()
    }

    fn default_dump_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("vectors")
    }

    fn default_is_testnet() -> bool {
        true
    }
}


