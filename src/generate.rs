//! Functionality related to running `cargo-generate`.

use crate::child;
use crate::emoji;
use crate::install::{self, Tool};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Default git repository used by `wasm-pack new`.
pub const DEFAULT_TEMPLATE: &str = "https://github.com/wasm-bindgen/wasm-pack";

/// Marker passed when the user selects `wasm-pack new --emscripten`.
///
/// Same repository as `DEFAULT_TEMPLATE`, but `generate()` extracts the
/// emscripten-specific subdirectory via cargo-generate's `--subfolder`
/// option so users don't have to know the layout.
pub const EMSCRIPTEN_TEMPLATE: &str = "https://github.com/wasm-bindgen/wasm-pack";

/// Subfolder inside `EMSCRIPTEN_TEMPLATE` that holds the emscripten template.
const EMSCRIPTEN_SUBFOLDER: &str = "wasm-pack-emscripten-template";

/// Run `cargo generate` in the current directory to create a new
/// project from a template.
///
/// When `emscripten` is true, `--subfolder` selects the emscripten template
/// from the multi-template wasm-pack repo.
pub fn generate(
    template: &str,
    name: &str,
    emscripten: bool,
    install_status: &install::Status,
) -> Result<()> {
    let bin_path = install::get_tool_path(install_status, Tool::CargoGenerate)?
        .binary(&Tool::CargoGenerate.to_string())?;
    let mut cmd = Command::new(&bin_path);
    cmd.arg("generate");
    if Path::new(template).exists() {
        cmd.arg("--path").arg(template);
    } else {
        cmd.arg("--git").arg(template);
    }
    cmd.arg("--name").arg(name);
    // `SUBFOLDER` is a positional in cargo-generate's CLI; it must come
    // after all named args.
    if emscripten {
        cmd.arg(EMSCRIPTEN_SUBFOLDER);
    }

    println!(
        "{} Generating a new rustwasm project with name '{}'...",
        emoji::SHEEP,
        name
    );
    child::run(cmd, "cargo-generate").context("Running cargo-generate")?;
    Ok(())
}
