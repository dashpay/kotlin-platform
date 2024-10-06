extern crate cbindgen;
extern crate ferment;
use std::env;
use std::process::Command;
use ferment::{Builder, Crate};

pub const SELF_NAME: &str = "rs_sdk_bindings";
fn main() {

   let target_triple = env::var("TARGET").expect("TARGET environment variable is not set");
   let c_header = format!("target/{}/{}.h", target_triple, SELF_NAME);
   match Builder::new(Crate::current_with_name(SELF_NAME))
       .with_mod_name("fermented")
       .with_crates(
          vec![
             "platform-value",
             "dpp",
             "drive",
             "drive-proof-verifier",
             "dash-sdk",
             "rs-dapi-client",
              "platform-mobile"
          ]
       )
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header.as_str()])
          .status() {
         Ok(status) => println!("[cbindgen] [ok] generated into {c_header} with status: {status}"),
         Err(err) => panic!("[cbindgen] [error] {err}")
      }
      Err(err) => panic!("[ferment] Can't create FFI expansion: {err}")
   }
}