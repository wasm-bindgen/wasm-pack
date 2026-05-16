//! Reading and writing Cargo.toml and package.json manifests.

#![allow(
    clippy::new_ret_no_self,
    clippy::needless_pass_by_value,
    clippy::redundant_closure
)]

use anyhow::{anyhow, bail, Context, Result};
mod npm;

use std::path::Path;
use std::{collections::HashMap, fs};

use self::npm::{
    repository::Repository, CommonJSPackage, ESModulesPackage, NoModulesPackage, NpmPackage,
};
use crate::command::build::{BuildProfile, Target};
use crate::PBAR;
use cargo_metadata::{CrateType, Metadata, TargetKind};
use chrono::offset;
use chrono::DateTime;
use serde::{self, Deserialize};
use serde_json;
use std::collections::BTreeSet;
use std::env;
use std::io::Write;
use strsim::levenshtein;
use toml;

const WASM_PACK_METADATA_KEY: &str = "package.metadata.wasm-pack";
const WASM_PACK_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const WASM_PACK_REPO_URL: &str = "https://github.com/wasm-bindgen/wasm-pack";

/// Store for metadata learned about a crate
pub struct CrateData {
    data: Metadata,
    current_idx: usize,
    manifest: CargoManifest,
    out_name: Option<String>,
}

#[doc(hidden)]
#[derive(Deserialize)]
pub struct CargoManifest {
    package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,

    #[serde(default)]
    metadata: CargoMetadata,
}

#[derive(Default, Deserialize)]
struct CargoMetadata {
    #[serde(default, rename = "wasm-pack")]
    wasm_pack: CargoWasmPack,
}

#[derive(Default, Deserialize)]
struct CargoWasmPack {
    #[serde(default)]
    profile: CargoWasmPackProfiles,
}

#[derive(Deserialize)]
struct CargoWasmPackProfiles {
    #[serde(
        default = "CargoWasmPackProfile::default_dev",
        deserialize_with = "CargoWasmPackProfile::deserialize_dev"
    )]
    dev: CargoWasmPackProfile,

    #[serde(
        default = "CargoWasmPackProfile::default_release",
        deserialize_with = "CargoWasmPackProfile::deserialize_release"
    )]
    release: CargoWasmPackProfile,

    #[serde(
        default = "CargoWasmPackProfile::default_profiling",
        deserialize_with = "CargoWasmPackProfile::deserialize_profiling"
    )]
    profiling: CargoWasmPackProfile,

    #[serde(
        default = "CargoWasmPackProfile::default_custom",
        deserialize_with = "CargoWasmPackProfile::deserialize_custom"
    )]
    custom: CargoWasmPackProfile,
}

impl Default for CargoWasmPackProfiles {
    fn default() -> CargoWasmPackProfiles {
        CargoWasmPackProfiles {
            dev: CargoWasmPackProfile::default_dev(),
            release: CargoWasmPackProfile::default_release(),
            profiling: CargoWasmPackProfile::default_profiling(),
            custom: CargoWasmPackProfile::default_custom(),
        }
    }
}

/// This is where configuration goes for wasm-bindgen, wasm-opt, wasm-snip, or
/// anything else that wasm-pack runs.
#[derive(Default, Deserialize)]
pub struct CargoWasmPackProfile {
    #[serde(default, rename = "wasm-bindgen")]
    wasm_bindgen: CargoWasmPackProfileWasmBindgen,
    #[serde(default, rename = "wasm-opt")]
    wasm_opt: Option<CargoWasmPackProfileWasmOpt>,
}

#[derive(Default, Deserialize)]
struct CargoWasmPackProfileWasmBindgen {
    #[serde(default, rename = "debug-js-glue")]
    debug_js_glue: Option<bool>,

    #[serde(default, rename = "demangle-name-section")]
    demangle_name_section: Option<bool>,

    #[serde(default, rename = "dwarf-debug-info")]
    dwarf_debug_info: Option<bool>,

    #[serde(default, rename = "omit-default-module-path")]
    omit_default_module_path: Option<bool>,

    #[serde(default, rename = "split-linked-modules")]
    split_linked_modules: Option<bool>,
}

/// Struct for storing information received from crates.io
#[derive(Deserialize, Debug)]
pub struct Crate {
    #[serde(rename = "crate")]
    crt: CrateInformation,
}

#[derive(Deserialize, Debug)]
struct CrateInformation {
    max_version: String,
}

impl Crate {
    /// Returns latest wasm-pack version
    pub fn return_wasm_pack_latest_version() -> Result<Option<String>> {
        let current_time = chrono::offset::Local::now();
        let old_metadata_file = Self::return_wasm_pack_file();

        match old_metadata_file {
            Some(ref file_contents) => {
                let last_updated = Self::return_stamp_file_value(&file_contents, "created")
                    .and_then(|t| DateTime::parse_from_str(t.as_str(), "%+").ok());

                last_updated
                    .map(|last_updated| {
                        if current_time.signed_duration_since(last_updated).num_hours() > 24 {
                            Self::return_api_call_result(current_time).map(Some)
                        } else {
                            Ok(Self::return_stamp_file_value(&file_contents, "version"))
                        }
                    })
                    .unwrap_or_else(|| Ok(None))
            }
            None => Self::return_api_call_result(current_time).map(Some),
        }
    }

    fn return_api_call_result(current_time: DateTime<offset::Local>) -> Result<String> {
        let version = Self::return_latest_wasm_pack_version();

        // We always override the stamp file with the current time because we don't
        // want to hit the API all the time if it fails. It should follow the same
        // "policy" as the success. This means that the 24 hours rate limiting
        // will be active regardless if the check succeeded or failed.
        match version {
            Ok(ref version) => Self::override_stamp_file(current_time, Some(&version)).ok(),
            Err(_) => Self::override_stamp_file(current_time, None).ok(),
        };

        version
    }

    fn override_stamp_file(
        current_time: DateTime<offset::Local>,
        version: Option<&str>,
    ) -> Result<()> {
        let path = env::current_exe()?;

        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path.with_extension("stamp"))?;

        file.set_len(0)?;

        write!(file, "created {:?}", current_time)?;

        if let Some(version) = version {
            write!(file, "\nversion {}", version)?;
        }

        Ok(())
    }

    /// Return stamp file where metadata is stored.
    fn return_wasm_pack_file() -> Option<String> {
        if let Ok(path) = env::current_exe() {
            if let Ok(file) = fs::read_to_string(path.with_extension("stamp")) {
                return Some(file);
            }
        }
        None
    }

    /// Returns wasm-pack latest version (if it's received) by executing check_wasm_pack_latest_version function.
    fn return_latest_wasm_pack_version() -> Result<String> {
        Self::check_wasm_pack_latest_version().map(|crt| crt.crt.max_version)
    }

    /// Read the stamp file and return value assigned to a certain key.
    fn return_stamp_file_value(file: &str, word: &str) -> Option<String> {
        let created = file
            .lines()
            .find(|line| line.starts_with(word))
            .and_then(|l| l.split_whitespace().nth(1));

        created.map(|s| s.to_string())
    }

    /// Call to the crates.io api and return the latest version of `wasm-pack`
    fn check_wasm_pack_latest_version() -> Result<Crate> {
        let url = "https://crates.io/api/v1/crates/wasm-pack";
        let agent = ureq::builder()
            .try_proxy_from_env(true)
            .user_agent(&format!(
                "wasm-pack/{} ({})",
                WASM_PACK_VERSION.unwrap_or_else(|| "unknown"),
                WASM_PACK_REPO_URL
            ))
            .build();
        let resp = agent
            .get(url)
            .call()
            .context("failed to get wasm-pack version")?;

        let status_code = resp.status();

        if 200 <= status_code && status_code < 300 {
            let json = resp.into_json()?;

            Ok(json)
        } else {
            bail!(
                "Received a bad HTTP status code ({}) when checking for newer wasm-pack version at: {}",
                status_code,
                url
            )
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum CargoWasmPackProfileWasmOpt {
    Enabled(bool),
    ExplicitArgs(Vec<String>),
}

impl Default for CargoWasmPackProfileWasmOpt {
    fn default() -> Self {
        CargoWasmPackProfileWasmOpt::Enabled(false)
    }
}

impl CargoWasmPackProfile {
    fn default_dev() -> Self {
        CargoWasmPackProfile {
            wasm_bindgen: CargoWasmPackProfileWasmBindgen {
                debug_js_glue: Some(true),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: None,
        }
    }

    fn default_release() -> Self {
        CargoWasmPackProfile {
            wasm_bindgen: CargoWasmPackProfileWasmBindgen {
                debug_js_glue: Some(false),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: Some(CargoWasmPackProfileWasmOpt::Enabled(true)),
        }
    }

    fn default_profiling() -> Self {
        CargoWasmPackProfile {
            wasm_bindgen: CargoWasmPackProfileWasmBindgen {
                debug_js_glue: Some(false),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: Some(CargoWasmPackProfileWasmOpt::Enabled(true)),
        }
    }

    fn default_custom() -> Self {
        CargoWasmPackProfile {
            wasm_bindgen: CargoWasmPackProfileWasmBindgen {
                debug_js_glue: Some(false),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: Some(CargoWasmPackProfileWasmOpt::Enabled(true)),
        }
    }

    fn deserialize_dev<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut profile = <Option<Self>>::deserialize(deserializer)?.unwrap_or_default();
        profile.update_with_defaults(&Self::default_dev());
        Ok(profile)
    }

    fn deserialize_release<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut profile = <Option<Self>>::deserialize(deserializer)?.unwrap_or_default();
        profile.update_with_defaults(&Self::default_release());
        Ok(profile)
    }

    fn deserialize_profiling<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut profile = <Option<Self>>::deserialize(deserializer)?.unwrap_or_default();
        profile.update_with_defaults(&Self::default_profiling());
        Ok(profile)
    }

    fn deserialize_custom<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut profile = <Option<Self>>::deserialize(deserializer)?.unwrap_or_default();
        profile.update_with_defaults(&Self::default_custom());
        Ok(profile)
    }

    fn update_with_defaults(&mut self, defaults: &Self) {
        macro_rules! d {
            ( $( $path:ident ).* ) => {
                self. $( $path ).* .get_or_insert(defaults. $( $path ).* .unwrap());
            }
        }
        d!(wasm_bindgen.debug_js_glue);
        d!(wasm_bindgen.demangle_name_section);
        d!(wasm_bindgen.dwarf_debug_info);
        d!(wasm_bindgen.omit_default_module_path);
        d!(wasm_bindgen.split_linked_modules);

        if self.wasm_opt.is_none() {
            self.wasm_opt = defaults.wasm_opt.clone();
        }
    }

    /// Get this profile's configured `[wasm-bindgen.debug-js-glue]` value.
    pub fn wasm_bindgen_debug_js_glue(&self) -> bool {
        self.wasm_bindgen.debug_js_glue.unwrap()
    }

    /// Get this profile's configured `[wasm-bindgen.demangle-name-section]` value.
    pub fn wasm_bindgen_demangle_name_section(&self) -> bool {
        self.wasm_bindgen.demangle_name_section.unwrap()
    }

    /// Get this profile's configured `[wasm-bindgen.dwarf-debug-info]` value.
    pub fn wasm_bindgen_dwarf_debug_info(&self) -> bool {
        self.wasm_bindgen.dwarf_debug_info.unwrap()
    }

    /// Get this profile's configured `[wasm-bindgen.omit-default-module-path]` value.
    pub fn wasm_bindgen_omit_default_module_path(&self) -> bool {
        self.wasm_bindgen.omit_default_module_path.unwrap()
    }

    /// Get this profile's configured `[wasm-bindgen.split-linked-modules]` value.
    pub fn wasm_bindgen_split_linked_modules(&self) -> bool {
        self.wasm_bindgen.split_linked_modules.unwrap()
    }

    /// Get this profile's configured arguments for `wasm-opt`, if enabled.
    pub fn wasm_opt_args(&self) -> Option<Vec<String>> {
        match self.wasm_opt.as_ref()? {
            CargoWasmPackProfileWasmOpt::Enabled(false) => None,
            CargoWasmPackProfileWasmOpt::Enabled(true) => Some(vec!["-O".to_string()]),
            CargoWasmPackProfileWasmOpt::ExplicitArgs(s) => Some(s.clone()),
        }
    }
}

struct NpmData {
    name: String,
    files: Vec<String>,
    dts_file: Option<String>,
    main: String,
    homepage: Option<String>, // https://docs.npmjs.com/files/package.json#homepage,
    keywords: Option<Vec<String>>, // https://docs.npmjs.com/files/package.json#keywords
}

#[doc(hidden)]
pub struct ManifestAndUnusedKeys {
    pub manifest: CargoManifest,
    pub unused_keys: BTreeSet<String>,
}

impl CrateData {
    /// Reads all metadata for the crate whose manifest is inside the directory
    /// specified by `path`.
    pub fn new(crate_path: &Path, out_name: Option<String>) -> Result<CrateData> {
        let manifest_path = crate_path.join("Cargo.toml");
        if !manifest_path.is_file() {
            bail!(
                "crate directory is missing a `Cargo.toml` file; is `{}` the \
                 wrong directory?",
                crate_path.display()
            )
        }

        let data = cargo_metadata::MetadataCommand::new()
            .manifest_path(&manifest_path)
            .exec()?;

        let manifest_and_keys = CrateData::parse_crate_data(&manifest_path)?;
        CrateData::warn_for_unused_keys(&manifest_and_keys);

        let manifest = manifest_and_keys.manifest;
        let current_idx = data
            .packages
            .iter()
            .position(|pkg| {
                pkg.name == manifest.package.name
                    && CrateData::is_same_path(pkg.manifest_path.as_std_path(), &manifest_path)
            })
            .ok_or_else(|| anyhow!("failed to find package in metadata"))?;

        Ok(CrateData {
            data,
            manifest,
            current_idx,
            out_name,
        })
    }

    fn is_same_path(path1: &Path, path2: &Path) -> bool {
        if let Ok(path1) = fs::canonicalize(&path1) {
            if let Ok(path2) = fs::canonicalize(&path2) {
                return path1 == path2;
            }
        }
        path1 == path2
    }

    /// Read the `manifest_path` file and deserializes it using the toml Deserializer.
    /// Returns a Result containing `ManifestAndUnusedKeys` which contains `CargoManifest`
    /// and a `BTreeSet<String>` containing the unused keys from the parsed file.
    ///
    /// # Errors
    /// Will return Err if the file (manifest_path) couldn't be read or
    /// if deserialize to `CargoManifest` fails.
    pub fn parse_crate_data(manifest_path: &Path) -> Result<ManifestAndUnusedKeys> {
        let manifest = fs::read_to_string(&manifest_path)
            .with_context(|| anyhow!("failed to read: {}", manifest_path.display()))?;
        let manifest = toml::Deserializer::parse(&manifest).with_context(|| {
            anyhow!(
                "failed to create TOML deserializer for: {}",
                manifest_path.display()
            )
        })?;

        let mut unused_keys = BTreeSet::new();
        let levenshtein_threshold = 1;

        let manifest: CargoManifest = serde_ignored::deserialize(manifest, |path| {
            let path_string = path.to_string();

            if path_string.starts_with("package.metadata")
                && (path_string.contains("wasm-pack")
                    || levenshtein(WASM_PACK_METADATA_KEY, &path_string) <= levenshtein_threshold)
            {
                unused_keys.insert(path_string);
            }
        })
        .with_context(|| anyhow!("failed to parse manifest: {}", manifest_path.display()))?;

        Ok(ManifestAndUnusedKeys {
            manifest,
            unused_keys,
        })
    }

    /// Iterating through all the passed `unused_keys` and output
    /// a warning for each unknown key.
    pub fn warn_for_unused_keys(manifest_and_keys: &ManifestAndUnusedKeys) {
        manifest_and_keys.unused_keys.iter().for_each(|path| {
            PBAR.warn(&format!(
                "\"{}\" is an unknown key and will be ignored. Please check your Cargo.toml.",
                path
            ));
        });
    }

    /// Get the configured profile.
    pub fn configured_profile(&self, profile: BuildProfile) -> &CargoWasmPackProfile {
        match profile {
            BuildProfile::Dev => &self.manifest.package.metadata.wasm_pack.profile.dev,
            BuildProfile::Profiling => &self.manifest.package.metadata.wasm_pack.profile.profiling,
            BuildProfile::Release => &self.manifest.package.metadata.wasm_pack.profile.release,
            BuildProfile::Custom(_) => &self.manifest.package.metadata.wasm_pack.profile.custom,
        }
    }

    /// Check that the crate is configured correctly for the chosen target.
    ///
    /// The emscripten target requires `staticlib`; every other wasm target
    /// requires `cdylib`. Mismatches produce a clear diagnostic up front
    /// instead of a downstream "cargo produced no .wasm" error.
    pub fn check_crate_config(&self, emscripten: bool) -> Result<()> {
        self.check_crate_type(emscripten)
    }

    fn check_crate_type(&self, emscripten: bool) -> Result<()> {
        let pkg = &self.data.packages[self.current_idx];
        let has = |needle: CrateType| {
            pkg.targets
                .iter()
                .any(|t| t.crate_types.iter().any(|c| c == &needle))
        };
        let has_cdylib = has(CrateType::CDyLib);
        let has_staticlib = has(CrateType::StaticLib);

        if emscripten {
            if has_staticlib {
                return Ok(());
            }
            if has_cdylib {
                bail!(
                    "Targeting wasm32-unknown-emscripten requires crate-type = [\"staticlib\"], \
                     but this crate is configured for cdylib. emcc links the staticlib into the \
                     final wasm; cdylib would skip emcc entirely.\n\n\
                     Change your Cargo.toml to:\n\n\
                     [lib]\n\
                     crate-type = [\"staticlib\"]"
                )
            }
            bail!(
                "Targeting wasm32-unknown-emscripten requires crate-type = [\"staticlib\"]. \
                 Add the following to your Cargo.toml:\n\n\
                 [lib]\n\
                 crate-type = [\"staticlib\"]"
            )
        }

        if has_cdylib {
            return Ok(());
        }
        if has_staticlib {
            bail!(
                "This crate is configured as a staticlib, which is only used by the \
                 wasm32-unknown-emscripten target. For wasm32-unknown-unknown (the default), \
                 use cdylib instead:\n\n\
                 [lib]\n\
                 crate-type = [\"cdylib\", \"rlib\"]"
            )
        }
        bail!(
            "crate-type must be cdylib to compile to WebAssembly. Add the following to your \
             Cargo.toml file:\n\n\
             [lib]\n\
             crate-type = [\"cdylib\", \"rlib\"]"
        )
    }

    fn pkg(&self) -> &cargo_metadata::Package {
        &self.data.packages[self.current_idx]
    }

    /// Get the crate name for the crate at the given path.
    pub fn crate_name(&self) -> String {
        let pkg = self.pkg();
        match pkg
            .targets
            .iter()
            .find(|t| t.kind.iter().any(|k| k == &TargetKind::CDyLib))
        {
            Some(lib) => lib.name.replace("-", "_"),
            None => pkg.name.replace("-", "_"),
        }
    }

    /// Get the prefix for output file names
    pub fn name_prefix(&self) -> String {
        match &self.out_name {
            Some(value) => value.clone(),
            None => self.crate_name(),
        }
    }

    /// Gets the optional path to the readme, or None if disabled.
    pub fn crate_readme(&self) -> Option<String> {
        self.pkg()
            .readme
            .clone()
            .map(|readme_file| readme_file.into_string())
    }

    /// Get the license for the crate at the given path.
    pub fn crate_license(&self) -> &Option<String> {
        &self.pkg().license
    }

    /// Get the license file path for the crate at the given path.
    pub fn crate_license_file(&self) -> Option<String> {
        self.pkg()
            .license_file
            .clone()
            .map(|license_file| license_file.into_string())
    }

    /// Returns the path to this project's target directory where artifacts are
    /// located after a cargo build.
    pub fn target_directory(&self) -> &Path {
        Path::new(&self.data.target_directory)
    }

    /// Returns the path to this project's root cargo workspace directory
    pub fn workspace_root(&self) -> &Path {
        Path::new(&self.data.workspace_root)
    }

    /// Generate a package.json file inside in `./pkg`.
    pub fn write_package_json(
        &self,
        out_dir: &Path,
        scope: &Option<String>,
        disable_dts: bool,
        target: Target,
        emscripten: bool,
    ) -> Result<()> {
        let pkg_file_path = out_dir.join("package.json");
        // Check if a `package.json` was already generated by wasm-bindgen, if so
        // we merge the NPM dependencies already specified in it.
        let existing_deps = if pkg_file_path.exists() {
            // It's just a map of dependency names to versions
            Some(serde_json::from_str::<HashMap<String, String>>(
                &fs::read_to_string(&pkg_file_path)?,
            )?)
        } else {
            None
        };

        // Emscripten emits a single ESM module (`<name>.mjs`) + bare wasm,
        // regardless of which wasm-pack `--target` is selected. The
        // conventional wasm-pack manifest layout (`<name>.js` + `<name>_bg.wasm`
        // + `sideEffects: ["./snippets/*"]`) doesn't match. Use a
        // tailored variant.
        let npm_data = if emscripten {
            // Deno historically skips package.json — keep that for parity.
            if matches!(target, Target::Deno) {
                return Ok(());
            }
            self.to_emscripten(scope, disable_dts, existing_deps, out_dir)
        } else {
            match target {
                Target::Nodejs => self.to_commonjs(scope, disable_dts, existing_deps, out_dir),
                Target::NoModules => self.to_nomodules(scope, disable_dts, existing_deps, out_dir),
                Target::Bundler => self.to_esmodules(scope, disable_dts, existing_deps, out_dir),
                Target::Web => self.to_web(scope, disable_dts, existing_deps, out_dir),
                Target::Module => self.to_web(scope, disable_dts, existing_deps, out_dir),
                // Deno does not need package.json
                Target::Deno => return Ok(()),
            }
        };

        let npm_json = format!("{}\n", serde_json::to_string_pretty(&npm_data)?);

        fs::write(&pkg_file_path, npm_json)
            .with_context(|| anyhow!("failed to write: {}", pkg_file_path.display()))?;
        Ok(())
    }

    fn npm_data(
        &self,
        scope: &Option<String>,
        add_js_bg_to_package_json: bool,
        disable_dts: bool,
        out_dir: &Path,
    ) -> NpmData {
        let name_prefix = self.name_prefix();
        let wasm_file = format!("{}_bg.wasm", name_prefix);
        let js_file = format!("{}.js", name_prefix);
        let mut files = vec![wasm_file];

        files.push(js_file.clone());
        if add_js_bg_to_package_json {
            let js_bg_file = format!("{}_bg.js", name_prefix);
            files.push(js_bg_file);
        }

        let pkg = &self.data.packages[self.current_idx];
        let npm_name = match scope {
            Some(s) => format!("@{}/{}", s, pkg.name),
            None => pkg.name.to_string(),
        };

        let dts_file = if !disable_dts {
            let file = format!("{}.d.ts", name_prefix);
            files.push(file.to_string());
            Some(file)
        } else {
            None
        };

        let keywords = if pkg.keywords.len() > 0 {
            Some(pkg.keywords.clone())
        } else {
            None
        };

        if let Ok(entries) = fs::read_dir(out_dir) {
            let file_names = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.metadata().map(|m| m.is_file()).unwrap_or(false))
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|f| f.starts_with("LICENSE"))
                .filter(|f| f != "LICENSE");
            for file_name in file_names {
                files.push(file_name);
            }
        }

        NpmData {
            name: npm_name,
            dts_file,
            files,
            main: js_file,
            homepage: self.pkg().homepage.clone(),
            keywords,
        }
    }

    fn license(&self) -> Option<String> {
        self.crate_license().clone().or_else(|| {
            self.crate_license_file().clone().map(|file| {
                // When license is written in file: https://docs.npmjs.com/files/package.json#license
                format!("SEE LICENSE IN {}", file)
            })
        })
    }

    fn to_commonjs(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        dependencies: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, false, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        self.check_optional_fields();

        NpmPackage::CommonJSPackage(CommonJSPackage {
            name: data.name,
            collaborators: pkg.authors.clone(),
            description: self.pkg().description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: self.pkg().repository.clone().map(|repo_url| Repository {
                ty: "git".to_string(),
                url: repo_url,
            }),
            files: data.files,
            main: data.main,
            homepage: data.homepage,
            types: data.dts_file,
            keywords: data.keywords,
            dependencies,
        })
    }

    fn to_esmodules(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        dependencies: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, true, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        self.check_optional_fields();

        NpmPackage::ESModulesPackage(ESModulesPackage {
            name: data.name,
            ty: "module".into(),
            collaborators: pkg.authors.clone(),
            description: self.pkg().description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: self.pkg().repository.clone().map(|repo_url| Repository {
                ty: "git".to_string(),
                url: repo_url,
            }),
            files: data.files,
            main: data.main.clone(),
            homepage: data.homepage,
            types: data.dts_file,
            side_effects: vec![format!("./{}", data.main), "./snippets/*".to_owned()],
            keywords: data.keywords,
            dependencies,
        })
    }

    fn to_web(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        dependencies: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, false, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        self.check_optional_fields();

        NpmPackage::ESModulesPackage(ESModulesPackage {
            name: data.name,
            ty: "module".into(),
            collaborators: pkg.authors.clone(),
            description: self.pkg().description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: self.pkg().repository.clone().map(|repo_url| Repository {
                ty: "git".to_string(),
                url: repo_url,
            }),
            files: data.files,
            main: data.main,
            homepage: data.homepage,
            types: data.dts_file,
            side_effects: vec!["./snippets/*".to_owned()],
            keywords: data.keywords,
            dependencies,
        })
    }

    /// Package manifest for the emscripten pipeline.
    ///
    /// Differs from `to_web` in three places:
    ///   - `main` is `<name>.mjs` (the ESM module emcc emits)
    ///   - `files` lists `<name>.wasm`, `<name>.mjs`, optional `.d.ts`
    ///     (no `<name>_bg.wasm`, no `<name>_bg.js`)
    ///   - `sideEffects` omits `./snippets/*` because emscripten builds
    ///     don't generate JS snippets the way wasm-bindgen's default
    ///     pipeline does
    fn to_emscripten(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        dependencies: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let name_prefix = self.name_prefix();
        let main = format!("{}.mjs", name_prefix);
        let wasm = format!("{}.wasm", name_prefix);
        let mut files = vec![wasm, main.clone()];

        let dts_file = if !disable_dts {
            let dts = format!("{}.d.ts", name_prefix);
            files.push(dts.clone());
            Some(dts)
        } else {
            None
        };

        // Include any user-supplied LICENSE-* sidecars, mirroring npm_data.
        if let Ok(entries) = fs::read_dir(out_dir) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    if name.starts_with("LICENSE") && name != "LICENSE" {
                        files.push(name);
                    }
                }
            }
        }

        let pkg = &self.data.packages[self.current_idx];
        let npm_name = match scope {
            Some(s) => format!("@{}/{}", s, pkg.name),
            None => pkg.name.to_string(),
        };
        let keywords = if !pkg.keywords.is_empty() {
            Some(pkg.keywords.clone())
        } else {
            None
        };

        self.check_optional_fields();

        NpmPackage::ESModulesPackage(ESModulesPackage {
            name: npm_name,
            ty: "module".into(),
            collaborators: pkg.authors.clone(),
            description: pkg.description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: pkg.repository.clone().map(|url| Repository {
                ty: "git".to_string(),
                url,
            }),
            files,
            main,
            homepage: pkg.homepage.clone(),
            types: dts_file,
            // No snippets in the emscripten flow.
            side_effects: vec![],
            keywords,
            dependencies,
        })
    }

    fn to_nomodules(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        dependencies: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, false, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        self.check_optional_fields();

        NpmPackage::NoModulesPackage(NoModulesPackage {
            name: data.name,
            collaborators: pkg.authors.clone(),
            description: self.pkg().description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: self.pkg().repository.clone().map(|repo_url| Repository {
                ty: "git".to_string(),
                url: repo_url,
            }),
            files: data.files,
            browser: data.main,
            homepage: data.homepage,
            types: data.dts_file,
            keywords: data.keywords,
            dependencies,
        })
    }

    fn check_optional_fields(&self) {
        let mut messages = vec![];
        if self.pkg().description.is_none() {
            messages.push("description");
        }
        if self.pkg().repository.is_none() {
            messages.push("repository");
        }
        if self.pkg().license.is_none() && self.pkg().license_file.is_none() {
            messages.push("license");
        }

        match messages.len() {
            1 => PBAR.info(&format!("Optional field missing from Cargo.toml: '{}'. This is not necessary, but recommended", messages[0])),
            2 => PBAR.info(&format!("Optional fields missing from Cargo.toml: '{}', '{}'. These are not necessary, but recommended", messages[0], messages[1])),
            3 => PBAR.info(&format!("Optional fields missing from Cargo.toml: '{}', '{}', and '{}'. These are not necessary, but recommended", messages[0], messages[1], messages[2])),
            _ => ()
        };
    }
}
