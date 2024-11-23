use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use dapi_grpc::platform::v0::GetContestedResourceIdentityVotesRequest;
use dash_sdk::platform::transition::put_document::PutDocument;
use dash_sdk::platform::transition::put_settings::PutSettings;
use dash_sdk::platform::transition::vote::PutVote;
use dash_sdk::{Error, Sdk};
use dash_sdk::platform::{Fetch, FetchMany};
use dashcore::{base58, PrivateKey};
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use dpp::data_contract::DataContract;
use dpp::data_contract::document_type::accessors::DocumentTypeV0Getters;
use dpp::document::Document;
use dpp::identity::{Identity, identity_public_key::IdentityPublicKey, identity_public_key::TimestampMillis};
use dpp::prelude::{BlockHeight, CoreBlockHeight};
use dpp::ProtocolError;
use dpp::state_transition::StateTransition;
use dpp::util::entropy_generator::DefaultEntropyGenerator;
use dpp::voting::contender_structs::ContenderWithSerializedDocument;
use dpp::voting::vote_choices::resource_vote_choice::ResourceVoteChoice;
use dpp::voting::vote_polls::contested_document_resource_vote_poll::ContestedDocumentResourceVotePoll;
use dpp::voting::vote_polls::VotePoll;
use dpp::voting::votes::resource_vote::ResourceVote;
use dpp::voting::votes::resource_vote::v0::ResourceVoteV0;
use dpp::voting::votes::Vote;
use drive::query::contested_resource_votes_given_by_identity_query::ContestedResourceVotesGivenByIdentityQuery;
use drive::query::vote_poll_vote_state_query::{ContestedDocumentVotePollDriveQuery, ContestedDocumentVotePollDriveQueryResultType};
use drive::query::vote_polls_by_document_type_query::VotePollsByDocumentTypeQuery;
use drive::query::VotePollsByEndDateDriveQuery;
use drive_proof_verifier::types::{Contenders, ContestedResource, ContestedResources, ResourceVotesByIdentity, VotePollsGroupedByTimestamp};
use platform_value::types::identifier::Identifier;
use platform_value::{IdentifierBytes32, Value};
use platform_value::string_encoding::Encoding;
use platform_version::version::PlatformVersion;
use simple_signer::signer::SimpleSigner;
use tracing::trace;
use crate::config::{Config, EntryPoint};
use crate::fetch_document::fetch_documents_with_query_and_sdk;
use crate::put::{CallbackSigner, SignerCallback, wait_for_response_concurrent};
use crate::sdk::{create_dash_sdk_using_core_testnet, create_dash_sdk_using_core_mainnet, DashSdk};

/// push a vote from a single masternode designated by [voter_pro_tx_hash]
#[ferment_macro::export]
pub fn put_vote_to_platform(
    rust_sdk: *mut DashSdk,
    vote: Vote,
    voter_pro_tx_hash: Identifier,
    voting_public_key: IdentityPublicKey,
    signer_context: usize,
    signer_callback: u64
) -> Result<Vote, String> {

    let rt = unsafe { (*rust_sdk).get_runtime() };
    rt.block_on(async {

        let sdk = unsafe { (*rust_sdk).get_sdk() };
        let signer = CallbackSigner::new(signer_context, signer_callback).expect("signer");
        let request_settings = unsafe { (*rust_sdk).get_request_settings() };

        let settings = PutSettings {
            request_settings,
            identity_nonce_stale_time_s: None,
            user_fee_increase: None,
        };

        tracing::info!("Call Vote::put_to_platform");

        let masternode_vote_transition = vote.put_to_platform(
            voter_pro_tx_hash,
            &voting_public_key,
            &sdk,
            &signer,
            Some(settings)
        ).await.or_else(|err|Err(err.to_string()))?;
        tracing::info!("Call Vote::wait_for_response");
        tracing::info!(
            "state transition (hash): {}",
            hex::encode(hash_double_to_vec(masternode_vote_transition.serialize_to_bytes().unwrap()))
        );

        let vote = <Vote as PutVote<SimpleSigner>>::wait_for_response::<'_, '_, '_>(
            &vote,
            masternode_vote_transition,
            &sdk,
            Some(settings)
        ).await.or_else(|err|Err(err.to_string()))?;

        Ok(vote)
    })
}

// #[test]
// fn put_vote_test() {
//     let mut sdk = create_dash_sdk_using_core_testnet();
//     let contract_id = Identifier::from(dpns_contract::ID_BYTES);
//     let vote = Vote::ResourceVote(
//         ResourceVote::V0(
//             ResourceVoteV0 {
//                 vote_poll: VotePoll::ContestedDocumentResourceVotePoll(
//                     ContestedDocumentResourceVotePoll {
//                         contract_id,
//                         document_type_name: "domain".to_string(),
//                         index_name: "parentNameAndLabel".to_string(),
//                         index_values: vec![Value::Text("dash".to_string()), Value::Text("test000".to_string())],
//                     }
//                 ),
//                 resource_vote_choice: ResourceVoteChoice::Lock,
//             }
//         )
//     );
//
//     let voter_pro_tx_hash = Identifier::from_string("").unwrap();
//     let wif_private_key = "a7285a6108fcd2a7b64060cbec68dddaf70c2d0514d8e0a447c8c933aef11b81";
//     let voting_private_key = PrivateKey::from_wif(wif_private_key).unwrap();
//     let voting_public_key = voting_private_key.public_key()
//     let mut signer = SimpleSigner::default();
//
//     signer.add_key(identity_public_key.clone(), Vec::from(voting_private_key.as_slice()));
//
//     let request_settings = unsafe { (*sdk).get_request_settings() };
//
//     let settings = PutSettings {
//         request_settings,
//         identity_nonce_stale_time_s: None,
//         user_fee_increase: None,
//     };
//
//     tracing::info!("Call Vote::put_to_platform");
//
//     let masternode_vote_transition = vote.put_to_platform(
//         voter_pro_tx_hash,
//         &voting_public_key,
//         &sdk,
//         &signer,
//         Some(settings)
//     ).await.or_else(|err|Err(err.to_string()))?;
//     tracing::info!("Call Vote::wait_for_response");
//
//     let vote = <Vote as PutVote<SimpleSigner>>::wait_for_response::<'_, '_, '_>(
//         &vote,
//         masternode_vote_transition,
//         &sdk,
//         Some(settings)
//     ).await.or_else(|err|Err(err.to_string()))?;
//
//     match resources_result {
//         Ok(resources) => println!("contested resources = {:?}", resources),
//         Err(e) => panic!("error: {}", e)
//     }
// }

// pub async fn wait_for_response_concurrent_vote(
//     vote: &Vote,
//     sdk: &Sdk,
//     preorder_transition: StateTransition,
//     settings: PutSettings
// ) -> Result<Vote, Error> {
//     let mut handles = vec![];
//
//     for i in 0..5 {
//         let new_preorder_document = vote.clone();
//         let sdk = sdk.clone();
//         let preorder_transition = preorder_transition.clone();
//         let settings = Some(settings.clone());
//
//         tracing::info!("spawning thread {} of 5", i + 1);
//         let handle = tokio::spawn(async move {
//             <Vote as PutVote<SimpleSigner>>::wait_for_response::<'_, '_, '_>(
//                 &new_preorder_document,
//                 &sdk,
//                 preorder_transition,
//                 settings
//             ).await
//         });
//
//         handles.push(handle);
//     }
//
//     let mut success_count = 0;
//     let mut last_error: Option<Error> = None;
//
//     for handle in handles {
//         match handle.await {
//             Ok(Ok(document)) => {
//                 success_count += 1;
//                 if success_count >= 3 {
//                     tracing::warn!("wait_for_response_concurrent, success: {:?}", document);
//                     return Ok(document);
//                 }
//             }
//             Ok(Err(e)) => {
//                 tracing::warn!("wait_for_response_concurrent, response error: {:?}", e);
//                 last_error = Some(Error::from(e));
//             }
//             Err(e) => {
//                 tracing::warn!("wait_for_response_concurrent, join error: {:?}", e);
//                 last_error = Some(Error::Generic(e.to_string()));
//             }
//         }
//     }
//     tracing::warn!("wait_for_response_concurrent, all requests failed");
//
//     Err(last_error.unwrap_or(Error::Generic("All requests failed".to_string())))
// }
#[ferment_macro::export]
pub fn get_vote_contenders(
    rust_sdk: * mut DashSdk,
    index_name: String,
    index_values: Vec<Value>,
    document_type_name: String,
    contract_id: Identifier
) -> Result<Contenders, String>{

    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };


        let query = ContestedDocumentVotePollDriveQuery {
            limit: None,
            offset: None,
            start_at: None,
            vote_poll: ContestedDocumentResourceVotePoll {
                index_name: index_name.to_string(),
                index_values: index_values.to_vec(),
                document_type_name: document_type_name.to_string(),
                contract_id: contract_id,
            },
            allow_include_locked_and_abstaining_vote_tally: true,
            result_type:
            ContestedDocumentVotePollDriveQueryResultType::DocumentsAndVoteTally,
        };
        let settings = unsafe { (*rust_sdk).get_request_settings() };
        match ContenderWithSerializedDocument::fetch_many(&sdk, query.clone()).await {
                Ok(contenders) => Ok(contenders),
                Err(e) => Err(e.to_string())
        }
    })
}

/// Get the contested resources for a given data contract ([data_contract_id]) and the specified
/// document type ([document_type_name]).
///
/// For `domain` documents of the DPNS data contract, the contested resources are text items
/// from the `normalizedLabel` field.
#[ferment_macro::export]
pub fn get_contested_resources(
    rust_sdk: * mut DashSdk,
    document_type_name: String,
    data_contract_id: Identifier,
    limit: u16,
    start_at: Option<Value>,
    start_at_include: bool
) -> Result<ContestedResources, String>{

    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        tracing::info!("get_contested_resources: starting...");
        tracing::info!("  sdk: {:?}", sdk);
        let data_contract = match unsafe { ((*rust_sdk).get_data_contract(&data_contract_id)) } {
            Some(data_contract) => data_contract.clone(),
            None => {
                tracing::info!("Rust SDK doesn't have this contract.  Search platform...");
                match (DataContract::fetch(&sdk, data_contract_id.clone())
                    .await) {
                    Ok(Some(data_contract)) => {
                        unsafe { (*rust_sdk).add_data_contract(&data_contract); };
                        Arc::new(data_contract)
                    },
                    Ok(None) => return Err("data contract not found".to_string()),
                    Err(e) => {
                        tracing::error!("data contract not found {:?}", e);
                        return Err(e.to_string())
                    }
                }
            }
        };

        tracing::info!("get_contested_resources: found data contract");

        let document_type_result = data_contract
            .document_type_for_name(&document_type_name);

        let document_type = match document_type_result {
            Ok(dt) => dt,
            Err(e) => return Err(e.to_string())
        };
        let start_at_value = match start_at {
            Some(start_at) => Some((start_at, start_at_include)),
            None => None
        };

        if let Some(contested_index) = document_type.find_contested_index() {
            let query = VotePollsByDocumentTypeQuery {
                contract_id: data_contract.id(),
                document_type_name: document_type.name().to_string(),
                index_name: contested_index.name.clone(),
                start_at_value: start_at_value,
                start_index_values: vec!["dash".into()], // hardcoded for dpns
                end_index_values: vec![],
                limit: Some(limit),
                order_ascending: true,
            };

            tracing::info!("get_contested_resources: query ContestedResources for {:?}", query);
            let settings = unsafe { (*rust_sdk).get_request_settings() };
            let contested_resources = ContestedResource::fetch_many(&sdk, query).await;

            match contested_resources {
                Ok(resources) => Ok(resources),
                Err(e) => Err(e.to_string())
            }
        } else {
            return Err("cannot find contested index".to_string());
        }
    })
}

#[test]
fn get_contested_resources_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    tracing::warn!("sdk: {:?}", sdk.get_sdk());
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let resources_result = get_contested_resources(
            &mut sdk,
            "domain".to_string(),
            contract_id,
            100,
            None,
            false
        );
    match resources_result {
        Ok(resources) => println!("contested resources = {:?}", resources),
        Err(e) => panic!("error: {}", e)
    }
}

#[test]
fn get_vote_contenders_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    tracing::warn!("sdk: {:?}", sdk.get_sdk());
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let resources_result = get_vote_contenders(
        &mut sdk,
        "parentNameAndLabel".to_string(),
        vec![Value::Text("dash".to_string()), Value::Text("test110".to_string())],
        "domain".to_string(),
        contract_id
    );
    match resources_result {
        Ok(resources) => println!("contested resources = {:?}", resources),
        Err(e) => panic!("error: {}", e)
    }
}

#[ferment_macro::export]
pub fn get_votes(
    rust_sdk: * mut DashSdk,
    data_contract_id: Identifier
) -> Result<Option<Vote>, String>{

    let rt = unsafe { (*rust_sdk).get_runtime() }.clone();

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };
        let settings = unsafe { (*rust_sdk).get_request_settings() };

        let query = ContestedResourceVotesGivenByIdentityQuery {
            identity_id: data_contract_id,
            offset: None,
            limit: None,
            start_at: None,
            order_ascending: true,
        };

        match Vote::fetch_with_settings(&sdk, query.clone(), settings).await {
            Ok(vote) => Ok(vote),
            Err(e) => Err(e.to_string())
        }
    })
}

#[test]
fn get_votes_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    tracing::warn!("sdk: {:?}", sdk.get_sdk());
    // let contract_id = Identifier::from_string("HLWuAX1TebsXFNC8W2e8yUzaqLRCaB29pPxomNcRbBjK", Encoding::Base58).unwrap();
    let resources_result = get_votes(
        &mut sdk,
        Identifier::from(dpns_contract::ID_BYTES)
    );
    match resources_result {
        Ok(resources) => println!("votes = {:?}", resources),
        Err(e) => panic!("error: {}", e)
    }
}

/// Gets the vote polls that have end dates within the range of ([start_time], [end_time])
///
/// This is useful to determine the active contests for a particular document field such as the
/// `normalizedLabel` of the `domain` document of the DPNS contract.  [start_time] should be set to
/// the current time and the [end_time] should be set to 14 days from the [start_time].
#[ferment_macro::export]
pub fn get_vote_polls(
    rust_sdk: * mut DashSdk,
    start_time: TimestampMillis,
    start_time_included: bool,
    end_time: TimestampMillis,
    end_time_included: bool,
    limit: u16,
    offset: u16,
    order_ascending: bool
) -> Result<VotePollsGroupedByTimestamp, String>{

    let rt = unsafe { (*rust_sdk).get_runtime() }.clone();

    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        tracing::info!("get_vote_polls({}, {})", start_time, end_time);

        let query = VotePollsByEndDateDriveQuery {
            start_time: Some((start_time, start_time_included)),
            end_time: Some((end_time, end_time_included)),
            limit: Some(limit),
            offset: None,
            order_ascending,
        };

        match VotePoll::fetch_many(&sdk, query.clone()).await {
            Ok(votes) => {
                tracing::info!("get_vote_polls: {}", votes.0.len());
                Ok(votes)
            },
            Err(e) => Err(e.to_string())
        }
    })
}

#[test]
fn get_votepolls_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    tracing::warn!("sdk: {:?}", sdk.get_sdk());

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let start_mills = since_the_epoch.as_millis() as u64;

    let resources_result = get_vote_polls(
        &mut sdk,
        start_mills,
        true,
        start_mills + 14 * 24 * 3600 * 1000,
        true,
        100,
        0,
        true
    );
    match resources_result {
        Ok(resources) => println!("contested resources = {:?}", resources),
        Err(e) => panic!("error: {}", e)
    }
}

#[test]
fn get_votepolls_mainnet_test() {
    let mut sdk = create_dash_sdk_using_core_mainnet();
    tracing::warn!("sdk: {:?}", sdk.get_sdk());

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let start_mills = since_the_epoch.as_millis() as u64;

    let resources_result = get_vote_polls(
        &mut sdk,
        start_mills,
        true,
        start_mills + 14 * 24 * 3600 * 1000,
        true,
        100,
        0,
        true
    );
    match resources_result {
        Ok(resources) => println!("contested resources = {:?}", resources),
        Err(e) => panic!("error: {}", e)
    }
}

use dash_sdk::platform::query::VoteQuery;
use dpp::serialization::PlatformSerializable;
use dpp::util::hash::hash_double_to_vec;

#[ferment_macro::export]
pub fn get_last_vote_from_masternode(
    rust_sdk: * mut DashSdk,
    masternode_protxhash: Identifier,
    index_name: String,
    index_values: Vec<Value>,
    document_type_name: String,
    contract_id: Identifier
) -> Result<ResourceVotesByIdentity, String>{

    let rt = unsafe { (*rust_sdk).get_runtime() }.clone();

    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };
        let settings = unsafe { (*rust_sdk).get_request_settings() };

        let query = VoteQuery {
            identity_id: masternode_protxhash,
            vote_poll_id: ContestedDocumentResourceVotePoll {
                contract_id,
                document_type_name,
                index_name,
                index_values,
            }.unique_id().unwrap(),
        };

        match ResourceVote::fetch_many(&sdk, query.clone()).await {
            Ok(votes) => Ok(votes),
            Err(e) => Err(e.to_string())
        }
    })
}

#[test]
fn get_votes_from_identity_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    tracing::warn!("sdk: {:?}", sdk.get_sdk());

    //let masternode_identifier = Identifier::from_string("2Ey6wdP5YYSqhq96KmU349CeSCsV4avrsNCaXqogGEr9", Encoding::Base58).unwrap();
    let masternode_identifier = Identifier::from_string("c0aae8ab24aab988cc84385d16af7ffcfd365d0e016f5799759e0525a435a617", Encoding::Hex).unwrap();
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let index_name =  "parentNameAndLabel".to_string();
    let index_values = vec![Value::Text("dash".to_string()), Value::Text("test-10101".to_string())];
    let document_type_name = "domain".to_string();

    let result = get_last_vote_from_masternode(
        &mut sdk,
        masternode_identifier,
        index_name.clone(),
        index_values.clone(),
        document_type_name.clone(),
        contract_id,
    );
    match result {
        Ok(votesByIdentity) => println!("result 1 = {:?}", votesByIdentity),
        Err(e) => panic!("error: {}", e)
    }
    let masternode_identifier = Identifier::from_string("AZW2PvF35kBGgmF3naWqdUZt7A74b7CszFgoxncHViEv", Encoding::Base58).unwrap();
    let result = get_last_vote_from_masternode(
        &mut sdk,
        masternode_identifier,
        index_name,
        index_values,
        document_type_name,
        contract_id,
    );
    match result {
        Ok(votesByIdentity) => println!("result 2 = {:?}", votesByIdentity),
        Err(e) => panic!("error: {}", e)
    }
}