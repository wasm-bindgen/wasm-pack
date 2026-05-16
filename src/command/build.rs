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
use anyhow::{anyhow, bail, Context, Error, Result};
use binary_install::Cache;
use clap::Args;
use log::info;
use path_clean::PathClean;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;
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
    /// Correspond to `--target module` where the output uses source phase
    /// imports syntax to obtain the compiled WebAssembly module.
    Module,
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
            Target::Module => "module",
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
            "module" => Ok(Target::Module),
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

    #[clap(long = "target", short = 't', default_value = "bundler")]
    /// Sets the target environment. [possible values: bundler, nodejs, web, no-modules, deno, module]
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

        // Resolve the target triple in order of decreasing specificity:
        //   1. `--target` in extra options (after `--`)
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
                .or_else(|| read_cargo_build_target(&crate_path))
                .unwrap_or_else(|| "wasm32-unknown-unknown".to_string())
        };

        Ok(Build {
            crate_path: crate_path.clone(),
            crate_data,
            scope: build_opts.scope,
            disable_dts: build_opts.disable_dts,
            weak_refs: build_opts.weak_refs,
            reference_types: build_opts.reference_types,
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

    /// Returns true if this build targets emscripten (e.g.
    /// `wasm32-unknown-emscripten`). The emscripten target requires a
    /// completely different build pipeline: cargo produces a staticlib
    /// (`.a`), which we then drive through `emcc` for linking,
    /// `wasm-bindgen` for JS bindings, and `emcc --post-link` for the
    /// final ESM + wasm pair.
    fn is_emscripten(&self) -> bool {
        self.target_triple.ends_with("-emscripten")
    }

    /// Execute this `Build` command.
    pub fn run(&mut self) -> Result<()> {
        let process_steps = if self.is_emscripten() {
            Build::get_process_steps_emscripten(self.mode, self.no_pack)
        } else {
            Build::get_process_steps(self.mode, self.no_pack, self.no_opt)
        };

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

    /// Build steps for the `wasm32-*-emscripten` target.
    ///
    /// Phases:
    ///   1. `cargo build` → staticlib `.a`
    ///   2. `emcc` link → bare `.wasm` (no JS, exports preserved)
    ///   3. `wasm-bindgen` → rewritten `.wasm` + `library_bindgen.js`
    ///   4. `emcc --post-link` → final `<name>.mjs` (ESM) + `<name>.wasm`
    ///
    /// No `wasm-opt` step — emcc handles optimization itself.
    fn get_process_steps_emscripten(
        mode: InstallMode,
        no_pack: bool,
    ) -> Vec<(&'static str, BuildStep)> {
        macro_rules! steps {
            ($($name:ident),+) => {{
                let mut steps: Vec<(&'static str, BuildStep)> = Vec::new();
                $(steps.push((stringify!($name), Build::$name));)*
                steps
            }};
            ($($name:ident,)*) => (steps![$($name),*])
        }
        let mut steps = Vec::new();
        if !matches!(mode, InstallMode::Force) {
            steps.extend(steps![
                step_check_rustc_version,
                step_check_crate_config,
                step_check_for_wasm_target,
                step_check_for_emcc,
            ]);
        }
        steps.extend(steps![
            step_build_wasm_emscripten,
            step_create_dir,
            step_emcc_link,
            step_install_wasm_bindgen,
            step_run_wasm_bindgen,
            step_emcc_post_link,
        ]);
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
        self.crate_data.check_crate_config(self.is_emscripten())?;
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
            self.is_emscripten(),
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
            self.target,
            self.profile.clone(),
            self.is_emscripten(),
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

    // ---- emscripten pipeline steps -----------------------------------------

    /// Verify that `emcc` is reachable. Required for the emscripten pipeline.
    ///
    /// Discovery order:
    ///   1. `emcc` on `PATH` (the normal case after `source emsdk_env.sh`)
    ///   2. `$EMSDK/upstream/emscripten/emcc` (CI-friendly env-var fallback)
    ///   3. `~/emsdk/upstream/emscripten/emcc` (manual-install convention)
    ///
    /// If found via 2 or 3, the discovered directories are prepended to `PATH`
    /// so subsequent emcc invocations Just Work. If not found at all, prints a
    /// helpful setup message with a link to the emscripten install docs.
    fn step_check_for_emcc(&mut self) -> Result<()> {
        info!("Checking for emcc...");
        let emcc_path = if let Ok(p) = which::which("emcc") {
            info!("Found emcc at {p:?}.");
            p
        } else if try_locate_emsdk_and_extend_path() {
            // PATH was just updated; re-probe.
            match which::which("emcc") {
                Ok(p) => {
                    info!("Found emcc at {p:?} (via $EMSDK).");
                    p
                }
                Err(_) => bail!(emcc_missing_install_message()),
            }
        } else {
            bail!(emcc_missing_install_message());
        };

        // Soft version check: warn if emcc is older than the minimum we
        // know to handle. `-sSOURCE_PHASE_IMPORTS=1` landed in 3.1.60;
        // `--post-link` semantics have been stable since 3.1.0. Anything
        // older than 3.1.60 will likely fail at build time, so we warn
        // up front for clearer diagnostics.
        warn_if_emcc_too_old(&emcc_path);
        Ok(())
    }

    /// `cargo build` variant for the emscripten target. Same as the default
    /// step but locates the produced staticlib (`.a`) instead of a `.wasm`.
    fn step_build_wasm_emscripten(&mut self) -> Result<()> {
        info!("Building staticlib for wasm32-unknown-emscripten...");
        let lib_path = build::cargo_build_staticlib(
            &self.crate_path,
            self.profile.clone(),
            &self.extra_options,
            &self.target_triple,
            self.panic_unwind,
        )?;
        info!("staticlib built at {lib_path:?}.");
        self.wasm_path = Some(lib_path);
        Ok(())
    }

    /// Phase 2: `emcc` link.
    ///
    /// Turns the cargo-produced `.a` into a bare `.wasm` ready for
    /// wasm-bindgen. Preserves all `rs_*`, `__wbg_*`, `__wbindgen_*`,
    /// `__externref_*` extern symbols via `-sEXPORTED_FUNCTIONS` so they
    /// survive wasm-ld's dead-stripping.
    fn step_emcc_link(&mut self) -> Result<()> {
        info!("Linking staticlib with emcc...");
        let lib_path = self
            .wasm_path
            .as_ref()
            .context("step_emcc_link called before cargo build set wasm_path")?
            .clone();
        let out_wasm = self.intermediate_wasm_path();
        emcc_link(&lib_path, &out_wasm)?;
        // step_run_wasm_bindgen reads `wasm_path` as its input — point it at
        // the freshly-linked wasm so wasm-bindgen processes the right file.
        self.wasm_path = Some(out_wasm.to_string_lossy().into_owned());
        info!("emcc link produced {out_wasm:?}.");
        Ok(())
    }

    /// Phase 4: `emcc --post-link`.
    ///
    /// Combines the wasm-bindgen-rewritten wasm with `library_bindgen.js`
    /// (also produced by wasm-bindgen) to emit the final ESM module and
    /// wasm pair. Uses source-phase imports + `MODULARIZE=1 EXPORT_ES6=1`
    /// so the output is a clean async factory function.
    fn step_emcc_post_link(&mut self) -> Result<()> {
        info!("Running emcc --post-link...");
        let name = self.bindings_basename();
        // wasm-bindgen wrote these into out_dir during step_run_wasm_bindgen:
        let in_wasm = self.out_dir.join(format!("{name}_bg.wasm"));
        let in_library = self.out_dir.join("library_bindgen.js");
        let bindgen_dts = self.out_dir.join(format!("{name}.d.ts"));

        // Drop stale artifacts from prior non-emscripten builds in the
        // same out_dir. The non-emscripten flow ships `<name>.js`, the
        // emscripten flow ships `<name>.mjs`; both can co-exist as
        // leftovers and confuse downstream tooling.
        for stale_ext in ["js", "cjs"] {
            let _ = std::fs::remove_file(self.out_dir.join(format!("{name}.{stale_ext}")));
        }

        let post_link_settings = emcc_post_link_settings_for(self.target)?;
        let out_js = self
            .out_dir
            .join(format!("{name}.{}", post_link_settings.extension));
        // `--no-opt` skips optimization regardless of profile, matching the
        // non-emscripten pipeline's contract. Users hitting toolchain
        // limitations (e.g. emcc's bundled JS optimizer not yet parsing
        // source-phase imports for `--target module`) can pass `--no-opt`
        // or set `wasm-opt = false` in Cargo.toml's wasm-pack metadata.
        let opt_level = if self.no_opt {
            "-O0"
        } else {
            emcc_opt_level_for(&self.profile)
        };

        // TODO: re-enable emcc's `--emit-tsd` once
        // https://github.com/emscripten-core/emscripten (TSD multi-value PR)
        // lands. emcc currently asserts in its TSD generator on any wasm
        // function with multiple return values, and wasm-bindgen emits
        // those for every Rust-side String / Vec / Result / struct
        // return. The merge code path (`merge_emscripten_and_bindgen_dts`)
        // is kept ready below — flip `emscripten_dts` from `None` to
        // `Some(...)` once emcc cooperates and the merge will produce a
        // union `.d.ts` describing both EmscriptenModule and BindgenModule.
        let emscripten_dts: Option<PathBuf> = None;
        let _ = (
            &bindgen_dts,
            merge_emscripten_and_bindgen_dts as fn(&Path, &Path) -> Result<()>,
        );

        emcc_post_link(
            &in_wasm,
            &in_library,
            &out_js,
            emscripten_dts.as_deref(),
            &post_link_settings,
            opt_level,
        )?;
        info!("emcc --post-link produced {out_js:?}.");

        // Clean up intermediate artifacts that shouldn't ship in pkg/.
        // `_bg.wasm` (pre-post-link) is superseded by the post-linked
        // `<name>.wasm`; `library_bindgen.js` was a build-time-only DSL file.
        for stale in [
            in_library,
            in_wasm,
            self.out_dir.join(format!("{name}_bg.wasm.d.ts")),
        ] {
            let _ = std::fs::remove_file(&stale);
        }
        Ok(())
    }

    /// Path used for the bare wasm produced by `step_emcc_link` and consumed
    /// by `step_run_wasm_bindgen`. Lives in cargo's target dir, not pkg/,
    /// because it's an intermediate artifact.
    ///
    /// The filename matches what wasm-bindgen would have produced from a
    /// regular `wasm32-unknown-unknown` build (`<crate_name>.wasm`), so its
    /// downstream output filenames (`<crate_name>_bg.wasm`, etc.) match the
    /// same conventions wasm-pack already uses for the non-emscripten path.
    fn intermediate_wasm_path(&self) -> PathBuf {
        let profile_dir = match &self.profile {
            BuildProfile::Release | BuildProfile::Profiling => "release",
            BuildProfile::Dev => "debug",
            BuildProfile::Custom(name) => name.as_str(),
        };
        self.crate_path
            .join("target")
            .join(&self.target_triple)
            .join(profile_dir)
            .join(format!("{}.wasm", self.bindings_basename()))
    }

    /// Basename wasm-bindgen uses for its output files. Mirrors
    /// `bindgen::wasm_bindgen_build`'s logic: out_name override wins,
    /// otherwise the crate's library name.
    fn bindings_basename(&self) -> String {
        self.out_name
            .clone()
            .unwrap_or_else(|| self.crate_data.crate_name())
    }
}

/// Read `[build] target = "..."` from the crate's `.cargo/config.toml`,
/// if present. Returns `None` if the file is missing, malformed, or
/// doesn't set a target.
///
/// Mirrors cargo's own discovery: walk up from `crate_path` checking
/// `.cargo/config.toml` at each ancestor (workspace-aware) and finally
/// check `$CARGO_HOME/config.toml` for user-level defaults. The first
/// file that declares `[build] target = "..."` wins.
pub(crate) fn read_cargo_build_target(crate_path: &std::path::Path) -> Option<String> {
    for dir in crate_path.ancestors() {
        if let Some(target) = parse_build_target(&dir.join(".cargo/config.toml")) {
            return Some(target);
        }
    }
    // Cargo falls back to $CARGO_HOME/config.toml (default ~/.cargo/config.toml)
    // for user-wide settings. Honour the same precedence.
    let cargo_home = std::env::var_os("CARGO_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".cargo")))?;
    parse_build_target(&cargo_home.join("config.toml"))
}

/// Parse `[build] target = "..."` from a single config file, if it
/// exists and is well-formed. Returns `None` for missing files or
/// configs that don't declare a target.
fn parse_build_target(path: &std::path::Path) -> Option<String> {
    let cfg = std::fs::read_to_string(path).ok()?;
    let parsed: toml::Value = toml::from_str(&cfg).ok()?;
    parsed
        .get("build")?
        .get("target")?
        .as_str()
        .map(str::to_owned)
}

/// Look for emsdk in well-known locations and prepend its directories to
/// `PATH` so subsequent emcc/llvm-nm invocations resolve correctly.
///
/// Probes `$EMSDK` first (set by `source emsdk_env.sh`) and then `~/emsdk`.
/// Returns true if a candidate was found and PATH was updated.
fn try_locate_emsdk_and_extend_path() -> bool {
    let candidates: Vec<PathBuf> = [
        std::env::var_os("EMSDK").map(PathBuf::from),
        dirs::home_dir().map(|h| h.join("emsdk")),
    ]
    .into_iter()
    .flatten()
    .collect();

    for root in candidates {
        let emcc = root.join("upstream/emscripten/emcc");
        if !emcc.exists() {
            continue;
        }
        let mut new_dirs = vec![root.clone(), root.join("upstream/emscripten")];
        // Also pick up emsdk's bundled python and node if present, so emcc's
        // child processes find a matching toolchain.
        for sub in ["python", "node"] {
            if let Ok(entries) = std::fs::read_dir(root.join(sub)) {
                for entry in entries.flatten() {
                    let bin = entry.path().join("bin");
                    if bin.is_dir() {
                        new_dirs.push(bin);
                    }
                }
            }
        }
        let current = std::env::var_os("PATH").unwrap_or_default();
        let mut all = new_dirs;
        all.extend(std::env::split_paths(&current));
        if let Ok(joined) = std::env::join_paths(&all) {
            std::env::set_var("PATH", joined);
            return true;
        }
    }
    false
}

/// Minimum emscripten version we support. `-sSOURCE_PHASE_IMPORTS=1`
/// landed in 3.1.60; building against anything older will fail at link
/// time with an obscure flag error. We surface it as a warning up front.
const MIN_EMCC_VERSION: (u32, u32, u32) = (3, 1, 60);

/// Warn (don't fail) if the discovered emcc is older than `MIN_EMCC_VERSION`.
///
/// Parses the first line of `emcc --version` which looks like:
///   emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 3.1.60 (...)
///
/// We're lenient — if we can't parse the version for any reason we just
/// skip the check rather than blocking the user. The actual build will
/// fail explicitly if the version is too old.
fn warn_if_emcc_too_old(emcc_path: &Path) {
    let Ok(output) = Command::new(emcc_path).arg("--version").output() else {
        return;
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(version) = parse_emcc_version(&stdout) else {
        return;
    };
    if version < MIN_EMCC_VERSION {
        let (a, b, c) = version;
        let (x, y, z) = MIN_EMCC_VERSION;
        PBAR.warn(&format!(
            "Detected emcc {a}.{b}.{c}, but wasm-pack expects at least \
             {x}.{y}.{z}. Older versions lack the `-sSOURCE_PHASE_IMPORTS=1` \
             flag and may fail. Upgrade with `emsdk install latest && \
             emsdk activate latest`."
        ));
    } else {
        log::info!(
            "emcc version OK ({}.{}.{}).",
            version.0,
            version.1,
            version.2
        );
    }
}

/// Extract the (major, minor, patch) tuple from `emcc --version` output.
///
/// Scans whitespace-separated tokens on the first line and returns the
/// first that parses as `N.N.N` (with optional `-suffix` on the patch).
fn parse_emcc_version(stdout: &str) -> Option<(u32, u32, u32)> {
    let first = stdout.lines().next()?;
    for token in first.split_whitespace() {
        if let Some(v) = parse_dotted_version(token) {
            return Some(v);
        }
    }
    None
}

/// Parse `N.N.N` or `N.N.N-suffix` from a single token. Returns `None`
/// if any of the three components aren't all-digit (after stripping the
/// trailing suffix, if any, from the patch).
fn parse_dotted_version(token: &str) -> Option<(u32, u32, u32)> {
    let mut parts = token.split('.');
    let major = parts.next()?.parse::<u32>().ok()?;
    let minor = parts.next()?.parse::<u32>().ok()?;
    let patch_tok = parts.next()?;
    if parts.next().is_some() {
        // Four or more dotted components — not a version we recognise.
        return None;
    }
    let patch_digits: String = patch_tok
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if patch_digits.is_empty() {
        return None;
    }
    let patch = patch_digits.parse::<u32>().ok()?;
    Some((major, minor, patch))
}

/// Stable text returned from emcc-missing diagnostics. Pulled into a fn so
/// both branches of `step_check_for_emcc` share the same wording.
fn emcc_missing_install_message() -> String {
    "emcc not found. The Emscripten SDK is required to build the \
     wasm32-unknown-emscripten target.\n\n\
     To install it:\n\
       git clone https://github.com/emscripten-core/emsdk.git ~/emsdk\n\
       cd ~/emsdk && ./emsdk install latest && ./emsdk activate latest\n\
       source ./emsdk_env.sh\n\n\
     Or follow the official guide: https://emscripten.org/docs/getting_started/downloads.html\n\n\
     Once installed, re-run `wasm-pack build`."
        .to_string()
}

/// Invoke `emcc` to link a staticlib into a bare `.wasm` (no JS yet).
///
/// `--no-entry --oformat=bare` stops emcc after wasm-ld, before it would
/// generate any JS runtime. `-sEXPORTED_FUNCTIONS=` is populated by
/// scanning the staticlib for the symbols wasm-bindgen needs so they
/// survive dead-stripping.
fn emcc_link(lib_path: &str, out_wasm: &PathBuf) -> Result<()> {
    if let Some(parent) = out_wasm.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating intermediate dir {parent:?}"))?;
    }

    let exports = collect_wasm_bindgen_exports(lib_path)?;
    let exports_joined = exports.join(",");

    let mut cmd = Command::new("emcc");
    cmd.arg(lib_path)
        .arg("--no-entry")
        .arg("--oformat=bare")
        .arg("-Wno-experimental")
        .arg(format!("-sEXPORTED_FUNCTIONS={exports_joined}"))
        .arg("-o")
        .arg(out_wasm);

    let status = cmd.status().context("running emcc to link staticlib")?;
    if !status.success() {
        bail!("emcc link exited with status {status}");
    }
    Ok(())
}

/// Scan a staticlib for the wasm-bindgen-relevant extern symbols using
/// `llvm-nm`. Returns names with the emscripten `_`-prefix convention
/// applied (emcc strips one leading underscore before passing to wasm-ld).
fn collect_wasm_bindgen_exports(lib_path: &str) -> Result<Vec<String>> {
    let llvm_nm = which::which("llvm-nm")
        .or_else(|_| which::which("nm"))
        .context("llvm-nm not found on PATH; required to discover wasm-bindgen export symbols")?;

    let output = Command::new(&llvm_nm)
        .arg("--defined-only")
        .arg("--extern-only")
        .arg("--format=just-symbols")
        .arg(lib_path)
        .output()
        .with_context(|| format!("running {llvm_nm:?} on {lib_path:?}"))?;
    if !output.status.success() {
        bail!("{llvm_nm:?} exited with status {}", output.status);
    }

    // wasm-bindgen-known prefixes. Plus user-exported names — those have
    // no leading underscore (a #[wasm_bindgen] pub fn foo emits `foo`),
    // so we accept names that don't start with `_` and aren't mangled
    // (no `_Z`/`_R`/`anon.` markers). System symbols like `__addtf3`,
    // `__wasm_call_ctors`, `emscripten_*` etc. start with underscores
    // (or `emscripten_`) and are excluded — emcc pulls them in itself.
    let mut names: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        // llvm-nm emits per-object-file headers like `foo.o:` interspersed
        // with the symbol list — skip them.
        .filter(|s| !s.ends_with(':'))
        .filter(|s| {
            s.starts_with("__wbg_")
                || s.starts_with("__wbindgen_")
                || s.starts_with("__externref_")
                || (!s.starts_with('_') && !s.starts_with("anon.") && !s.starts_with("emscripten_"))
        })
        .map(|s| format!("_{s}"))
        .collect();
    names.sort();
    names.dedup();
    Ok(names)
}

/// Settings derived from a wasm-pack `--target` value, used to drive
/// `emcc --post-link` so the produced module matches what consumers of
/// that target expect.
struct EmccPostLinkSettings {
    /// Comma-separated value for `-sENVIRONMENT` (e.g. `web,node`, `node`).
    /// Restricts which environment-detection paths emcc bakes into the JS,
    /// which both shrinks the output and avoids runtime probe noise.
    environment: &'static str,
    /// Whether to use source-phase `import source` for the wasm module.
    /// Requires the host to support the proposal (modern bundlers, Node 22+,
    /// Deno). We enable it for `--target module`.
    source_phase_imports: bool,
    /// File extension for the produced JS module. `mjs` everywhere — we
    /// don't emit legacy CJS.
    extension: &'static str,
}

/// Map a wasm-pack `--target` value to a coherent set of emcc post-link
/// settings. All shapes are ESM (no CJS). `--target no-modules` is rejected
/// because emcc can't produce a non-modular global-namespace JS bundle.
fn emcc_post_link_settings_for(target: Target) -> Result<EmccPostLinkSettings> {
    // `ENVIRONMENT=web,node` is used for every target except `nodejs` so the
    // produced bundle works both in browsers/bundlers and directly under
    // Node — useful for tests, CLIs, and SSR. The `nodejs` target stays
    // node-only since that's its explicit intent.
    Ok(match target {
        Target::Bundler | Target::Web => EmccPostLinkSettings {
            environment: "web,node",
            source_phase_imports: false,
            extension: "mjs",
        },
        Target::Module => EmccPostLinkSettings {
            environment: "web,node",
            source_phase_imports: true,
            extension: "mjs",
        },
        Target::Nodejs => EmccPostLinkSettings {
            environment: "node",
            source_phase_imports: false,
            extension: "mjs",
        },
        Target::Deno => EmccPostLinkSettings {
            environment: "web,node",
            source_phase_imports: false,
            extension: "mjs",
        },
        Target::NoModules => bail!(
            "`--target no-modules` is not supported for wasm32-unknown-emscripten builds. \
             The emscripten toolchain produces module-shaped output only. \
             Use `--target web`, `--target bundler`, or `--target module` instead."
        ),
    })
}

/// Translate a `BuildProfile` into the `-O*` flag we pass to
/// `emcc --post-link`.
///
/// Optimization happens here (the final phase that touches the wasm)
/// because:
///   1. Anything emcc-link emits is rewritten by wasm-bindgen anyway.
///   2. emcc's `-O*` triggers wasm-opt internally with the correct
///      feature flags for the runtime's wasm features — no separate
///      wasm-opt install or flag wrangling needed.
///
/// We cap release builds at `-O2`. `-O3` enables wasm-opt's
/// `--minify-imports-and-exports` pass, which renames exports to
/// single-letter names — but wasm-bindgen's generated JS glue references
/// the original names (`__wbindgen_start`, `rs_add`, etc.). The minified
/// wasm would fail at instantiation. `-O2` produces nearly the same
/// optimization quality without the renaming.
fn emcc_opt_level_for(profile: &BuildProfile) -> &'static str {
    match profile {
        BuildProfile::Release => "-O2",
        BuildProfile::Profiling => "-O2",
        BuildProfile::Dev => "-O0",
        // Custom profiles could be anything. Pick a sane middle ground;
        // users who want a specific level can set it via Cargo profile flags.
        BuildProfile::Custom(_) => "-O2",
    }
}

/// Invoke `emcc --post-link` to merge the wasm-bindgen output with
/// emscripten's standard JS runtime, producing the final JS module.
///
/// If `emit_tsd` is supplied, emcc also tries to emit an EmscriptenModule
/// `.d.ts` to that path. Currently emcc's TSD generator asserts on
/// wasm-bindgen-style multi-value returns, so the file may not be
/// produced — we leave it as best-effort and rely on
/// `merge_emscripten_and_bindgen_dts` to fall back gracefully when the
/// file is missing.
fn emcc_post_link(
    in_wasm: &Path,
    in_library: &Path,
    out_js: &Path,
    emit_tsd: Option<&Path>,
    settings: &EmccPostLinkSettings,
    opt_level: &str,
) -> Result<()> {
    let mut cmd = Command::new("emcc");
    cmd.arg("--post-link")
        .arg(in_wasm)
        .arg("--js-library")
        .arg(in_library)
        .arg(opt_level)
        .arg("-sMODULARIZE=1")
        .arg("-sEXPORT_ES6=1")
        .arg(format!("-sENVIRONMENT={}", settings.environment))
        .arg("-Wno-experimental");
    if settings.source_phase_imports {
        cmd.arg("-sSOURCE_PHASE_IMPORTS=1");
    }
    if let Some(tsd) = emit_tsd {
        cmd.arg("--emit-tsd").arg(tsd);
    }
    cmd.arg("-o").arg(out_js);

    let status = cmd.status().context("running emcc --post-link")?;
    if !status.success() {
        bail!("emcc --post-link exited with status {status}");
    }
    Ok(())
}

/// Fuse emscripten's `EmscriptenModule` typings with wasm-bindgen's
/// `BindgenModule` typings into a single `.d.ts` written at `bindgen_dts`
/// (overwriting it in place).
///
/// emcc's `--emit-tsd` output ends with:
///   ```ts
///   export type MainModule = EmscriptenModule;
///   ```
/// We strip that line, append wasm-bindgen's typings, and replace it with
/// an intersection:
///   ```ts
///   export type MainModule = EmscriptenModule & BindgenModule;
///   ```
/// so consumers see a single typed factory covering both surfaces.
fn merge_emscripten_and_bindgen_dts(emscripten_dts: &Path, bindgen_dts: &Path) -> Result<()> {
    let em_src = std::fs::read_to_string(emscripten_dts)
        .with_context(|| format!("reading emscripten .d.ts at {emscripten_dts:?}"))?;
    let bg_src = std::fs::read_to_string(bindgen_dts)
        .with_context(|| format!("reading bindgen .d.ts at {bindgen_dts:?}"))?;

    let mut merged = String::new();
    for line in em_src.lines() {
        if line.trim_start().starts_with("export type MainModule") {
            continue;
        }
        merged.push_str(line);
        merged.push('\n');
    }
    merged.push_str("\n// --- wasm-bindgen-generated types ---\n");
    merged.push_str(&bg_src);
    if !merged.ends_with('\n') {
        merged.push('\n');
    }
    merged.push_str("\nexport type MainModule = EmscriptenModule & BindgenModule;\n");

    std::fs::write(bindgen_dts, merged)
        .with_context(|| format!("writing merged .d.ts to {bindgen_dts:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_emcc_version_real_world() {
        // Stock emsdk format.
        let line = "emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 3.1.60 (abc123)";
        assert_eq!(parse_emcc_version(line), Some((3, 1, 60)));

        // Newer style with five-digit hash.
        let line = "emcc (...) 5.0.7 (263db4cffa6f9fc2ec514a70abac81362ea41849)";
        assert_eq!(parse_emcc_version(line), Some((5, 0, 7)));

        // Pre-release suffix on patch.
        let line = "emcc (...) 3.1.61-git (abc)";
        assert_eq!(parse_emcc_version(line), Some((3, 1, 61)));
    }

    #[test]
    fn parse_emcc_version_unparseable() {
        // No version at all.
        assert_eq!(parse_emcc_version(""), None);
        // Empty input.
        assert_eq!(parse_emcc_version("no numbers here"), None);
    }

    #[test]
    fn read_cargo_build_target_walks_up_to_workspace_root() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Workspace root holds the .cargo/config.toml.
        std::fs::create_dir_all(root.join(".cargo")).unwrap();
        std::fs::write(
            root.join(".cargo/config.toml"),
            "[build]\ntarget = \"wasm32-unknown-emscripten\"\n",
        )
        .unwrap();
        // Crate lives two levels deeper with no config of its own.
        let crate_path = root.join("crates/foo");
        std::fs::create_dir_all(&crate_path).unwrap();

        assert_eq!(
            read_cargo_build_target(&crate_path),
            Some("wasm32-unknown-emscripten".to_string())
        );
    }

    #[test]
    fn read_cargo_build_target_prefers_crate_over_workspace() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".cargo")).unwrap();
        std::fs::write(
            root.join(".cargo/config.toml"),
            "[build]\ntarget = \"wasm32-unknown-unknown\"\n",
        )
        .unwrap();
        let crate_path = root.join("crates/foo");
        std::fs::create_dir_all(crate_path.join(".cargo")).unwrap();
        std::fs::write(
            crate_path.join(".cargo/config.toml"),
            "[build]\ntarget = \"wasm32-unknown-emscripten\"\n",
        )
        .unwrap();

        // Crate-level config should win.
        assert_eq!(
            read_cargo_build_target(&crate_path),
            Some("wasm32-unknown-emscripten".to_string())
        );
    }

    #[test]
    fn read_cargo_build_target_missing_returns_none() {
        let tmp = tempfile::TempDir::new().unwrap();
        // We can't usefully exercise the $CARGO_HOME fallback in a unit test
        // (would race with developer state), but we can assert the walk returns
        // None when no config exists anywhere in the temp tree.
        // Use a deliberately-unreachable CARGO_HOME so the test is hermetic.
        std::env::set_var("CARGO_HOME", tmp.path().join("nonexistent"));
        assert_eq!(read_cargo_build_target(tmp.path()), None);
    }
}
