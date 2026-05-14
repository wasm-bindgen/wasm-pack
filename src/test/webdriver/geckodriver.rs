use super::get_and_notify;
use crate::install::InstallMode;
use crate::stamps;
use crate::target;
use anyhow::{anyhow, bail, Context, Result};
use binary_install::Cache;
use chrono::DateTime;
use std::path::PathBuf;

// Keep it up to date with each `wasm-pack` release.
// https://github.com/mozilla/geckodriver/releases/latest
const DEFAULT_GECKODRIVER_VERSION: &str = "v0.36.0";
const DEFAULT_WINDOWS_GECKODRIVER_VERSION: &str = "v0.24.0";

const GECKODRIVER_LAST_UPDATED_STAMP: &str = "geckodriver_last_updated";
const GECKODRIVER_VERSION_STAMP: &str = "geckodriver_version";

/// Get the path to an existing `geckodriver`, or install it if no existing
/// binary is found or if there is a new binary version.
pub fn get_or_install_geckodriver(cache: &Cache, mode: InstallMode) -> Result<PathBuf> {
    // geckodriver Windows binaries >v0.24.0 have an additional
    // runtime dependency that we cannot be sure is present on the
    // user's machine
    //
    // https://github.com/mozilla/geckodriver/issues/1617
    //
    // until this is resolved, always install v0.24.0 on windows
    if !target::WINDOWS {
        if let Ok(path) = which::which("geckodriver") {
            log::info!("[geckodriver] Found geckodriver at {:?}", path);
            return Ok(path);
        }
    }
    install_geckodriver(cache, mode.install_permitted())
}

/// Download and install a pre-built `geckodriver` binary.
pub fn install_geckodriver(cache: &Cache, installation_allowed: bool) -> Result<PathBuf> {
    let (target, ext) = if target::LINUX && target::x86 {
        ("linux32", "tar.gz")
    } else if target::LINUX && target::x86_64 {
        ("linux64", "tar.gz")
    } else if target::LINUX && target::aarch64 {
        ("linux-aarch64", "tar.gz")
    } else if target::MACOS && target::x86_64 {
        ("macos", "tar.gz")
    } else if target::MACOS && target::aarch64 {
        ("macos-aarch64", "tar.gz")
    } else if target::WINDOWS && target::x86 {
        ("win32", "zip")
    } else if target::WINDOWS && target::x86_64 {
        ("win64", "zip")
    } else {
        bail!("geckodriver binaries are unavailable for this target")
    };

    let url = get_geckodriver_url(target, ext);

    match get_and_notify(cache, installation_allowed, "geckodriver", &url)? {
        Some(path) => Ok(path),
        None => bail!(
            "No cached `geckodriver` binary found, and could not find a global `geckodriver` \
             on the `$PATH`. Not installing `geckodriver` because of noinstall mode."
        ),
    }
}

/// Get `geckodriver` download URL.
///
/// _Algorithm_:
/// 1. Try to open `*.stamps` file and deserialize its content to JSON object.
/// 2. Try to compare current time with the saved one.
/// 3. If the saved time is older than 1 day or something failed
///    => fetch a new version and save version & time.
/// 4. If everything failed, use the default version.
/// 5. Return URL.
///
/// _Notes:_
///
/// It returns the latest one without checking the installed `Firefox` version
/// - it should be relatively safe because each `geckodriver` supports many `Firefox` versions:
/// https://firefox-source-docs.mozilla.org/testing/geckodriver/Support.html#supported-platforms
fn get_geckodriver_url(target: &str, ext: &str) -> String {
    let fetch_and_save_version =
        || fetch_latest_geckodriver_tag_json().and_then(save_geckodriver_version);

    let geckodriver_version = if target::WINDOWS {
        log::info!(
            "[geckodriver] Windows detected, holding geckodriver version to {}",
            DEFAULT_WINDOWS_GECKODRIVER_VERSION
        );
        DEFAULT_WINDOWS_GECKODRIVER_VERSION.to_owned()
    } else {
        log::info!("[geckodriver] Looking up latest version of geckodriver...");
        match stamps::read_stamps_file_to_json() {
            Ok(json) => {
                if should_load_geckodriver_version_from_stamp(&json) {
                    stamps::get_stamp_value(GECKODRIVER_VERSION_STAMP, &json)
                } else {
                    fetch_and_save_version()
                }
            }
            Err(_) => fetch_and_save_version(),
        }
        .unwrap_or_else(|error| {
            log::warn!(
                "[geckodriver] Cannot load or fetch geckodriver's latest version data, \
                 the default version {} will be used. Error: {}",
                DEFAULT_GECKODRIVER_VERSION,
                error
            );
            DEFAULT_GECKODRIVER_VERSION.to_owned()
        })
    };
    let url = assemble_geckodriver_url(&geckodriver_version, target, ext);
    log::info!("[geckodriver] Fetching geckodriver at {}", url);
    url
}

// ------ `get_geckodriver_url` helpers  ------

fn save_geckodriver_version(version: String) -> Result<String> {
    stamps::save_stamp_value(GECKODRIVER_VERSION_STAMP, &version)?;

    let current_time = chrono::offset::Local::now().to_rfc3339();
    stamps::save_stamp_value(GECKODRIVER_LAST_UPDATED_STAMP, current_time)?;

    Ok(version)
}

fn should_load_geckodriver_version_from_stamp(json: &serde_json::Value) -> bool {
    let last_updated = stamps::get_stamp_value(GECKODRIVER_LAST_UPDATED_STAMP, json)
        .ok()
        .and_then(|last_updated| DateTime::parse_from_rfc3339(&last_updated).ok());

    match last_updated {
        None => false,
        Some(last_updated) => {
            let current_time = chrono::offset::Local::now();
            current_time.signed_duration_since(last_updated).num_hours() < 24
        }
    }
}

fn fetch_latest_geckodriver_tag_json() -> Result<String> {
    let content: serde_json::Value = ureq::builder()
        .try_proxy_from_env(true)
        .build()
        .get("https://github.com/mozilla/geckodriver/releases/latest")
        .set("Accept", "application/json")
        .call()
        .context("fetching of geckodriver's latest release data failed")?
        .into_json()?;

    get_version_from_json(content)
}

/// JSON example: `{"id":15227534,"tag_name":"v0.24.0","update_url":"/mozzila...`
fn get_version_from_json(json: serde_json::Value) -> Result<String> {
    json.get("tag_name")
        .and_then(|tag_name| tag_name.as_str().map(ToOwned::to_owned))
        .ok_or_else(|| anyhow!("cannot get `tag_name` from geckodriver's latest release data"))
}

fn assemble_geckodriver_url(tag: &str, target: &str, ext: &str) -> String {
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/{tag}/geckodriver-{tag}-{target}.{ext}",
        tag=tag,
        target=target,
        ext=ext,
    )
}
