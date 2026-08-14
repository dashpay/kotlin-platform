use dpp::identity::identity_public_key::IdentityPublicKey;
use dpp::serialization::PlatformDeserializable;
use dpp::state_transition::StateTransition;
use dpp::state_transition::identity_update_transition::accessors::IdentityUpdateTransitionAccessorsV0;

/// A summary of a deserialized state transition.
///
/// The full [`StateTransition`] enum is not exposed across the FFI boundary; callers that need to
/// inspect a serialized (bincode) transition receive this instead. The identity-update fields
/// ([`revision`](Self::revision), [`identity_nonce`](Self::identity_nonce)) are only meaningful
/// when [`transition_type`](Self::transition_type) is IdentityUpdate (5); the keys an
/// identity-update adds are fetched via [`identity_update_public_keys_to_add`] (a `Vec` maps to a
/// usable list only as a function return, not as a struct field).
#[derive(Clone)]
#[ferment_macro::export]
pub struct StateTransitionInfo {
    /// The [`StateTransitionType`](dpp::state_transition::state_transition_types::StateTransitionType)
    /// discriminant (e.g. 5 = IdentityUpdate).
    pub transition_type: u8,
    /// Human-readable transition name.
    pub name: String,
    /// The 32-byte owner/identity id, or empty when the transition has no owner.
    pub owner_id: Vec<u8>,
    /// Identity-update: the new identity revision (0 for other transition types).
    pub revision: u64,
    /// Identity-update: the identity nonce (0 for other transition types).
    pub identity_nonce: u64,
}

/// Deserializes a bincode-serialized platform state transition and returns a summary of it.
///
/// The legacy Kotlin `StateTransitionFactory` only understands the old CBOR envelope and a
/// handful of transition types; modern transitions (produced by the Rust SDK / wasm-sdk) are
/// bincode-serialized, so decoding them requires the native deserializer.
#[ferment_macro::export]
pub fn deserialize_state_transition(bytes: Vec<u8>) -> Result<StateTransitionInfo, String> {
    let state_transition =
        StateTransition::deserialize_from_bytes(&bytes).map_err(|e| e.to_string())?;

    let (revision, identity_nonce) = match &state_transition {
        StateTransition::IdentityUpdate(transition) => (transition.revision(), transition.nonce()),
        _ => (0u64, 0u64),
    };

    Ok(StateTransitionInfo {
        transition_type: state_transition.state_transition_type() as u8,
        name: state_transition.name(),
        owner_id: state_transition
            .owner_id()
            .map(|id| id.to_vec())
            .unwrap_or_default(),
        revision,
        identity_nonce,
    })
}

/// Returns the public keys added by an identity-update transition (converted to the standard
/// [`IdentityPublicKey`] type), or an empty list for any other transition type.
#[ferment_macro::export]
pub fn identity_update_public_keys_to_add(
    bytes: Vec<u8>,
) -> Result<Vec<IdentityPublicKey>, String> {
    let state_transition =
        StateTransition::deserialize_from_bytes(&bytes).map_err(|e| e.to_string())?;
    let keys = match state_transition {
        StateTransition::IdentityUpdate(transition) => transition
            .public_keys_to_add()
            .iter()
            .map(|key| IdentityPublicKey::from(key.clone()))
            .collect(),
        _ => Vec::new(),
    };
    Ok(keys)
}
