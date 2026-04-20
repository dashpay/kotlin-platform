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
    "68.67.122.1",
    "68.67.122.2",
    "68.67.122.3",
    "68.67.122.4",
    "68.67.122.5",
    "68.67.122.6",
    "68.67.122.7",
    "68.67.122.8",
    "68.67.122.9",
    "68.67.122.10",
    "68.67.122.11",
    "68.67.122.12",
    "68.67.122.13",
    "68.67.122.14",
    "68.67.122.15",
    "68.67.122.16",
    "68.67.122.17",
    "68.67.122.18",
    "68.67.122.19",
    "68.67.122.20",
    "68.67.122.21",
    "68.67.122.22",
    "68.67.122.23",
    "68.67.122.24",
    "68.67.122.25",
    "68.67.122.26",
    "68.67.122.27",
    "68.67.122.28",
    "68.67.122.29"
];

const MAINNET_ADDRESS_LIST: [&str; 152] = [
    "216.238.75.46",
    "149.28.241.190",
    "194.163.172.206",
    "89.125.209.110",
    "84.247.180.201",
    "134.255.182.186",
    "93.115.172.39",
    "5.189.164.253",
    "161.97.165.115",
    "89.117.57.27",
    "62.171.133.125",
    "38.242.197.189",
    "95.111.241.20",
    "66.245.196.52",
    "173.212.245.118",
    "202.71.14.79",
    "185.252.234.238",
    "178.215.237.134",
    "207.180.233.6",
    "89.125.209.69",
    "136.244.99.17",
    "173.212.232.90",
    "178.215.237.135",
    "37.60.244.253",
    "157.90.238.161",
    "213.199.54.171",
    "84.247.180.198",
    "132.243.197.252",
    "161.97.75.36",
    "23.88.63.58",
    "207.244.247.40",
    "45.32.70.131",
    "31.220.91.43",
    "52.33.9.172",
    "178.157.91.177",
    "161.97.88.199",
    "109.199.124.30",
    "185.198.234.17",
    "139.84.236.208",
    "38.242.218.26",
    "207.180.224.96",
    "62.171.170.222",
    "65.108.4.213",
    "103.214.68.30",
    "75.119.153.10",
    "31.220.91.60",
    "65.108.74.95",
    "132.243.197.251",
    "95.179.159.65",
    "65.109.114.211",
    "44.240.99.214",
    "5.75.133.148",
    "65.108.0.153",
    "213.199.54.37",
    "161.97.88.219",
    "89.35.131.149",
    "192.248.178.237",
    "161.97.153.122",
    "37.60.235.218",
    "207.180.241.242",
    "195.26.254.228",
    "45.76.36.241",
    "45.77.11.194",
    "139.84.232.129",
    "161.97.175.233",
    "104.156.154.179",
    "89.125.209.195",
    "161.97.176.38",
    "91.198.108.35",
    "161.97.180.105",
    "109.199.105.14",
    "213.199.54.34",
    "65.108.246.145",
    "64.176.10.71",
    "158.247.247.241",
    "37.60.254.213",
    "65.108.0.151",
    "149.102.140.101",
    "139.180.143.115",
    "37.60.235.205",
    "213.199.44.112",
    "51.79.109.200",
    "213.199.53.161",
    "162.212.35.100",
    "185.239.209.6",
    "37.27.67.154",
    "173.212.196.214",
    "37.60.254.202",
    "134.255.182.185",
    "91.199.149.177",
    "51.79.109.182",
    "206.245.167.76",
    "31.220.77.84",
    "139.84.137.143",
    "144.91.87.82",
    "161.97.120.156",
    "161.97.91.217",
    "132.243.197.249",
    "62.171.168.44",
    "65.109.114.215",
    "194.163.159.171",
    "80.240.19.200",
    "144.126.141.62",
    "173.249.21.12",
    "161.97.159.172",
    "194.163.156.190",
    "139.84.170.10",
    "164.68.118.37",
    "62.171.171.253",
    "143.198.145.184",
    "84.247.180.200",
    "65.109.108.141",
    "37.60.234.205",
    "65.108.4.196",
    "43.133.171.101",
    "192.248.175.198",
    "65.21.237.225",
    "161.97.142.210",
    "43.167.244.109",
    "146.59.45.235",
    "37.27.67.159",
    "172.236.244.81",
    "62.171.144.192",
    "193.70.43.172",
    "84.247.180.190",
    "185.215.164.84",
    "75.119.138.9",
    "172.238.7.25",
    "157.173.122.20",
    "57.128.212.163",
    "82.208.20.153",
    "95.111.239.54",
    "85.190.243.3",
    "51.195.235.166",
    "156.67.29.45",
    "132.243.197.38",
    "93.115.172.37",
    "15.235.102.216",
    "65.109.108.138",
    "89.35.131.39",
    "93.115.172.38",
    "161.97.104.37",
    "75.119.128.71",
    "93.95.115.187",
    "161.97.117.125",
    "49.13.28.255",
    "95.111.242.220",
    "52.36.102.91",
    "139.99.201.103",
    "65.109.108.139",
    "109.199.120.79",
    "161.97.74.173"
];

#[test]
fn check_all_testnet_nodes_test() {

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
    assert!(bad_nodes.len() < good_nodes.len(), "not enough good nodes")
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
    assert!(bad_nodes.len() < good_nodes.len(), "not enough good nodes")
}
