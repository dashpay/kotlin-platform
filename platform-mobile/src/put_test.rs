use std::collections::BTreeMap;
use std::fs;
use std::future::Future;
use std::io::{Read, Write};
use std::sync::Arc;
use dapi_grpc::platform::v0::StateTransitionBroadcastError;
use dash_sdk::platform::Fetch;
use dash_sdk::platform::transition::broadcast_request::BroadcastRequestForStateTransition;
use dash_sdk::platform::transition::put_contract::PutContract;
use dash_sdk::platform::transition::put_document::PutDocument;
use dash_sdk::platform::transition::put_settings::PutSettings;
use dash_sdk::{RequestSettings, Sdk};
use dash_sdk::dapi_client::DapiClientError;
use dashcore::hashes::{Hash, sha256d};
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use dpp::data_contract::{DataContract, DataContractFactory};
//use dpp::data_contract::v0::data_contract::DataContractV0;
use dpp::data_contract::created_data_contract::CreatedDataContract;
use dpp::data_contract::document_type::methods::DocumentTypeV0Methods;
use dpp::document::{Document, DocumentV0Getters};
use dpp::document::v0::DocumentV0;
use dpp::identity::identity_public_key::v0::IdentityPublicKeyV0;
use dpp::identity::identity_public_key::{IdentityPublicKey, KeyType, Purpose, SecurityLevel};
use dpp::ProtocolError;
use dpp::state_transition::StateTransition;
use dpp::util::entropy_generator::{DefaultEntropyGenerator, EntropyGenerator};
use platform_value::{BinaryData, Identifier, Value};
use platform_value::string_encoding::Encoding;
use platform_version::version::PlatformVersion;
use rand::random;
use simple_signer::signer::SimpleSigner;
use tokio::runtime::Builder;
use tracing::trace;
use crate::config::{Config, EntryPoint};
use crate::logs::setup_logs;
use crate::put::{get_wait_result_error, wait_for_response_concurrent};
use dash_sdk::Error;
use crate::sdk::{create_dash_sdk_using_core_testnet};

fn get_salted_domain_hash(
    pre_order_salt_raw: &[u8],
    full_name: &str
) -> [u8; 32] {
    let mut baos = Vec::with_capacity(pre_order_salt_raw.len() + full_name.len());
    baos.write_all(pre_order_salt_raw).expect("Writing to buffer failed");
    baos.write_all(full_name.as_bytes()).expect("Writing to buffer failed");

    sha256d::Hash::hash(&baos.as_slice()).into()
}


fn vec_to_array(vec: Vec<u8>) -> Result<[u8; 32], &'static str> {
    if vec.len() == 32 {
        let mut array = [0u8; 32];
        array.copy_from_slice(&vec);
        Ok(array)
    } else {
        Err("Vector must have exactly 32 elements")
    }
}

// #[test]
fn test_put_documents_for_username() {
    let entropy_generator = DefaultEntropyGenerator;
    let owner_id = Identifier::from_string("7Yowk46VwwHqmD5yZyyygggh937aP6h2UW7aQWBdWpM5", Encoding::Base58).expect("identifier");
    let identity_public_key = IdentityPublicKey::V0(
        IdentityPublicKeyV0 {
            id: 1,
            purpose: Purpose::AUTHENTICATION,
            security_level: SecurityLevel::HIGH,
            contract_bounds: None,
            key_type: KeyType::ECDSA_SECP256K1,
            read_only: false,
            data: BinaryData::from_string("AmM7AZIyWC1uOj0/3lFManLaTAK3RVToY+1yMvbpkvDa", Encoding::Base64).unwrap(),
            disabled_at: None,
        }
    );
    let preorder_salt = entropy_generator.generate().unwrap();
    let label = "my-unit-test-3".to_string();
    let normalized_label = "my-un1t-test-3".to_string();
    let full_name = "my-un1t-test-3.dash";
    let mut preorder_props: BTreeMap<String, Value> = BTreeMap::new();
    preorder_props.insert(
        "saltedDomainHash".to_string(),
        Value::Bytes32(get_salted_domain_hash(preorder_salt.as_slice(), full_name))
    );
    let preorder_document = Document::V0(
        DocumentV0 {
            id: Default::default(),
            owner_id: owner_id,
            properties: preorder_props,
            revision: Some(1),
            created_at: None,
            updated_at: None,
            transferred_at: None,
            created_at_block_height: None,
            updated_at_block_height: None,
            transferred_at_block_height: None,
            created_at_core_block_height: None,
            updated_at_core_block_height: None,
            transferred_at_core_block_height: None,
        }
    );
    // data = {HashMap@3953}  size = 7
    // "records" -> {HashMap@3968}  size = 1
    // key = "records"
    // value = {HashMap@3968}  size = 1
    // "dashUniqueIdentityId" -> {Identifier@3728} 9Qv1fnN59iNWYBmQNk3a63N1ciW7oEKr6dXNQH8ryUaj
    // "label" -> "bob1"
    // "preorderSalt" -> {byte[32]@3971} [106, 10, 84, 11, 1, -56, 70, 106, 69, -53, -1, 91, -103, -40, 64, -49, 20, -82, 80, 4, 43, 9, -79, 118, -71, 118, 89, 78, 52, 18, 44, -75]
    // "normalizedParentDomainName" -> "dash"
    // "parentDomainName" -> "dash"
    // "normalizedLabel" -> "b0b1"
    // "subdomain_rules" -> {HashMap@3978}  size = 1
    // key = "subdomain_rules"
    // value = {HashMap@3978}  size = 1
    // "allowSubdomains" -> {Boolean@3983} false
    let mut domain_props: BTreeMap<String, Value> = BTreeMap::new();
    let records = vec![(Value::Text("identity".to_string()), Value::Identifier(owner_id.into()))];

    // this is no longer a feature
    // let records = vec![(Value::Text("dashAliasIdentityId".to_string()), Value::Identifier(owner_id.into()))];

    let subdomain_rules = vec![(Value::Text("allowSubdomains".to_string()), Value::Bool(false))];
    domain_props.insert("records".to_string(), Value::Map(records));
    domain_props.insert("label".to_string(), Value::Text(label.clone()));
    domain_props.insert("preorderSalt".to_string(), Value::Bytes32(preorder_salt));
    domain_props.insert("normalizedParentDomainName".to_string(), Value::Text("dash".to_string()));
    domain_props.insert("parentDomainName".to_string(), Value::Text("dash".to_string()));
    domain_props.insert("normalizedLabel".to_string(), Value::Text(normalized_label.to_string()));
    domain_props.insert("subdomainRules".to_string(), Value::Map(subdomain_rules));

    let domain_document = Document::V0(
        DocumentV0 {
            id: Default::default(),
            owner_id: owner_id,
            properties: domain_props,
            revision: Some(1),
            created_at: None,
            updated_at: None,
            transferred_at: None,
            created_at_block_height: None,
            updated_at_block_height: None,
            transferred_at_block_height: None,
            created_at_core_block_height: None,
            updated_at_core_block_height: None,
            transferred_at_core_block_height: None,
        }
    );

    let mut rust_sdk = create_dash_sdk_using_core_testnet();
    let rt = unsafe { rust_sdk.get_runtime() };


    // Execute the async block using the Tokio runtime
    match rt.block_on(async {
        // Your async code here
        tracing::warn!("Setting up SDK");
        let sdk = rust_sdk.get_sdk();
        tracing::warn!("Finished SDK, {:?}", sdk);
        tracing::warn!("Set up entropy, data contract and signer");

        let data_contract =  DataContract::fetch(&sdk, Identifier::from(dpns_contract::ID_BYTES)).await
            .expect("fetching data contract")
            .expect("data contract not found");

        let preorder_document_type = data_contract
            .document_type_for_name(&"preorder")
            .expect("expected a profile document type");

        let hex_private_key = "a7285a6108fcd2a7b64060cbec68dddaf70c2d0514d8e0a447c8c933aef11b81";
        let private_key = hex::decode(hex_private_key).expect("Decoding failed");
        let mut signer = SimpleSigner::default();
        signer.add_key(identity_public_key.clone(), vec_to_array(private_key).expect("reason"));
        let entropy = entropy_generator.generate().unwrap();
        trace!("document_entropy: {:?}", entropy);

        // recreate the document using the same entropy value as when it is submitted below
        let new_preorder_document = preorder_document_type.create_document_from_data(
            preorder_document.properties().into(),
            preorder_document.owner_id(),
            random(),
            random(),
            entropy,
            PlatformVersion::latest()
        )?;

        let settings = PutSettings {
            request_settings: RequestSettings {
                connect_timeout: None,
                timeout: None,
                retries: Some(2),
                ban_failed_address: Some(true),
            },
            identity_nonce_stale_time_s: None,
            user_fee_increase: None,
            wait_timeout: None,
        };

        tracing::warn!("Call Document::put_to_platform_and_wait_for_response");
        // let first_document_result = new_preorder_document.put_to_platform_and_wait_for_response(
        //     &sdk,
        //     preorder_document_type.to_owned_document_type(),
        //     entropy,
        //     identity_public_key.clone(),
        //     Arc::new(data_contract.clone()),
        //     &signer,
        //     Some(settings)
        // ).await.or_else(|err|Err(ProtocolError::Generic(err.to_string())))?;

        let preorder_transition = new_preorder_document.put_to_platform(
            &sdk,
            preorder_document_type.to_owned_document_type(),
            entropy.clone(),
            identity_public_key.clone(),
            &signer,
            Some(settings)
        ).await.or_else(|err|Err(ProtocolError::Generic(err.to_string())))?;

        wait_for_response_concurrent(
            &new_preorder_document,
            &sdk,
            preorder_transition.clone(),
            data_contract.clone().into(),
            settings
        ).await.or_else(|err|Err(ProtocolError::Generic(err.to_string())))?;

        let domain_document_type = data_contract
            .document_type_for_name(&"domain")
            .expect("expected a profile document type");

        let entropy2 = entropy_generator.generate().unwrap();
        tracing::warn!("document_entropy: {:?}", entropy2);

        // recreate the document using the same entropy value as when it is submitted below
        let new_domain_document = domain_document_type.create_document_from_data(
            domain_document.properties().into(),
            domain_document.owner_id(),
            random(),
            random(),
            entropy2,
            PlatformVersion::latest()
        )?;

        tracing::warn!("Call Document::put_to_platform_and_wait_for_response");
        let second_document_result = new_domain_document.put_to_platform_and_wait_for_response(
            &sdk,
            domain_document_type.to_owned_document_type(),
            entropy2,
            identity_public_key,
            &signer,
            Some(settings)
        ).await.or_else(|err|Err(ProtocolError::Generic(err.to_string())))?;

        Ok::<Document, ProtocolError>(second_document_result)
    }) {
        Ok(doc) => tracing::info!("Success!"),
        Err(err) => panic!("{:?}", err.to_string())
    };
}

//#[test]
fn test_put_txmetadata_contract() {
    let testnet = false;
    let mainnet_owner_id  = Identifier::from_string("EuopfWEY7r4p8rKpcPsW7ZF2ytAbw4VGpZJwAuLHMdC3", Encoding::Base58).expect("identifier");
    let mainnet_public_key = BinaryData::from_string("03bcdaa3d8dad4e3d8e5375e4939bbafd2afeb243cfaa2c3c7cf8e9b8465f50af7", Encoding::Hex).unwrap();
    let mainnet_private_key = "e02df1f9cb9712a0cf5c8139f32d3e7943f94dc7364c9ad4fbac75f492be35c5";
    let testnet_owner_id  = Identifier::from_string("7Yowk46VwwHqmD5yZyyygggh937aP6h2UW7aQWBdWpM5", Encoding::Base58).expect("identifier");
    let testnet_public_key = BinaryData::from_string("AmM7AZIyWC1uOj0/3lFManLaTAK3RVToY+1yMvbpkvDa", Encoding::Base64).unwrap();
    let testnet_private_key = "a7285a6108fcd2a7b64060cbec68dddaf70c2d0514d8e0a447c8c933aef11b81";

    let entropy_generator = DefaultEntropyGenerator;
    let owner_id = if testnet { testnet_owner_id } else { mainnet_owner_id };

    let identity_public_key = IdentityPublicKey::V0(
        IdentityPublicKeyV0 {
            id: 1,
            purpose: Purpose::AUTHENTICATION,
            security_level: SecurityLevel::HIGH,
            contract_bounds: None,
            key_type: KeyType::ECDSA_SECP256K1,
            read_only: false,
            data: if testnet { testnet_public_key } else { mainnet_public_key },
            disabled_at: None,
        }
    );

    setup_logs();
    // Create a new Tokio runtime
    //let rt = tokio::runtime::Runtime::new().expect("Failed to create a runtime");
    let rt = Builder::new_current_thread()
        .enable_all() // Enables all I/O and time drivers
        .build()
        .expect("Failed to create a runtime");


    // Execute the async block using the Tokio runtime
    match rt.block_on(async {
        // Your async code here
        let cfg = if testnet { Config::new_testnet() } else { Config::new_mainnet() };
        tracing::warn!("Setting up SDK");
        let sdk = cfg.setup_api().await;
        tracing::warn!("Finished SDK, {:?}", sdk);
        tracing::warn!("Set up entropy, data contract and signer");

        let hex_private_key = if testnet { testnet_private_key } else { mainnet_private_key };
        let private_key = hex::decode(hex_private_key).expect("Decoding failed");
        let mut signer = SimpleSigner::default();
        signer.add_key(identity_public_key.clone(), vec_to_array(private_key).expect("reason"));
        let entropy = entropy_generator.generate().unwrap();
        trace!("document_entropy: {:?}", entropy);

        let settings = PutSettings {
            request_settings: RequestSettings {
                connect_timeout: None,
                timeout: None,
                retries: Some(2),
                ban_failed_address: Some(true),
            },
            identity_nonce_stale_time_s: None,
            user_fee_increase: None,
            wait_timeout: None,
        };

        let file_path = "dashwallet-contract.json";
        let mut file = fs::File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Parse the file contents as JSON and convert to platform_value::Value
        let value: Value = serde_json::from_str(&contents)?;
        let data_contract_factory = DataContractFactory::new(1).expect("create data contract factory");

        let created_data_contract = data_contract_factory.create_with_value_config(
            owner_id,
            0,
            value,
            None,
            None,
        )?;

        let data_contract = match created_data_contract {
            CreatedDataContract::V0(dc) => dc.data_contract,
        };

        let data_contract_result = data_contract.put_to_platform_and_wait_for_response(
            &sdk,
            identity_public_key,
            &signer,
            Some(settings)
        ).await.or_else(|e| Err(ProtocolError::Generic(e.to_string())));

        Ok::<DataContract, ProtocolError>(data_contract_result.unwrap())
    }) {
        Ok(data_contract) => tracing::info!("Success!\n{}: {:?}", data_contract.id(), data_contract),
        Err(err) => panic!("{:?}", err.to_string())
    };
}