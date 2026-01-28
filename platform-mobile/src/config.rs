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

pub const TESTNET_ADDRESS_LIST: [&str; 24] = [
    "34.214.48.68",
    "35.166.18.166",
    "52.12.176.90",
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
    "52.89.154.48",
    "52.24.124.162",
    "44.227.137.77",
    "35.85.21.179",
    "54.187.14.232",
    "54.68.235.201"
];


pub const MAINNET_ADDRESS_LIST: [&str; 170] = [
    "147.45.183.128", "134.255.182.186", "93.115.172.39", "5.189.164.253", "38.242.197.189", "83.222.9.253", "66.245.196.52", "202.71.14.79", "185.252.234.238", "178.215.237.134", "207.180.233.6", "136.244.99.17", "173.212.232.90", "157.90.238.161", "23.88.63.58", "207.244.247.40", "45.32.70.131", "52.33.9.172", "109.199.124.30", "185.198.234.17", "139.84.236.208", "38.242.218.26", "194.146.13.7", "75.119.153.10", "44.240.99.214", "5.75.133.148", "89.35.131.149", "192.248.178.237", "161.97.153.122", "195.26.254.228", "45.76.36.241", "147.45.103.99", "85.193.90.107", "194.195.87.34", "65.108.246.145", "158.247.247.241", "139.180.143.115", "213.199.44.112", "51.79.109.200", "178.253.42.64", "162.212.35.100", "173.212.196.214", "134.255.182.185", "51.79.109.182", "161.97.91.217", "217.25.89.156", "173.212.239.124", "144.126.141.62", "173.249.21.12", "161.97.159.172", "5.189.186.78", "139.84.170.10", "164.68.118.37", "192.248.175.198", "81.200.152.144", "43.167.244.109", "92.53.120.89", "172.236.244.81", "157.173.122.20", "45.153.70.126", "57.128.212.163", "185.141.216.4", "82.208.20.153", "156.67.29.45", "82.211.21.38", "93.115.172.37", "15.235.102.216", "89.35.131.39", "93.115.172.38", "134.255.183.250", "185.192.96.70", "134.255.183.248", "52.36.102.91", "139.99.201.103", "134.255.183.247", "49.13.28.255", "161.97.74.173", "168.119.102.10", "45.135.180.79", "45.135.180.130", "173.212.251.130", "38.242.206.103", "49.13.237.193", "75.119.156.254", "37.27.83.17", "45.135.180.114", "89.35.131.23", "38.242.206.56", "89.35.131.219", "185.166.217.154", "31.220.88.116", "157.173.122.21", "38.242.200.227", "193.168.3.82", "167.86.93.21", "51.195.47.118", "45.135.180.70", "167.88.169.16", "91.222.237.98", "157.173.122.26", "82.211.21.18", "52.10.213.198", "51.79.109.199", "45.94.58.58", "198.7.115.38", "37.60.244.220", "49.13.193.251", "77.232.129.86", "167.86.94.138", "198.27.68.167", "89.23.112.100", "95.179.241.182", "92.63.176.202", "188.225.39.14", "176.57.213.170", "93.115.172.36", "82.211.21.16", "95.216.146.18", "167.114.153.110", "31.220.84.93", "185.197.250.227", "161.97.91.68", "162.250.188.207", "45.85.147.192", "158.220.101.10", "93.190.140.101", "95.171.21.131", "85.92.111.111", "5.189.151.7", "64.176.35.235", "185.215.164.186", "161.97.102.156", "49.12.102.105", "147.45.236.121", "54.69.95.118", "15.235.102.215", "82.211.21.169", "195.201.238.55", "135.181.110.216", "207.180.213.141", "89.223.120.216", "45.76.141.74", "161.97.66.31", "84.247.187.76", "188.245.90.255", "43.167.239.145", "75.119.137.66", "178.18.254.136", "157.173.122.25", "82.211.21.40", "62.171.170.14", "45.77.129.235", "185.209.230.13", "75.119.152.219", "89.23.117.144", "37.60.243.59", "192.99.232.138", "89.35.131.218", "5.189.145.80", "178.18.244.255", "77.221.148.204", "207.180.218.245", "158.220.101.28", "185.215.166.126", "164.132.55.103", "162.250.190.133", "89.35.131.158", "82.97.240.124", "213.159.77.221", "114.132.172.215"
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


