//! Building a Rust crate into a `.wasm` binary.

use crate::child;
use crate::command::build::BuildProfile;
use crate::emoji;
use crate::manifest::Crate;
use crate::PBAR;
use anyhow::{anyhow, bail, Context, Result};
use cargo_metadata::Message;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;

pub mod wasm_target;

/// Used when comparing the currently installed
/// wasm-pack version with the latest on crates.io.
pub struct WasmPackVersion {
    /// The currently installed wasm-pack version.
    pub local: String,
    /// The latest version of wasm-pack that's released at
    /// crates.io.
    pub latest: String,
}

/// Ensure that `rustc` is present and that it is >= 1.30.0
pub fn check_rustc_version() -> Result<String> {
    let local_minor_version = rustc_minor_version();
    match local_minor_version {
        Some(mv) => {
            if mv < 30 {
                bail!(
                    "Your version of Rust, '1.{}', is not supported. Please install Rust version 1.30.0 or higher.",
                    mv.to_string()
                )
            } else {
                Ok(mv.to_string())
            }
        }
        None => bail!("We can't figure out what your Rust version is- which means you might not have Rust installed. Please install Rust version 1.30.0 or higher."),
    }
}

// from https://github.com/alexcrichton/proc-macro2/blob/79e40a113b51836f33214c6d00228934b41bd4ad/build.rs#L44-L61
fn rustc_minor_version() -> Option<u32> {
    macro_rules! otry {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => return None,
            }
        };
    }
    let output = otry!(Command::new("rustc").arg("--version").output().ok());
    let version = otry!(str::from_utf8(&output.stdout).ok());
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    otry!(pieces.next()).parse().ok()
}

/// Checks and returns local and latest versions of wasm-pack
pub fn check_wasm_pack_versions() -> Result<WasmPackVersion> {
    match wasm_pack_local_version() {
        Some(local) => Ok(WasmPackVersion {local, latest: Crate::return_wasm_pack_latest_version()?.unwrap_or_else(|| "".to_string())}),
        None => bail!("We can't figure out what your wasm-pack version is, make sure the installation path is correct.")
    }
}

fn wasm_pack_local_version() -> Option<String> {
    let output = env!("CARGO_PKG_VERSION");
    Some(output.to_string())
}

/// Returns true for tier-3 wasm targets that have no rustup-prebuilt sysroot
/// and must be built via `-Z build-std`. Currently this is the wasm64 family
/// (`wasm64-unknown-unknown`, future `wasm64-*` variants).
pub fn is_tier3_wasm(target_triple: &str) -> bool {
    target_triple.starts_with("wasm64")
}

/// Run `cargo build` for Wasm with config derived from the given `BuildProfile`.
pub fn cargo_build_wasm(
    path: &Path,
    profile: BuildProfile,
    extra_options: &[String],
    target_triple: &str,
    panic_unwind: bool,
) -> Result<String> {
    let msg = if panic_unwind {
        format!("{}Compiling to Wasm (with panic=unwind)...", emoji::CYCLONE)
    } else {
        format!("{}Compiling to Wasm...", emoji::CYCLONE)
    };
    PBAR.info(&msg);

    let mut cmd = Command::new("cargo");
    cmd.current_dir(path);
    // `+nightly` must be the first argument to cargo.
    if panic_unwind {
        cmd.arg("+nightly");
    }
    cmd.arg("build").arg("--lib");

    if PBAR.quiet() {
        cmd.arg("--quiet");
    }

    match profile {
        BuildProfile::Profiling => {
            // Once there are DWARF debug info consumers, force enable debug
            // info, because builds that use the release cargo profile disables
            // debug info.
            //
            // cmd.env("RUSTFLAGS", "-g");
            cmd.arg("--release");
        }
        BuildProfile::Release => {
            cmd.arg("--release");
        }
        BuildProfile::Dev => {
            // Plain cargo builds use the dev cargo profile, which includes
            // debug info by default.
        }
        BuildProfile::Custom(arg) => {
            cmd.arg("--profile").arg(arg);
        }
    }

    cmd.env("CARGO_BUILD_TARGET", target_triple);

    if panic_unwind {
        cmd.arg("-Z").arg("build-std=std,panic_unwind");

        // Append `-Cpanic=unwind` to any user-provided RUSTFLAGS.
        let existing = std::env::var("RUSTFLAGS").unwrap_or_default();
        let combined = if existing.is_empty() {
            "-Cpanic=unwind".to_string()
        } else {
            format!("{existing} -Cpanic=unwind")
        };
        cmd.env("RUSTFLAGS", combined);
    }

    cmd.args(absolutize_extra_options(extra_options)?);

    cmd.arg("--message-format=json");

    let mut cargo_process = cmd.stdout(Stdio::piped()).spawn()?;

    let final_artifact =
        Message::parse_stream(BufReader::new(cargo_process.stdout.as_mut().unwrap()))
            .filter_map(|msg| {
                match msg {
                    Ok(Message::CompilerArtifact(artifact)) => return Some(artifact),
                    Ok(Message::CompilerMessage(msg)) => eprintln!("{msg}"),
                    Ok(Message::TextLine(text)) => eprintln!("{text}"),
                    Err(err) => log::error!("Couldn't parse cargo message: {err}"),
                    _ => {} // ignore messages irrelevant to the user
                }
                None
            })
            .last();

    if !cargo_process
        .wait()
        .context("Failed to wait for cargo build process")?
        .success()
    {
        bail!("`cargo build` failed, see the output above for details");
    }

    let wasm_files: Vec<_> = final_artifact
        .context("Expected at least one compiler artifact in the output of `cargo build`")?
        .filenames
        .into_iter()
        .filter(|path| path.extension() == Some("wasm"))
        .collect();

    match <[_; 1]>::try_from(wasm_files) {
        Ok([filename]) => Ok(filename.into_string()),
        Err(filenames) => {
            bail!(
                "Expected exactly one .wasm file in the compiler artifact, but found {filenames:?}"
            )
        }
    }
}

/// The `cargo` command is executed inside the crate directory, so relative
/// paths in extra options won't resolve. Convert path-valued options to
/// absolute paths.
fn absolutize_extra_options(extra_options: &[String]) -> Result<Vec<String>> {
    let mut handle_path = false;
    extra_options
        .iter()
        .map(|option| -> Result<String> {
            let value = if handle_path && Path::new(option).is_relative() {
                std::env::current_dir()?
                    .join(option)
                    .to_str()
                    .ok_or_else(|| anyhow!("path contains non-UTF-8 characters"))?
                    .to_string()
            } else {
                option.to_string()
            };
            handle_path = matches!(&**option, "--target-dir" | "--out-dir" | "--manifest-path");
            Ok(value)
        })
        .collect::<Result<Vec<_>>>()
}

/// Run `cargo rustc` for the emscripten target.
///
/// The whole emscripten build is this one cargo invocation: rustc drives
/// `emcc` as the linker, and the injected `-Clink-arg=-sWASM_BINDGEN`
/// makes emcc run `wasm-bindgen` (resolved from `PATH`, hence
/// `bindgen_dir`) as a post-link step. `cargo rustc` is used rather than
/// `cargo build` because its trailing flags apply only to the final crate
/// and *compose* with any user rustflags from `.cargo/config.toml`, instead
/// of replacing them the way a `RUSTFLAGS` env var would.
///
/// Returns the paths of the emitted `.js` entry point and `.wasm` binary.
pub fn cargo_rustc_emscripten(
    path: &Path,
    profile: BuildProfile,
    extra_options: &[String],
    target_triple: &str,
    bin_name: &str,
    link_args: &[String],
    bindgen_dir: &Path,
    emcc_env: &crate::emscripten::EmccEnv,
) -> Result<(PathBuf, PathBuf)> {
    let msg = format!("{}Compiling to Wasm via emcc...", emoji::CYCLONE);
    PBAR.info(&msg);

    let mut cmd = Command::new("cargo");
    cmd.current_dir(path);
    cmd.arg("rustc").arg("--bin").arg(bin_name);

    if PBAR.quiet() {
        cmd.arg("--quiet");
    }

    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            cmd.arg("--release");
        }
        BuildProfile::Dev => {}
        BuildProfile::Custom(arg) => {
            cmd.arg("--profile").arg(arg);
        }
    }

    cmd.env("CARGO_BUILD_TARGET", target_triple);

    // emcc locates `wasm-bindgen` via PATH; prepend the version-matched CLI
    // wasm-pack installed, plus any emcc toolchain dirs from resolution.
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    let paths = std::iter::once(bindgen_dir.to_path_buf())
        .chain(emcc_env.path_prepends.iter().cloned())
        .chain(std::env::split_paths(&path_var))
        .collect::<Vec<_>>();
    cmd.env("PATH", std::env::join_paths(paths)?);
    for (key, value) in &emcc_env.vars {
        cmd.env(key, value);
    }

    cmd.args(absolutize_extra_options(extra_options)?);
    cmd.arg("--message-format=json");
    cmd.arg("--");
    if let Some(linker) = &emcc_env.linker {
        cmd.arg(format!("-Clinker={}", linker.display()));
    }
    cmd.args(link_args);

    let mut cargo_process = cmd.stdout(Stdio::piped()).spawn()?;

    let bin_artifact =
        Message::parse_stream(BufReader::new(cargo_process.stdout.as_mut().unwrap()))
            .filter_map(|msg| {
                match msg {
                    Ok(Message::CompilerArtifact(artifact)) if artifact.executable.is_some() => {
                        return Some(artifact)
                    }
                    Ok(Message::CompilerMessage(msg)) => eprintln!("{msg}"),
                    Ok(Message::TextLine(text)) => eprintln!("{text}"),
                    Err(err) => log::error!("Couldn't parse cargo message: {err}"),
                    _ => {}
                }
                None
            })
            .last();

    if !cargo_process
        .wait()
        .context("Failed to wait for cargo build process")?
        .success()
    {
        bail!("`cargo rustc` failed, see the output above for details");
    }

    let artifact =
        bin_artifact.context("Expected a binary artifact in the output of `cargo rustc`")?;
    let js_path: PathBuf = artifact.executable.as_ref().unwrap().clone().into();

    // The `.wasm` is emitted alongside the JS. Its filename follows the crate
    // name (underscores) rather than the bin name, so probe both.
    let wasm_path = artifact
        .filenames
        .iter()
        .find(|f| f.extension() == Some("wasm"))
        .map(|f| PathBuf::from(f.clone()))
        .or_else(|| {
            let sibling = js_path.with_extension("wasm");
            sibling.exists().then_some(sibling)
        })
        .or_else(|| {
            let sibling = js_path
                .parent()?
                .join(format!("{}.wasm", bin_name.replace('-', "_")));
            sibling.exists().then_some(sibling)
        })
        .context("Could not locate the .wasm emitted next to the emcc JS output")?;

    Ok((js_path, wasm_path))
}

/// Runs `cargo build --tests` targeting `wasm32-unknown-unknown`.
///
/// This generates the `Cargo.lock` file that we use in order to know which version of
/// wasm-bindgen-cli to use when running tests.
///
/// Note that the command to build tests and the command to run tests must use the same parameters, i.e. features to be
/// disabled / enabled must be consistent for both `cargo build` and `cargo test`.
///
/// # Parameters
///
/// * `path`: Path to the crate directory to build tests.
/// * `debug`: Whether to build tests in `debug` mode.
/// * `extra_options`: Additional parameters to pass to `cargo` when building tests.
/// * `target_triple`: The wasm target triple to build for (e.g.
///   `wasm32-unknown-unknown` or `wasm64-unknown-unknown`).
/// * `panic_unwind`: Whether to build tests with `panic=unwind` via the nightly
///   toolchain and `-Z build-std`.
pub fn cargo_build_wasm_tests(
    path: &Path,
    debug: bool,
    extra_options: &[String],
    target_triple: &str,
    panic_unwind: bool,
) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(path);
    // `+nightly` must be the first argument to cargo.
    if panic_unwind {
        cmd.arg("+nightly");
    }
    cmd.arg("build").arg("--tests");

    if PBAR.quiet() {
        cmd.arg("--quiet");
    }

    if !debug {
        cmd.arg("--release");
    }

    cmd.env("CARGO_BUILD_TARGET", target_triple);

    if panic_unwind {
        cmd.arg("-Z").arg("build-std=std,panic_unwind");

        let existing = std::env::var("RUSTFLAGS").unwrap_or_default();
        let combined = if existing.is_empty() {
            "-Cpanic=unwind".to_string()
        } else {
            format!("{existing} -Cpanic=unwind")
        };
        cmd.env("RUSTFLAGS", combined);
    }

    cmd.args(extra_options);

    child::run(cmd, "cargo build").context("Compilation of your program failed")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier3_wasm_detection() {
        assert!(is_tier3_wasm("wasm64-unknown-unknown"));
        assert!(is_tier3_wasm("wasm64-wasi"));
        assert!(!is_tier3_wasm("wasm32-unknown-unknown"));
        assert!(!is_tier3_wasm("wasm32-wasi"));
        assert!(!is_tier3_wasm("wasm32-unknown-emscripten"));
        assert!(!is_tier3_wasm("x86_64-unknown-linux-gnu"));
    }
}
