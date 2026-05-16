/// Data structure to represent published package access level.
pub mod access;

use self::access::Access;
use crate::command::build::{Build, BuildOptions, Target};
use crate::command::utils::{find_pkg_directory, get_crate_path};
use crate::npm;
use crate::PBAR;
use anyhow::{anyhow, bail, Result};
use dialoguer::{Confirm, Input, Select};
use log::info;
use std::path::PathBuf;
use std::str::FromStr;

/// Creates a tarball from a 'pkg' directory
/// and publishes it to the NPM registry
pub fn publish(
    _target: &str,
    path: Option<PathBuf>,
    access: Option<Access>,
    tag: Option<String>,
    pkg_directory: PathBuf,
) -> Result<()> {
    let crate_path = get_crate_path(path)?;

    info!("Publishing the npm package...");
    info!("npm info located in the npm debug log");

    let pkg_directory = match find_pkg_directory(&crate_path, &pkg_directory) {
        Some(path) => Ok(path),
        None => {
            // while `wasm-pack publish`, if the pkg directory cannot be found,
            // then try to `wasm-pack build`
            if Confirm::new()
                .with_prompt("Your package hasn't been built, build it?")
                .interact()?
            {
                let out_dir = Input::new()
                    .with_prompt("out_dir[default: pkg]")
                    .default(".".to_string())
                    .show_default(false)
                    .interact()?;
                let out_dir = format!("{}/pkg", out_dir);
                let target = Select::new()
                    .with_prompt("target[default: bundler]")
                    .items(&["bundler", "nodejs", "web", "no-modules", "deno", "module"])
                    .default(0)
                    .interact()?
                    .to_string();
                let target = Target::from_str(&target)?;
                let build_opts = BuildOptions {
                    path: Some(crate_path.clone()),
                    target,
                    out_dir: out_dir.clone(),
                    ..Default::default()
                };
                Build::try_from_opts(build_opts)
                    .and_then(|mut build| build.run())
                    .map(|()| crate_path.join(out_dir))
                    .map_err(|_| {
                        anyhow!(
                            "Unable to find the pkg directory at path '{:#?}',\
                             or in a child directory of '{:#?}'",
                            &crate_path,
                            &crate_path
                        )
                    })
            } else {
                bail!(
                    "Unable to find the pkg directory at path '{:#?}',\
                     or in a child directory of '{:#?}'",
                    &crate_path,
                    &crate_path
                )
            }
        }
    }?;
    npm::npm_publish(&pkg_directory.to_string_lossy(), access, tag)?;
    info!("Published your package!");

    PBAR.info("💥  published your package!");
    Ok(())
}
