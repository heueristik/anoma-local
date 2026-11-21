//! On macOS, adds the library folder of OR-Tools to the search path of the binaries of this crate, so that the tests
//! load `libortools` from the prefix that `cp_sat` links against.

fn main() {
    println!("cargo:rerun-if-env-changed=ORTOOLS_PREFIX");
    let is_macos = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "macos");
    if let (true, Ok(prefix)) = (is_macos, std::env::var("ORTOOLS_PREFIX")) {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{prefix}/lib");
    }
}
