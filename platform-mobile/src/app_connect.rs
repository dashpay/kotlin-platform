use crate::config::EntryPoint;
use crate::put::CallbackSigner;
use crate::sdk::DashSdk;
use dash_sdk::platform::contract_groups::{
    ContractGroupInfo, ContractGroupMembersPage, ContractGroupMembersPageQuery,
};
use dapi_grpc::platform::v0::{Proof, ResponseMetadata};
use dash_sdk::platform::documents::transitions::delete::DocumentDeleteTransitionBuilder;
use dash_sdk::platform::transition::put_settings::PutSettings;
use dash_sdk::platform::Fetch;
use dpp::data_contract::DataContract;
use dpp::document::Document;
use dpp::identity::Identity;
use dpp::identity::identity_public_key::IdentityPublicKey;
use platform_value::types::identifier::Identifier;
use platform_value::string_encoding::Encoding;
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

const GROUP_SNAPSHOT_CHANGED: &str = "Contract group changed while reading its verified scope. Retry to refresh.";
const GROUP_PAGE_SIZE: u16 = 100;
const MAX_REVIEW_GROUP_MEMBERS: usize = 1000;

#[derive(PartialEq, Eq)]
struct GroupSnapshotBlock {
    height: u64,
    chain_id: String,
    block_id: Vec<u8>,
}

impl GroupSnapshotBlock {
    fn from_proof(metadata: &ResponseMetadata, proof: &Proof) -> Result<Self, String> {
        if metadata.height == 0 || proof.block_id_hash.len() != 32 {
            return Err("Missing verified contract group block identity".into());
        }
        Ok(Self { height: metadata.height, chain_id: metadata.chain_id.clone(), block_id: proof.block_id_hash.clone() })
    }

    fn require_same(&self, metadata: &ResponseMetadata, proof: &Proof) -> Result<(), String> {
        if *self != Self::from_proof(metadata, proof)? {
            return Err(GROUP_SNAPSHOT_CHANGED.into());
        }
        Ok(())
    }
}

fn group_page_values(page: &ContractGroupMembersPage) -> Vec<serde_json::Value> {
    match page {
        ContractGroupMembersPage::Contracts(ids) => ids.iter().map(|id| serde_json::json!(id.to_string(Encoding::Base58))).collect(),
        ContractGroupMembersPage::DocumentTypes(entries) => entries.iter().map(|(id, name)| serde_json::json!({
            "contractId": id.to_string(Encoding::Base58), "documentType": name,
        })).collect(),
        ContractGroupMembersPage::Tokens(entries) => entries.iter().map(|(id, position)| serde_json::json!({
            "contractId": id.to_string(Encoding::Base58), "tokenPosition": position,
        })).collect(),
    }
}

fn append_group_page(
    query: &ContractGroupMembersPageQuery,
    page: &ContractGroupMembersPage,
    values: &mut Vec<serde_json::Value>,
) -> Result<Option<ContractGroupMembersPageQuery>, String> {
    if page.len() > GROUP_PAGE_SIZE as usize || values.len() + page.len() > MAX_REVIEW_GROUP_MEMBERS {
        return Err("Contract group is too large to review completely in this wallet".into());
    }
    values.extend(group_page_values(page));
    if page.len() < GROUP_PAGE_SIZE as usize {
        return Ok(None);
    }
    let next = query.after(page).ok_or("Missing contract group page cursor")?;
    if next == *query {
        return Err("Contract group page cursor did not advance".into());
    }
    Ok(Some(next))
}

async fn fetch_group_members(
    handle: &DashSdk,
    mut query: ContractGroupMembersPageQuery,
    block: &GroupSnapshotBlock,
) -> Result<Vec<serde_json::Value>, String> {
    let mut values = Vec::new();
    loop {
        let (page, metadata, proof) = ContractGroupMembersPage::fetch_with_metadata_and_proof(
            &handle.get_sdk(), query.clone(), Some(handle.get_request_settings()),
        ).await.map_err(|e| e.to_string())?;
        block.require_same(&metadata, &proof)?;
        let page = page.ok_or("Missing proved contract group members page")?;
        match append_group_page(&query, &page, &mut values)? {
            Some(next) => query = next,
            None => return Ok(values),
        }
    }
}

async fn fetch_group_snapshot(handle: &DashSdk, group_id: Identifier) -> Result<serde_json::Value, String> {
    let (info, metadata, proof) = ContractGroupInfo::fetch_with_metadata_and_proof(
        &handle.get_sdk(), group_id, Some(handle.get_request_settings()),
    ).await.map_err(|e| e.to_string())?;
    let Some(info) = info else { return Ok(serde_json::Value::Null); };
    let block = GroupSnapshotBlock::from_proof(&metadata, &proof)?;
    let (contracts, document_types, tokens) = tokio::try_join!(
        fetch_group_members(handle, ContractGroupMembersPageQuery::contracts(group_id).with_limit(GROUP_PAGE_SIZE), &block),
        fetch_group_members(handle, ContractGroupMembersPageQuery::document_types(group_id).with_limit(GROUP_PAGE_SIZE), &block),
        fetch_group_members(handle, ContractGroupMembersPageQuery::tokens(group_id).with_limit(GROUP_PAGE_SIZE), &block),
    )?;
    if contracts.len() + document_types.len() + tokens.len() > MAX_REVIEW_GROUP_MEMBERS {
        return Err("Contract group is too large to review completely in this wallet".into());
    }
    let admins: Vec<String> = info.owner().admin_ids().into_iter().flatten()
        .map(|id| id.to_string(Encoding::Base58)).collect();
    Ok(serde_json::json!({
        "id": group_id.to_string(Encoding::Base58), "ownerId": info.owner().owner_id().to_string(Encoding::Base58),
        "adminIds": admins, "name": info.name(), "description": info.description(),
        "contracts": contracts, "documentTypes": document_types, "tokens": tokens,
    }))
}

/// A complete proof-verified group scope at one block, not an unproved or owner-only approximation.
/// Null JSON means proved absence; partial reads and proof failures are errors.
#[ferment_macro::export]
pub fn fetch_contract_group_snapshot_with_sdk(rust_sdk: *mut DashSdk, group_id: Identifier) -> Result<Vec<u8>, String> {
    let handle = unsafe { &*rust_sdk };
    handle.get_runtime().block_on(async {
        for attempt in 0..3 {
            match fetch_group_snapshot(handle, group_id).await {
                Err(e) if e == GROUP_SNAPSHOT_CHANGED && attempt < 2 => continue,
                result => return serde_json::to_vec(&result?).map_err(|e| e.to_string()),
            }
        }
        unreachable!()
    })
}

#[cfg(test)]
mod group_scope_tests {
    use super::*;

    #[test]
    fn scope_requires_one_verified_block() {
        let metadata = ResponseMetadata { height: 42, chain_id: "testnet".into(), ..Default::default() };
        let proof = Proof { block_id_hash: vec![1; 32], ..Default::default() };
        let block = GroupSnapshotBlock::from_proof(&metadata, &proof).unwrap();
        assert!(block.require_same(&metadata, &proof).is_ok());
        let changed = Proof { block_id_hash: vec![2; 32], ..Default::default() };
        assert_eq!(block.require_same(&metadata, &changed).unwrap_err(), GROUP_SNAPSHOT_CHANGED);
        let next = ResponseMetadata { height: 43, ..metadata.clone() };
        assert_eq!(block.require_same(&next, &proof).unwrap_err(), GROUP_SNAPSHOT_CHANGED);
        assert!(GroupSnapshotBlock::from_proof(&metadata, &Proof::default()).is_err());
    }

    #[test]
    fn pagination_requires_the_page_after_a_full_page_and_never_truncates() {
        let id = Identifier::from([1; 32]);
        let query = ContractGroupMembersPageQuery::contracts(id).with_limit(GROUP_PAGE_SIZE);
        let full = ContractGroupMembersPage::Contracts((0..100).map(|n| Identifier::from([n; 32])).collect());
        let mut values = Vec::new();
        let next = append_group_page(&query, &full, &mut values).unwrap().unwrap();
        assert_eq!(values.len(), 100);
        assert_ne!(query, next);
        assert!(append_group_page(&next, &ContractGroupMembersPage::Contracts(vec![]), &mut values).unwrap().is_none());
        let mut oversized = vec![serde_json::Value::Null; MAX_REVIEW_GROUP_MEMBERS];
        assert!(append_group_page(&query, &full, &mut oversized).is_err());
        assert_eq!(oversized.len(), MAX_REVIEW_GROUP_MEMBERS);
        assert!(append_group_page(&next, &full, &mut values).is_err());
    }

    #[test]
    fn scope_preserves_all_member_kinds() {
        let id = Identifier::from([3; 32]);
        assert_eq!(group_page_values(&ContractGroupMembersPage::Contracts(vec![id]))[0], id.to_string(Encoding::Base58));
        assert_eq!(group_page_values(&ContractGroupMembersPage::DocumentTypes(vec![(id, "profile".into())]))[0]["documentType"], "profile");
        assert_eq!(group_page_values(&ContractGroupMembersPage::Tokens(vec![(id, 65535)]))[0]["tokenPosition"], 65535);
    }
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
