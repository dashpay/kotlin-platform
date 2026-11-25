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
use crate::provider::DataContractCache;
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
    data_contract_cache: Arc<DataContractCache>,
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
    let mut sdk = create_dash_sdk_using_single_evonode("68.67.122.29".into(), 0, 0, true);
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

// Refreshed 2026-11-24 from a synced mainnet Dash Core: all 362 valid Evo nodes with Platform HTTPS on :443. See docs/updating-evonode-lists.md.
const MAINNET_ADDRESS_LIST: [&str; 362] = [
    "2.31.109.195",
    "2.59.156.192",
    "5.39.27.224",
    "5.75.133.148",
    "5.78.73.117",
    "5.161.103.48",
    "5.189.145.80",
    "5.189.151.7",
    "5.189.164.253",
    "5.199.166.45",
    "5.199.166.140",
    "5.199.166.218",
    "15.235.102.215",
    "15.235.102.216",
    "23.88.63.58",
    "31.220.84.93",
    "31.220.91.43",
    "31.220.91.60",
    "34.210.238.67",
    "37.27.67.154",
    "37.27.67.156",
    "37.27.67.159",
    "37.27.67.163",
    "37.27.67.164",
    "37.27.83.17",
    "37.60.234.168",
    "37.60.234.205",
    "37.60.235.24",
    "37.60.235.39",
    "37.60.235.89",
    "37.60.235.205",
    "37.60.235.224",
    "37.60.236.230",
    "37.60.243.59",
    "37.60.244.35",
    "37.60.244.87",
    "37.60.244.253",
    "37.60.246.228",
    "37.60.254.201",
    "37.60.254.202",
    "37.60.254.205",
    "37.60.254.213",
    "37.60.255.160",
    "38.242.197.189",
    "38.242.200.227",
    "38.242.206.56",
    "38.242.206.103",
    "38.242.218.26",
    "43.133.171.101",
    "43.167.239.145",
    "43.167.244.109",
    "44.240.99.214",
    "45.76.36.241",
    "45.76.141.74",
    "45.85.147.192",
    "45.94.58.58",
    "45.135.180.70",
    "45.135.180.79",
    "45.135.180.114",
    "45.135.180.130",
    "46.62.250.247",
    "48.103.145.105",
    "49.12.102.105",
    "49.13.28.255",
    "49.13.193.251",
    "49.13.237.193",
    "50.116.28.103",
    "51.68.3.29",
    "51.68.35.98",
    "51.77.216.185",
    "51.79.109.182",
    "51.161.34.134",
    "52.10.213.198",
    "52.33.9.172",
    "52.36.102.91",
    "54.69.95.118",
    "57.128.212.163",
    "57.131.28.197",
    "62.171.136.93",
    "62.171.136.245",
    "62.171.138.186",
    "62.171.144.192",
    "62.171.145.24",
    "62.171.145.234",
    "62.171.150.24",
    "62.171.168.4",
    "62.171.168.44",
    "62.171.171.253",
    "62.171.174.123",
    "62.171.176.79",
    "62.171.178.71",
    "62.171.189.0",
    "63.186.97.17",
    "64.176.10.71",
    "64.176.35.235",
    "65.21.237.222",
    "65.21.237.223",
    "65.21.237.224",
    "65.21.237.225",
    "65.21.237.231",
    "65.108.0.149",
    "65.108.0.150",
    "65.108.0.151",
    "65.108.0.153",
    "65.108.0.158",
    "65.108.4.194",
    "65.108.4.195",
    "65.108.4.196",
    "65.108.4.213",
    "65.108.74.75",
    "65.108.74.78",
    "65.108.74.79",
    "65.108.74.95",
    "65.108.74.109",
    "65.108.246.145",
    "65.109.84.201",
    "65.109.84.202",
    "65.109.84.203",
    "65.109.84.204",
    "65.109.108.138",
    "65.109.108.139",
    "65.109.108.141",
    "65.109.108.142",
    "65.109.108.150",
    "65.109.114.210",
    "65.109.114.211",
    "65.109.114.212",
    "65.109.114.215",
    "65.109.114.236",
    "66.29.147.83",
    "66.245.196.52",
    "70.34.206.123",
    "72.60.38.160",
    "72.62.58.108",
    "75.119.128.71",
    "75.119.137.66",
    "75.119.138.9",
    "75.119.152.219",
    "75.119.153.10",
    "75.119.156.254",
    "77.42.74.68",
    "77.105.61.11",
    "77.105.61.24",
    "77.105.61.44",
    "77.105.61.45",
    "77.105.61.79",
    "77.105.61.142",
    "77.105.61.222",
    "77.105.61.233",
    "77.105.61.237",
    "77.221.148.204",
    "80.87.206.28",
    "80.87.206.169",
    "80.240.19.200",
    "84.32.22.20",
    "84.32.22.70",
    "84.32.22.141",
    "84.32.22.199",
    "84.32.54.119",
    "84.32.54.120",
    "84.32.54.121",
    "84.247.180.190",
    "84.247.180.198",
    "84.247.180.201",
    "84.247.187.76",
    "85.190.243.3",
    "86.107.168.78",
    "87.228.24.64",
    "88.216.191.0",
    "88.216.191.113",
    "88.216.191.114",
    "88.216.191.115",
    "89.35.131.23",
    "89.35.131.39",
    "89.35.131.149",
    "89.35.131.158",
    "89.35.131.218",
    "89.35.131.219",
    "89.125.50.14",
    "89.125.50.206",
    "89.125.209.27",
    "89.125.209.69",
    "89.125.209.106",
    "89.125.209.110",
    "89.125.209.120",
    "89.125.209.133",
    "89.125.209.152",
    "89.125.209.156",
    "89.125.209.178",
    "89.125.209.193",
    "89.125.209.195",
    "89.125.209.234",
    "91.198.108.35",
    "91.198.108.36",
    "91.198.108.37",
    "91.198.108.38",
    "91.199.149.177",
    "93.115.172.36",
    "93.115.172.37",
    "93.115.172.38",
    "93.115.172.39",
    "95.111.232.18",
    "95.111.233.139",
    "95.111.242.220",
    "95.179.139.125",
    "95.179.159.65",
    "95.179.241.182",
    "95.216.146.18",
    "103.214.68.30",
    "103.214.68.131",
    "104.156.154.179",
    "108.61.165.170",
    "109.123.244.131",
    "109.173.198.67",
    "109.199.97.144",
    "109.199.105.14",
    "109.199.107.155",
    "109.199.108.234",
    "109.199.109.39",
    "109.199.120.79",
    "109.199.124.30",
    "109.199.124.135",
    "114.132.172.215",
    "130.162.233.186",
    "130.185.118.3",
    "132.243.197.16",
    "132.243.197.18",
    "132.243.197.38",
    "132.243.197.40",
    "132.243.197.210",
    "132.243.197.249",
    "132.243.197.250",
    "132.243.197.251",
    "132.243.197.252",
    "134.255.182.185",
    "134.255.182.186",
    "135.181.110.216",
    "135.181.229.49",
    "136.244.99.17",
    "137.74.166.122",
    "139.84.137.143",
    "139.84.170.10",
    "139.84.231.221",
    "139.84.232.129",
    "139.84.236.208",
    "139.99.201.103",
    "139.180.143.115",
    "142.44.136.69",
    "143.198.145.184",
    "144.91.99.105",
    "144.91.105.209",
    "144.91.121.81",
    "149.28.241.190",
    "149.28.247.165",
    "149.102.140.101",
    "151.244.85.45",
    "151.244.85.114",
    "151.244.85.131",
    "151.244.85.153",
    "155.133.26.122",
    "155.138.159.73",
    "156.67.29.45",
    "157.90.238.161",
    "157.173.122.20",
    "157.173.122.21",
    "157.173.122.25",
    "157.173.122.26",
    "158.220.101.10",
    "158.220.101.28",
    "158.247.247.241",
    "159.69.204.162",
    "161.97.66.31",
    "161.97.70.66",
    "161.97.74.173",
    "161.97.75.36",
    "161.97.85.159",
    "161.97.85.182",
    "161.97.88.199",
    "161.97.88.219",
    "161.97.91.68",
    "161.97.91.217",
    "161.97.96.120",
    "161.97.102.156",
    "161.97.106.14",
    "161.97.117.125",
    "161.97.120.156",
    "161.97.142.210",
    "161.97.153.122",
    "161.97.159.172",
    "161.97.163.75",
    "161.97.165.115",
    "161.97.166.49",
    "161.97.173.67",
    "161.97.175.233",
    "161.97.176.38",
    "161.97.180.182",
    "162.212.35.100",
    "167.86.77.58",
    "167.86.93.21",
    "167.86.94.138",
    "167.88.169.16",
    "167.235.102.194",
    "168.119.102.10",
    "169.58.206.63",
    "172.236.244.81",
    "172.238.7.25",
    "173.199.71.83",
    "173.212.196.214",
    "173.212.231.230",
    "173.212.232.90",
    "173.212.245.118",
    "173.212.251.130",
    "173.249.21.12",
    "177.93.141.60",
    "178.18.244.255",
    "178.18.254.136",
    "185.50.195.156",
    "185.162.233.16",
    "185.162.233.17",
    "185.162.233.20",
    "185.162.233.21",
    "185.162.233.24",
    "185.162.233.25",
    "185.162.233.34",
    "185.166.217.154",
    "185.185.80.117",
    "185.198.234.12",
    "185.202.236.58",
    "185.209.230.13",
    "185.215.164.84",
    "185.215.164.186",
    "185.215.166.126",
    "185.239.208.110",
    "185.239.209.6",
    "185.252.234.238",
    "188.245.90.255",
    "192.99.153.216",
    "192.175.127.198",
    "192.248.175.198",
    "192.248.178.237",
    "193.70.43.172",
    "193.104.58.62",
    "194.163.166.76",
    "195.26.254.228",
    "195.201.238.55",
    "202.71.14.79",
    "206.245.167.74",
    "206.245.167.76",
    "207.180.213.141",
    "207.180.224.96",
    "207.180.233.6",
    "207.180.233.246",
    "207.244.247.40",
    "213.199.44.112",
    "213.199.53.161",
    "213.199.54.34",
    "213.199.54.35",
    "213.199.54.37",
    "213.199.54.171",
    "216.238.75.46",
    "216.238.99.9",
    "217.76.54.175"
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
