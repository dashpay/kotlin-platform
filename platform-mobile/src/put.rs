use std::collections::{BTreeMap, HashMap};
use std::convert::identity;
use std::io;
use std::io::{Cursor, Write};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;
use dash_sdk::{Error, RequestSettings, Sdk};
use dash_sdk::platform::{DocumentQuery, Fetch};
use dash_sdk::platform::transition::put_document::PutDocument;
use dash_sdk::platform::transition::put_identity::PutIdentity;
use dash_sdk::platform::transition::put_settings::PutSettings;
use dash_sdk::platform::transition::TxId;
use dashcore::hashes::{Hash, sha256};
use dashcore::key::Secp256k1;
use dashcore::secp256k1::{Message, SecretKey};
use dashcore::signer::sign;
use dpp::bincode::{Decode, Encode};
use dashcore::blockdata::transaction::OutPoint;
use dpp::dashcore::{InstantLock, Network, PrivateKey, Transaction, Txid};
use dpp::dashcore::bls_sig_utils::BLSSignature;
use dpp::dashcore::consensus::Decodable;
use dpp::dashcore::hash_types::CycleHash;
use dpp::dashcore::hashes::sha256d;
use dpp::data_contract::{DataContract};
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use dpp::data_contract::DataContract::V0;
use dpp::data_contract::document_type::DocumentType;
use dpp::data_contract::document_type::methods::DocumentTypeV0Methods;
use dpp::document::{Document, DocumentV0Getters};
use dpp::document::v0::DocumentV0;
use dpp::identity::identity::Identity;
use dpp::identity::identity_public_key::accessors::v0::IdentityPublicKeyGettersV0;
use dpp::identity::identity_public_key::IdentityPublicKey;
use dpp::identity::identity_public_key::v0::IdentityPublicKeyV0;
use dpp::identity::identity_public_key::{KeyID, KeyType, Purpose, SecurityLevel};
use dpp::identity::signer::Signer;
use dpp::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;
use dpp::identity::state_transition::asset_lock_proof::{AssetLockProof, InstantAssetLockProof};
use dpp::prelude::{BlockHeight, CoreBlockHeight, UserFeeIncrease};
//use dpp::prelude::{AssetLockProof, BlockHeight, CoreBlockHeight};
use dpp::ProtocolError;
use dpp::util::entropy_generator::{DefaultEntropyGenerator, EntropyGenerator};
use platform_value::{Identifier, IdentifierBytes32, Value};
use platform_value::string_encoding::Encoding;
use platform_value::types::binary_data::BinaryData;
use platform_version::version::PlatformVersion;
use simple_signer::signer::SimpleSigner;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;
use tracing::trace;
use rand::random;
use crate::config::{Config, EntryPoint};
use crate::logs::setup_logs;
use crate::provider::Cache;
use dapi_grpc::platform::v0::{StateTransitionBroadcastError, WaitForStateTransitionResultResponse};
use dapi_grpc::platform::v0::wait_for_state_transition_result_response::{Version, wait_for_state_transition_result_response_v0};
use dash_sdk::platform::transition::top_up_identity::TopUpIdentity;
use dpp::data_contract::document_type::accessors::DocumentTypeV0Getters;
use dpp::state_transition::StateTransition;
use crate::sdk::DashSdk;
use dpp::serialization::Signable;
use dpp::serialization::PlatformSerializable;
use drive_proof_verifier::types::Documents;
use rs_dapi_client::transport::BoxFuture;
use dash_sdk::platform::transition::replace_document::ReplaceDocument;
use dpp::util::hash::hash_double_to_vec;

pub fn get_wait_result_error(response: &WaitForStateTransitionResultResponse) -> Option<&StateTransitionBroadcastError> {
    match &response.version {
        Some(dapi_grpc::platform::v0::wait_for_state_transition_result_response::Version::V0(response_v0)) => {
            return match &response_v0.result {
                Some(wait_for_state_transition_result_response_v0::Result::Error(error)) => Some(&error),
                _ => None
            }
        }
        _ => {}
    }
    None
}

pub async fn wait_for_response_concurrent(
    new_preorder_document: &Document,
    sdk: &Sdk,
    preorder_transition: StateTransition,
    data_contract: Arc<DataContract>,
    settings: PutSettings
) -> Result<Document, dash_sdk::Error> {
    let mut handles = vec![];

    for i in 0..5 {
        let new_preorder_document = new_preorder_document.clone();
        let sdk = sdk.clone();
        let preorder_transition = preorder_transition.clone();
        let data_contract = data_contract.clone();
        let settings = Some(settings.clone());

        tracing::info!("spawning thread {} of 5", i + 1);
        let handle = tokio::spawn(async move {
            <dpp::document::Document as PutDocument<SimpleSigner>>::wait_for_response::<'_, '_, '_>(
                &new_preorder_document,
                &sdk,
                preorder_transition,
                data_contract,
                settings
            ).await
        });

        handles.push(handle);
    }

    let mut success_count = 0;
    let mut error_count = 0;
    let mut last_error: Option<Error> = None;

    for handle in handles {
        match handle.await {
            Ok(Ok(document)) => {
                success_count += 1;
                if success_count >= 3 {
                    tracing::info!("wait_for_response_concurrent, success: {:?}", document);
                    return Ok(document);
                }
            }
            Ok(Err(e)) => {
                error_count += 1;
                tracing::warn!("wait_for_response_concurrent, response error: {:?}", e);
                last_error = Some(Error::from(e));
            }
            Err(e) => {
                tracing::warn!("wait_for_response_concurrent, join error: {:?}", e);
                last_error = Some(Error::Generic(e.to_string()));
            }
        }
    }
    tracing::warn!("wait_for_response_concurrent, all requests failed");

    Err(last_error.unwrap_or(Error::Generic("All requests failed".to_string())))
}

pub async fn wait_for_response_concurrent_identity(
    identity: &Identity,
    sdk: &Sdk,
    state_transition: &StateTransition,
) -> Result<Identity, dash_sdk::Error> {
    let mut handles = vec![];

    for i in 0..5 {
        let sdk = sdk.clone();
        //let settings = Some(settings.clone());
        let identity = identity.clone();
        let state_transition = state_transition.clone();
        tracing::info!("spawning thread {} of 5", i + 1);
        let handle = tokio::spawn(async move {
            <Identity as PutIdentity<SimpleSigner>>::wait_for_response::<'_, '_, '_, '_>(
                &identity,
                &sdk,
                &state_transition
            ).await
        });

        handles.push(handle);
    }

    let mut success_count = 0;
    let mut last_error: Option<Error> = None;

    for handle in handles {
        match handle.await {
            Ok(Ok(identity)) => {
                success_count += 1;
                if success_count >= 3 {
                    tracing::info!("wait_for_response_concurrent, success: {:?}", identity);
                    return Ok(identity);
                }
            }
            Ok(Err(e)) => {
                tracing::warn!("wait_for_response_concurrent, response error: {:?}", e);
                last_error = Some(Error::from(e));
            }
            Err(e) => {
                tracing::warn!("wait_for_response_concurrent, join error: {:?}", e);
                last_error = Some(Error::Generic(e.to_string()));
            }
        }
    }
    tracing::warn!("wait_for_response_concurrent, all requests failed");

    Err(last_error.unwrap_or(Error::Generic("All requests failed".to_string())))
}

//#[ferment_macro::export]
pub type SignerCallback = extern "C" fn(context: usize, key_data: * const u8, key_len: u32, data: * const u8, data_len: u32, result: * mut u8) -> u32;

#[derive(Debug)]
pub struct CallbackSigner {
    signer_callback: SignerCallback,
    signer_context: usize
}

impl CallbackSigner {
    pub fn new(
        signer_context: usize,
        signer_callback: u64,
    ) -> Result<Self, Error> {
        unsafe {
            let callback: SignerCallback = std::mem::transmute(signer_callback as usize);
            Ok(Self {
                signer_callback: callback,
                signer_context: signer_context
            })
        }
    }
}



impl Signer for CallbackSigner {
    /// the public key bytes are only used to look up the private key
    fn sign(
        &self,
        identity_public_key: &IdentityPublicKey,
        data: &[u8],
    ) -> Result<BinaryData, ProtocolError> {
        // stub
        let key_data = identity_public_key.data();
        let mut result = [0u8; 128];
        tracing::info!("CallbackSigner::sign({:?}, {:?})", key_data.as_slice(), data);
        let length = (self.signer_callback)(self.signer_context, key_data.as_slice().as_ptr(), key_data.len() as u32, data.as_ptr(), data.len() as u32, result.as_mut_ptr());

        // Check the return value to determine if the operation was successful
        if length > 0 {
            // If 'length' is positive, it indicates the size of the signature
            // Create a Vec<u8> from 'result' up to 'length'
            Ok(BinaryData(result[..length as usize].to_vec()))
        } else {
            // Handle error scenario, for example by converting 'length' to an error code
            Err(ProtocolError::InvalidSigningKeyTypeError("something is wrong.  signer callback returned 0".to_string())) // Assuming there is a way to convert to ProtocolError
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[ferment_macro::export]
pub struct OutPointFFI {
    /// The referenced transaction's txid.
    pub txid: [u8; 32],
    /// The index of the referenced output in its transaction's vout.
    pub vout: u32,
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn OutPointFFI_clone(a: OutPoint) -> OutPoint {
    a.clone()
}

// impl From<OutPointFFI> for OutPoint {
//     fn from(value: OutPointFFI) -> Self {
//         Self {
//             txid: Txid::from_raw_hash(sha256d::Hash::from_slice(value.txid.as_slice()).unwrap()),
//             vout: value.vout,
//         }
//     }
// }

// #[derive(Clone, Eq, PartialEq)]
// /// Instant send lock is a mechanism used by the Dash network to
// /// confirm transaction within 1 or 2 seconds. This data structure
// /// represents a p2p message containing a data to verify such a lock.
// pub struct InstantLockFFI {
//     /// Instant lock version
//     pub version: u8,
//     /// Transaction inputs locked by this instant lock
//     pub inputs: Vec<OutPointFFI>,
//     /// Transaction hash locked by this lock
//     pub txid: [u8; 32],
//     /// Hash to figure out which quorum was used to sign this IS lock
//     pub cyclehash: [u8; 32],
//     /// Quorum signature for this IS lock
//     pub signature: [u8; 96],
// }

#[derive(Clone, PartialEq, Eq, Debug)]
#[ferment_macro::export]
pub struct ChainAssetLockProofFFI {
    /// Core height on which the asset lock transaction was chain locked or higher
    pub core_chain_locked_height: u32,
    /// A reference to Asset Lock Special Transaction ID and output index in the payload
    pub out_point: OutPoint,
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn ChainAssetLockProofFFI_clone(a: ChainAssetLockProofFFI) -> ChainAssetLockProofFFI {
    a.clone()
}

impl From<ChainAssetLockProofFFI> for ChainAssetLockProof {
    fn from(value: ChainAssetLockProofFFI) -> Self {
        ChainAssetLockProof {
            core_chain_locked_height: value.core_chain_locked_height,
            out_point: value.out_point.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[ferment_macro::export]
pub struct InstantAssetLockProofFFI {
    /// The transaction's Instant Lock
    pub instant_lock: Vec<u8>,
    /// Asset Lock Special Transaction
    pub transaction: Vec<u8>,
    /// Index of the output in the transaction payload
    pub output_index: u32,
}

#[allow(non_snake_case)]

#[ferment_macro::export]
pub fn InstantAssetLockProofFFI_clone(a: InstantAssetLockProofFFI) -> InstantAssetLockProofFFI {
    a.clone()
}

impl From<InstantAssetLockProofFFI> for InstantAssetLockProof {
    fn from(value: InstantAssetLockProofFFI) -> Self {
        let mut islock_cursor = Cursor::new(value.instant_lock);
        let mut transaction_cursor = Cursor::new(value.transaction);
        InstantAssetLockProof {
            instant_lock: InstantLock::consensus_decode(&mut islock_cursor).unwrap(),
            transaction: Transaction::consensus_decode(&mut transaction_cursor).unwrap(),
            output_index: value.output_index,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[ferment_macro::export]
pub enum AssetLockProofFFI {
    Instant(InstantAssetLockProofFFI),
    Chain(ChainAssetLockProofFFI),
}

impl From<AssetLockProofFFI> for AssetLockProof {
    fn from(value: AssetLockProofFFI) -> Self {
        match value {
            AssetLockProofFFI::Instant(instant) => AssetLockProof::Instant(instant.into()),
            AssetLockProofFFI::Chain(chain) => AssetLockProof::Chain(chain.into())
        }
    }
}

#[ferment_macro::export]
pub fn put_identity_sdk(
    rust_sdk: *mut DashSdk,
    identity: Identity,
    asset_lock_proof: AssetLockProofFFI,
    asset_lock_proof_private_key: Vec<u8>,
    signer_context: usize,
    signer_callback: u64,
    is_testnet: bool
) -> Result<Identity, String> {
    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        // Your async code here
        trace!("Setting up SDK");
        let sdk = unsafe { (*rust_sdk).get_sdk() };
        trace!("Finished SDK, {:?}", sdk);
        trace!("Set up network, private key and signer");

        let network = if is_testnet {
            Network::Testnet
        } else {
            Network::Dash
        };
        let private_key = match PrivateKey::from_slice(asset_lock_proof_private_key.as_slice(), network) {
            Ok(pk) => pk,
            Err(e) => return Err(e.to_string())
        };
        let signer = CallbackSigner::new(signer_context, signer_callback).expect("signer");
        let request_settings = unsafe { (*rust_sdk).get_request_settings() };
        tracing::info!("call Identity::put_to_platform & wait_for_response");

        let asset_lock_proof: AssetLockProof = asset_lock_proof.into();
        // this PR has not been merged yet, but there is a way to detect if the put_identity will fail
        // match asset_lock_proof.verify(&sdk) {
        //     Ok(_) => {},
        //     Err(error) => {
        //         return Err(error.to_string())
        //     }
        // }

        let state_transition_result = Identity::put_to_platform(
            &identity,
            &sdk,
            asset_lock_proof.into(),
            &private_key,
            &signer,
            request_settings
        ).await;

        let state_transition = match state_transition_result {
            Ok(st) => st,
            Err(err) => return Err(err.to_string())
        };

        //tracing::info!("state transition (signable): {}", hex::encode(state_transition.signable_bytes().unwrap()));
        //tracing::info!("state transition (serialized): {}", hex::encode(state_transition.serialize_to_bytes().unwrap()));
        tracing::info!(
            "state transition (hash): {}",
            hex::encode(hash_double_to_vec(state_transition.serialize_to_bytes().unwrap()))
        );

        let identity_result = wait_for_response_concurrent_identity(
            &identity,
            &sdk,
            &state_transition
        ).await;

        return match identity_result {
            Ok(identity) => Ok(identity),
            Err(e) => Err(e.to_string())
        }
    })
}

#[ferment_macro::export]
pub fn topup_identity_sdk(
    rust_sdk: *mut DashSdk,
    identity: Identity,
    asset_lock_proof: AssetLockProofFFI,
    asset_lock_proof_private_key: Vec<u8>,
    is_testnet: bool
) -> Result<u64, String> {
    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        // Your async code here
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        let network = if is_testnet {
            Network::Testnet
        } else {
            Network::Dash
        };
        let private_key = match PrivateKey::from_slice(asset_lock_proof_private_key.as_slice(), network) {
            Ok(pk) => pk,
            Err(e) => return Err(e.to_string())
        };

        let user_fee_increase = 1;
        let request_settings = unsafe { (*rust_sdk).get_request_settings() };

        trace!("call Identity::top_up_identity");
        let identity_result = identity.top_up_identity(
            &sdk,
            asset_lock_proof.into(),
            &private_key,
            Some(user_fee_increase),
            request_settings
        ).await;

        match identity_result {
            Ok(identity) => Ok(identity),
            Err(err) => Err(err.to_string())
        }
    })
}

fn put_document_with_retry(
    sdk: Arc<Sdk>,
    data_contract_cache: Arc<Cache<Identifier, DataContract>>,
    new_document: Document,
    document_type: DocumentType,
    entropy: [u8; 32],
    identity_public_key: IdentityPublicKey,
    signer_callback: CallbackSigner,
    put_settings: PutSettings,
    retries_left: usize,
) -> BoxFuture<'static, Result<StateTransition, Error>> {
    Box::pin(async move {
        match new_document.put_to_platform(
            &sdk,
            document_type.clone(),
            entropy.clone(),
            identity_public_key.clone(),
            &signer_callback,
            Some(put_settings)
        ).await {
            Ok(documents) => Ok(documents),
            Err(error) => {
                if retries_left > 1 {
                    if error.to_string().contains("contract not found error") {
                        if (data_contract_cache.get(&document_type.data_contract_id()) != None) {
                            return put_document_with_retry(
                                sdk,
                                data_contract_cache,
                                new_document,
                                document_type,
                                entropy,
                                identity_public_key,
                                signer_callback,
                                put_settings,
                                retries_left - 1
                            ).await;
                        }
                    }
                }
                Err(error)
            }
        }
    })
}

#[ferment_macro::export]
pub fn put_document_sdk(
    rust_sdk: *mut DashSdk,
    document: Document,
    data_contract_id: Identifier,
    document_type_str: String,
    identity_public_key: IdentityPublicKey,
    block_height: BlockHeight,
    core_block_height: CoreBlockHeight,
    signer_context: usize,
    signer_callback: u64
) -> Result<Document, String> {

    let rt = unsafe { (*rust_sdk).get_runtime() };
    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        let data_contract = match unsafe { (*rust_sdk).get_data_contract(&data_contract_id) } {
            Some(data_contract) => data_contract.clone(),
            None => {
                let request_settings = unsafe { (*rust_sdk).get_request_settings() };
                match (DataContract::fetch_with_settings(&sdk, data_contract_id.clone(), request_settings)
                    .await) {
                    Ok(Some(data_contract)) => {
                        unsafe { (*rust_sdk).add_data_contract(&data_contract); };
                        Arc::new(data_contract)
                    },
                    Ok(None) => return Err("data contract not found".to_string()),
                    Err(e) => return Err(e.to_string())
                }
            }
        };

        let document_type = data_contract
            .document_type_for_name(&document_type_str)
            .expect("expected a profile document type");

        let signer = CallbackSigner::new(signer_context, signer_callback).expect("signer");
        let entropy_generator = DefaultEntropyGenerator;
        let entropy = entropy_generator.generate().unwrap();
        trace!("document_entropy: {:?}", entropy);
        trace!("IdentityPublicKey: {:?}", identity_public_key);

        // recreate the document using the same entropy value as when it is submitted below
        let new_document_result = document_type.create_document_from_data(
            document.properties().into(),
            document.owner_id(),
            block_height,
            core_block_height,
            entropy,
            PlatformVersion::latest()
        );

        let new_document = match new_document_result {
            Ok(doc) => doc,
            Err(e) => return Err(e.to_string())
        };
        let request_settings = unsafe { (*rust_sdk).get_request_settings() };

        let settings = PutSettings {
            request_settings,
            identity_nonce_stale_time_s: None,
            user_fee_increase: None,
        };

        trace!("call Document::put_to_platform & wait_for_response");
        let data_contract_cache = unsafe {&(*rust_sdk).data_contract_cache.clone() };
        let extra_retries = settings.request_settings.retries.unwrap_or_else(|| 5usize);
        let transition = put_document_with_retry(
            sdk.clone(),
            data_contract_cache.clone(),
            new_document.clone(),
            document_type.to_owned_document_type(),
            entropy.clone(),
            identity_public_key.clone(),
            signer,
            settings,
            extra_retries
        ).await.or_else(|err|Err(err.to_string()))?;
        tracing::info!(
            "state transition (hash): {}",
            hex::encode(hash_double_to_vec(transition.serialize_to_bytes().unwrap()))
        );
        let result_document = wait_for_response_concurrent(
            &new_document,
            &sdk,
            transition.clone(),
            data_contract.clone(),
            settings
        ).await.or_else(|err|Err(err.to_string()))?;

        Ok(result_document)
    })
}

fn replace_document_with_retry(
    sdk: Arc<Sdk>,
    data_contract_cache: Arc<Cache<Identifier, DataContract>>,
    new_document: Document,
    document_type: DocumentType,
    identity_public_key: IdentityPublicKey,
    signer_callback: CallbackSigner,
    put_settings: PutSettings,
    retries_left: usize,
) -> BoxFuture<'static, Result<StateTransition, Error>> {
    Box::pin(async move {
        match new_document.replace_on_platform(
            &sdk,
            document_type.clone(),
            identity_public_key.clone(),
            &signer_callback,
            Some(put_settings)
        ).await {
            Ok(documents) => Ok(documents),
            Err(error) => {
                if retries_left > 1 {
                    if error.to_string().contains("contract not found error") {
                        if (data_contract_cache.get(&document_type.data_contract_id()) != None) {
                            return replace_document_with_retry(
                                sdk,
                                data_contract_cache,
                                new_document,
                                document_type,
                                identity_public_key,
                                signer_callback,
                                put_settings,
                                retries_left - 1
                            ).await;
                        }
                    }
                }
                Err(error)
            }
        }
    })
}

#[ferment_macro::export]
pub fn replace_document_sdk(
    rust_sdk: *mut DashSdk,
    document: Document,
    data_contract_id: Identifier,
    document_type_str: String,
    identity_public_key: IdentityPublicKey,
    block_height: BlockHeight,
    core_block_height: CoreBlockHeight,
    signer_context: usize,
    signer_callback: u64
) -> Result<Document, String> {
    let rt = unsafe { (*rust_sdk).get_runtime() };


    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        // Your async code here
        let cfg = Config::new();
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        let data_contract = match unsafe { (*rust_sdk).get_data_contract(&data_contract_id) } {
            Some(data_contract) => data_contract.clone(),
            None => {
                let request_settings = unsafe { (*rust_sdk).get_request_settings() };
                match (DataContract::fetch_with_settings(&sdk, data_contract_id.clone(), request_settings)
                    .await) {
                    Ok(Some(data_contract)) => {
                        unsafe { (*rust_sdk).add_data_contract(&data_contract); };
                        Arc::new(data_contract)
                    },
                    Ok(None) => return Err("data contract not found".to_string()),
                    Err(e) => return Err(e.to_string())
                }
            }
        };

        let document_type = data_contract
            .document_type_for_name(&document_type_str)
            .expect("expected a profile document type");

        let signer = CallbackSigner::new(signer_context, signer_callback).expect("signer");

        trace!("IdentityPublicKey: {:?}", identity_public_key);
        let request_settings = unsafe { (*rust_sdk).get_request_settings() };

        let settings = PutSettings {
            request_settings,
            identity_nonce_stale_time_s: None,
            user_fee_increase: None,
        };

        trace!("call Document::replace_on_platform & wait_for_response");

        let data_contract_cache = unsafe {&(*rust_sdk).data_contract_cache.clone() };
        let extra_retries = settings.request_settings.retries.unwrap_or_else(|| 5usize);
        let transition = replace_document_with_retry(
            sdk.clone(),
            data_contract_cache.clone(),
            document.clone(),
            document_type.to_owned_document_type(),
            identity_public_key.clone(),
            signer,
            settings,
            extra_retries
        ).await.or_else(|err|Err(err.to_string()))?;
        tracing::info!(
            "state transition (hash): {}",
            hex::encode(hash_double_to_vec(transition.serialize_to_bytes().unwrap()))
        );
        let result_document = wait_for_response_concurrent(
            &document,
            &sdk,
            transition.clone(),
            data_contract,
            settings
        ).await.or_else(|err|Err(err.to_string()))?;

        Ok(result_document)
    })
}
