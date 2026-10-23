//! Reading Cargo.lock lock file.

#![allow(clippy::new_ret_no_self)]

use std::fs;
use std::path::PathBuf;

use crate::manifest::CrateData;
use anyhow::{anyhow, bail, Context, Result};
use console::style;
use toml;

/// This struct represents the contents of `Cargo.lock`.
#[derive(Clone, Debug, Deserialize)]
pub struct Lockfile {
    package: Vec<Package>,
}

/// This struct represents a single package entry in `Cargo.lock`
#[derive(Clone, Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    source: Option<String>,
}

/// A git-sourced lockfile package, e.g.
/// `git+https://github.com/wasm-bindgen/wasm-bindgen?branch=main#<rev>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitSource {
    /// Repository URL, without cargo's `?branch=`/`?tag=`/`?rev=` query.
    pub url: String,
    /// The exact locked commit.
    pub rev: String,
}

impl GitSource {
    fn parse(source: &str) -> Option<GitSource> {
        let rest = source.strip_prefix("git+")?;
        let (url, rev) = rest.split_once('#')?;
        let url = url.split_once('?').map_or(url, |(url, _)| url);
        Some(GitSource {
            url: url.to_string(),
            rev: rev.to_string(),
        })
    }
}

impl Lockfile {
    /// Read the `Cargo.lock` file for the crate at the given path.
    pub fn new(crate_data: &CrateData) -> Result<Lockfile> {
        let lock_path = get_lockfile_path(crate_data)?;
        let lockfile = fs::read_to_string(&lock_path)
            .with_context(|| anyhow!("failed to read: {}", lock_path.display()))?;
        let lockfile = toml::from_str(&lockfile)
            .with_context(|| anyhow!("failed to parse: {}", lock_path.display()))?;
        Ok(lockfile)
    }

    /// Get the version of `wasm-bindgen` dependency used in the `Cargo.lock`.
    pub fn wasm_bindgen_version(&self) -> Option<&str> {
        self.get_package_version("wasm-bindgen")
    }

    /// Like `wasm_bindgen_version`, except it returns an error instead of
    /// `None`.
    pub fn require_wasm_bindgen(&self) -> Result<&str> {
        self.wasm_bindgen_version().ok_or_else(|| {
            anyhow!(
                "Ensure that you have \"{}\" as a dependency in your Cargo.toml file:\n\
                 [dependencies]\n\
                 wasm-bindgen = \"0.2\"",
                style("wasm-bindgen").bold().dim(),
            )
        })
    }

    /// Get the version of `wasm-bindgen` dependency used in the `Cargo.lock`.
    pub fn wasm_bindgen_test_version(&self) -> Option<&str> {
        self.get_package_version("wasm-bindgen-test")
    }

    /// The git source of the `wasm-bindgen` dependency, if it is not a
    /// registry release. The matching CLI then has to be built from the same
    /// revision.
    pub fn wasm_bindgen_git_source(&self) -> Option<GitSource> {
        self.get_package("wasm-bindgen")?
            .source
            .as_deref()
            .and_then(GitSource::parse)
    }

    fn get_package(&self, package: &str) -> Option<&Package> {
        self.package.iter().find(|p| p.name == package)
    }

    fn get_package_version(&self, package: &str) -> Option<&str> {
        self.get_package(package).map(|p| &p.version[..])
    }
}

#[cfg(test)]
mod tests {
    use super::GitSource;

    #[test]
    fn parses_git_sources() {
        let parsed = GitSource::parse(
            "git+https://github.com/wasm-bindgen/wasm-bindgen?branch=main#0123abcd",
        );
        assert_eq!(
            parsed,
            Some(GitSource {
                url: "https://github.com/wasm-bindgen/wasm-bindgen".into(),
                rev: "0123abcd".into(),
            })
        );
        assert_eq!(
            GitSource::parse("git+https://example.com/repo#deadbeef").map(|s| s.url),
            Some("https://example.com/repo".into())
        );
        assert_eq!(
            GitSource::parse("registry+https://github.com/rust-lang/crates.io-index"),
            None
        );
    }
}

/// Given the path to the crate that we are building, return a `PathBuf`
/// containing the location of the lock file, by finding the workspace root.
fn get_lockfile_path(crate_data: &CrateData) -> Result<PathBuf> {
    // Check that a lock file can be found in the directory. Return an error
    // if it cannot, otherwise return the path buffer.
    let lockfile_path = crate_data.workspace_root().join("Cargo.lock");
    if !lockfile_path.is_file() {
        bail!("Could not find lockfile at {:?}", lockfile_path)
    } else {
        Ok(lockfile_path)
    }
}
