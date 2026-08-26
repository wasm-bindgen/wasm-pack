//! Support for downloading and executing `wasm-opt`

use crate::child;
use crate::install;
use crate::PBAR;
use anyhow::Result;
use binary_install::Cache;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// Execute `wasm-opt` over wasm binaries found in `out_dir`, downloading if
/// necessary into `cache`. Passes `args` to each invocation of `wasm-opt`.
pub fn run(cache: &Cache, out_dir: &Path, args: &[String], install_permitted: bool) -> Result<()> {
    let wasm_opt_path = match find_wasm_opt(cache, install_permitted)? {
        Some(path) => path,
        // `find_wasm_opt` will have already logged a message about this, so we don't need to here.
        None => return Ok(()),
    };

    PBAR.info("Optimizing wasm binaries with `wasm-opt`...");

    let mut input_paths = Vec::new();
    for file in out_dir.read_dir()? {
        let file = file?;
        let path = file.path();
        if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
            continue;
        }
        input_paths.push(path);
    }
    for path in input_paths {
        let tmp = path.with_extension("wasm-opt");
        let mut cmd = Command::new(&wasm_opt_path);
        cmd.arg(&path).arg("-o").arg(&tmp).args(args);
        child::run(cmd, "wasm-opt")?;
        std::fs::rename(&tmp, &path)?;
    }

    Ok(())
}

/// Attempts to find `wasm-opt` in `PATH` locally, or failing that downloads a
/// precompiled binary.
///
/// Returns `Some` if a binary was found or it was successfully downloaded.
/// Returns `None` if a binary wasn't found in `PATH` and this platform doesn't
/// have precompiled binaries. Returns an error if we failed to download the
/// binary.
pub fn find_wasm_opt(cache: &Cache, install_permitted: bool) -> Result<Option<PathBuf>> {
    // First attempt to look up in PATH. If found assume it works.
    if let Ok(path) = which::which("wasm-opt") {
        PBAR.info(&format!("found wasm-opt at {:?}", path));
        return Ok(Some(path));
    }

    match install::download_prebuilt(&install::Tool::WasmOpt, cache, "latest", install_permitted)? {
        install::Status::Found(download) => Ok(Some(download.binary("bin/wasm-opt")?)),
        install::Status::CannotInstall => {
            PBAR.info("Skipping wasm-opt as no downloading was requested");
            Ok(None)
        }
        install::Status::PlatformNotSupported => {
            PBAR.info("Skipping wasm-opt because it is not supported on this platform");
            Ok(None)
        }
    }
}
