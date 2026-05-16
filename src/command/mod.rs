//! CLI command structures, parsing, and execution.
#![allow(clippy::redundant_closure)]

pub mod build;
mod generate;
mod login;
mod pack;
/// Data structures and functions for publishing a package.
pub mod publish;
pub mod test;
pub mod utils;

use self::build::{Build, BuildOptions};
use self::generate::generate;
use self::login::login;
use self::pack::pack;
use self::publish::{access::Access, publish};
use self::test::{Test, TestOptions};
use crate::install::InstallMode;
use anyhow::Result;
use clap::Subcommand;
use log::info;
use std::path::PathBuf;
/// The various kinds of commands that `wasm-pack` can execute.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// 🏗️  build your npm package!
    #[clap(name = "build", alias = "init")]
    Build(BuildOptions),

    #[clap(name = "pack")]
    /// 🍱  create a tar of your npm package but don't publish!
    Pack {
        #[clap(long = "pkg-dir", short = 'd', default_value = "pkg")]
        /// The name of the output directory where the npm package is stored
        pkg_directory: PathBuf,

        /// The path to the Rust crate. If not set, searches up the path from the current directory.
        #[clap()]
        path: Option<PathBuf>,
    },

    #[clap(name = "new")]
    /// 🐑 create a new project with a template
    Generate {
        /// The name of the project
        name: String,
        /// The URL to the template
        #[clap(
            long = "template",
            default_value = crate::generate::DEFAULT_TEMPLATE
        )]
        template: String,
        /// Scaffold a project targeting wasm32-unknown-emscripten instead
        /// of the default wasm32-unknown-unknown. Implies a staticlib +
        /// emcc pipeline. Requires emsdk on PATH.
        #[clap(long = "emscripten", conflicts_with = "template")]
        emscripten: bool,
        #[clap(long = "mode", short = 'm', default_value = "normal")]
        /// Should we install or check the presence of binary tools. [possible values: no-install, normal, force]
        mode: InstallMode,
    },

    #[clap(name = "publish")]
    /// 🎆  pack up your npm package and publish!
    Publish {
        #[clap(long = "target", short = 't', default_value = "bundler")]
        /// Sets the target environment. [possible values: bundler, nodejs, web, no-modules, deno, module]
        target: String,

        /// The access level for the package to be published
        #[clap(long = "access", short = 'a')]
        access: Option<Access>,

        /// The distribution tag being used for publishing.
        /// See https://docs.npmjs.com/cli/dist-tag
        #[clap(long = "tag")]
        tag: Option<String>,

        #[clap(long = "pkg-dir", short = 'd', default_value = "pkg")]
        /// The name of the output directory where the npm package is stored
        pkg_directory: PathBuf,

        /// The path to the Rust crate. If not set, searches up the path from the current directory.
        #[clap()]
        path: Option<PathBuf>,
    },

    #[clap(name = "login", alias = "adduser", alias = "add-user")]
    /// 👤  Add an npm registry user account! (aliases: adduser, add-user)
    Login {
        #[clap(long = "registry", short = 'r')]
        /// Default: 'https://registry.npmjs.org/'.
        /// The base URL of the npm package registry. If scope is also
        /// specified, this registry will only be used for packages with that
        /// scope. scope defaults to the scope of the project directory you're
        /// currently in, if any.
        registry: Option<String>,

        #[clap(long = "scope", short = 's')]
        /// Default: none.
        /// If specified, the user and login credentials given will be
        /// associated with the specified scope.
        scope: Option<String>,

        #[clap(long = "auth-type", short = 't')]
        /// Default: 'legacy'.
        /// Type: 'legacy', 'sso', 'saml', 'oauth'.
        /// What authentication strategy to use with adduser/login. Some npm
        /// registries (for example, npmE) might support alternative auth
        /// strategies besides classic username/password entry in legacy npm.
        auth_type: Option<String>,
    },

    #[clap(name = "test")]
    /// 👩‍🔬  test your wasm!
    Test(TestOptions),
}

/// Run a command with the given logger!
pub fn run_wasm_pack(command: Command) -> Result<()> {
    // Run the correct command based off input and store the result of it so that we can clear
    // the progress bar then return it
    match command {
        Command::Build(build_opts) => {
            info!("Running build command...");
            Build::try_from_opts(build_opts).and_then(|mut b| b.run())
        }
        Command::Pack {
            path,
            pkg_directory,
        } => {
            info!("Running pack command...");
            info!("Path: {:?}", &path);
            pack(path, pkg_directory)
        }
        Command::Generate {
            template,
            name,
            emscripten,
            mode,
        } => {
            info!("Running generate command...");
            info!("Name: {:?}", &name);
            // `--emscripten` is shorthand for selecting the in-tree emscripten
            // template subdirectory from the default wasm-pack repo. Users
            // who pass `--template` keep full control; clap rejects the
            // combination via `conflicts_with`.
            let template = if emscripten {
                crate::generate::EMSCRIPTEN_TEMPLATE.to_owned()
            } else {
                template
            };
            info!("Template: {:?}", &template);
            generate(template, name, emscripten, mode.install_permitted())
        }
        Command::Publish {
            target,
            path,
            access,
            tag,
            pkg_directory,
        } => {
            info!("Running publish command...");
            info!("Path: {:?}", &path);
            publish(&target, path, access, tag, pkg_directory)
        }
        Command::Login {
            registry,
            scope,
            auth_type,
        } => {
            info!("Running login command...");
            info!(
                "Registry: {:?}, Scope: {:?}, Auth Type: {:?}",
                &registry, &scope, &auth_type
            );
            login(registry, &scope, &auth_type)
        }
        Command::Test(test_opts) => {
            info!("Running test command...");
            Test::try_from_opts(test_opts).and_then(|t| t.run())
        }
    }
}
