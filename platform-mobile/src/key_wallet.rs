use key_wallet::bip32::{ChildNumber, DerivationPath};
use key_wallet::Network;

fn application_session_authentication_derivation_path(
    is_testnet: bool,
    identity_id: [u8; 32],
    request_id: [u8; 32],
) -> (Network, DerivationPath) {
    let network = if is_testnet {
        Network::Testnet
    } else {
        Network::Mainnet
    };
    let coin_type = match network {
        Network::Mainnet => 5,
        _ => 1,
    };
    let path = DerivationPath::from(vec![
        ChildNumber::Hardened { index: 9 },
        ChildNumber::Hardened { index: coin_type },
        ChildNumber::Hardened { index: 5 },
        ChildNumber::Hardened { index: 6 },
        ChildNumber::Hardened { index: 0 },
        ChildNumber::Hardened256 { index: identity_id },
        ChildNumber::Hardened256 { index: request_id },
    ]);
    (network, path)
}

/// Returns the frozen DIP-13 application session authentication path.
///
/// The caller should pass `hash256(appEphPub)` as `request_id`; the
/// `hash160(appEphPub)` value is the on-chain lookup key and must not be used
/// as this derivation leaf.
#[ferment_macro::export]
pub fn application_session_authentication_path(
    is_testnet: bool,
    identity_id: [u8; 32],
    request_id: [u8; 32],
) -> String {
    let (_, path) =
        application_session_authentication_derivation_path(is_testnet, identity_id, request_id);
    path.to_string()
}

/// Derives the DIP-13 application session authentication private key.
///
/// The `request_id` parameter is `hash256(appEphPub)`. The on-chain lookup key
/// remains `hash160(appEphPub)`, so callers must keep the two digests separate.
#[ferment_macro::export]
pub fn application_session_authentication_private_key_from_seed(
    is_testnet: bool,
    seed: Vec<u8>,
    identity_id: [u8; 32],
    request_id: [u8; 32],
) -> Result<Vec<u8>, String> {
    let mut seed = seed;
    let (network, path) =
        application_session_authentication_derivation_path(is_testnet, identity_id, request_id);
    let result = path
        .derive_priv_ecdsa_for_master_seed(&seed, network)
        .map(|key| key.private_key.secret_bytes().to_vec())
        .map_err(|e| e.to_string());
    seed.fill(0);
    result
}

/// The DIP-13 rotation level is a hardened 31-bit index, so it must be below 2^31.
const APPLICATION_ROTATION_LIMIT: u32 = 1 << 31;

/// Builds `m/9'/coin'/5'/7'/0'/identityId'/contractId'/purpose'`, the part of the
/// application encryption path shared by the rotation-aware and legacy derivations.
fn application_encryption_base_path(
    is_testnet: bool,
    identity_id: [u8; 32],
    contract_id: [u8; 32],
    key_purpose: u32,
) -> Result<(Network, Vec<ChildNumber>), String> {
    if key_purpose != 1 && key_purpose != 2 {
        return Err("Application key purpose must be ENCRYPTION (1) or DECRYPTION (2)".into());
    }
    let network = if is_testnet {
        Network::Testnet
    } else {
        Network::Mainnet
    };
    let path = vec![
        ChildNumber::Hardened { index: 9 },
        ChildNumber::Hardened {
            index: if is_testnet { 1 } else { 5 },
        },
        ChildNumber::Hardened { index: 5 },
        ChildNumber::Hardened { index: 7 },
        ChildNumber::Hardened { index: 0 },
        ChildNumber::Hardened256 { index: identity_id },
        ChildNumber::Hardened256 { index: contract_id },
        ChildNumber::Hardened { index: key_purpose },
    ];
    Ok((network, path))
}

/// `m/9'/coin'/5'/7'/0'/identityId'/contractId'/purpose'/rotation'`
fn application_encryption_derivation_path(
    is_testnet: bool,
    identity_id: [u8; 32],
    contract_id: [u8; 32],
    key_purpose: u32,
    rotation: u32,
) -> Result<(Network, DerivationPath), String> {
    let (network, mut path) =
        application_encryption_base_path(is_testnet, identity_id, contract_id, key_purpose)?;
    if rotation >= APPLICATION_ROTATION_LIMIT {
        return Err("Application key rotation must be below 2^31".into());
    }
    path.push(ChildNumber::Hardened { index: rotation });
    Ok((network, DerivationPath::from(path)))
}

/// `m/9'/coin'/5'/7'/0'/identityId'/contractId'/purpose'`, without the rotation level.
fn application_encryption_legacy_derivation_path(
    is_testnet: bool,
    identity_id: [u8; 32],
    contract_id: [u8; 32],
    key_purpose: u32,
) -> Result<(Network, DerivationPath), String> {
    let (network, path) =
        application_encryption_base_path(is_testnet, identity_id, contract_id, key_purpose)?;
    Ok((network, DerivationPath::from(path)))
}

fn derive_private_key(
    seed: &mut [u8],
    derivation: Result<(Network, DerivationPath), String>,
) -> Result<Vec<u8>, String> {
    let result = derivation.and_then(|(network, path)| {
        path.derive_priv_ecdsa_for_master_seed(seed, network)
            .map(|key| key.private_key.secret_bytes().to_vec())
            .map_err(|e| e.to_string())
    });
    seed.fill(0);
    result
}

/// Derives the DIP-13 application encryption key at
/// `m/9'/coin'/5'/7'/0'/identityId'/contractId'/purpose'/rotation'`.
///
/// Pass `true` for every non-mainnet network. `key_purpose` is the Platform
/// ENCRYPTION (1) or DECRYPTION (2) purpose. `rotation` is a hardened 31-bit
/// index (below 2^31); new keys start at rotation 0.
#[ferment_macro::export]
pub fn application_encryption_private_key_from_seed(
    is_testnet: bool,
    seed: Vec<u8>,
    identity_id: [u8; 32],
    contract_id: [u8; 32],
    key_purpose: u32,
    rotation: u32,
) -> Result<Vec<u8>, String> {
    let mut seed = seed;
    derive_private_key(
        &mut seed,
        application_encryption_derivation_path(
            is_testnet,
            identity_id,
            contract_id,
            key_purpose,
            rotation,
        ),
    )
}

/// Derives the pre-rotation application encryption key at
/// `m/9'/coin'/5'/7'/0'/identityId'/contractId'/purpose'` (no rotation level).
///
/// Only for recognising keys that wallets registered on testnet/devnet before
/// DIP-13 gained the rotation level. Never use it to derive new keys: new keys
/// must go through `application_encryption_private_key_from_seed`.
#[ferment_macro::export]
pub fn application_encryption_legacy_private_key_from_seed(
    is_testnet: bool,
    seed: Vec<u8>,
    identity_id: [u8; 32],
    contract_id: [u8; 32],
    key_purpose: u32,
) -> Result<Vec<u8>, String> {
    let mut seed = seed;
    derive_private_key(
        &mut seed,
        application_encryption_legacy_derivation_path(
            is_testnet,
            identity_id,
            contract_id,
            key_purpose,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use dashcore::secp256k1::{PublicKey, Secp256k1, SecretKey};
    use key_wallet::Mnemonic;

    const MNEMONIC: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn abandon_seed() -> Vec<u8> {
        Mnemonic::from_phrase(MNEMONIC)
            .unwrap()
            .to_seed("")
            .to_vec()
    }

    fn public_key_hex(private_key: &[u8]) -> String {
        let secret = SecretKey::from_slice(private_key).unwrap();
        hex::encode(PublicKey::from_secret_key(&Secp256k1::new(), &secret).serialize())
    }

    fn ids_0x35_0x6b() -> ([u8; 32], [u8; 32]) {
        ([0x35; 32], [0x6b; 32])
    }

    fn ids_asc_desc() -> ([u8; 32], [u8; 32]) {
        (
            core::array::from_fn(|i| i as u8),
            core::array::from_fn(|i| 0xff - i as u8),
        )
    }

    fn rotated(ids: ([u8; 32], [u8; 32]), purpose: u32, rotation: u32) -> String {
        public_key_hex(
            &application_encryption_private_key_from_seed(
                false,
                abandon_seed(),
                ids.0,
                ids.1,
                purpose,
                rotation,
            )
            .unwrap(),
        )
    }

    fn legacy(ids: ([u8; 32], [u8; 32]), purpose: u32) -> String {
        public_key_hex(
            &application_encryption_legacy_private_key_from_seed(
                false,
                abandon_seed(),
                ids.0,
                ids.1,
                purpose,
            )
            .unwrap(),
        )
    }

    #[test]
    fn rotation_level_vectors_on_mainnet() {
        assert_eq!(
            rotated(ids_0x35_0x6b(), 1, 0),
            "031a882c119e2b1f8e0349b444d877afa2599efeef49a26930fd25518cb6e9d57a"
        );
        assert_eq!(
            rotated(ids_0x35_0x6b(), 1, 1),
            "03d0f6f0f48d968b9554d8e7fe467d7135bacf29d5c4994ca593bd10ebeed696f6"
        );
        assert_eq!(
            rotated(ids_asc_desc(), 1, 0),
            "02dfed57ddaaea8c0758e8da44513520038eb8d6b0b92109953c14d6ab1087259b"
        );
        assert_eq!(
            rotated(ids_asc_desc(), 2, 0),
            "020bcde800f41ad1f924477a9502f111445224cc9ab70114ae4e5099ba02bdd034"
        );
        assert_eq!(
            rotated(ids_asc_desc(), 2, 1),
            "021b97869c21933673076a938337c28848c8bf8094bbe6883e3a4acbf33c639072"
        );
    }

    #[test]
    fn legacy_no_rotation_vectors_on_mainnet() {
        assert_eq!(
            legacy(ids_0x35_0x6b(), 1),
            "03e989de1b62f137231cc06659c810c17100becd1f5e23d5710faa7602769fe434"
        );
        assert_eq!(
            legacy(ids_asc_desc(), 1),
            "03c4e462229f176595ec17e28e9096d3cfbc42496fb8ed4d48f8f58355d6105aa9"
        );
        assert_eq!(
            legacy(ids_asc_desc(), 2),
            "031fd0a3cfc01bebb89d43c2e12a6321c37ebdacb62c09495899e5b65a24fa5c99"
        );
    }

    #[test]
    fn rotation_appends_a_hardened_level_to_the_legacy_path() {
        let (_, legacy_path) =
            application_encryption_legacy_derivation_path(false, [1; 32], [2; 32], 1).unwrap();
        let (_, rotated_path) =
            application_encryption_derivation_path(false, [1; 32], [2; 32], 1, 7).unwrap();
        assert_eq!(rotated_path.len(), legacy_path.len() + 1);
        assert_eq!(&rotated_path[..legacy_path.len()], &legacy_path[..]);
        assert_eq!(
            rotated_path[legacy_path.len()],
            ChildNumber::Hardened { index: 7 }
        );
    }

    #[test]
    fn rotation_must_fit_in_31_bits() {
        let derive = |rotation| {
            application_encryption_private_key_from_seed(
                false,
                vec![42; 32],
                [1; 32],
                [2; 32],
                1,
                rotation,
            )
        };
        assert!(derive((1 << 31) - 1).is_ok());
        assert!(derive(1 << 31).is_err());
        assert!(derive(u32::MAX).is_err());
    }

    #[test]
    fn application_keys_separate_networks_purposes_contracts_and_rotations() {
        let derive = |testnet, contract, purpose, rotation| {
            application_encryption_private_key_from_seed(
                testnet,
                vec![42; 32],
                [1; 32],
                contract,
                purpose,
                rotation,
            )
            .unwrap()
        };
        let encryption = derive(true, [2; 32], 1, 0);
        assert_eq!(encryption, derive(true, [2; 32], 1, 0));
        assert_ne!(encryption, derive(false, [2; 32], 1, 0));
        assert_ne!(encryption, derive(true, [2; 32], 2, 0));
        assert_ne!(encryption, derive(true, [3; 32], 1, 0));
        assert_ne!(encryption, derive(true, [2; 32], 1, 1));
        assert_ne!(
            encryption,
            application_encryption_legacy_private_key_from_seed(
                true,
                vec![42; 32],
                [1; 32],
                [2; 32],
                1
            )
            .unwrap()
        );
        assert_ne!(
            encryption,
            application_session_authentication_private_key_from_seed(
                true,
                vec![42; 32],
                [1; 32],
                [2; 32]
            )
            .unwrap()
        );
        for purpose in [0, 3] {
            assert!(application_encryption_private_key_from_seed(
                true,
                vec![42; 32],
                [1; 32],
                [2; 32],
                purpose,
                0
            )
            .is_err());
            assert!(application_encryption_legacy_private_key_from_seed(
                true,
                vec![42; 32],
                [1; 32],
                [2; 32],
                purpose
            )
            .is_err());
        }
    }
}
