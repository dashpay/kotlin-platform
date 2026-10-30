use crate::config::EntryPoint;
use crate::put::CallbackSigner;
use crate::sdk::DashSdk;
use dash_sdk::platform::transition::broadcast::BroadcastStateTransition;
use dash_sdk::platform::Fetch;
use dashcore::hashes::{sha256, Hash};
use dpp::data_contract::accessors::v1::DataContractV1Getters;
use dpp::data_contract::associated_token::token_configuration::accessors::v0::TokenConfigurationV0Getters;
use dpp::data_contract::associated_token::token_configuration_convention::accessors::v0::TokenConfigurationConventionV0Getters;
use dpp::data_contract::DataContract;
use dpp::identity::identity_public_key::accessors::v0::IdentityPublicKeyGettersV0;
use dpp::identity::identity_public_key::IdentityPublicKey;
use dpp::identity::signer::Signer;
use dpp::identity::{Purpose, SecurityLevel};
use dpp::serialization::PlatformDeserializableUntrusted;
use dpp::state_transition::batch_transition::batched_transition::{
    token_transition::TokenTransition, BatchedTransition,
};
use dpp::state_transition::batch_transition::methods::v1::DocumentsBatchTransitionMethodsV1;
use dpp::state_transition::batch_transition::token_base_transition::TokenBaseTransition;
use dpp::state_transition::batch_transition::token_direct_purchase_transition::TokenDirectPurchaseTransition;
use dpp::state_transition::batch_transition::BatchTransition;
use dpp::state_transition::proof_result::StateTransitionProofResult;
use dpp::state_transition::StateTransition;
use dpp::version::PlatformVersion;
use platform_value::{string_encoding::Encoding, Identifier};
use serde_json::json;

#[derive(Debug, PartialEq)]
struct Purchase {
    owner: Identifier,
    contract: Identifier,
    token: Identifier,
    position: u16,
    amount: u64,
    price: u64,
    nonce: u64,
}

impl Purchase {
    fn request_id(&self) -> Result<String, String> {
        let canonical = serde_json::to_vec(&(
            "dash-st-purchase-v2",
            self.owner.to_string(Encoding::Base58),
            self.contract.to_string(Encoding::Base58),
            self.token.to_string(Encoding::Base58),
            self.position,
            self.amount.to_string(),
            self.price.to_string(),
            self.nonce.to_string(),
        ))
        .map_err(|e| e.to_string())?;
        Ok(sha256::Hash::hash(&canonical).to_string())
    }
}

async fn sign_purchase<S: Signer<IdentityPublicKey>>(
    purchase: &Purchase,
    key: &IdentityPublicKey,
    signer: &S,
    version: &PlatformVersion,
) -> Result<StateTransition, String> {
    BatchTransition::new_token_direct_purchase_transition(
        purchase.token,
        purchase.owner,
        purchase.contract,
        purchase.position,
        purchase.amount,
        purchase.price,
        key,
        purchase.nonce,
        0,
        signer,
        version,
        None,
    )
    .await
    .map_err(|e| e.to_string())
}

fn decode(bytes: &[u8]) -> Result<Purchase, String> {
    if bytes.is_empty() || bytes.len() > 4096 {
        return Err("Invalid purchase payload size".into());
    }
    let transition = StateTransition::deserialize_from_bytes_untrusted_exact(bytes)
        .map_err(|e| e.to_string())?;
    let StateTransition::Batch(BatchTransition::V1(batch)) = transition else {
        return Err("Only a single token direct purchase is supported; login key registration, tips and other operations are not supported".into());
    };
    if batch.transitions.len() != 1 || batch.user_fee_increase != 0 || !batch.signature.is_empty() {
        return Err("Purchase must be one unsigned operation with no fee increase".into());
    }
    let BatchedTransition::Token(TokenTransition::DirectPurchase(
        TokenDirectPurchaseTransition::V0(purchase),
    )) = &batch.transitions[0]
    else {
        return Err("Only a single token direct purchase is supported".into());
    };
    let TokenBaseTransition::V0(base) = &purchase.base;
    if base.using_group_info.is_some()
        || purchase.token_count == 0
        || purchase.total_agreed_price == 0
    {
        return Err("Grouped, empty or zero-price purchases are not supported".into());
    }
    if base.token_id
        != Identifier::from(dpp::tokens::calculate_token_id(
            base.data_contract_id.as_bytes(),
            base.token_contract_position,
        ))
    {
        return Err("Token ID does not match its contract and position".into());
    }
    Ok(Purchase {
        owner: batch.owner_id,
        contract: base.data_contract_id,
        token: base.token_id,
        position: base.token_contract_position,
        amount: purchase.token_count,
        price: purchase.total_agreed_price,
        nonce: base.identity_contract_nonce,
    })
}

async fn contract_and_summary(
    sdk: &dash_sdk::Sdk,
    purchase: &Purchase,
) -> Result<(DataContract, Vec<u8>), String> {
    let contract = DataContract::fetch(sdk, purchase.contract)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Token contract not found")?;
    let token = contract
        .tokens()
        .get(&purchase.position)
        .ok_or("Token not found at contract position")?;
    let conventions = token.conventions();
    let summary = json!({
        "requestId": purchase.request_id()?,
        "nonce": purchase.nonce.to_string(),
        "ownerId": purchase.owner.to_string(Encoding::Base58),
        "contractId": purchase.contract.to_string(Encoding::Base58),
        "tokenId": purchase.token.to_string(Encoding::Base58),
        "position": purchase.position,
        "amount": purchase.amount.to_string(), "maximumCredits": purchase.price.to_string(),
        "decimals": conventions.decimals(),
        "name": conventions.singular_form_by_language_code_or_default("en")
    });
    Ok((
        contract,
        serde_json::to_vec(&summary).map_err(|e| e.to_string())?,
    ))
}

/// Exact, bounded decoding plus proof-verified token denomination. Never signs incoming bytes.
#[ferment_macro::export]
pub fn prepare_token_purchase_sdk(
    rust_sdk: *mut DashSdk,
    bytes: Vec<u8>,
    owner: Identifier,
) -> Result<Vec<u8>, String> {
    let purchase = decode(&bytes)?;
    if purchase.owner != owner {
        return Err("Purchase targets a different identity".into());
    }
    let handle = unsafe { &*rust_sdk };
    handle.get_runtime().block_on(async {
        contract_and_summary(&handle.get_sdk(), &purchase)
            .await
            .map(|(_, summary)| summary)
    })
}

/// Rebuilds the approved operation with its original nonce and the wallet's own key.
/// Success is a proof-verified affected-state result, not necessarily proof of this operation's execution.
#[ferment_macro::export]
pub fn submit_token_purchase_sdk(
    rust_sdk: *mut DashSdk,
    bytes: Vec<u8>,
    approved_summary: Vec<u8>,
    owner: Identifier,
    signing_key: IdentityPublicKey,
    signer_context: usize,
    signer_callback: u64,
) -> Result<Vec<u8>, String> {
    let purchase = decode(&bytes)?;
    if purchase.owner != owner
        || signer_callback == 0
        || signing_key.purpose() != Purpose::AUTHENTICATION
        || signing_key.security_level() != SecurityLevel::CRITICAL
        || signing_key.contract_bounds().is_some()
    {
        return Err(
            "Purchase requires the approving identity's unbound CRITICAL authentication key".into(),
        );
    }
    let handle = unsafe { &*rust_sdk };
    handle.get_runtime().block_on(async {
        let sdk = handle.get_sdk();
        let (_, summary) = contract_and_summary(&sdk, &purchase).await?;
        if summary != approved_summary {
            return Err("Token metadata or purchase terms changed. Scan again.".into());
        }
        let signer =
            CallbackSigner::new(signer_context, signer_callback).map_err(|e| e.to_string())?;
        // Never replace an incoming operation's nonce: doing so would turn a replay into a new debit.
        let transition = sign_purchase(&purchase, &signing_key, &signer, sdk.version()).await?;
        let result = transition
            .broadcast_and_wait_for_affected_state::<StateTransitionProofResult>(&sdk, None)
            .await;
        // This operation did not allocate its nonce through the SDK cache.
        sdk.refresh_identity_nonce(&owner).await;
        let proof = result.map_err(|e| e.to_string())?;
        match proof {
            StateTransitionProofResult::VerifiedTokenBalance(..)
            | StateTransitionProofResult::VerifiedTokenActionWithDocument(..)
            | StateTransitionProofResult::VerifiedTokenGroupActionWithDocument(..) => {}
            _ => return Err("Unexpected token purchase proof result".into()),
        }
        Ok(b"submitted".to_vec())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use dpp::serialization::PlatformSerializable;
    use dpp::state_transition::batch_transition::token_base_transition::v0::TokenBaseTransitionV0;
    use dpp::state_transition::batch_transition::token_direct_purchase_transition::TokenDirectPurchaseTransitionV0;
    use dpp::state_transition::batch_transition::BatchTransitionV1;

    fn batch() -> BatchTransitionV1 {
        let contract = Identifier::from([2; 32]);
        BatchTransitionV1 {
            owner_id: Identifier::from([1; 32]),
            transitions: vec![BatchedTransition::Token(TokenTransition::DirectPurchase(
                TokenDirectPurchaseTransition::V0(TokenDirectPurchaseTransitionV0 {
                    base: TokenBaseTransition::V0(TokenBaseTransitionV0 {
                        data_contract_id: contract,
                        token_contract_position: 0,
                        token_id: Identifier::from(dpp::tokens::calculate_token_id(
                            contract.as_bytes(),
                            0,
                        )),
                        identity_contract_nonce: 42,
                        using_group_info: None,
                    }),
                    token_count: u64::MAX,
                    total_agreed_price: 50_000_000_000,
                }),
            ))],
            ..Default::default()
        }
    }
    fn encode(batch: BatchTransitionV1) -> Vec<u8> {
        StateTransition::Batch(BatchTransition::V1(batch))
            .serialize_to_bytes()
            .unwrap()
    }
    #[test]
    fn decodes_single_purchase_without_losing_u64_precision() {
        let parsed = decode(&encode(batch())).unwrap();
        assert_eq!(parsed.amount, u64::MAX);
        assert_eq!(parsed.price, 50_000_000_000);
        assert_eq!(parsed.nonce, 42);
    }
    #[test]
    fn canonical_id_ignores_envelope_key_id_but_binds_the_signed_nonce_and_terms() {
        let original = decode(&encode(batch())).unwrap();
        let mut changed_key = batch();
        changed_key.signature_public_key_id = 1234;
        assert_eq!(
            original.request_id().unwrap(),
            decode(&encode(changed_key)).unwrap().request_id().unwrap()
        );
        let mut changed_nonce = batch();
        let BatchedTransition::Token(TokenTransition::DirectPurchase(
            TokenDirectPurchaseTransition::V0(p),
        )) = &mut changed_nonce.transitions[0]
        else {
            unreachable!()
        };
        let TokenBaseTransition::V0(base) = &mut p.base;
        base.identity_contract_nonce += 1;
        assert_ne!(
            original.request_id().unwrap(),
            decode(&encode(changed_nonce))
                .unwrap()
                .request_id()
                .unwrap()
        );
        let mut changed_terms = batch();
        let BatchedTransition::Token(TokenTransition::DirectPurchase(
            TokenDirectPurchaseTransition::V0(p),
        )) = &mut changed_terms.transitions[0]
        else {
            unreachable!()
        };
        p.total_agreed_price += 1;
        assert_ne!(
            original.request_id().unwrap(),
            decode(&encode(changed_terms))
                .unwrap()
                .request_id()
                .unwrap()
        );
    }
    #[tokio::test]
    async fn wallet_signing_preserves_the_operation_nonce_and_all_purchase_terms() {
        use dashcore::secp256k1::{PublicKey, Secp256k1, SecretKey};
        use dpp::identity::identity_public_key::v0::IdentityPublicKeyV0;
        use dpp::identity::KeyType;
        use simple_signer::signer::SimpleSigner;
        let secret = [1; 32];
        let public =
            PublicKey::from_secret_key(&Secp256k1::new(), &SecretKey::from_slice(&secret).unwrap());
        let key = IdentityPublicKey::V0(IdentityPublicKeyV0 {
            id: 7,
            purpose: Purpose::AUTHENTICATION,
            security_level: SecurityLevel::CRITICAL,
            contract_bounds: None,
            key_type: KeyType::ECDSA_SECP256K1,
            read_only: false,
            data: public.serialize().to_vec().into(),
            disabled_at: None,
        });
        let mut signer = SimpleSigner::default();
        signer.add_identity_public_key(key.clone(), secret);
        let original = decode(&encode(batch())).unwrap();
        let transition = sign_purchase(&original, &key, &signer, PlatformVersion::latest())
            .await
            .unwrap();
        let StateTransition::Batch(BatchTransition::V1(mut signed)) = transition else {
            panic!("Expected batch V1")
        };
        assert_eq!(signed.signature_public_key_id, 7);
        assert!(!signed.signature.is_empty());
        signed.signature = Default::default();
        let rebuilt = decode(&encode(signed)).unwrap();
        assert_eq!(rebuilt, original);
        assert_eq!(
            rebuilt.request_id().unwrap(),
            original.request_id().unwrap()
        );
    }
    #[test]
    fn rejects_multiple_operations_fee_increase_and_trailing_bytes() {
        let mut multiple = batch();
        multiple.transitions.push(multiple.transitions[0].clone());
        assert!(decode(&encode(multiple)).is_err());
        let mut fee = batch();
        fee.user_fee_increase = 1;
        assert!(decode(&encode(fee)).is_err());
        let mut trailing = encode(batch());
        trailing.push(0);
        assert!(decode(&trailing).is_err());
        assert!(decode(&vec![0; 4097]).is_err());
    }
    #[test]
    fn rejects_wrong_token_id_and_zero_quantity() {
        let mut wrong = batch();
        let BatchedTransition::Token(TokenTransition::DirectPurchase(
            TokenDirectPurchaseTransition::V0(p),
        )) = &mut wrong.transitions[0]
        else {
            unreachable!()
        };
        let TokenBaseTransition::V0(base) = &mut p.base;
        base.token_id = Identifier::from([9; 32]);
        assert!(decode(&encode(wrong)).is_err());
        let mut zero = batch();
        let BatchedTransition::Token(TokenTransition::DirectPurchase(
            TokenDirectPurchaseTransition::V0(p),
        )) = &mut zero.transitions[0]
        else {
            unreachable!()
        };
        p.token_count = 0;
        assert!(decode(&encode(zero)).is_err());
    }
    #[test]
    fn rejects_signed_and_grouped_requests() {
        let mut signed = batch();
        signed.signature = vec![1; 65].into();
        assert!(decode(&encode(signed)).is_err());
        let mut grouped = batch();
        let BatchedTransition::Token(TokenTransition::DirectPurchase(
            TokenDirectPurchaseTransition::V0(p),
        )) = &mut grouped.transitions[0]
        else {
            unreachable!()
        };
        let TokenBaseTransition::V0(base) = &mut p.base;
        base.using_group_info = Some(dpp::group::GroupStateTransitionInfo {
            group_contract_position: 1,
            action_id: Identifier::from([3; 32]),
            action_is_proposer: true,
        });
        assert!(decode(&encode(grouped)).is_err());
    }
}
