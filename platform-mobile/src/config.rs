use std::os::raw::c_void;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use platform_value::{Identifier, IdentifierBytes32};
use dash_sdk::dapi_client::AddressList;
use std::sync::Arc;
use std::str::FromStr;
use dpp::data_contract::DataContract;
use drive_proof_verifier::ContextProvider;
use drive_proof_verifier::error::ContextProviderError;
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

pub const TESTNET_ADDRESS_LIST: [&str; 26] = [
    "34.214.48.68",
    // "35.166.18.166",
    // "35.165.50.126",
    "52.42.202.128",
    "52.12.176.90",
    // "44.233.44.95",
    "35.167.145.149",
    "52.34.144.50",
    "44.240.98.102",
    "54.201.32.131",
    "52.10.229.11",
    "52.13.132.146",
    "44.228.242.181",
    "35.82.197.197",
    "52.40.219.41",
    "44.239.39.153",
    "54.149.33.167",
    "35.164.23.245",
    "52.33.28.47",
    "52.43.86.231",
    "52.43.13.92",
    "35.163.144.230",
    "52.89.154.48",
    "52.24.124.162",
    "44.227.137.77",
    "35.85.21.179",
    "54.187.14.232",
    "54.68.235.201",
    "52.13.250.182",
    //"35.82.49.196",
    //"44.232.196.6",
   // "54.189.164.39",
    //"54.213.204.85"
];


pub const MAINNET_ADDRESS_LIST: [&str; 128] = [
    "149.28.241.190", "216.238.75.46", "134.255.182.186", "157.66.81.162", "213.199.34.250", "157.90.238.161", "185.198.234.68", "37.60.236.212", "207.244.247.40", "45.32.70.131", "158.220.122.76", "52.33.9.172", "185.158.107.124", "185.198.234.17", "93.190.140.101", "194.163.153.225", "194.146.13.7", "93.190.140.112", "65.108.74.95", "44.240.99.214", "5.75.133.148", "192.248.178.237", "95.179.159.65", "139.84.232.129", "37.60.243.119", "194.195.87.34", "46.254.241.7", "161.97.160.92", "65.108.246.145", "64.176.10.71", "37.60.244.220", "2.58.82.231", "185.198.234.54", "37.27.67.154", "134.255.182.185", "139.84.137.143", "173.212.239.124", "5.189.186.78", "173.249.53.139", "37.60.236.151", "37.27.67.159", "104.200.24.196", "37.60.236.225", "57.128.212.163", "158.220.122.74", "185.198.234.25", "134.255.183.250", "185.192.96.70", "134.255.183.248", "52.36.102.91", "134.255.183.247", "49.13.28.255", "168.119.102.10", "37.27.83.17", "134.255.182.187", "38.242.198.100", "37.27.67.163", "198.7.115.43", "70.34.206.123", "65.108.74.78", "108.61.165.170", "157.10.199.79", "31.220.88.116", "185.166.217.154", "37.27.67.164", "31.220.85.180", "161.97.170.251", "157.10.199.82", "91.107.226.241", "167.88.169.16", "216.238.99.9", "62.169.17.112", "52.10.213.198", "198.7.115.38", "37.60.236.161", "49.13.193.251", "46.254.241.9", "185.215.167.70", "65.108.74.75", "95.179.241.182", "95.216.146.18", "31.220.84.93", "185.197.250.227", "149.28.247.165", "213.199.34.251", "185.198.234.12", "87.228.24.64", "45.32.52.10", "91.107.204.136", "157.66.81.130", "157.10.199.125", "46.254.241.8", "49.12.102.105", "134.255.182.189", "81.17.101.141", "65.108.74.79", "64.23.134.67", "54.69.95.118", "158.220.122.13", "49.13.154.121", "82.211.25.69", "75.119.149.9", "93.190.140.111", "93.190.140.114", "195.201.238.55", "135.181.110.216", "45.76.141.74", "109.199.123.11", "50.116.28.103", "188.245.90.255", "130.162.233.186", "65.109.65.126", "95.179.139.125", "213.199.34.248", "213.199.35.18", "213.199.35.6", "37.60.243.59", "37.27.67.156", "37.60.236.247", "159.69.204.162", "46.254.241.11", "173.199.71.83", "185.215.166.126", "157.66.81.218", "213.199.35.15", "114.132.172.215", "93.190.140.162", "65.108.74.109"
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
                &self.core_ip,
                self.core_port,
                &self.core_user,
                &self.core_password,
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
                &self.core_ip,
                self.core_port,
                &self.core_user,
                &self.core_password,
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


