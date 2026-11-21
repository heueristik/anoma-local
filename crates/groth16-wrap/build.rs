//! Builds the Go library in `go/ffi` as a C archive and links it.

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

const LIBRARY: &str = "groth16_wrap";

fn main() -> Result<(), Box<dyn Error>> {
    let go_directory =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("cargo sets no CARGO_MANIFEST_DIR")?)
            .join("go");
    let out_directory = PathBuf::from(env::var_os("OUT_DIR").ok_or("cargo sets no OUT_DIR")?);
    let go = env::var_os("GO").unwrap_or_else(|| "go".into());

    let status = Command::new(&go)
        .current_dir(&go_directory)
        .args(["build", "-buildmode=c-archive", "-trimpath", "-o"])
        .arg(out_directory.join(format!("lib{LIBRARY}.a")))
        .arg("./ffi")
        .env("CGO_ENABLED", "1")
        .status()
        .map_err(|error| {
            format!(
                "cannot run {}: {error}; install Go 1.26.8 or later, or set GO",
                go.display()
            )
        })?;
    if !status.success() {
        return Err(format!("go build in {} failed: {status}", go_directory.display()).into());
    }

    println!("cargo::rerun-if-changed=go");
    println!("cargo::rerun-if-env-changed=GO");
    println!(
        "cargo::rustc-link-search=native={}",
        out_directory.display()
    );
    println!("cargo::rustc-link-lib=static={LIBRARY}");
    Ok(())
}
