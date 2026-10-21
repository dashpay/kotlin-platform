use crate::config::EntryPoint;
use crate::put::CallbackSigner;
use crate::sdk::DashSdk;
use dash_sdk::platform::contract_groups::ContractGroupInfo;
use dash_sdk::platform::documents::transitions::delete::DocumentDeleteTransitionBuilder;
use dash_sdk::platform::transition::put_settings::PutSettings;
use dash_sdk::platform::Fetch;
use dpp::data_contract::DataContract;
use dpp::document::Document;
use dpp::identity::Identity;
use dpp::identity::identity_public_key::IdentityPublicKey;
use platform_value::types::identifier::Identifier;
use std::sync::Arc;

/// Reads Platform's proof-verified current contract-key index, including disabled entries.
/// Empty bytes mean the index has no key; errors are never converted into absence.
#[ferment_macro::export]
pub fn fetch_current_contract_key_id_with_sdk(
    rust_sdk: *mut DashSdk,
    identity_id: Identifier,
    contract_id: Identifier,
    purpose: u32,
) -> Result<Vec<u8>, String> {
    use dash_sdk::platform::identities_contract_keys_query::IdentitiesContractKeysQuery;
    use dpp::identity::identity_public_key::accessors::v0::IdentityPublicKeyGettersV0;
    use dpp::identity::identities_contract_keys::IdentitiesContractKeys;
    use dpp::identity::Purpose;
    let purpose = match purpose {
        1 => Purpose::ENCRYPTION,
        2 => Purpose::DECRYPTION,
        _ => return Err("Current contract-key lookup requires encryption or decryption".into()),
    };
    let handle = unsafe { &*rust_sdk };
    handle.get_runtime().block_on(async {
        let query = IdentitiesContractKeysQuery::new(vec![identity_id], contract_id, None, vec![purpose])
            .map_err(|e| e.to_string())?;
        let found = IdentitiesContractKeys::fetch_with_settings(
            &handle.get_sdk(), query, handle.get_request_settings(),
        ).await.map_err(|e| e.to_string())?;
        let key = found.and_then(|mut identities| identities.remove(&identity_id))
            .and_then(|mut keys| keys.remove(&purpose)).flatten();
        Ok(key.map(|key| key.id().to_string().into_bytes()).unwrap_or_default())
    })
}

/// Fetches a group's owner through the SDK's proof-verifying query path.
/// `None` means the group does not exist; transport/proof failures remain errors.
#[ferment_macro::export]
pub fn fetch_contract_group_owner_with_sdk(
    rust_sdk: *mut DashSdk,
    group_id: Identifier,
) -> Result<Option<Identifier>, String> {
    let sdk_handle = unsafe { &*rust_sdk };
    sdk_handle.get_runtime().block_on(async {
        ContractGroupInfo::fetch_with_settings(
            &sdk_handle.get_sdk(),
            group_id,
            sdk_handle.get_request_settings(),
        )
        .await
        .map(|info| info.map(|info| *info.owner().owner_id()))
        .map_err(|e| e.to_string())
    })
}

/// Reads the block timestamp from a fresh proof-verified identity query.
#[ferment_macro::export]
pub fn fetch_platform_time_with_sdk(
    rust_sdk: *mut DashSdk,
    identity_id: Identifier,
) -> Result<u64, String> {
    let sdk_handle = unsafe { &*rust_sdk };
    sdk_handle.get_runtime().block_on(async {
        Identity::fetch_with_metadata(
            &sdk_handle.get_sdk(),
            identity_id,
            Some(sdk_handle.get_request_settings()),
        )
        .await
        .map(|(_, metadata)| metadata.time_ms)
        .map_err(|e| e.to_string())
    })
}

/// Deletes using the complete original document, including all index-only values.
/// Success proves absence of the entry, not that this particular delete executed.
#[ferment_macro::export]
pub fn delete_document_sdk(
    rust_sdk: *mut DashSdk,
    document: Document,
    data_contract_id: Identifier,
    document_type: String,
    identity_public_key: IdentityPublicKey,
    signer_context: usize,
    signer_callback: u64,
) -> Result<Document, String> {
    if signer_callback == 0 {
        return Err("A document delete requires a signer callback".to_string());
    }
    let sdk_handle = unsafe { &*rust_sdk };
    sdk_handle.get_runtime().block_on(async {
        let sdk = sdk_handle.get_sdk();
        let contract = DataContract::fetch_with_settings(
            &sdk,
            data_contract_id,
            sdk_handle.get_request_settings(),
        )
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "data contract not found".to_string())?;
        let signer =
            CallbackSigner::new(signer_context, signer_callback).map_err(|e| e.to_string())?;
        let builder = DocumentDeleteTransitionBuilder::from_document(
            Arc::new(contract),
            document_type,
            &document,
        )
        .with_settings(PutSettings {
            request_settings: sdk_handle.get_request_settings(),
            identity_nonce_stale_time_s: None,
            user_fee_increase: None,
            wait_timeout: None,
            state_transition_creation_options: None,
        });
        sdk.document_delete(builder, &identity_public_key, &signer)
            .await
            .map_err(|e| e.to_string())?;
        Ok(document)
    })
}
