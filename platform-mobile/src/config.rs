use std::os::raw::c_void;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use platform_value::{Identifier, IdentifierBytes32};
use dash_sdk::dapi_client::AddressList;
use std::sync::Arc;
use std::str::FromStr;
use dpp::data_contract::DataContract;
use dpp::dashcore::Network;
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
use platform_version::version::v7::PLATFORM_V7;
use tokio::runtime::{Builder, Runtime};
use crate::logs::setup_logs;
use crate::provider::{CallbackContextProvider, DataContractCache};
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


pub const MAINNET_ADDRESS_LIST: [&str; 350] = [
    "2.31.109.195",
    "2.59.156.192",
    "5.39.27.224",
    "5.75.133.148",
    "5.78.73.117",
    "5.161.103.48",
    "5.189.145.80",
    "5.189.151.7",
    "5.189.164.253",
    "5.199.166.45",
    "5.199.166.140",
    "15.235.102.215",
    "15.235.102.216",
    "23.88.63.58",
    "31.220.84.93",
    "31.220.91.43",
    "31.220.91.60",
    "34.210.238.67",
    "37.27.67.154",
    "37.27.67.156",
    "37.27.67.159",
    "37.27.67.163",
    "37.27.83.17",
    "37.60.234.168",
    "37.60.234.205",
    "37.60.235.24",
    "37.60.235.39",
    "37.60.235.89",
    "37.60.235.205",
    "37.60.235.224",
    "37.60.236.230",
    "37.60.243.59",
    "37.60.244.35",
    "37.60.244.87",
    "37.60.244.253",
    "37.60.246.228",
    "37.60.254.201",
    "37.60.254.202",
    "37.60.254.205",
    "37.60.254.213",
    "37.60.255.160",
    "38.242.197.189",
    "38.242.200.227",
    "38.242.206.56",
    "38.242.206.103",
    "38.242.218.26",
    "43.133.171.101",
    "43.167.239.145",
    "43.167.244.109",
    "44.240.99.214",
    "45.76.36.241",
    "45.76.141.74",
    "45.85.147.192",
    "45.94.58.58",
    "45.135.180.70",
    "45.135.180.79",
    "45.135.180.114",
    "45.135.180.130",
    "46.62.250.247",
    "48.103.145.105",
    "49.12.102.105",
    "49.13.28.255",
    "49.13.193.251",
    "49.13.237.193",
    "50.116.28.103",
    "51.68.3.29",
    "51.68.35.98",
    "51.79.109.182",
    "51.161.34.134",
    "52.10.213.198",
    "52.33.9.172",
    "52.36.102.91",
    "54.69.95.118",
    "57.128.212.163",
    "57.131.28.197",
    "62.171.136.93",
    "62.171.136.245",
    "62.171.138.186",
    "62.171.144.192",
    "62.171.145.24",
    "62.171.145.234",
    "62.171.150.24",
    "62.171.168.4",
    "62.171.168.44",
    "62.171.171.253",
    "62.171.174.123",
    "62.171.176.79",
    "62.171.178.71",
    "62.171.189.0",
    "63.186.97.17",
    "64.176.10.71",
    "64.176.35.235",
    "65.21.237.223",
    "65.21.237.224",
    "65.21.237.225",
    "65.21.237.231",
    "65.108.0.149",
    "65.108.0.150",
    "65.108.0.151",
    "65.108.0.153",
    "65.108.0.158",
    "65.108.4.194",
    "65.108.4.195",
    "65.108.4.196",
    "65.108.4.213",
    "65.108.74.75",
    "65.108.74.79",
    "65.108.74.109",
    "65.108.246.145",
    "65.109.84.201",
    "65.109.84.203",
    "65.109.84.204",
    "65.109.108.138",
    "65.109.108.141",
    "65.109.108.142",
    "65.109.114.210",
    "65.109.114.211",
    "65.109.114.212",
    "65.109.114.215",
    "65.109.114.236",
    "66.29.147.83",
    "66.245.196.52",
    "70.34.206.123",
    "72.60.38.160",
    "72.62.58.108",
    "75.119.128.71",
    "75.119.137.66",
    "75.119.138.9",
    "75.119.152.219",
    "75.119.153.10",
    "75.119.156.254",
    "77.42.74.68",
    "77.105.61.11",
    "77.105.61.24",
    "77.105.61.44",
    "77.105.61.45",
    "77.105.61.79",
    "77.105.61.142",
    "77.105.61.222",
    "77.105.61.233",
    "77.105.61.237",
    "80.87.206.28",
    "80.87.206.169",
    "80.240.19.200",
    "84.32.22.20",
    "84.32.22.70",
    "84.32.22.141",
    "84.32.22.199",
    "84.32.54.119",
    "84.32.54.120",
    "84.32.54.121",
    "84.247.180.190",
    "84.247.180.198",
    "84.247.180.201",
    "84.247.187.76",
    "85.190.243.3",
    "86.107.168.78",
    "87.228.24.64",
    "88.216.191.0",
    "88.216.191.113",
    "88.216.191.114",
    "88.216.191.115",
    "89.35.131.23",
    "89.35.131.39",
    "89.35.131.149",
    "89.35.131.158",
    "89.35.131.218",
    "89.35.131.219",
    "89.125.50.14",
    "89.125.50.206",
    "89.125.209.27",
    "89.125.209.69",
    "89.125.209.106",
    "89.125.209.110",
    "89.125.209.120",
    "89.125.209.133",
    "89.125.209.152",
    "89.125.209.156",
    "89.125.209.178",
    "89.125.209.193",
    "89.125.209.195",
    "89.125.209.234",
    "91.198.108.35",
    "91.198.108.36",
    "91.198.108.37",
    "91.198.108.38",
    "91.199.149.177",
    "93.115.172.36",
    "93.115.172.37",
    "93.115.172.38",
    "93.115.172.39",
    "95.111.232.18",
    "95.111.233.139",
    "95.111.242.220",
    "95.179.139.125",
    "95.179.159.65",
    "95.179.241.182",
    "95.216.146.18",
    "103.214.68.30",
    "103.214.68.131",
    "104.156.154.179",
    "108.61.165.170",
    "109.123.244.131",
    "109.173.198.67",
    "109.199.97.144",
    "109.199.105.14",
    "109.199.107.155",
    "109.199.108.234",
    "109.199.109.39",
    "109.199.120.79",
    "109.199.124.30",
    "109.199.124.135",
    "114.132.172.215",
    "130.162.233.186",
    "130.185.118.3",
    "132.243.197.16",
    "132.243.197.18",
    "132.243.197.38",
    "132.243.197.40",
    "132.243.197.210",
    "132.243.197.249",
    "132.243.197.250",
    "132.243.197.251",
    "132.243.197.252",
    "134.255.182.185",
    "134.255.182.186",
    "135.181.110.216",
    "135.181.229.49",
    "136.244.99.17",
    "137.74.166.122",
    "139.84.137.143",
    "139.84.170.10",
    "139.84.231.221",
    "139.84.232.129",
    "139.84.236.208",
    "139.99.201.103",
    "139.180.143.115",
    "142.44.136.69",
    "143.198.145.184",
    "144.91.99.105",
    "144.91.105.209",
    "144.91.121.81",
    "149.28.241.190",
    "149.28.247.165",
    "149.102.140.101",
    "151.244.85.45",
    "151.244.85.114",
    "151.244.85.131",
    "151.244.85.153",
    "155.133.26.122",
    "155.138.159.73",
    "156.67.29.45",
    "157.90.238.161",
    "157.173.122.20",
    "157.173.122.21",
    "157.173.122.25",
    "157.173.122.26",
    "158.220.101.10",
    "158.220.101.28",
    "158.247.247.241",
    "159.69.204.162",
    "161.97.66.31",
    "161.97.70.66",
    "161.97.74.173",
    "161.97.75.36",
    "161.97.85.159",
    "161.97.85.182",
    "161.97.88.199",
    "161.97.88.219",
    "161.97.91.68",
    "161.97.91.217",
    "161.97.96.120",
    "161.97.102.156",
    "161.97.106.14",
    "161.97.117.125",
    "161.97.120.156",
    "161.97.142.210",
    "161.97.153.122",
    "161.97.159.172",
    "161.97.163.75",
    "161.97.165.115",
    "161.97.166.49",
    "161.97.173.67",
    "161.97.175.233",
    "161.97.176.38",
    "161.97.180.182",
    "167.86.77.58",
    "167.86.93.21",
    "167.86.94.138",
    "167.88.169.16",
    "167.235.102.194",
    "168.119.102.10",
    "169.58.206.63",
    "172.236.244.81",
    "172.238.7.25",
    "173.199.71.83",
    "173.212.196.214",
    "173.212.231.230",
    "173.212.232.90",
    "173.212.245.118",
    "173.212.251.130",
    "173.249.21.12",
    "177.93.141.60",
    "178.18.244.255",
    "178.18.254.136",
    "185.50.195.156",
    "185.162.233.16",
    "185.162.233.17",
    "185.162.233.20",
    "185.162.233.21",
    "185.162.233.24",
    "185.162.233.25",
    "185.166.217.154",
    "185.185.80.117",
    "185.198.234.12",
    "185.202.236.58",
    "185.209.230.13",
    "185.215.164.84",
    "185.215.164.186",
    "185.215.166.126",
    "185.239.208.110",
    "185.239.209.6",
    "185.252.234.238",
    "188.245.90.255",
    "192.99.153.216",
    "192.175.127.198",
    "192.248.175.198",
    "192.248.178.237",
    "193.70.43.172",
    "193.104.58.62",
    "194.163.166.76",
    "195.26.254.228",
    "195.201.238.55",
    "202.71.14.79",
    "206.245.167.74",
    "206.245.167.76",
    "207.180.213.141",
    "207.180.224.96",
    "207.180.233.6",
    "207.180.233.246",
    "207.244.247.40",
    "213.199.44.112",
    "213.199.53.161",
    "213.199.54.34",
    "213.199.54.35",
    "213.199.54.37",
    "213.199.54.171",
    "216.238.75.46",
    "216.238.99.9",
    "217.76.54.175"
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
    pub is_testnet: bool,
    #[serde(skip)]
    pub explicit_network: Option<Network>,
    #[serde(skip)]
    pub endpoints: Vec<String>,
    #[serde(skip)]
    pub devnet_name: Option<String>,
}

pub trait EntryPoint {
    fn get_runtime(&self) -> Arc<Runtime>;
    fn get_sdk(&self) -> Arc<Sdk>;
    fn get_data_contract(&self, identifier: &Identifier) -> Option<Arc<DataContract>>;
    fn add_data_contract(&mut self, data_contract: &DataContract);
    fn get_request_settings(&self) -> RequestSettings;
}

impl Config {
    /// Explicit client configuration never reads process-wide environment variables.
    pub fn for_network(network: &str, endpoints: Vec<String>) -> Result<Self, String> {
        let (network, devnet_name) = match network {
            "mainnet" => (Network::Mainnet, None),
            "testnet" => (Network::Testnet, None),
            name if name.starts_with("devnet-") && name.len() > 7
                && name[7..].bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') =>
                (Network::Devnet, Some(name.to_owned())),
            _ => return Err("Expected mainnet, testnet, or devnet-<name>".to_owned()),
        };
        if endpoints.is_empty() {
            return Err("At least one explicit DAPI endpoint is required".to_owned());
        }
        for endpoint in &endpoints {
            let uri = Uri::from_str(endpoint).map_err(|e| format!("Invalid DAPI endpoint: {e}"))?;
            if uri.scheme_str() != Some("https") || uri.host().is_none()
                || uri.authority().map_or(true, |a| a.as_str().contains('@'))
                || uri.path() != "/" && !uri.path().is_empty()
                || uri.query().is_some() || endpoint.contains('#')
                || uri.port_u16().is_none() || uri.port_u16() == Some(0) {
                return Err("DAPI endpoints must be HTTPS URLs with an explicit port and no credentials, path, query or fragment".to_owned());
            }
            let _: Address = uri.try_into().map_err(|e| format!("Invalid DAPI address: {e}"))?;
        }
        Ok(Self {
            platform_host: String::new(), platform_port: 0, platform_ssl: true,
            core_ip: String::new(), core_port: 0, core_user: String::new(), core_password: String::new(),
            dump_dir: Self::default_dump_dir(), is_testnet: network != Network::Mainnet,
            explicit_network: Some(network), endpoints, devnet_name,
        })
    }

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
        if self.explicit_network.is_some() {
            return AddressList::from_iter(self.endpoints.iter().map(|endpoint| {
                let uri = Uri::from_str(endpoint).expect("validated endpoint");
                Address::try_from(uri).expect("validated address")
            }));
        }
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
    pub async fn setup_api(&self) -> Arc<Sdk> {
        let sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.address_list())
                .with_network(self.network())
                .with_core(
                &self.core_ip.as_str(),
                self.core_port,
                &self.core_user.as_str(),
                &self.core_password.as_str(),
            );

            builder.build().expect("cannot initialize api")
        };

        sdk.into()
    }

    pub async fn setup_api_list(&self, address_list: Vec<String>) -> Arc<Sdk> {
        let sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.new_address_list(address_list))
                .with_network(self.network())
                .with_core(
                    &self.core_ip.as_str(),
                    self.core_port,
                    &self.core_user.as_str(),
                    &self.core_password.as_str(),
                );

            builder.build().expect("cannot initialize api")
        };

        sdk.into()
    }

    pub async fn setup_api_with_callbacks(&self, q: u64, d: u64) -> Arc<Sdk> {
        let mut context_provider = CallbackContextProvider::new(
            std::ptr::null(),
            q,
            d,
            None,
            Arc::new(DataContractCache::new(NonZeroUsize::new(100).expect("Non Zero"))),
            NonZeroUsize::new(100).expect("Non Zero"),
        ).expect("context provider");
        let mut sdk = {
            // Dump all traffic to disk
            let builder = dash_sdk::SdkBuilder::new(self.address_list())
                .with_network(self.network());
            builder.build().expect("cannot initialize api")
        };
        // not ideal because context provider has a clone of the sdk
        context_provider.set_sdk(Some(Arc::new(sdk.clone())));
        sdk.set_context_provider(context_provider);
        sdk.into()
    }

    pub(crate) async fn setup_api_with_callbacks_cache(
        &self,
        context_provider_context: * const c_void,
        q: u64,
        d: u64,
        data_contract_cache: Arc<DataContractCache>,
        connect_timeout: usize,
        timeout: usize,
        retries: usize,
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
                .with_network(self.network())
                .with_settings(
                    RequestSettings {
                        connect_timeout: Some(Duration::from_secs(connect_timeout as u64)),
                        timeout: Some(Duration::from_secs(timeout as u64)),
                        retries: Some(retries),
                        ban_failed_address: Some(true),
                        max_decoding_message_size: Some(16 * 1024 * 1024)
                    }
                )
                .with_context_provider(context_provider_clone);
            builder.build().expect("cannot initialize api")
        };
        // not ideal because context provider has a clone of the sdk
        context_provider.set_sdk(Some(Arc::new(sdk.clone())));
        sdk.set_context_provider(context_provider);
        sdk.into()
    }

    pub(crate) async fn setup_api_with_callbacks_cache_list(
        &self,
        context: * const c_void,
        q: u64,
        d: u64,
        data_contract_cache: Arc<DataContractCache>,
        address_list: Vec<String>,
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
            let builder = dash_sdk::SdkBuilder::new(self.new_address_list(address_list))
                .with_network(self.network());
            builder.build().expect("cannot initialize api")
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

    pub(crate) fn network(&self) -> Network {
        if let Some(network) = self.explicit_network {
            return network;
        }
        if self.is_testnet {
            Network::Testnet
        } else {
            Network::Mainnet
        }
    }
}

#[cfg(test)]
mod network_configuration_tests {
    use super::*;

    #[test]
    fn explicit_devnets_do_not_fall_back_to_testnet_nodes() {
        let cfg = Config::for_network("devnet-sakura", vec!["https://127.0.0.1:1443".into()]).unwrap();
        assert_eq!(cfg.network(), Network::Devnet);
        assert_eq!(cfg.devnet_name.as_deref(), Some("devnet-sakura"));
        assert_eq!(cfg.endpoints, vec!["https://127.0.0.1:1443"]);
        assert_eq!(cfg.address_list().len(), 1);
    }

    #[test]
    fn explicit_mainnet_and_testnet_remain_distinct() {
        for (name, network) in [("mainnet", Network::Mainnet), ("testnet", Network::Testnet)] {
            let cfg = Config::for_network(name, vec!["https://127.0.0.1:443".into()]).unwrap();
            assert_eq!(cfg.network(), network);
        }
    }

    #[test]
    fn sdk_constructor_preserves_devnet_network_and_endpoints() {
        extern "C" fn quorum_callback(_: *const c_void, _: u32, _: *const u8, _: u32, _: *mut u8) {}
        let endpoints = vec!["https://127.0.0.1:1443".to_owned()];
        let sdk = crate::sdk::create_dash_sdk_with_network(0, quorum_callback as usize as u64, 0,
            "devnet-sakura".into(), endpoints.clone(), 5, 5, 1).unwrap();
        assert_eq!(sdk.sdk.network, Network::Devnet);
        assert_eq!(sdk.config.endpoints, endpoints);
        assert_eq!(sdk.config.devnet_name.as_deref(), Some("devnet-sakura"));
    }

    #[test]
    fn rejects_invalid_networks_and_endpoints() {
        assert!(Config::for_network("devnet-", vec!["https://127.0.0.1:1443".into()]).is_err());
        assert!(Config::for_network("devnet-sakura", vec![]).is_err());
        for endpoint in ["127.0.0.1", "http://127.0.0.1:1443", "https://127.0.0.1",
            "https://127.0.0.1:0", "https://user@127.0.0.1:1443", "https://127.0.0.1:1443/path",
            "https://127.0.0.1:1443?secret=x", "https://127.0.0.1:1443#fragment"] {
            assert!(Config::for_network("devnet-sakura", vec![endpoint.into()]).is_err(), "{endpoint}");
        }
    }
}
