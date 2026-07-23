use dpp::serialization::PlatformDeserializable;
use dpp::state_transition::StateTransition;

/// A small, FFI-friendly summary of a deserialized state transition.
///
/// The full [`StateTransition`] enum is not exposed across the FFI boundary, so callers that
/// only need to inspect a serialized (bincode) transition — e.g. to learn its type and owner
/// before deciding how to handle it — receive this instead. Extend with more fields as callers
/// need them (added public keys, revision, nonce, ...).
#[ferment_macro::export]
pub struct StateTransitionInfo {
    /// The [`StateTransitionType`](dpp::state_transition::state_transition_types::StateTransitionType)
    /// discriminant (e.g. 5 = IdentityUpdate).
    pub transition_type: u8,
    /// Human-readable transition name.
    pub name: String,
    /// The 32-byte owner/identity id, or empty when the transition has no owner.
    pub owner_id: Vec<u8>,
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
    Ok(StateTransitionInfo {
        transition_type: state_transition.state_transition_type() as u8,
        name: state_transition.name(),
        owner_id: state_transition
            .owner_id()
            .map(|id| id.to_vec())
            .unwrap_or_default(),
    })
}
