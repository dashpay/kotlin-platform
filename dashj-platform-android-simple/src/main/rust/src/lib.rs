//mod fermented;

use std::env;

pub fn get_env() {
    env::var("CARGO_FEATURE_STD").unwrap();
}

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn mymain() -> io::Result<()> {
    let path = Path::new("hello.txt");
    let message = "Hello, Rustaceans!";

    // Writing to a file
    {
        let mut file = File::create(&path)?;
        file.write_all(message.as_bytes())?;
    }

    // Reading from a file
    {
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("File contents: {}", contents);
    }

    // Clean up
    std::fs::remove_file(&path)?;

    Ok(())
}
//use rs_sdk::dpp::identity::IdentityV0;
//use rs_sdk::platform::Identity;

//pub fn get_default_identity() -> Identity {
//    Identity::V0(IdentityV0::default())
//}

// use bls_signatures::PrivateKey;
// pub fn get_secret_key() -> PrivateKey {
//     PrivateKey::FromBytes();
// }