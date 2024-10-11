use platform_value::Value;
use serde::Serialize;
use serde_json::to_string;
use sha2::{Sha256, Digest};

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Value_eq(a: Value, b: Value) -> bool {
    a == b
}

#[allow(non_snake_case)]
#[ferment_macro::export]
pub fn Value_hash(a: Value) -> i32 {
    let serialized = to_string(&a).unwrap();
    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Process the serialized JSON string
    hasher.update(serialized.as_bytes());

    // Acquire the resulting hash as a byte array
    let hash = hasher.finalize();

    // Extract the first four bytes and convert to an i32
    let first_four_bytes = &hash[0..4];
    let result = i32::from_be_bytes(first_four_bytes.try_into().expect("slice with incorrect length"));

    result
}

#[test]
fn test() {
    let a = Value::I8(1);
    tracing::info!("hashcode {} = {}", a, Value_hash(a.clone()))
}