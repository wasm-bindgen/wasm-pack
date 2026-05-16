//! Implementation of the `wasm-pack build` command.

use crate::bindgen;
use crate::build;
use crate::cache;
use crate::command::utils::{create_pkg_dir, get_crate_path};
use crate::emoji;
use crate::install::{self, InstallMode, Tool};
use crate::license;
use crate::lockfile::Lockfile;
use crate::manifest;
use crate::readme;
use crate::wasm_opt;
use crate::PBAR;
use anyhow::{anyhow, bail, Error, Result};
use binary_install::Cache;
use clap::Args;
use log::info;
use path_clean::PathClean;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

/// Everything required to configure and run the `wasm-pack build` command.
#[allow(missing_docs)]
pub struct Build {
    pub crate_path: PathBuf,
    pub crate_data: manifest::CrateData,
    pub scope: Option<String>,
    pub disable_dts: bool,
    pub weak_refs: bool,
    pub reference_types: bool,
    pub browser_only: bool,
    pub target: Target,
    pub no_pack: bool,
    pub no_opt: bool,
    pub profile: BuildProfile,
    pub mode: InstallMode,
    pub out_dir: PathBuf,
    pub out_name: Option<String>,
    pub bindgen: Option<install::Status>,
    pub cache: Cache,
    pub extra_options: Vec<String>,
    pub panic_unwind: bool,
    target_triple: String,
    wasm_path: Option<String>,
}

/// What sort of output we're going to be generating and flags we're invoking
/// `wasm-bindgen` with.
#[derive(Clone, Copy, Debug)]
pub enum Target {
    /// Default output mode or `--target bundler`, indicates output will be
    /// used with a bundle in a later step.
    Bundler,
    /// Correspond to `--target web` where the output is natively usable as an
    /// ES module in a browser and the wasm is manually instantiated.
    Web,
    /// Correspond to `--target nodejs` where the output is natively usable as
    /// a Node.js module loaded with `require`.
    Nodejs,
    /// Correspond to `--target no-modules` where the output is natively usable
    /// in a browser but pollutes the global namespace and must be manually
    /// instantiated.
    NoModules,
    /// Correspond to `--target deno` where the output is natively usable as
    /// a Deno module loaded with `import`.
    Deno,
}

impl Default for Target {
    fn default() -> Target {
        Target::Bundler
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Target::Bundler => "bundler",
            Target::Web => "web",
            Target::Nodejs => "nodejs",
            Target::NoModules => "no-modules",
            Target::Deno => "deno",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Target {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "bundler" | "browser" => Ok(Target::Bundler),
            "web" => Ok(Target::Web),
            "nodejs" => Ok(Target::Nodejs),
            "no-modules" => Ok(Target::NoModules),
            "deno" => Ok(Target::Deno),
            _ => bail!("Unknown target: {}", s),
        }
    }
}

/// The build profile controls whether optimizations, debug info, and assertions
/// are enabled or disabled.
#[derive(Clone, Debug)]
pub enum BuildProfile {
    /// Enable assertions and debug info. Disable optimizations.
    Dev,
    /// Enable optimizations. Disable assertions and debug info.
    Release,
    /// Enable optimizations and debug info. Disable assertions.
    Profiling,
    /// User-defined profile with --profile flag
    Custom(String),
}

/// Everything required to configure and run the `wasm-pack build` command.
#[derive(Debug, Args)]
#[command(allow_hyphen_values = true, trailing_var_arg = true)]
pub struct BuildOptions {
    /// The path to the Rust crate. If not set, searches up the path from the current directory.
    #[clap()]
    pub path: Option<PathBuf>,

    /// The npm scope to use in package.json, if any.
    #[clap(long = "scope", short = 's')]
    pub scope: Option<String>,

    #[clap(long = "mode", short = 'm', default_value = "normal")]
    /// Sets steps to be run. [possible values: no-install, normal, force]
    pub mode: InstallMode,

    #[clap(long = "no-typescript")]
    /// By default a *.d.ts file is generated for the generated JS file, but
    /// this flag will disable generating this TypeScript file.
    pub disable_dts: bool,

    #[clap(long = "weak-refs")]
    /// Enable usage of the JS weak references proposal.
    pub weak_refs: bool,

    #[clap(long = "reference-types")]
    /// Enable usage of WebAssembly reference types.
    pub reference_types: bool,

    #[clap(long = "browser")]
    /// Hint that the generated JS should only be compatible with a browser.
    /// Only supported when using the 'bundler' target.
    pub browser_only: bool,

    #[clap(long = "target", short = 't', default_value = "bundler")]
    /// Sets the target environment. [possible values: bundler, nodejs, web, no-modules, deno]
    pub target: Target,

    #[clap(long = "debug")]
    /// Deprecated. Renamed to `--dev`.
    pub debug: bool,

    #[clap(long = "dev")]
    /// Create a development build. Enable debug info, and disable
    /// optimizations.
    pub dev: bool,

    #[clap(long = "release")]
    /// Create a release build. Enable optimizations and disable debug info.
    pub release: bool,

    #[clap(long = "profiling")]
    /// Create a profiling build. Enable optimizations and debug info.
    pub profiling: bool,

    #[clap(long = "profile")]
    /// User-defined profile with --profile flag
    pub profile: Option<String>,

    #[clap(long = "out-dir", short = 'd', default_value = "pkg")]
    /// Sets the output directory with a relative path.
    pub out_dir: String,

    #[clap(long = "out-name")]
    /// Sets the output file names. Defaults to package name.
    pub out_name: Option<String>,

    #[clap(long = "no-pack", alias = "no-package")]
    /// Option to not generate a package.json
    pub no_pack: bool,

    #[clap(long = "no-opt", alias = "no-optimization")]
    /// Option to skip optimization with wasm-opt
    pub no_opt: bool,

    #[clap(long = "panic-unwind")]
    /// Build with panic=unwind. Requires the nightly Rust toolchain; uses
    /// `-Z build-std` to rebuild `std` with `-Cpanic=unwind` so panics can be
    /// caught at FFI boundaries instead of aborting the WebAssembly instance.
    /// The nightly toolchain, `rust-src` component, and nightly
    /// `wasm32-unknown-unknown` target will be installed via `rustup` if not
    /// already present.
    pub panic_unwind: bool,

    /// List of extra options to pass to `cargo build`
    pub extra_options: Vec<String>,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            path: None,
            scope: None,
            mode: InstallMode::default(),
            disable_dts: false,
            weak_refs: false,
            reference_types: false,
            browser_only: false,
            target: Target::default(),
            debug: false,
            dev: false,
            no_pack: false,
            no_opt: false,
            release: false,
            profiling: false,
            profile: None,
            out_dir: String::new(),
            out_name: None,
            panic_unwind: false,
            extra_options: Vec::new(),
        }
    }
}

type BuildStep = fn(&mut Build) -> Result<()>;

impl Build {
    /// Construct a build command from the given options.
    pub fn try_from_opts(mut build_opts: BuildOptions) -> Result<Self> {
        if let Some(path) = &build_opts.path {
            if path.to_string_lossy().starts_with("--") {
                let path = build_opts.path.take().unwrap();
                build_opts
                    .extra_options
                    .insert(0, path.to_string_lossy().into_owned());
            }
        }
        let crate_path = get_crate_path(build_opts.path)?;
        let crate_data = manifest::CrateData::new(&crate_path, build_opts.out_name.clone())?;
        let out_dir = crate_path.join(PathBuf::from(build_opts.out_dir)).clean();

        let dev = build_opts.dev || build_opts.debug;
        let profile = match (
            dev,
            build_opts.release,
            build_opts.profiling,
            build_opts.profile,
        ) {
            (false, false, false, None) | (false, true, false, None) => BuildProfile::Release,
            (true, false, false, None) => BuildProfile::Dev,
            (false, false, true, None) => BuildProfile::Profiling,
            (false, false, false, Some(profile)) => BuildProfile::Custom(profile),
            // Unfortunately, `clap` doesn't expose clap's `conflicts_with`
            // functionality yet, so we have to implement it ourselves.
            _ => bail!("Can only supply one of the --dev, --release, --profiling, or --profile 'name' flags"),
        };

        let extra_options = build_opts.extra_options;

        let target_triple = {
            let mut extra_options_iter = extra_options.iter();
            if extra_options_iter
                .by_ref()
                .any(|option| option == "--target")
            {
                extra_options_iter.next().map(|s| s.as_str())
            } else {
                None
            }
            .unwrap_or("wasm32-unknown-unknown")
        };

        Ok(Build {
            crate_path,
            crate_data,
            scope: build_opts.scope,
            disable_dts: build_opts.disable_dts,
            weak_refs: build_opts.weak_refs,
            reference_types: build_opts.reference_types,
            browser_only: build_opts.browser_only,
            target: build_opts.target,
            no_pack: build_opts.no_pack,
            no_opt: build_opts.no_opt,
            profile,
            mode: build_opts.mode,
            out_dir,
            out_name: build_opts.out_name,
            bindgen: None,
            cache: cache::get_wasm_pack_cache()?,
            target_triple: target_triple.to_owned(),
            extra_options,
            panic_unwind: build_opts.panic_unwind,
            wasm_path: None,
        })
    }

    /// Configures the global binary cache used for this build
    pub fn set_cache(&mut self, cache: Cache) {
        self.cache = cache;
    }

    /// Execute this `Build` command.
    pub fn run(&mut self) -> Result<()> {
        let process_steps = Build::get_process_steps(self.mode, self.no_pack, self.no_opt);

        let started = Instant::now();

        for (_, process_step) in process_steps {
            process_step(self)?;
        }

        let duration = crate::command::utils::elapsed(started.elapsed());
        info!("Done in {}.", &duration);
        info!(
            "Your wasm pkg is ready to publish at {}.",
            self.out_dir.display()
        );

        PBAR.info(&format!("{} Done in {}", emoji::SPARKLE, &duration));

        PBAR.info(&format!(
            "{} Your wasm pkg is ready to publish at {}.",
            emoji::PACKAGE,
            self.out_dir.display()
        ));
        Ok(())
    }

    fn get_process_steps(
        mode: InstallMode,
        no_pack: bool,
        no_opt: bool,
    ) -> Vec<(&'static str, BuildStep)> {
        macro_rules! steps {
            ($($name:ident),+) => {
                {
                let mut steps: Vec<(&'static str, BuildStep)> = Vec::new();
                    $(steps.push((stringify!($name), Build::$name));)*
                        steps
                    }
                };
            ($($name:ident,)*) => (steps![$($name),*])
        }
        let mut steps = Vec::new();
        match &mode {
            InstallMode::Force => {}
            _ => {
                steps.extend(steps![
                    step_check_rustc_version,
                    step_check_crate_config,
                    step_check_for_wasm_target,
                ]);
            }
        }

        steps.extend(steps![
            step_build_wasm,
            step_create_dir,
            step_install_wasm_bindgen,
            step_run_wasm_bindgen,
        ]);

        if !no_opt {
            steps.extend(steps![step_run_wasm_opt]);
        }

        if !no_pack {
            steps.extend(steps![
                step_create_json,
                step_copy_readme,
                step_copy_license,
            ]);
        }

        steps
    }

    fn step_check_rustc_version(&mut self) -> Result<()> {
        // The stable rustc version is irrelevant when --panic-unwind is set,
        // since cargo will be invoked via `+nightly`.
        if self.panic_unwind {
            info!("Skipping rustc version check (using nightly via --panic-unwind).");
            return Ok(());
        }
        info!("Checking rustc version...");
        let version = build::check_rustc_version()?;
        let msg = format!("rustc version is {}.", version);
        info!("{}", &msg);
        Ok(())
    }

    fn step_check_crate_config(&mut self) -> Result<()> {
        info!("Checking crate configuration...");
        self.crate_data.check_crate_config()?;
        info!("Crate is correctly configured.");
        Ok(())
    }

    fn step_check_for_wasm_target(&mut self) -> Result<()> {
        if self.panic_unwind {
            info!("Checking nightly toolchain prerequisites for panic=unwind...");
            build::wasm_target::check_nightly_prerequisites()?;
            info!("Nightly prerequisites check was successful.");
            return Ok(());
        }
        info!("Checking for wasm-target...");
        build::wasm_target::check_for_wasm_target(&self.target_triple)?;
        info!("Checking for wasm-target was successful.");
        Ok(())
    }

    fn step_build_wasm(&mut self) -> Result<()> {
        info!("Building wasm...");
        let wasm_path = build::cargo_build_wasm(
            &self.crate_path,
            self.profile.clone(),
            &self.extra_options,
            &self.target_triple,
            self.panic_unwind,
        )?;
        info!("wasm built at {wasm_path:#?}.");
        self.wasm_path = Some(wasm_path);
        Ok(())
    }

    fn step_create_dir(&mut self) -> Result<()> {
        info!("Creating a pkg directory...");
        create_pkg_dir(&self.out_dir)?;
        info!("Created a pkg directory at {:#?}.", &self.crate_path);
        Ok(())
    }

    fn step_create_json(&mut self) -> Result<()> {
        self.crate_data.write_package_json(
            &self.out_dir,
            &self.scope,
            self.disable_dts,
            self.target,
        )?;
        info!(
            "Wrote a package.json at {:#?}.",
            &self.out_dir.join("package.json")
        );
        Ok(())
    }

    fn step_copy_readme(&mut self) -> Result<()> {
        info!("Copying readme from crate...");
        readme::copy_from_crate(&self.crate_data, &self.crate_path, &self.out_dir)?;
        info!("Copied readme from crate to {:#?}.", &self.out_dir);
        Ok(())
    }

    fn step_copy_license(&mut self) -> Result<()> {
        info!("Copying license from crate...");
        license::copy_from_crate(&self.crate_data, &self.crate_path, &self.out_dir)?;
        info!("Copied license from crate to {:#?}.", &self.out_dir);
        Ok(())
    }

    fn step_install_wasm_bindgen(&mut self) -> Result<()> {
        info!("Identifying wasm-bindgen dependency...");
        let lockfile = Lockfile::new(&self.crate_data)?;
        let bindgen_version = lockfile.require_wasm_bindgen()?;
        info!("Installing wasm-bindgen-cli...");
        let bindgen = install::download_prebuilt_or_cargo_install(
            Tool::WasmBindgen,
            &self.cache,
            bindgen_version,
            self.mode.install_permitted(),
        )?;
        self.bindgen = Some(bindgen);
        info!("Installing wasm-bindgen-cli was successful.");
        Ok(())
    }

    fn step_run_wasm_bindgen(&mut self) -> Result<()> {
        info!("Building the wasm bindings...");
        bindgen::wasm_bindgen_build(
            self.wasm_path.as_ref().unwrap(),
            &self.crate_data,
            self.bindgen.as_ref().unwrap(),
            &self.out_dir,
            &self.out_name,
            self.disable_dts,
            self.weak_refs,
            self.reference_types,
            self.browser_only,
            self.target,
            self.profile.clone(),
        )?;
        info!("wasm bindings were built at {:#?}.", &self.out_dir);
        Ok(())
    }

    fn step_run_wasm_opt(&mut self) -> Result<()> {
        let mut args = match self
            .crate_data
            .configured_profile(self.profile.clone())
            .wasm_opt_args()
        {
            Some(args) => args,
            None => return Ok(()),
        };
        if self.reference_types {
            args.push("--enable-reference-types".into());
        }
        if self.target_triple.starts_with("wasm64") {
            args.push("--enable-memory64".into());
        }
        info!("executing wasm-opt with {:?}", args);
        wasm_opt::run(
            &self.cache,
            &self.out_dir,
            &args,
            self.mode.install_permitted(),
        ).map_err(|e| {
            anyhow!(
                "{}\nTo disable `wasm-opt`, add `wasm-opt = false` to your package metadata in your `Cargo.toml`.", e
            )
        })
    }
}
