#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use anyhow::Context;

/// Reads a file of the `one_input` fixture, which `tests/fixtures/one_input/prove.sh` writes.
pub fn fixture(name: &str) -> anyhow::Result<Vec<u8>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/one_input")
        .join(name);
    fs::read(&path).with_context(|| format!("read the fixture file {}", path.display()))
}

/// Returns a directory for keys below the cargo directory for test files.
pub fn keys_directory(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name)
}
