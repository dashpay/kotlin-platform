extern crate cbindgen;
extern crate ferment_sys;

fn main() {
    const SELF_NAME: &str = "dash_sdk_bindings";
    match ferment_sys::Ferment::with_crate_name(SELF_NAME)
        .with_default_mod_name()
        .with_cbindgen_config_from_file("cbindgen.toml")
        .with_external_crates(vec![
            "platform-value",
            "dpp",
            "drive",
            "dapi-grpc",
            "drive-proof-verifier",
            "dash-sdk",
            "rs-dapi-client",
            "platform-mobile"
        ])
        .generate() {
        Ok(_) => println!("[ferment] [ok]: {SELF_NAME}"),
        Err(err) => panic!("[ferment] [err]: {}", err)
    }
}