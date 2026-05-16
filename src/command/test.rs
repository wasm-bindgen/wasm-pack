//! Implementation of the `wasm-pack test` command.

use crate::build;
use crate::cache;
use crate::command::utils::get_crate_path;
use crate::install::{self, InstallMode, Tool};
use crate::lockfile::Lockfile;
use crate::manifest;
use crate::test::{self, webdriver};
use anyhow::{bail, Result};
use binary_install::Cache;
use clap::Args;
use console::style;
use log::info;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Default, Args)]
#[command(allow_hyphen_values = true, trailing_var_arg = true)]
/// Everything required to configure the `wasm-pack test` command.
pub struct TestOptions {
    #[clap(long = "node")]
    /// Run the tests in Node.js.
    pub node: bool,

    #[clap(long = "firefox")]
    /// Run the tests in Firefox. This machine must have a Firefox installation.
    /// If the `geckodriver` WebDriver client is not on the `$PATH`, and not
    /// specified with `--geckodriver`, then `wasm-pack` will download a local
    /// copy.
    pub firefox: bool,

    #[clap(long = "geckodriver")]
    /// The path to the `geckodriver` WebDriver client for testing in
    /// Firefox. Implies `--firefox`.
    pub geckodriver: Option<PathBuf>,

    #[clap(long = "chrome")]
    /// Run the tests in Chrome. This machine must have a Chrome installation.
    /// If the `chromedriver` WebDriver client is not on the `$PATH`, and not
    /// specified with `--chromedriver`, then `wasm-pack` will download a local
    /// copy.
    pub chrome: bool,

    #[clap(long = "chromedriver")]
    /// The path to the `chromedriver` WebDriver client for testing in
    /// Chrome. Implies `--chrome`.
    pub chromedriver: Option<PathBuf>,

    #[clap(long = "safari")]
    /// Run the tests in Safari. This machine must have a Safari installation,
    /// and the `safaridriver` WebDriver client must either be on the `$PATH` or
    /// specified explicitly with the `--safaridriver` flag. `wasm-pack` cannot
    /// download the `safaridriver` WebDriver client for you.
    pub safari: bool,

    #[clap(long = "safaridriver")]
    /// The path to the `safaridriver` WebDriver client for testing in
    /// Safari. Implies `--safari`.
    pub safaridriver: Option<PathBuf>,

    #[clap(long = "headless")]
    /// When running browser tests, run the browser in headless mode without any
    /// UI or windows.
    pub headless: bool,

    #[clap(long = "mode", short = 'm', default_value = "normal")]
    /// Sets steps to be run. [possible values: no-install, normal]
    pub mode: InstallMode,

    #[clap(long = "release", short = 'r')]
    /// Build with the release profile.
    pub release: bool,

    #[clap(long = "panic-unwind")]
    /// Build tests with panic=unwind. Requires the nightly Rust toolchain;
    /// uses `-Z build-std` to rebuild `std` with `-Cpanic=unwind`. The nightly
    /// toolchain, `rust-src` component, and nightly `wasm32-unknown-unknown`
    /// target will be installed via `rustup` if not already present.
    pub panic_unwind: bool,

    /// Path to the Rust crate, and extra options to pass to `cargo test`.
    ///
    /// If the path is not provided, this command searches up the path from the current directory.
    ///
    /// This is a workaround to allow wasm pack to provide the same command line interface as `cargo`.
    /// See <https://github.com/wasm-bindgen/wasm-pack/pull/851> for more information.
    pub path_and_extra_options: Vec<String>,
}

/// A configured `wasm-pack test` command.
pub struct Test {
    crate_path: PathBuf,
    crate_data: manifest::CrateData,
    cache: Cache,
    node: bool,
    mode: InstallMode,
    firefox: bool,
    geckodriver: Option<PathBuf>,
    chrome: bool,
    chromedriver: Option<PathBuf>,
    safari: bool,
    safaridriver: Option<PathBuf>,
    headless: bool,
    release: bool,
    panic_unwind: bool,
    test_runner_path: Option<PathBuf>,
    extra_options: Vec<String>,
    target_triple: String,
}

type TestStep = fn(&mut Test) -> Result<()>;

impl Test {
    /// Construct a test command from the given options.
    pub fn try_from_opts(test_opts: TestOptions) -> Result<Self> {
        let TestOptions {
            node,
            mode,
            headless,
            release,
            panic_unwind,
            chrome,
            chromedriver,
            firefox,
            geckodriver,
            safari,
            safaridriver,
            mut path_and_extra_options,
        } = test_opts;

        let first_arg_is_path = path_and_extra_options
            .get(0)
            .map(|first_arg| !first_arg.starts_with("-"))
            .unwrap_or(false);

        let (path, extra_options) = if first_arg_is_path {
            let path = PathBuf::from_str(&path_and_extra_options.remove(0))?;
            let extra_options = path_and_extra_options;

            (Some(path), extra_options)
        } else {
            (None, path_and_extra_options)
        };

        let crate_path = get_crate_path(path)?;
        let crate_data = manifest::CrateData::new(&crate_path, None)?;
        let any_browser = chrome || firefox || safari;

        // Same target-resolution precedence as `wasm-pack build`:
        //   1. `--target` in extra options after `--`
        //   2. CARGO_BUILD_TARGET env var
        //   3. `[build] target = "..."` in the crate's .cargo/config.toml
        //   4. fallback to wasm32-unknown-unknown
        let target_triple = {
            let mut iter = extra_options.iter();
            let from_args = iter
                .by_ref()
                .find(|o| o.as_str() == "--target")
                .and_then(|_| iter.next())
                .cloned();
            from_args
                .or_else(|| std::env::var("CARGO_BUILD_TARGET").ok())
                .or_else(|| crate::command::build::read_cargo_build_target(&crate_path))
                .unwrap_or_else(|| "wasm32-unknown-unknown".to_string())
        };

        if !node && !any_browser {
            bail!("Must specify at least one of `--node`, `--chrome`, `--firefox`, or `--safari`")
        }

        if headless && !any_browser {
            bail!(
                "The `--headless` flag only applies to browser tests. Node does not provide a UI, \
                 so it doesn't make sense to talk about a headless version of Node tests."
            )
        }

        Ok(Test {
            cache: cache::get_wasm_pack_cache()?,
            crate_path,
            crate_data,
            node,
            mode,
            chrome,
            chromedriver,
            firefox,
            geckodriver,
            safari,
            safaridriver,
            headless,
            release,
            panic_unwind,
            test_runner_path: None,
            target_triple,
            extra_options,
        })
    }

    /// Configures the cache that this test command uses
    pub fn set_cache(&mut self, cache: Cache) {
        self.cache = cache;
    }

    /// Execute this test command.
    pub fn run(mut self) -> Result<()> {
        // `wasm-pack test` runs `#[wasm_bindgen_test]` cases via
        // `wasm-bindgen-test-runner`, which expects a wasm-bindgen-compiled
        // wasm binary. The emscripten target produces a fundamentally
        // different artifact (a staticlib linked through emcc), so the
        // runner can't drive it. Fail fast with a clear message until a
        // dedicated emscripten test runner is available.
        if self.target_triple.ends_with("-emscripten") {
            bail!(
                "`wasm-pack test` does not currently support the {} target. \
                 Run your tests with `cargo test` directly, or target \
                 wasm32-unknown-unknown for `wasm-pack test`.",
                self.target_triple,
            );
        }

        let process_steps = self.get_process_steps();

        let started = Instant::now();
        for (_, process_step) in process_steps {
            process_step(&mut self)?;
        }
        let duration = crate::command::utils::elapsed(started.elapsed());
        info!("Done in {}.", &duration);

        Ok(())
    }

    fn get_process_steps(&self) -> Vec<(&'static str, TestStep)> {
        macro_rules! steps {
            ($($name:ident $(if $e:expr)* ),+) => {
                {
                    let mut steps: Vec<(&'static str, TestStep)> = Vec::new();
                    $(
                        $(if $e)* {
                            steps.push((stringify!($name), Test::$name));
                        }
                    )*
                    steps
                }
            };
            ($($name:ident $(if $e:expr)* ,)*) => (steps![$($name $(if $e)* ),*])
        }
        match self.mode {
            InstallMode::Normal => steps![
                step_check_rustc_version,
                step_check_for_wasm_target,
                step_build_tests,
                step_install_wasm_bindgen,
                step_test_node if self.node,
                step_get_chromedriver if self.chrome && self.chromedriver.is_none(),
                step_test_chrome if self.chrome,
                step_get_geckodriver if self.firefox && self.geckodriver.is_none(),
                step_test_firefox if self.firefox,
                step_get_safaridriver if self.safari && self.safaridriver.is_none(),
                step_test_safari if self.safari,
            ],
            InstallMode::Force => steps![
                step_check_for_wasm_target,
                step_build_tests,
                step_install_wasm_bindgen,
                step_test_node if self.node,
                step_get_chromedriver if self.chrome && self.chromedriver.is_none(),
                step_test_chrome if self.chrome,
                step_get_geckodriver if self.firefox && self.geckodriver.is_none(),
                step_test_firefox if self.firefox,
                step_get_safaridriver if self.safari && self.safaridriver.is_none(),
                step_test_safari if self.safari,
            ],
            InstallMode::Noinstall => steps![
                step_build_tests,
                step_install_wasm_bindgen,
                step_test_node if self.node,
                step_get_chromedriver if self.chrome && self.chromedriver.is_none(),
                step_test_chrome if self.chrome,
                step_get_geckodriver if self.firefox && self.geckodriver.is_none(),
                step_test_firefox if self.firefox,
                step_get_safaridriver if self.safari && self.safaridriver.is_none(),
                step_test_safari if self.safari,
            ],
        }
    }

    fn step_check_rustc_version(&mut self) -> Result<()> {
        // Stable rustc version is irrelevant when --panic-unwind is set.
        if self.panic_unwind {
            info!("Skipping rustc version check (using nightly via --panic-unwind).");
            return Ok(());
        }
        info!("Checking rustc version...");
        let _ = build::check_rustc_version()?;
        info!("Rustc version is correct.");
        Ok(())
    }

    fn step_check_for_wasm_target(&mut self) -> Result<()> {
        if self.panic_unwind {
            info!("Checking nightly toolchain prerequisites for panic=unwind...");
            build::wasm_target::check_nightly_prerequisites()?;
            info!("Nightly prerequisites check was successful.");
            return Ok(());
        }
        info!("Adding wasm-target...");
        build::wasm_target::check_for_wasm_target(&self.target_triple)?;
        info!("Adding wasm-target was successful.");
        Ok(())
    }

    fn step_build_tests(&mut self) -> Result<()> {
        info!("Compiling tests to wasm...");

        // If the user has run `wasm-pack test -- --features "f1" -- test_name`, then we want to only pass through
        // `--features "f1"` to `cargo build`
        let extra_options =
            if let Some(index) = self.extra_options.iter().position(|arg| arg == "--") {
                &self.extra_options[..index]
            } else {
                &self.extra_options
            };
        build::cargo_build_wasm_tests(
            &self.crate_path,
            !self.release,
            extra_options,
            &self.target_triple,
            self.panic_unwind,
        )?;

        info!("Finished compiling tests to wasm.");
        Ok(())
    }

    fn step_install_wasm_bindgen(&mut self) -> Result<()> {
        info!("Identifying wasm-bindgen dependency...");
        let lockfile = Lockfile::new(&self.crate_data)?;
        let bindgen_version = lockfile.require_wasm_bindgen()?;

        // Unlike `wasm-bindgen` and `wasm-bindgen-cli`, `wasm-bindgen-test`
        // will work with any semver compatible `wasm-bindgen-cli`, so just make
        // sure that it is depended upon, so we can run tests on
        // `wasm32-unkown-unknown`. Don't enforce that it is the same version as
        // `wasm-bindgen`.
        if lockfile.wasm_bindgen_test_version().is_none() {
            bail!(
                "Ensure that you have \"{}\" as a dependency in your Cargo.toml file:\n\
                 [dev-dependencies]\n\
                 wasm-bindgen-test = \"0.2\"",
                style("wasm-bindgen-test").bold().dim(),
            )
        }

        let status = install::download_prebuilt_or_cargo_install(
            Tool::WasmBindgen,
            &self.cache,
            &bindgen_version,
            self.mode.install_permitted(),
        )?;

        self.test_runner_path = match status {
            install::Status::Found(dl) => Some(dl.binary("wasm-bindgen-test-runner")?),
            _ => bail!("Could not find 'wasm-bindgen-test-runner'."),
        };

        info!("Getting wasm-bindgen-cli was successful.");
        Ok(())
    }

    fn step_test_node(&mut self) -> Result<()> {
        assert!(self.node);
        info!("Running tests in node...");
        let runner_env = format!(
            "CARGO_TARGET_{}_RUNNER",
            self.target_triple.replace('-', "_").to_uppercase()
        );
        let runner_path = self
            .test_runner_path
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        test::cargo_test_wasm(
            &self.crate_path,
            self.release,
            vec![
                (runner_env, runner_path),
                ("WASM_BINDGEN_TEST_ONLY_NODE".to_string(), "1".to_string()),
            ],
            &self.extra_options,
            &self.target_triple,
        )?;
        info!("Finished running tests in node.");
        Ok(())
    }

    fn step_get_chromedriver(&mut self) -> Result<()> {
        assert!(self.chrome && self.chromedriver.is_none());

        self.chromedriver = Some(webdriver::get_or_install_chromedriver(
            &self.cache,
            self.mode,
        )?);
        Ok(())
    }

    fn step_test_chrome(&mut self) -> Result<()> {
        let chromedriver = self.chromedriver.as_ref().unwrap().display().to_string();
        info!(
            "Running tests in Chrome with chromedriver at {}",
            chromedriver
        );

        let mut envs = self.webdriver_env();
        envs.push(("CHROMEDRIVER".to_string(), chromedriver));

        test::cargo_test_wasm(
            &self.crate_path,
            self.release,
            envs,
            &self.extra_options,
            &self.target_triple,
        )?;
        Ok(())
    }

    fn step_get_geckodriver(&mut self) -> Result<()> {
        assert!(self.firefox && self.geckodriver.is_none());

        self.geckodriver = Some(webdriver::get_or_install_geckodriver(
            &self.cache,
            self.mode,
        )?);
        Ok(())
    }

    fn step_test_firefox(&mut self) -> Result<()> {
        let geckodriver = self.geckodriver.as_ref().unwrap().display().to_string();
        info!(
            "Running tests in Firefox with geckodriver at {}",
            geckodriver
        );

        let mut envs = self.webdriver_env();
        envs.push(("GECKODRIVER".to_string(), geckodriver));

        test::cargo_test_wasm(
            &self.crate_path,
            self.release,
            envs,
            &self.extra_options,
            &self.target_triple,
        )?;
        Ok(())
    }

    fn step_get_safaridriver(&mut self) -> Result<()> {
        assert!(self.safari && self.safaridriver.is_none());

        self.safaridriver = Some(webdriver::get_safaridriver()?);
        Ok(())
    }

    fn step_test_safari(&mut self) -> Result<()> {
        let safaridriver = self.safaridriver.as_ref().unwrap().display().to_string();
        info!(
            "Running tests in Safari with safaridriver at {}",
            safaridriver
        );

        let mut envs = self.webdriver_env();
        envs.push(("SAFARIDRIVER".to_string(), safaridriver));

        test::cargo_test_wasm(
            &self.crate_path,
            self.release,
            envs,
            &self.extra_options,
            &self.target_triple,
        )?;
        Ok(())
    }

    fn webdriver_env(&self) -> Vec<(String, String)> {
        let test_runner = self
            .test_runner_path
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        info!("Using wasm-bindgen test runner at {}", test_runner);
        let runner_env = format!(
            "CARGO_TARGET_{}_RUNNER",
            self.target_triple.replace('-', "_").to_uppercase()
        );
        let mut envs = vec![
            (runner_env, test_runner),
            ("WASM_BINDGEN_TEST_ONLY_WEB".to_string(), "1".to_string()),
        ];
        if !self.headless {
            envs.push(("NO_HEADLESS".to_string(), "1".to_string()));
        }
        envs
    }
}
