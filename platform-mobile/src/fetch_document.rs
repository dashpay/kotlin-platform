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


pub const ADDRESS_LIST: [&str; 33] = [
    "34.214.48.68",
    "35.166.18.166",
    "35.165.50.126",
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
     "35.82.49.196",
    "44.232.196.6",
    "54.189.164.39",
    "54.213.204.85"
];

const MAINNET_ADDRESS_LIST: [&str; 193] = [
    "149.28.241.190",
    "216.238.75.46",
    "134.255.182.186",
    "5.161.49.32",
    "185.217.127.139",
    "66.245.196.52",
    "109.199.122.22",
    "178.157.91.186",
    "80.147.135.74",
    "157.66.81.162",
    "147.135.199.138",
    "213.199.34.250",
    "43.167.240.39",
    "178.157.91.176",
    "157.90.238.161",
    "5.182.33.231",
    "185.198.234.68",
    "37.60.236.212",
    "49.12.233.191",
    "207.244.247.40",
    "195.201.34.10",
    "45.32.70.131",
    "158.220.122.76",
    "52.33.9.172",
    "185.158.107.124",
    "185.198.234.17",
    "93.190.140.101",
    "194.163.153.225",
    "194.146.13.7",
    "93.190.140.112",
    "38.242.253.223",
    "75.119.132.2",
    "65.108.74.95",
    "44.240.99.214",
    "5.75.133.148",
    "38.242.233.166",
    "78.47.86.233",
    "192.248.178.237",
    "95.179.159.65",
    "2.58.14.149",
    "139.84.232.129",
    "37.60.243.119",
    "194.195.87.34",
    "46.254.241.7",
    "161.97.160.92",
    "45.77.77.195",
    "65.108.246.145",
    "64.176.10.71",
    "158.247.247.241",
    "37.60.244.220",
    "2.58.82.231",
    "139.180.143.115",
    "185.198.234.54",
    "213.199.44.112",
    "37.27.67.154",
    "134.255.182.185",
    "91.199.149.177",
    "86.107.168.28",
    "139.84.137.143",
    "173.212.239.124",
    "157.10.199.77",
    "5.189.186.78",
    "139.84.170.10",
    "173.249.53.139",
    "37.60.236.151",
    "109.199.123.12",
    "128.140.107.66",
    "37.27.67.159",
    "104.200.24.196",
    "37.60.236.225",
    "172.104.90.249",
    "57.128.212.163",
    "37.60.236.249",
    "158.220.122.74",
    "185.198.234.25",
    "148.113.201.221",
    "134.255.183.250",
    "185.192.96.70",
    "134.255.183.248",
    "158.220.87.55",
    "52.36.102.91",
    "158.220.80.235",
    "161.97.160.87",
    "134.255.183.247",
    "49.13.28.255",
    "168.119.102.10",
    "86.107.168.44",
    "49.13.237.193",
    "37.27.83.17",
    "37.27.28.6",
    "134.255.182.187",
    "142.132.165.149",
    "193.203.15.209",
    "38.242.198.100",
    "192.175.127.198",
    "37.27.67.163",
    "79.137.71.84",
    "51.68.155.64",
    "198.7.115.43",
    "70.34.206.123",
    "163.172.20.205",
    "65.108.74.78",
    "108.61.165.170",
    "157.10.199.79",
    "95.217.187.31",
    "31.220.88.116",
    "185.166.217.154",
    "37.27.67.164",
    "31.220.85.180",
    "161.97.170.251",
    "157.10.199.82",
    "91.107.226.241",
    "167.88.169.16",
    "216.238.99.9",
    "62.169.17.112",
    "52.10.213.198",
    "149.28.201.164",
    "198.7.115.38",
    "37.60.236.161",
    "49.13.193.251",
    "46.254.241.9",
    "185.215.167.70",
    "65.108.74.75",
    "192.99.44.64",
    "95.179.241.182",
    "95.216.146.18",
    "109.199.123.10",
    "158.220.80.239",
    "194.163.143.174",
    "185.194.216.84",
    "31.220.84.93",
    "185.197.250.227",
    "149.28.247.165",
    "86.107.168.29",
    "213.199.34.251",
    "108.160.135.149",
    "167.235.146.99",
    "185.198.234.12",
    "87.228.24.64",
    "45.32.52.10",
    "91.107.204.136",
    "64.176.35.235",
    "167.179.90.255",
    "157.66.81.130",
    "157.10.199.125",
    "146.59.6.50",
    "46.254.241.8",
    "49.12.102.105",
    "134.255.182.189",
    "81.17.101.141",
    "65.108.74.79",
    "64.23.134.67",
    "54.69.95.118",
    "158.220.122.13",
    "49.13.154.121",
    "82.211.25.69",
    "75.119.149.9",
    "93.190.140.111",
    "93.190.140.114",
    "195.201.238.55",
    "135.181.110.216",
    "45.76.141.74",
    "109.199.123.11",
    "65.21.145.147",
    "50.116.28.103",
    "188.225.11.5",
    "188.245.90.255",
    "130.162.233.186",
    "65.109.65.126",
    "188.208.196.183",
    "38.242.224.124",
    "178.157.91.184",
    "37.60.236.201",
    "95.179.139.125",
    "213.199.34.248",
    "178.157.91.178",
    "213.199.35.18",
    "213.199.35.6",
    "65.21.147.225",
    "37.60.243.59",
    "37.27.67.156",
    "37.60.236.247",
    "159.69.204.162",
    "46.254.241.11",
    "173.199.71.83",
    "159.69.42.14",
    "185.215.166.126",
    "91.234.35.132",
    "157.66.81.218",
    "213.199.35.15",
    "114.132.172.215",
    "93.190.140.162",
    "65.108.74.109"
];

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
