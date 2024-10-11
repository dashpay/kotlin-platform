//! Example ContextProvider that uses the Core gRPC API to fetch data from the platform.

use std::hash::Hash;
use std::num::NonZeroUsize;
use std::os::raw::c_void;
use std::ptr::null;
use std::sync::Arc;
use std::thread;
use dapi_grpc::tonic::codegen::Body;

use dpp::data_contract::DataContract;
use platform_value::types::identifier::Identifier;
use drive_proof_verifier::error::ContextProviderError;
use drive_proof_verifier::ContextProvider;
use platform_value::converter::serde_json;
use dash_sdk::platform::Fetch;
use dash_sdk::{Sdk, Error};
use dpp::identity::IdentityPublicKey;
use dpp::prelude::CoreBlockHeight;
use platform_value::types::binary_data::BinaryData;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
use crate::config::Config;

// not supported

// not supported with ferment
type QuorumPublicKeyCallback = extern "C" fn(context: * const c_void, quorum_type: u32, quorum_hash: *const u8, core_chain_locked_height: u32, result: * mut u8);

//#[ferment_macro::export]
type DataContractCallback = extern "C" fn(id: &Identifier) -> DataContract;

/// Context provider that uses the Core gRPC API to fetch data from the platform.
///
/// Example [ContextProvider] used by the Sdk for testing purposes.
///
pub struct CallbackContextProvider {
    pub context: * const c_void,
    pub quorum_public_key_callback: QuorumPublicKeyCallback,
    pub data_contract_callback: DataContractCallback,
    /// [Sdk] to use when fetching data from Platform
    ///
    /// Note that if the `sdk` is `None`, the context provider will not be able to fetch data itself and will rely on
    /// values set by the user in the caches: `data_contracts_cache`, `quorum_public_keys_cache`.
    ///
    /// We use [Arc] as we have circular dependencies between Sdk and ContextProvider.
    sdk: Option<Arc<Sdk>>,

    /// Data contracts cache.
    ///
    /// Users can insert new data contracts into the cache using [`Cache::put`].
    pub data_contracts_cache: Arc<Cache<Identifier, DataContract>>,

    /// Quorum public keys cache.
    ///
    /// Key is a tuple of quorum hash and quorum type. Value is a quorum public key.
    ///
    /// Users can insert new quorum public keys into the cache using [`Cache::put`].
    pub quorum_public_keys_cache: Cache<([u8; 32], u32), [u8; 48]>,

    /// Directory where to store dumped data.
    ///
    /// This is used to store data that is fetched from the platform and can be used for testing purposes.
    #[cfg(feature = "mocks")]
    pub dump_dir: Option<std::path::PathBuf>,
}

impl CallbackContextProvider {
    /// Create new context provider.
    ///
    /// Note that if the `sdk` is `None`, the context provider will not be able to fetch data itself and will rely on
    /// values set by the user in the caches: `data_contracts_cache`, `quorum_public_keys_cache`.
    ///
    /// Sdk can be set later with [`CallbackContextProvider::set_sdk`].
    pub fn new(
        context: * const c_void,
        quorum_public_key_callback: u64,
        data_contract_callback: u64,
        sdk: Option<Arc<Sdk>>,
        data_contract_cache: Arc<Cache<Identifier, DataContract>>,
        quorum_public_keys_cache_size: NonZeroUsize,
    ) -> Result<Self, Error> {
        unsafe {
            let callback1: QuorumPublicKeyCallback = std::mem::transmute(quorum_public_key_callback as usize);
            let callback2: DataContractCallback = std::mem::transmute(data_contract_callback as usize);
            Ok(Self {
                context,
                quorum_public_key_callback: callback1,
                data_contract_callback: callback2,
                sdk,
                data_contracts_cache: data_contract_cache,
                quorum_public_keys_cache: Cache::new(quorum_public_keys_cache_size),
                #[cfg(feature = "mocks")]
                dump_dir: None,
            })
        }
    }

    /// Set the Sdk to use when fetching data from Platform.
    /// This is useful when the Sdk is created after the ContextProvider.
    ///
    /// Note that if the `sdk` is `None`, the context provider will not be able to fetch data itself and will rely on
    /// values set by the user in the caches: `data_contracts_cache`, `quorum_public_keys_cache`.
    pub fn set_sdk(&mut self, sdk: Option<Arc<Sdk>>) {
        self.sdk = sdk;
    }
}

impl ContextProvider for CallbackContextProvider {
    fn get_quorum_public_key(
        &self,
        quorum_type: u32,
        quorum_hash: [u8; 32], // quorum hash is 32 bytes
        core_chain_locked_height: u32,
    ) -> Result<[u8; 48], ContextProviderError> {
        tracing::info!("get_quorum_public_key: executing");
        if let Some(key) = self
            .quorum_public_keys_cache
            .get(&(quorum_hash, quorum_type))
        {
            tracing::info!("get_quorum_public_key: returning from Cache");
            return Ok(*key);
        };

        let mut key: [u8; 48] = [0; 48]; // To store the result

        (self.quorum_public_key_callback)(self.context, quorum_type, quorum_hash.as_ptr(), core_chain_locked_height, key.as_mut_ptr());
        tracing::info!("get_quorum_public_key: returning {:?}", key);

        if key == [0; 48] {
            let message = format!("quorum not found {}:{:?}", quorum_type, quorum_hash);
            Err(ContextProviderError::InvalidQuorum(message))
        } else {
            // store key in cache and return Ok
            self.quorum_public_keys_cache
                .put((quorum_hash, quorum_type), key);

            Ok(key)
        }
    }

    fn get_data_contract(
        &self,
        data_contract_id: &Identifier,
    ) -> Result<Option<Arc<DataContract>>, ContextProviderError> {
        return if let Some(contract) = self.data_contracts_cache.get(data_contract_id) {
            Ok(Some(contract))
        } else {
            Ok(None)
        };
        // tracing::info!("CallbackContextProvider: {:p}", &self as *const _);

        // on android, this gets stuck (deadlock)
        // let sdk = match &self.sdk {
        //     Some(sdk) => sdk,
        //     None => {
        //         tracing::warn!("data contract cache miss and no sdk provided, skipping fetch");
        //         return Ok(None);
        //     }
        // };
        //
        // let handle = Handle::current();
        // let data_contract = tokio::task::block_in_place(|| {
        //     handle.block_on(async {
        //         DataContract::fetch(&sdk, *data_contract_id).await
        //     }).map_err(|e| ContextProviderError::DataContractFailure(e.to_string()))
        // })?;
        // if let Some(ref dc) = data_contract {
        //     self.data_contracts_cache.put(*data_contract_id, dc.clone());
        // };
        //
        // Ok(data_contract.map(Arc::new))
    }

    fn get_platform_activation_height(&self) -> Result<CoreBlockHeight, ContextProviderError> {
        Ok(1_000_000)
    }
}

unsafe impl Send for CallbackContextProvider {}
unsafe impl Sync for CallbackContextProvider {}


/// Thread-safe cache of various objects inside the SDK.
///
/// This is used to cache objects that are expensive to fetch from the platform, like data contracts.
pub struct Cache<K: Hash + Eq, V> {
    // We use a Mutex to allow access to the cache when we don't have mutable &self
    // And we use Arc to allow multiple threads to access the cache without having to clone it
    inner: std::sync::RwLock<lru::LruCache<K, Arc<V>>>,
}

impl<K: Hash + Eq, V> Cache<K, V> {
    /// Create new cache
    pub fn new(capacity: NonZeroUsize) -> Self {
        Self {
            // inner: std::sync::Mutex::new(lru::LruCache::new(capacity)),
            inner: std::sync::RwLock::new(lru::LruCache::new(capacity)),
        }
    }

    /// Get a reference to the value stored under `k`.
    pub fn get(&self, k: &K) -> Option<Arc<V>> {
        let mut guard = self.inner.write().expect("cache lock poisoned");
        guard.get(k).map(Arc::clone)
    }

    /// Insert a new value into the cache.
    pub fn put(&self, k: K, v: V) {
        let mut guard = self.inner.write().expect("cache lock poisoned");
        guard.put(k, Arc::new(v));
    }
}

// #[ferment_macro::opaque]
// pub struct PlatformSDK {
//     context: * const std::os::raw::c_void
// }
//
// #[ferment_macro::export]
// impl PlatformSDK {
//     pub fn new<
//         QP: Fn(*const std::os::raw::c_void, u32, [u8; 32], u32) -> Result<[u8; 48], ContextProviderError> /*+ Send + Sync + 'static*/,
//         //DC: Fn(*const std::os::raw::c_void, Identifier) -> Result<Option<Arc<DataContract>>, ContextProviderError> + Send + Sync + 'static,
//         //CS: Fn(*const std::os::raw::c_void, IdentityPublicKey, Vec<u8>) -> Result<BinaryData, ProtocolError> + Send + Sync + 'static,
//     >(
//         get_quorum_public_key: QP,
//         //get_data_contract: DC,
//         //callback_signer: CS,
//         //address_list: Option<Vec<&'static str>>,
//         context: *const std::os::raw::c_void,
//     ) -> Self {
//         Self {
//             context,
//         }
//     }
// }
