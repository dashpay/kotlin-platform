pub mod documents;

//use drive_proof_verifier::ContextProvider;
use platform_value::types::binary_data::BinaryData;
use dash_sdk::platform::types::identity::PublicKeyHash;
//use dpp::bincode::{Decode, Encode};
use platform_value::{Value};

#[ferment_macro::export]
pub fn convert_to_pkh(pkh: [u8; 20]) -> PublicKeyHash {
    PublicKeyHash(pkh)
}

#[ferment_macro::export]
pub fn get_binary_data_empty() -> BinaryData {
    BinaryData::new(vec![])
}
#[ferment_macro::export]
pub fn get_binary_data_4() -> BinaryData {
    BinaryData(vec![0, 1, 2, 3])
}

#[ferment_macro::export]
pub fn get_platform_value_bool(value: bool) -> Value {
    Value::Bool(value)
}

#[ferment_macro::export]
pub fn get_platform_value_with_map() ->  Value {
    Value::Map(vec![(Value::Text("key".to_string()), Value::I32(4))])
}