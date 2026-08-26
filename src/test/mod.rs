//! Testing a Rust crate compiled to wasm.

pub mod webdriver;

use crate::child;
use crate::PBAR;
use anyhow::{Context, Result};
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

/// Run `cargo test` targeting `wasm32-unknown-unknown`.
pub fn cargo_test_wasm<I, K, V>(
    path: &Path,
    release: bool,
    envs: I,
    extra_options: &[String],
    target_triple: &str,
) -> Result<()>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut cmd = Command::new("cargo");

    cmd.envs(envs);
    cmd.current_dir(path).arg("test");

    if PBAR.quiet() {
        cmd.arg("--quiet");
    }

    if release {
        cmd.arg("--release");
    }

    cmd.arg("--target").arg(target_triple);

    cmd.args(extra_options);

    child::run(cmd, "cargo test").context("Running Wasm tests with wasm-bindgen-test failed")?;

    // NB: `child::run` took care of ensuring that test output gets printed.
    Ok(())
}
