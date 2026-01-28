use std::collections::BTreeMap;
use std::sync::Arc;
use dash_sdk::platform::{DocumentQuery, Fetch, FetchMany};
use dapi_grpc::platform::v0::get_documents_request::get_documents_request_v0::Start;
use dash_sdk::platform::proto::GetDataContractRequest;
use dash_sdk::Sdk;
use dpp::bincode::config::Limit;
use dpp::data_contract::DataContract;
use dpp::document::{Document, DocumentV0Getters};
use drive::query::{ordering::OrderClause, conditions::WhereClause, conditions::WhereOperator};
use platform_value::{types::identifier::Identifier, IdentifierBytes32, Value};
use tokio::runtime::{Builder, Runtime};
use crate::config::{Config, EntryPoint};
use crate::logs::setup_logs;
use crate::sdk::{create_dash_sdk_using_core_testnet, create_dash_sdk_using_single_evonode};
use crate::sdk::DashSdk;
use dash_sdk::Error;
use drive_proof_verifier::types::Documents;
use rs_dapi_client::{DapiClientError, RequestSettings};
use rs_dapi_client::transport::BoxFuture;
use dpp::version::LATEST_PLATFORM_VERSION;
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use dpp::document::serialization_traits::DocumentPlatformConversionMethodsV0;
use crate::provider::Cache;
use dash_sdk::query_types::IndexMap;
#[ferment_macro::export]
pub fn document_to_string(document: Document)-> String {
    document.to_string()
}

#[ferment_macro::export]
pub enum StartPoint {
    StartAfter(Vec<u8>),
    StartAt(Vec<u8>),
}

impl Into<Start> for StartPoint {
    fn into(self) -> Start {
        match self {
            StartPoint::StartAt(v) => Start::StartAt(v),
            StartPoint::StartAfter(v) => Start::StartAfter(v)
        }
    }
}

fn fetch_documents_with_retry(
    sdk: Arc<Sdk>,  // No need for a reference; pass the Arc by value
    data_contract_cache: Arc<Cache<Identifier, DataContract>>,
    query: DocumentQuery,
    request_settings: RequestSettings,
    retries_left: usize,
) -> BoxFuture<'static, Result<Documents, Error>> {
    // Clone the Arc<Sdk> here to ensure it's owned by the future
    Box::pin(async move {
        match Document::fetch_many(&sdk, query.clone()).await {
            Ok(documents) => Ok(documents),
            Err(error) => {
                if retries_left > 1 {
                    if error.to_string().contains("contract not found error: contract not found when querying from value with contract info") {
                        if (data_contract_cache.get(&query.data_contract.id()) != None) {
                            return fetch_documents_with_retry(sdk, data_contract_cache, query, request_settings, retries_left - 1).await;
                        }
                    }
                }
                Err(error)
            }
        }
    })
}

#[ferment_macro::export]
pub fn fetch_documents_with_query_and_sdk(
                                  rust_sdk: *mut DashSdk,
                                  data_contract_id: Identifier,
                                  document_type: String,
                                  where_clauses: Vec<WhereClause>,
                                  order_clauses: Vec<OrderClause>,
                                  limit: u32,
                                  start: Option<StartPoint>
) -> Result<Vec<Document>, String> {
    let rt = unsafe { (*rust_sdk).get_runtime() };

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let sdk = unsafe { (*rust_sdk).get_sdk() };

        let contract = match unsafe { (*rust_sdk).get_data_contract(&data_contract_id) } {
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

        tracing::warn!("contract_fetch_result: {:?}", contract);

        // Fetch multiple documents so that we get document ID
        let mut all_docs_query = match DocumentQuery::new(Arc::clone(&contract), &document_type) {
            Ok(result) => result,
            Err(e) => return Err(e.to_string())
        };
        for wc in where_clauses {
            all_docs_query = all_docs_query.with_where(wc);
        }
        for oc in order_clauses {
            all_docs_query = all_docs_query.with_order_by(oc);
        }
        all_docs_query.limit = limit;
        all_docs_query.start = match start {
            Some(s) => Some(s.into()),
            None => None
        };
        let settings = unsafe { (*rust_sdk).get_request_settings() };
        let extra_retries = match settings.retries {
            Some(retries) => retries,
            None => 5 as usize
        };
        let data_contract_cache = unsafe { (*rust_sdk).data_contract_cache.clone() };
        match fetch_documents_with_retry(sdk.clone(), data_contract_cache, all_docs_query, settings, extra_retries).await {
            Ok(docs) => {
                let into_vec = |map: IndexMap<Identifier, Option<Document>>| {
                    map.into_iter()
                        .filter_map(|(_key, value)| value)
                        .collect::<Vec<Document>>()
                };

                Ok(into_vec(docs))
            }
            Err(e) => Err(e.to_string())
        }
    })
}

// Contenders: 5
// Abstain: 0
// Lock: 0
// Identifier: 2GW4QGjKj7zJVtL8SYaGZxuCYbkRCSAVAm9R9JDTtDaX
// Serialized:AL58b+byMei1+Tsbf5maCTwfg28Jczy6Fg6LR4tB3DeSEtRBemB6s6voUDw0LXI5YYF9EzyMp0zHLYTs1XMMEKABAAcAAAGRYOWVHgAAAZFg5ZUeAAABkWDllR4ABnRlc3QxMQZ0ZXN0MTEBBGRhc2gEZGFzaAAhARLUQXpgerOr6FA8NC1yOWGBfRM8jKdMxy2E7NVzDBCgAQA=
// Votes: 0
// Identifier: 2Phpq9yHi97dSookJery5xo2e3HARxGbqKaDDSSCFZki
// Serialized:AHDmT7mDZwcKgFdSEp/7we2TDX7NX5Q9mgflQnpWEhE7FKygFLCHkgJMkAHXRSeZmNMNf+LmA16iBU196pS+UScBAAcAAAGRXZuz7QAAAZFdm7PtAAABkV2bs+0ABnRlc3QxMQZ0ZXN0MTEBBGRhc2gEZGFzaAAhARSsoBSwh5ICTJAB10UnmZjTDX/i5gNeogVNfeqUvlEnAQA=
// Votes: 0
// Identifier: CrAJrjAyy7L1ShDyvKxHRJ1Cp2XfqdVHqFhfSZLi9QdY
// Serialized:AJUAiE5IM4P/TPA4BQBSumSFvbUMmUIDJYpoTZyX4TF2sAiKULrGNc7AEM66uXW6D8oCihs3wZ2lzYxjuSNyFrMBAAcAAAGRYNznfwAAAZFg3Od/AAABkWDc538ABnRlc3QxMQZ0ZXN0MTEBBGRhc2gEZGFzaAAhAbAIilC6xjXOwBDOurl1ug/KAoobN8Gdpc2MY7kjchazAQA=
// Votes: 0
// Identifier: DRa5SomSnh8cENfu3gLWAd2UuNNQKuYEyHakHk7oKivm
// Serialized:AOLKw44CYT2J/bjXSuyObfue6V4/kdvTBn8tb6m+7W2iuJeVGJgxeHXOiwg5e+Zzc7BPmIDjEVS3mb5DQLRGIiIBAAcAAAGRay/0NgAAAZFrL/Q2AAABkWsv9DYABnRlc3QxMQZ0ZXN0MTEBBGRhc2gEZGFzaAAhAbiXlRiYMXh1zosIOXvmc3OwT5iA4xFUt5m+Q0C0RiIiAQA=
// Votes: 0
// Identifier: EeHNsyw3MTJGquJxR45K8Wnt7BvTCLyuxGDAgxHdoGnA
// Serialized:AGH4+kYLEEVx5P49R8qys8mejGccoym8xP537nFJKG1MyrTwEVcAzOVfnNN0jDdMkpGXzPCKainEbQEMSu+PuQcBAAcAAAGRXbiwhAAAAZFduLCEAAABkV24sIQABnRlc3QxMQZ0ZXN0MTEBBGRhc2gEZGFzaAAhAcq08BFXAMzlX5zTdIw3TJKRl8zwimopxG0BDErvj7kHAQA=
// Votes: 0
#[ferment_macro::export]
pub unsafe fn deserialize_document_sdk(
    rust_sdk: *mut DashSdk,
    bytes: Vec<u8>,
    data_contract_id: Identifier,
    document_type: String
) -> Result<Document, String> {

    let rt = (*rust_sdk).get_runtime();

    // Execute the async block using the Tokio runtime
    rt.block_on(async {
        let sdk = (*rust_sdk).get_sdk();

        tracing::info!("using existing data contract id and fetching...");

        let sdk = unsafe { (*rust_sdk).get_sdk() };

        let contract = match ((*rust_sdk).get_data_contract(&data_contract_id)) {
            Some(data_contract) => data_contract.clone(),
            None => {
                match (DataContract::fetch(&sdk, data_contract_id.clone())
                    .await) {
                    Ok(Some(data_contract)) => Arc::new(data_contract),
                    Ok(None) => return Err("data contract not found".to_string()),
                    Err(e) => return Err(e.to_string())
                }
            }
        };

        Document::from_bytes(&bytes, contract.document_type_for_name(&document_type).unwrap(), LATEST_PLATFORM_VERSION)
            .or_else(|e| Err(format!("deserialization failed: {}", e.to_string())))
    })
}


// TODO: need to rewrite these tests
// #[test]
// fn docs_test() {
//     let contract_id = Identifier::from(dpns_contract::ID_BYTES);
//     let docs = documents_with_callbacks(contract_id, "domain".to_string(), 0, 0);
//
//     for document in docs {
//         // Use `document` here
//         println!("{:?}", document); // Assuming Document implements Debug
//     }
// }
//
// #[test]
// fn docs_query_test() {
//    //let contract_id = Identifier(IdentifierBytes32(DPNS_DATACONTRACT_ID));
//     let docs = dpns_domain_starts_with("dq-".to_string(), 0, 0);
//
//     for document in docs {
//         // Use `document` here
//         println!("{:?}", document); // Assuming Document implements Debug
//     }
// }

// #[test]
// fn docs_query_id_test() {
//     let contract_id = Identifier::from(dpns_contract::ID_BYTES);
//     let docs = dpns_domain_by_id(contract_id, 0, 0);
//
//     for document in docs {
//         // Use `document` here
//         println!("{:?}", document); // Assuming Document implements Debug
//     }
//}

// #[test]
// fn docs_full_query_test() {
//     let contract_id = Identifier(IdentifierBytes32(DPNS_DATACONTRACT_ID));
//     let docs_result = fetch_documents_with_query(contract_id, "domain".to_string(),
//                                                  vec![WhereClause {
//                                               field: "normalizedLabel".to_string(),
//                                               operator: WhereOperator::Equal,
//                                               value: Value::Null,
//                                           }],
//                                                  vec![],
//                                                  100,
//                                                  None,
//                                                  0, 0);
//
//     match docs_result {
//         Ok(docs) => {
//             println!("query results");
//             for document in docs {
//                 // Use `document` here
//                 println!("{:?}", document); // Assuming Document implements Debug
//             }
//         }
//         Err(e) => panic!("{}", e)
//     }
// }

#[test]
fn docs_full_query_sdk_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let docs_result = unsafe {
        fetch_documents_with_query_and_sdk(
            &mut sdk,
            contract_id,
            "domain".to_string(),
            vec![],
            vec![],
            100,
            None
        )
    };

    match docs_result {
        Ok(docs) => {
            println!("query results");
            for document in docs {
                // Use `document` here
                println!("{:?}", document); // Assuming Document implements Debug
            }
        }
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn docs_startswith_query_sdk_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let docs_result = unsafe {
        fetch_documents_with_query_and_sdk(
            &mut sdk,
            contract_id,
            "domain".to_string(),
            vec![
                WhereClause { field: "normalizedLabel".into(), value: Value::Text("b0b1ee".into()), operator: WhereOperator::StartsWith },
                WhereClause { field: "normalizedParentDomainName".into(), value: Value::Text("dash".into()), operator: WhereOperator::Equal }
            ],
            vec![
                OrderClause { field: "normalizedLabel".into(), ascending: true }
            ],
            100,
            None
        )
    };

    match docs_result {
        Ok(docs) => {
            println!("query results: {}", docs.len());
            for document in docs {
                // Use `document` here
                println!("{:?}", document); // Assuming Document implements Debug
            }
        }
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn docs_get_all_query_sdk_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let docs_result = unsafe {
        fetch_documents_with_query_and_sdk(
            &mut sdk,
            contract_id,
            "domain".to_string(),
            vec![],
            vec![],
            100,
            None
        )
    };

    match docs_result {
        Ok(docs) => {
            println!("query results: {}", docs.len());
            let docs_result2 = unsafe {
                fetch_documents_with_query_and_sdk(
                    &mut sdk,
                    contract_id,
                    "domain".to_string(),
                    vec![],
                    vec![OrderClause { field: "normalizedLabel".into(), ascending: true }],
                    100,
                    // TODO - use StartAfter instead
                    Some(StartPoint::StartAt(docs.last().unwrap().id().to_vec()))
                )
            };
            match docs_result2 {
                Ok(documents) => {
                    println!("query results: {}", documents.len());
                },
                Err(e) => { panic!("error: {}", e)}
            }
            for document in docs {
                // Use `document` here
                println!("{:?}", document); // Assuming Document implements Debug
            }
        }
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn docs_startswith_query_sdk_using_single_node_test() {
    let mut sdk = create_dash_sdk_using_single_evonode("35.163.144.230".into(), 0, 0, true);
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let docs_result = unsafe {
        fetch_documents_with_query_and_sdk(
            &mut sdk,
            contract_id,
            "domain".to_string(),
            vec![
                WhereClause { field: "normalizedLabel".into(), value: Value::Text("b0b1ee".into()), operator: WhereOperator::StartsWith },
                WhereClause { field: "normalizedParentDomainName".into(), value: Value::Text("dash".into()), operator: WhereOperator::Equal }
            ],
            vec![
                OrderClause { field: "normalizedLabel".into(), ascending: true }
            ],
            100,
            None
        )
    };

    match docs_result {
        Ok(docs) => {
            println!("query results: {}", docs.len());
            for document in docs {
                // Use `document` here
                println!("{:?}", document); // Assuming Document implements Debug
            }
        }
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn docs_domain_query_sort_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();
    let contract_id = Identifier::from(dpns_contract::ID_BYTES);
    let docs_result = unsafe {
        fetch_documents_with_query_and_sdk(
            &mut sdk,
            contract_id,
            "domain".to_string(),
            vec![
                WhereClause { field: "normalizedLabel".into(), value: Value::Text("tut".into()), operator: WhereOperator::StartsWith },
                WhereClause { field: "normalizedParentDomainName".into(), value: Value::Text("dash".into()), operator: WhereOperator::Equal }
            ],
            vec![
                OrderClause { field: "normalizedLabel".into(), ascending: true }
            ],
            100,
            None
        )
    };

    match docs_result {
        Ok(docs) => {
            println!("query results: {}", docs.len());
            for document in docs {
                match document {
                    Document::V0(document_v0) => {
                        // Use `document` here
                        println!("{:?}", document_v0.properties().get("normalizedLabel")); // Assuming Document implements Debug
                    }
                }
            }
        }
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn doc_deserialization_sdk_test() {
    let mut sdk = create_dash_sdk_using_core_testnet();

    unsafe {
        println!(
            "{:?}", deserialize_document_sdk(
                &mut sdk,
                base64::decode("AGH4+kYLEEVx5P49R8qys8mejGccoym8xP537nFJKG1MyrTwEVcAzOVfnNN0jDdMkpGXzPCKainEbQEMSu+PuQcBAAcAAAGRXbiwhAAAAZFduLCEAAABkV24sIQABnRlc3QxMQZ0ZXN0MTEBBGRhc2gEZGFzaAAhAcq08BFXAMzlX5zTdIw3TJKRl8zwimopxG0BDErvj7kHAQA=").unwrap(),
                Identifier::from(dpns_contract::ID_BYTES),
                "domain".into()
            ).unwrap()
        );
    }
}


pub const ADDRESS_LIST: [&str; 29] = [
    "34.214.48.68",
    "35.166.18.166",
    "50.112.227.38",
    "52.42.202.128",
    "52.12.176.90",
    "44.233.44.95",
    "35.167.145.149",
    "52.34.144.50",
    "44.240.98.102",
    "54.201.32.131",
    "52.10.229.11",
    "52.13.132.146",
    "44.228.242.181",
    "35.82.197.197",
    "52.40.219.41",
    "44.239.39.153",
    "54.149.33.167",
    "35.164.23.245",
    "52.33.28.47",
    "52.43.86.231",
    "52.43.13.92",
    "35.163.144.230",
    "52.89.154.48",
    "52.24.124.162",
    "44.227.137.77",
    "35.85.21.179",
    "54.187.14.232",
    "54.68.235.201",
    "52.13.250.182",
];

const MAINNET_ADDRESS_LIST: [&str; 331] = ["149.28.241.190", "147.45.183.128", "216.238.75.46", "167.17.40.14", "198.7.115.48", "134.255.182.186", "93.115.172.39", "5.189.164.253", "38.242.197.189", "83.222.9.253", "66.245.196.52", "173.212.245.118", "202.71.14.79", "95.179.240.182", "188.225.82.165", "185.252.234.238", "178.215.237.134", "207.180.233.6", "167.17.40.95", "136.244.99.17", "173.212.232.90", "178.215.237.135", "5.182.33.231", "157.90.238.161", "109.199.104.243", "37.60.236.212", "23.88.63.58", "207.244.247.40", "45.32.70.131", "158.220.122.76", "52.33.9.172", "178.157.91.177", "194.163.166.185", "109.199.124.30", "185.198.234.17", "139.84.236.208", "38.242.218.26", "194.163.153.225", "194.146.13.7", "65.108.4.213", "158.247.208.247", "75.119.153.10", "75.119.132.2", "65.108.74.95", "65.109.114.211", "44.240.99.214", "5.75.133.148", "65.108.0.153", "62.84.182.155", "89.35.131.149", "192.248.178.237", "161.97.153.122", "95.179.159.65", "217.77.12.102", "195.26.254.228", "45.76.36.241", "45.77.11.194", "139.84.232.129", "147.45.103.99", "202.71.14.154", "46.254.241.7", "85.193.90.107", "194.195.87.34", "85.239.244.103", "65.108.246.145", "64.176.10.71", "158.247.247.241", "194.163.170.55", "95.179.234.163", "65.108.0.151", "5.252.55.187", "139.180.143.115", "198.7.119.139", "213.199.44.112", "155.138.220.69", "51.79.109.200", "209.145.48.154", "178.253.42.64", "162.212.35.100", "185.239.209.6", "37.27.67.154", "173.212.196.214", "157.173.113.158", "134.255.182.185", "91.199.149.177", "51.79.109.182", "139.84.137.143", "144.91.87.82", "161.97.91.217", "217.25.89.156", "65.109.114.215", "173.212.239.124", "144.126.141.62", "173.249.21.12", "161.97.159.172", "5.189.186.78", "139.84.170.10", "164.68.118.37", "65.109.108.141", "158.220.92.144", "65.108.4.196", "192.248.175.198", "65.21.237.225", "188.225.56.52", "81.200.152.144", "43.167.244.109", "146.59.45.235", "37.27.67.159", "92.53.120.89", "172.236.244.81", "62.171.144.192", "146.59.153.204", "37.60.236.225", "185.215.164.84", "172.233.66.70", "157.173.122.20", "45.153.70.126", "57.128.212.163", "185.141.216.4", "82.208.20.153", "85.190.243.3", "51.195.235.166", "92.53.119.105", "95.111.239.54", "156.67.29.45", "82.211.21.38", "93.115.172.37", "15.235.102.216", "65.109.108.138", "89.35.131.39", "93.115.172.38", "134.255.183.250", "185.192.96.70", "134.255.183.248", "52.36.102.91", "139.99.201.103", "134.255.183.247", "65.109.108.139", "49.13.28.255", "213.199.34.250", "161.97.74.173", "168.119.102.10", "45.135.180.79", "45.135.180.130", "173.212.251.130", "38.242.206.103", "157.173.122.157", "65.109.108.150", "49.13.237.193", "75.119.156.254", "37.27.83.17", "45.135.180.114", "89.35.131.61", "86.107.101.74", "173.212.239.247", "157.173.202.14", "177.93.141.60", "89.35.131.23", "65.109.108.142", "38.242.198.100", "192.175.127.198", "37.27.67.163", "38.242.206.56", "5.252.55.190", "198.7.115.43", "70.34.206.123", "51.158.169.237", "65.108.74.78", "157.173.122.158", "108.61.165.170", "89.35.131.219", "185.166.217.154", "31.220.88.116", "157.173.122.21", "37.27.67.164", "217.77.12.101", "38.242.200.227", "193.168.3.82", "5.252.55.189", "167.86.93.21", "195.26.241.252", "65.108.0.150", "161.97.170.251", "51.195.47.118", "45.135.180.70", "167.88.169.16", "216.238.99.9", "62.169.17.112", "91.222.237.98", "157.173.122.26", "82.211.21.18", "52.10.213.198", "51.79.109.199", "139.84.231.221", "65.108.0.149", "37.252.17.214", "45.94.58.58", "198.7.115.38", "37.60.236.161", "37.60.244.220", "49.13.193.251", "77.232.129.86", "46.254.241.9", "65.109.114.236", "167.86.94.138", "198.27.68.167", "89.23.112.100", "65.108.74.75", "192.95.32.205", "95.179.241.182", "92.63.176.202", "65.109.84.204", "188.225.39.14", "65.109.84.203", "176.57.213.170", "93.115.172.36", "82.211.21.16", "158.220.89.188", "95.216.146.18", "167.114.153.110", "89.250.75.61", "185.194.216.84", "158.220.87.156", "31.220.84.93", "185.197.250.227", "161.97.91.68", "93.115.172.72", "162.250.188.207", "65.108.4.195", "149.28.247.165", "45.85.147.192", "157.173.122.156", "213.199.34.251", "158.220.101.10", "93.190.140.101", "95.171.21.131", "142.132.160.64", "85.92.111.111", "185.198.234.12", "87.228.24.64", "5.189.151.7", "38.242.231.212", "64.176.35.235", "38.143.58.210", "185.215.164.186", "161.97.102.156", "167.17.40.120", "46.254.241.8", "49.12.102.105", "81.17.101.141", "93.115.172.183", "65.108.0.158", "65.108.74.79", "147.45.236.121", "86.107.101.128", "54.69.95.118", "158.220.122.13", "49.13.154.121", "15.235.102.215", "91.220.109.253", "82.211.21.169", "65.21.237.223", "194.58.47.180", "195.201.238.55", "135.181.110.216", "207.180.213.141", "89.223.120.216", "45.76.141.74", "161.97.66.31", "50.116.28.103", "84.247.187.76", "188.245.90.255", "149.28.223.171", "130.162.233.186", "80.68.156.191", "65.109.114.210", "144.91.106.188", "65.109.84.201", "167.235.102.194", "164.68.114.36", "167.88.165.175", "43.167.239.145", "37.60.236.247", "178.157.91.184", "185.239.208.110", "95.179.139.125", "65.108.4.194", "75.119.137.66", "213.199.34.248", "178.157.91.178", "178.18.254.136", "157.173.122.25", "82.211.21.40", "93.115.172.86", "65.21.237.222", "62.171.170.14", "45.77.129.235", "185.209.230.13", "75.119.152.219", "81.0.249.58", "89.23.117.144", "37.60.243.59", "37.27.67.156", "192.99.232.138", "89.35.131.218", "5.189.145.80", "159.69.204.162", "178.18.244.255", "65.21.237.231", "149.102.152.219", "77.221.148.204", "65.21.237.224", "167.17.40.92", "46.254.241.11", "207.180.218.245", "158.220.101.28", "5.252.55.188", "173.199.71.83", "185.215.166.126", "164.132.55.103", "162.250.190.133", "72.60.38.160", "89.35.131.158", "82.97.240.124", "167.17.40.97", "213.159.77.221", "213.199.35.15", "114.132.172.215", "167.17.40.218", "65.108.74.109", "65.109.114.212"];

#[test]
fn check_all_nodes_test() {

    let mut good_nodes: Vec<String> = Vec::new();
    let mut bad_nodes: Vec<String> = Vec::new();

    for address in ADDRESS_LIST {
        let mut sdk = create_dash_sdk_using_single_evonode(address.into(), 0, 0, true);
        let contract_id = Identifier::from(dpns_contract::ID_BYTES);
        let docs_result = unsafe {
            fetch_documents_with_query_and_sdk(
                &mut sdk,
                contract_id,
                "domain".to_string(),
                vec![
                    WhereClause { field: "normalizedLabel".into(), value: Value::Text("test111".into()), operator: WhereOperator::StartsWith },
                    WhereClause { field: "normalizedParentDomainName".into(), value: Value::Text("dash".into()), operator: WhereOperator::Equal }
                ],
                vec![
                    OrderClause { field: "normalizedLabel".into(), ascending: true }
                ],
                100,
                None
            )
        };

        match docs_result {
            Ok(docs) => {
                println!("{}: success", address);
                good_nodes.push(address.to_string());
            }
            Err(e) => {
                println!("{}: error/fail: {}", address, e);
                bad_nodes.push(address.to_string());
            }
        }
    }
    println!("good nodes: {:?}", good_nodes);
    println!("bad nodes: {:?}", bad_nodes);
}

#[test]
fn check_all_nodes_mainnet_test() {

    let mut good_nodes: Vec<String> = Vec::new();
    let mut bad_nodes: Vec<String> = Vec::new();

    for address in MAINNET_ADDRESS_LIST {
        let mut sdk = create_dash_sdk_using_single_evonode(address.into(), 0, 0, false);
        let contract_id = Identifier::from(dpns_contract::ID_BYTES);
        let docs_result = unsafe {
            fetch_documents_with_query_and_sdk(
                &mut sdk,
                contract_id,
                "domain".to_string(),
                vec![
                    WhereClause { field: "normalizedLabel".into(), value: Value::Text("test111".into()), operator: WhereOperator::StartsWith },
                    WhereClause { field: "normalizedParentDomainName".into(), value: Value::Text("dash".into()), operator: WhereOperator::Equal }
                ],
                vec![
                    OrderClause { field: "normalizedLabel".into(), ascending: true }
                ],
                100,
                None
            )
        };

        match docs_result {
            Ok(docs) => {
                println!("{}: success", address);
                good_nodes.push(address.to_string());
            }
            Err(e) => {
                println!("{}: error/fail: {}", address, e);
                bad_nodes.push(address.to_string());
            }
        }
    }
    println!("good nodes: {:?}", good_nodes);
    println!("bad nodes: {:?}", bad_nodes);
}
