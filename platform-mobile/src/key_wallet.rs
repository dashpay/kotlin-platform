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

fn application_encryption_derivation_path(
    is_testnet: bool,
    identity_id: [u8; 32],
    contract_id: [u8; 32],
    key_purpose: u32,
) -> Result<(Network, DerivationPath), String> {
    if key_purpose != 1 && key_purpose != 2 {
        return Err("Application key purpose must be ENCRYPTION (1) or DECRYPTION (2)".into());
    }
    let network = if is_testnet {
        Network::Testnet
    } else {
        Network::Mainnet
    };
    let path = DerivationPath::from(vec![
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
    ]);
    Ok((network, path))
}

/// DIP-13 rotation-zero application key. Pass `true` for every non-mainnet network.
/// `key_purpose` is the Platform ENCRYPTION (1) or DECRYPTION (2) purpose.
#[ferment_macro::export]
pub fn application_encryption_private_key_from_seed(
    is_testnet: bool,
    seed: Vec<u8>,
    identity_id: [u8; 32],
    contract_id: [u8; 32],
    key_purpose: u32,
) -> Result<Vec<u8>, String> {
    let mut seed = seed;
    let result =
        application_encryption_derivation_path(is_testnet, identity_id, contract_id, key_purpose)
            .and_then(|(network, path)| {
                path.derive_priv_ecdsa_for_master_seed(&seed, network)
                    .map(|key| key.private_key.secret_bytes().to_vec())
                    .map_err(|e| e.to_string())
            });
    seed.fill(0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_keys_separate_networks_purposes_and_contracts() {
        let derive = |testnet, contract, purpose| {
            application_encryption_private_key_from_seed(
                testnet,
                vec![42; 32],
                [1; 32],
                contract,
                purpose,
            )
            .unwrap()
        };
        let encryption = derive(true, [2; 32], 1);
        assert_eq!(encryption, derive(true, [2; 32], 1));
        assert_ne!(encryption, derive(false, [2; 32], 1));
        assert_ne!(encryption, derive(true, [2; 32], 2));
        assert_ne!(encryption, derive(true, [3; 32], 1));
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
        assert!(application_encryption_private_key_from_seed(
            true,
            vec![42; 32],
            [1; 32],
            [2; 32],
            0
        )
        .is_err());
    }
}
