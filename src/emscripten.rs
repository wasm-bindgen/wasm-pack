//! Locating or installing the Emscripten toolchain (`emcc`).
//!
//! The emscripten build needs `emcc` resolvable by rustc (which drives it as
//! the linker). Resolution order:
//!
//!   1. `emcc` already on `PATH` — the user's toolchain wins, untouched.
//!   2. `$EMSDK` pointing at an installed + activated emsdk.
//!   3. `~/emsdk` (the conventional install location).
//!   4. A wasm-pack-managed install in the wasm-pack cache.
//!   5. With installation permitted (and confirmed on a tty), install the
//!      pinned emsdk SDK into the wasm-pack cache.
//!
//! All environment adjustments are process-scoped: they apply only to the
//! `cargo` child wasm-pack spawns, so no shell activation (`emsdk_env.sh`)
//! is ever required.

use crate::PBAR;
use anyhow::{anyhow, bail, Context, Result};
use binary_install::Cache;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The emsdk checkout driving the install.
const EMSDK_REF: &str = "6.0.9";

/// The SDK `emsdk install` fetches into the wasm-pack cache: LLVM, binaryen,
/// emscripten, node, and (on Windows) python, all prebuilt from one
/// emscripten-releases revision. Either a release version or an
/// emscripten-releases commit hash (a tip-of-tree build).
// TODO: switch to "6.0.10" once tagged; this tot build carries the post-link
// `-sWASM_BINDGEN` support (emscripten-core/emscripten#27208) it will ship.
const EMSDK_SDK: &str = "dc4dcf8e7b4ef9ed84c8b72e73ec1d1c3714ea31";

/// Stamp file marking a completed install.
const READY_STAMP: &str = ".wasm-pack-ready";

/// Environment adjustments for spawning `cargo` such that rustc can drive
/// `emcc` as the linker.
#[derive(Default)]
pub struct EmccEnv {
    /// Directories to prepend to `PATH`.
    pub path_prepends: Vec<PathBuf>,
    /// Extra environment variables (`EM_CONFIG`, `EMSDK`).
    pub vars: Vec<(&'static str, PathBuf)>,
    /// The emcc entry point to pass as `-Clinker`, when resolved from an
    /// emsdk rather than left to rustc's default PATH lookup.
    pub linker: Option<PathBuf>,
}

/// Locate `emcc`, installing the toolchain into the wasm-pack cache if
/// permitted and confirmed.
pub fn ensure_emcc(cache: &Cache, install_permitted: bool) -> Result<EmccEnv> {
    // 1. The user's own emcc.
    if which::which("emcc").is_ok() {
        return Ok(EmccEnv::default());
    }

    // 2. / 3. An existing emsdk install (activated), via $EMSDK or ~/emsdk.
    let candidates = std::env::var_os("EMSDK")
        .map(PathBuf::from)
        .into_iter()
        .chain(dirs::home_dir().map(|home| home.join("emsdk")));
    for dir in candidates {
        if let Some(env) = emsdk_env(&dir) {
            PBAR.info(&format!("Using the Emscripten SDK at {}", dir.display()));
            return Ok(env);
        }
    }

    // 4. / 5. The wasm-pack-managed install.
    let emsdk_dir = cache.join(Path::new(&format!("emsdk-{}", EMSDK_SDK)));
    if !emsdk_ready(&emsdk_dir) {
        if !install_permitted {
            bail!(
                "Targeting wasm32-unknown-emscripten requires `emcc` (the Emscripten \
                 compiler driver), which was not found on PATH, and installation is \
                 disabled (--mode no-install).\n{}",
                manual_install_instructions(),
            );
        }
        confirm_install()?;
        install(&emsdk_dir)?;
    }
    emsdk_env(&emsdk_dir).ok_or_else(|| {
        anyhow!(
            "the emsdk install at {} is not usable; delete it to reinstall",
            emsdk_dir.display()
        )
    })
}

/// Whether the wasm-pack-managed install previously completed (stamp plus
/// key artifact, so a broken install re-runs).
fn emsdk_ready(emsdk_dir: &Path) -> bool {
    emsdk_dir.join(READY_STAMP).exists() && emsdk_dir.join(".emscripten").exists()
}

/// The emcc entry point in `dir`. Passed to rustc explicitly because its
/// default linker name on Windows is `emcc.bat`, whereas prebuilt SDKs ship
/// a pylauncher `emcc.exe` (`.bat` entry points are opt-in at bootstrap).
fn emcc_entry_point(dir: &Path) -> Option<PathBuf> {
    let names: &[&str] = if cfg!(windows) {
        &["emcc.bat", "emcc.exe"]
    } else {
        &["emcc"]
    };
    names.iter().map(|name| dir.join(name)).find(|p| p.exists())
}

/// Build the child-process environment for an activated emsdk directory.
fn emsdk_env(emsdk_dir: &Path) -> Option<EmccEnv> {
    let config = emsdk_dir.join(".emscripten");
    if !config.exists() {
        return None;
    }
    let emcc_dir = emsdk_dir.join("upstream").join("emscripten");
    let linker = emcc_entry_point(&emcc_dir)?;
    let mut path_prepends = vec![emcc_dir];
    let mut vars = vec![
        ("EM_CONFIG", config.clone()),
        ("EMSDK", emsdk_dir.to_path_buf()),
    ];
    // node (and on Windows, python) come from the emsdk; emcc resolves them
    // via the config, but the JS tooling it shells out to needs them on PATH.
    for key in ["NODE_JS", "PYTHON"] {
        if let Some(tool) = config_tool_path(emsdk_dir, &config, key) {
            if let Some(bin_dir) = tool.parent() {
                path_prepends.push(bin_dir.to_path_buf());
            }
            // The emcc launchers resolve python via EMSDK_PYTHON before
            // falling back to PATH.
            if key == "PYTHON" {
                vars.push(("EMSDK_PYTHON", tool));
            }
        }
    }
    Some(EmccEnv {
        path_prepends,
        vars,
        linker: Some(linker),
    })
}

/// Parse a tool path (e.g. `NODE_JS = '$CFGDIR/node/…/bin/node'`) out of an
/// emsdk-generated `.emscripten` config.
fn config_tool_path(emsdk_dir: &Path, config: &Path, key: &str) -> Option<PathBuf> {
    let body = std::fs::read_to_string(config).ok()?;
    let line = body.lines().find(|l| {
        l.trim_start()
            .strip_prefix(key)
            .is_some_and(|rest| rest.trim_start().starts_with('='))
    })?;
    let value = line.split('\'').nth(1)?;
    let path = match value
        .strip_prefix("$CFGDIR/")
        .or_else(|| value.strip_prefix("$CFGDIR\\"))
    {
        Some(rel) => emsdk_dir.join(rel),
        None => PathBuf::from(value),
    };
    Some(path)
}

/// One-time confirmation before the large toolchain download. Skipped (with
/// a notice) when unattended, so CI proceeds unprompted.
fn confirm_install() -> Result<()> {
    let msg = format!(
        "wasm-pack can install the Emscripten SDK ({}) into its cache (a one-time ~1.3 GB download)",
        EMSDK_SDK
    );
    if !console::user_attended() {
        PBAR.info(&format!("{}; installing...", msg));
        return Ok(());
    }
    let confirmed = dialoguer::Confirm::new()
        .with_prompt(format!("{}. Proceed?", msg))
        .default(true)
        .interact()?;
    if !confirmed {
        bail!(
            "Emscripten SDK installation declined.\n{}",
            manual_install_instructions()
        );
    }
    Ok(())
}

fn manual_install_instructions() -> String {
    format!(
        "To install manually:\n\n\
         \tgit clone https://github.com/emscripten-core/emsdk\n\
         \tcd emsdk\n\
         \t./emsdk install {sdk}\n\
         \t./emsdk activate {sdk}\n\
         \tsource ./emsdk_env.sh\n",
        sdk = EMSDK_SDK,
    )
}

/// Install the pinned emsdk SDK into the cache. Idempotent: stamped on
/// completion and re-run from scratch if incomplete.
fn install(emsdk_dir: &Path) -> Result<()> {
    let python = python_bin()?;
    clone_fresh(
        "https://github.com/emscripten-core/emsdk",
        EMSDK_REF,
        emsdk_dir,
    )?;
    PBAR.info("Installing the Emscripten toolchain (this downloads ~1.3 GB)...");
    for step in ["install", "activate"] {
        let mut cmd = Command::new(&python);
        cmd.arg(emsdk_dir.join("emsdk.py"))
            .arg(step)
            .arg(EMSDK_SDK)
            .current_dir(emsdk_dir);
        crate::child::run(cmd, "emsdk")
            .with_context(|| format!("running `emsdk {} {}`", step, EMSDK_SDK))?;
    }
    std::fs::write(emsdk_dir.join(READY_STAMP), "")?;
    PBAR.info("Emscripten toolchain installed.");
    Ok(())
}

/// Shallow-clone `git_ref` of `repo` into `dir`, clearing any partial
/// previous attempt.
fn clone_fresh(repo: &str, git_ref: &str, dir: &Path) -> Result<()> {
    if dir.exists() {
        std::fs::remove_dir_all(dir)?;
    }
    let mut cmd = Command::new("git");
    cmd.arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("-b")
        .arg(git_ref)
        .arg(repo)
        .arg(dir);
    crate::child::run(cmd, "git")
        .with_context(|| format!("cloning {} (is `git` installed and on PATH?)", repo))
}

/// A python interpreter for driving emsdk.py.
fn python_bin() -> Result<PathBuf> {
    which::which("python3")
        .or_else(|_| which::which("python"))
        .map_err(|_| {
            anyhow!(
                "Installing the Emscripten SDK requires `python3` on PATH.\n{}",
                manual_install_instructions()
            )
        })
}
