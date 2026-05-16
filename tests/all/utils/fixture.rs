use binary_install::Cache;
use lazy_static::lazy_static;
use std::env;
use std::fs;
use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{MutexGuard, Once};
use std::thread;
use tempfile::TempDir;
use wasm_pack;
use wasm_pack::install::{self, Tool};

/// A test fixture in a temporary directory.
pub struct Fixture {
    // NB: we wrap the fixture's tempdir in a `ManuallyDrop` so that if a test
    // fails, its directory isn't deleted, and we have a chance to manually
    // inspect its state and figure out what is going on.
    pub dir: ManuallyDrop<TempDir>,
    pub path: PathBuf,
}

impl Fixture {
    /// Create a new test fixture in a temporary directory.
    pub fn new() -> Fixture {
        // Make sure that all fixtures end up sharing a target dir, and we don't
        // recompile wasm-bindgen and friends many times over.
        static SET_TARGET_DIR: Once = Once::new();
        let target_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target");
        SET_TARGET_DIR.call_once(|| {
            env::set_var("CARGO_TARGET_DIR", &target_dir);
        });

        let root = target_dir.join("t");
        fs::create_dir_all(&root).unwrap();
        let dir = TempDir::new_in(&root).unwrap();
        let path = dir.path().join("wasm-pack");
        eprintln!("Created fixture at {}", path.display());
        Fixture {
            dir: ManuallyDrop::new(dir),
            path,
        }
    }

    /// Create a file within this fixture.
    ///
    /// `path` should be a relative path to the file (relative within this
    /// fixture's path).
    ///
    /// The `contents` are written to the file.
    pub fn file<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, contents: C) -> &Self {
        assert!(path.as_ref().is_relative());
        let path = self.path.join(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
        self
    }

    /// Add a generic `README.md` file to the fixture.
    pub fn readme(&self) -> &Self {
        self.file(
            "README.md",
            r#"
                # Fixture!
                > an example rust -> wasm project
            "#,
        )
    }

    /// Add `LICENSE` file to the fixture.
    pub fn license(&self) -> &Self {
        self.file(
            "LICENSE",
            r#"
                I'm a license!
            "#,
        )
    }

    /// Add `WTFPL LICENSE` file to the fixture.
    pub fn wtfpl_license(&self) -> &Self {
        self.file(
            "LICENSE-WTFPL",
            r#"
                DO WHATEVER YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

                Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

                Everyone is permitted to copy and distribute verbatim or modified
                copies of this license document, and changing it is allowed as long
                as the name is changed.

                DO WHATEVER YOU WANT TO PUBLIC LICENSE
                TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

                0. You just DO WHATEVER YOU WANT TO.
            "#,
        )
    }

    /// Add `MIT LICENSE` file to the fixture.
    pub fn mit_license(&self) -> &Self {
        self.file(
            "LICENSE-MIT",
            r#"
                Copyright <YEAR> <COPYRIGHT HOLDER>

                Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

                The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
            "#,
        )
    }

    /// Add a `Cargo.toml` with a correctly configured `wasm-bindgen`
    /// dependency, `wasm-bindgen-test` dev-dependency, and `crate-type =
    /// ["cdylib"]`.
    ///
    /// `name` is the crate's name.
    pub fn cargo_toml(&self, name: &str) -> &Self {
        self.file(
            "Cargo.toml",
            &format!(
                r#"
                    [package]
                    authors = ["The wasm-pack developers"]
                    description = "so awesome rust+wasm package"
                    license = "WTFPL"
                    name = "{}"
                    repository = "https://github.com/wasm-bindgen/wasm-pack.git"
                    version = "0.1.0"

                    [lib]
                    crate-type = ["cdylib"]

                    [dependencies]
                    # Note that this uses and `=` dependency because there are
                    # various tests which assert that the version of wasm
                    # bindgen downloaded is what we expect, and if `=` is
                    # removed then it will download whatever the newest version
                    # of wasm-bindgen is which may not be what's listed here.
                    wasm-bindgen = "=0.2.100"

                    [dev-dependencies]
                    wasm-bindgen-test = "0.3"
                "#,
                name
            ),
        )
    }

    /// Add a `Cargo.toml` with a correctly configured `wasm-bindgen`
    /// dependency, `wasm-bindgen-test` dev-dependency, and `crate-type =
    /// ["cdylib"]`.
    ///
    /// `name` is the crate's name.
    /// `profile` is the custom profile name.
    pub fn cargo_toml_with_custom_profile(&self, name: &str, profile_name: &str) -> &Self {
        self.file(
            "Cargo.toml",
            &format!(
                r#"
                    [package]
                    authors = ["The wasm-pack developers"]
                    description = "so awesome rust+wasm package"
                    license = "WTFPL"
                    name = "{}"
                    repository = "https://github.com/wasm-bindgen/wasm-pack.git"
                    version = "0.1.0"

                    [lib]
                    crate-type = ["cdylib"]

                    [dependencies]
                    # Note that this uses and `=` dependency because there are
                    # various tests which assert that the version of wasm
                    # bindgen downloaded is what we expect, and if `=` is
                    # removed then it will download whatever the newest version
                    # of wasm-bindgen is which may not be what's listed here.
                    wasm-bindgen = "=0.2.100"

                    [dev-dependencies]
                    wasm-bindgen-test = "0.3"

                    [profile.{}]
                    inherits = "release"
                    opt-level = 'z'
                    lto = true
                "#,
                name, profile_name
            ),
        )
    }

    /// Add a `Cargo.toml` with a correctly configured `wasm-bindgen`
    /// dependency, `wasm-bindgen-test` dev-dependency, and `crate-type =
    /// ["cdylib"]`.
    ///
    /// `name` is the crate's name.
    /// `license_file` is license file path
    pub fn cargo_toml_with_license_file(&self, name: &str, license_file: &str) -> &Self {
        self.file(
            "Cargo.toml",
            &format!(
                r#"
                    [package]
                    authors = ["The wasm-pack developers"]
                    description = "so awesome rust+wasm package"
                    name = "{}"
                    license-file = "{}"
                    repository = "https://github.com/wasm-bindgen/wasm-pack.git"
                    version = "0.1.0"

                    [lib]
                    crate-type = ["cdylib"]

                    [dependencies]
                    wasm-bindgen = "=0.2.100"

                    [dev-dependencies]
                    wasm-bindgen-test = "=0.2.21"
                "#,
                name, license_file
            ),
        )
    }

    /// Add a `src/lib.rs` file that contains a "hello world" program.
    pub fn hello_world_src_lib(&self) -> &Self {
        self.file(
            "src/lib.rs",
            r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                // Import the `window.alert` function from the Web.
                #[wasm_bindgen]
                extern {
                    fn alert(s: &str);
                }

                // Export a `greet` function from Rust to JavaScript, that alerts a
                // hello message.
                #[wasm_bindgen]
                pub fn greet(name: &str) {
                    alert(&format!("Hello, {}!", name));
                }
            "#,
        )
    }

    /// Install a local wasm-bindgen for this fixture.
    ///
    /// Takes care not to re-install for every fixture, but only the one time
    /// for the whole test suite.
    pub fn install_local_wasm_bindgen(&self) -> PathBuf {
        // If wasm-bindgen is being used then it's very likely wasm-opt is going
        // to be used as well.
        self.install_wasm_opt();

        static INSTALL_WASM_BINDGEN: Once = Once::new();
        let cache = self.cache();
        let version = "0.2.100";

        let download = || {
            if let Ok(download) =
                install::download_prebuilt(&Tool::WasmBindgen, &cache, version, true)
            {
                return Ok(download);
            }

            install::cargo_install(Tool::WasmBindgen, &cache, version, true)
        };

        // Only one thread can perform the actual download, and then afterwards
        // everything will hit the cache so we can run the same path.
        INSTALL_WASM_BINDGEN.call_once(|| {
            download().unwrap();
        });
        if let install::Status::Found(dl) = download().unwrap() {
            dl.binary("wasm-bindgen").unwrap()
        } else {
            panic!("Download failed")
        }
    }

    pub fn install_wasm_opt(&self) {
        static INSTALL_WASM_OPT: Once = Once::new();
        let cache = self.cache();

        INSTALL_WASM_OPT.call_once(|| {
            wasm_pack::wasm_opt::find_wasm_opt(&cache, true).unwrap();
        });
    }

    /// Install a local cargo-generate for this fixture.
    ///
    /// Takes care not to re-install for every fixture, but only the one time
    /// for the whole test suite.
    pub fn install_local_cargo_generate(&self) -> PathBuf {
        static INSTALL_CARGO_GENERATE: Once = Once::new();
        let cache = self.cache();

        let download = || {
            if let Ok(download) =
                install::download_prebuilt(&Tool::CargoGenerate, &cache, "latest", true)
            {
                return Ok(download);
            }

            install::cargo_install(Tool::CargoGenerate, &cache, "latest", true)
        };

        // Only one thread can perform the actual download, and then afterwards
        // everything will hit the cache so we can run the same path.
        INSTALL_CARGO_GENERATE.call_once(|| {
            download().unwrap();
        });
        if let install::Status::Found(dl) = download().unwrap() {
            dl.binary("cargo-generate").unwrap()
        } else {
            panic!("Download failed")
        }
    }

    /// Download `geckodriver` and return its path.
    ///
    /// Takes care to ensure that only one `geckodriver` is downloaded for the whole
    /// test suite.
    pub fn install_local_geckodriver(&self) -> PathBuf {
        static FETCH_GECKODRIVER: Once = Once::new();
        let cache = self.cache();

        // like above for synchronization
        FETCH_GECKODRIVER.call_once(|| {
            wasm_pack::test::webdriver::install_geckodriver(&cache, true).unwrap();
        });
        wasm_pack::test::webdriver::install_geckodriver(&cache, true).unwrap()
    }

    /// Download `chromedriver` and return its path.
    ///
    /// Takes care to ensure that only one `chromedriver` is downloaded for the whole
    /// test suite.
    pub fn install_local_chromedriver(&self) -> PathBuf {
        static FETCH_CHROMEDRIVER: Once = Once::new();
        let cache = self.cache();

        // like above for synchronization
        FETCH_CHROMEDRIVER.call_once(|| {
            wasm_pack::test::webdriver::install_chromedriver(&cache, true).unwrap();
        });
        wasm_pack::test::webdriver::install_chromedriver(&cache, true).unwrap()
    }

    pub fn cache_dir(&self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test_cache")
    }

    pub fn cache(&self) -> Cache {
        let cache_dir = self.cache_dir();
        fs::create_dir_all(&cache_dir).unwrap();
        Cache::at(&cache_dir)
    }

    /// The `step_install_wasm_bindgen` and `step_run_wasm_bindgen` steps only
    /// occur after the `step_build_wasm` step. In order to read the lockfile
    /// in the test fixture's temporary directory, we should first build the
    /// crate, targeting `wasm32-unknown-unknown`.
    pub fn cargo_check(&self) -> &Self {
        Command::new("cargo")
            .current_dir(&self.path)
            .arg("check")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        self
    }

    /// Get a `wasm-pack` command configured to run in this fixure's temp
    /// directory and using the test cache.
    pub fn wasm_pack(&self) -> Command {
        use assert_cmd::cargo;

        let mut cmd = Command::new(cargo::cargo_bin!(env!("CARGO_PKG_NAME")));

        cmd.current_dir(&self.path);
        cmd.env("WASM_PACK_CACHE", self.cache_dir());

        // Some of the tests assume that Cargo's output does not contain colors.
        cmd.env_remove("CARGO_TERM_COLOR");

        cmd
    }

    pub fn lock(&self) -> MutexGuard<'static, ()> {
        use std::sync::Mutex;
        lazy_static! {
            static ref ONE_TEST_AT_A_TIME: Mutex<()> = Mutex::new(());
        }
        ONE_TEST_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        if !thread::panicking() {
            unsafe { ManuallyDrop::drop(&mut self.dir) }
        }
    }
}

pub fn bad_cargo_toml() -> Fixture {
    let fixture = Fixture::new();
    fixture.readme().hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            name = "bad-cargo-toml"
            version = "0.1.0"
            authors = ["The wasm-pack developers"]

            [lib]
            crate-type = ["foo"]

            [dependencies]
            # Note: no wasm-bindgen dependency!
        "#,
    );
    fixture
}

pub fn js_hello_world() -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml("js-hello-world")
        .hello_world_src_lib();
    fixture
}

/// Fixture targeting wasm32-unknown-emscripten with a broad #[wasm_bindgen]
/// surface — each export exercises a distinct codegen path so a regression
/// in any one of them is caught:
///   - `rs_add`           — primitive args/returns (baseline)
///   - `rs_greet`         — string passing / cachedTextEncoder / $heap
///   - `rs_make_adder`    — closure construction / CLOSURE_DTORS / externref
///   - `rs_sum`           — typed slice → HEAPF64 / HEAP_DATA_VIEW
///   - `rs_xor`           — byte slice → HEAPU8
///   - `rs_divide`        — Result<T, JsError> error propagation
///   - `Counter`          — class with constructor + method + getter
///   - `rs_double_via_js` — Rust calling out to a JS-provided import
///
/// Crate config:
///   - `crate-type = ["staticlib"]` (emcc consumes the .a)
///   - `.cargo/config.toml` selects the emscripten target so the user
///     doesn't need to pass `-- --target wasm32-unknown-emscripten`
pub fn emscripten_hello_world() -> Fixture {
    let fixture = Fixture::new();

    // wasm-bindgen's CLI and macro share a version-locked binary protocol, and
    // the fixes required for emscripten output mode aren't on a stable release
    // yet. Pin the fixture at the `emscripten-output-fixes` branch via
    // `[patch.crates-io]` so the local crate, wasm-bindgen CLI, and CI all
    // agree on the same descriptor format.
    //
    // TODO: once these fixes land in a published wasm-bindgen release, drop
    // this `[patch.crates-io]` block entirely and bump the `wasm-bindgen`
    // and `js-sys` versions below to the release that contains them.
    const WASM_BINDGEN_BRANCH: &str = "emscripten-output-fixes";
    const WASM_BINDGEN_REPO: &str = "https://github.com/wasm-bindgen/wasm-bindgen";
    let patch_section = format!(
        "\n\n[patch.crates-io]\n\
         wasm-bindgen = {{ git = \"{WASM_BINDGEN_REPO}\", branch = \"{WASM_BINDGEN_BRANCH}\" }}\n\
         wasm-bindgen-macro = {{ git = \"{WASM_BINDGEN_REPO}\", branch = \"{WASM_BINDGEN_BRANCH}\" }}\n\
         wasm-bindgen-macro-support = {{ git = \"{WASM_BINDGEN_REPO}\", branch = \"{WASM_BINDGEN_BRANCH}\" }}\n\
         wasm-bindgen-shared = {{ git = \"{WASM_BINDGEN_REPO}\", branch = \"{WASM_BINDGEN_BRANCH}\" }}\n\
         js-sys = {{ git = \"{WASM_BINDGEN_REPO}\", branch = \"{WASM_BINDGEN_BRANCH}\" }}\n"
    );

    let cargo_toml = format!(
        r#"[package]
authors = ["The wasm-pack developers"]
description = "emscripten + wasm-bindgen integration test"
license = "WTFPL"
name = "em-hello-world"
repository = "https://github.com/wasm-bindgen/wasm-pack.git"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
{patch_section}"#
    );

    fixture
        .readme()
        .file("Cargo.toml", cargo_toml)
        .file(
            ".cargo/config.toml",
            r#"
[build]
target = "wasm32-unknown-emscripten"

[target.wasm32-unknown-emscripten]
rustflags = [
    "-Cllvm-args=-enable-emscripten-cxx-exceptions=0",
    "-Cpanic=abort",
    "-Crelocation-model=static",
]
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn rs_greet(name: &str) -> String {
    format!("hello, {name}!")
}

#[wasm_bindgen]
pub fn rs_make_adder(n: i32) -> Function {
    let f = Closure::<dyn Fn(i32) -> i32>::new(move |x| x + n);
    let ret = f.as_ref().unchecked_ref::<Function>().clone();
    f.forget();
    ret
}

#[wasm_bindgen]
pub fn rs_sum(values: &[f64]) -> f64 {
    values.iter().sum()
}

#[wasm_bindgen]
pub fn rs_xor(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |acc, &b| acc ^ b)
}

#[wasm_bindgen]
pub fn rs_divide(num: i32, den: i32) -> Result<i32, JsError> {
    if den == 0 {
        Err(JsError::new("division by zero"))
    } else {
        Ok(num / den)
    }
}

#[wasm_bindgen]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: i32) -> Counter {
        Counter { value: initial }
    }

    pub fn increment(&mut self, by: i32) -> i32 {
        self.value += by;
        self.value
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = rs_test_doubler)]
    fn js_doubler(n: i32) -> i32;
}

#[wasm_bindgen]
pub fn rs_double_via_js(n: i32) -> i32 {
    js_doubler(n)
}
            "#,
        );
    fixture
}

pub fn js_hello_world_with_custom_profile(profile_name: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml_with_custom_profile("js-hello-world", profile_name)
        .hello_world_src_lib();
    fixture
}

pub fn no_cdylib() -> Fixture {
    let fixture = Fixture::new();
    fixture.readme().hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "foo"
            repository = "https://github.com/wasm-bindgen/wasm-pack.git"
            version = "0.1.0"

            # [lib]
            # crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"

            [dev-dependencies]
            wasm-bindgen-test = "0.3"
        "#,
    );
    fixture
}

pub fn not_a_crate() -> Fixture {
    let fixture = Fixture::new();
    fixture.file("README.md", "This is not a Rust crate!");
    fixture
}

pub fn serde_feature() -> Fixture {
    let fixture = Fixture::new();
    fixture.readme().hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            name = "serde-serialize"
            version = "0.1.0"
            authors = ["The wasm-pack developers"]

            [lib]
            crate-type = ["cdylib"]

            [dependencies.wasm-bindgen]
            version = "^0.2"
            features = ["serde-serialize"]
        "#,
    );
    fixture
}

pub fn wbg_test_diff_versions() -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "wbg-test-diff-versions"
                version = "0.1.0"
                authors = ["The wasm-pack developers"]

                [lib]
                crate-type = ["cdylib", "rlib"]

                [dependencies]
                # We depend on the latest wasm-bindgen 0.2
                wasm-bindgen = "0.2"

                [dev-dependencies]
                # And we depend on wasm-bindgen-test 0.2.29. This should still
                # work, and we should end up with the latest `wasm-bindgen` and
                # wasm-bindgen-test at 0.2.29, and everything should still work.
                wasm-bindgen-test = "0.2.29"
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn one() -> u32 { 1 }
            "#,
        )
        .file(
            "tests/node.rs",
            r#"
                extern crate wbg_test_diff_versions;
                extern crate wasm_bindgen_test;
                use wasm_bindgen_test::*;

                #[wasm_bindgen_test]
                fn pass() {
                    assert_eq!(wbg_test_diff_versions::one(), 1);
                }
            "#,
        );
    fixture
}

pub fn wbg_test_browser() -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml("wbg-test-browser")
        .hello_world_src_lib()
        .file(
            "tests/browser.rs",
            r#"
                extern crate wasm_bindgen_test;
                use wasm_bindgen_test::*;

                wasm_bindgen_test_configure!(run_in_browser);

                #[wasm_bindgen_test]
                fn pass() {
                    assert_eq!(1, 1);
                }
            "#,
        );
    fixture
}

pub fn wbg_test_fail() -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml("wbg-test-fail")
        .hello_world_src_lib()
        .file(
            "tests/node.rs",
            r#"
                extern crate wasm_bindgen_test;
                use wasm_bindgen_test::*;

                #[wasm_bindgen_test]
                fn pass() {
                    assert_eq!(1, 2);
                }
            "#,
        );
    fixture
}

pub fn wbg_test_node() -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml("wbg-test-node")
        .hello_world_src_lib()
        .file(
            "tests/node.rs",
            r#"
                extern crate wasm_bindgen_test;
                use wasm_bindgen_test::*;

                #[wasm_bindgen_test]
                fn pass() {
                    assert_eq!(1, 1);
                }
            "#,
        );
    fixture
}

pub fn transitive_dependencies() -> Fixture {
    fn project_main_fixture(fixture: &mut Fixture) {
        fixture.file(PathBuf::from("main/README"), "# Main Fixture\n");
        fixture.file(
            PathBuf::from("main/Cargo.toml"),
            r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "main_project"
            repository = "https://github.com/wasm-bindgen/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"
            project_a = { path = "../project_a" }
            project_b = { path = "../project_b" }

            [dev-dependencies]
            wasm-bindgen-test = "0.3"
        "#,
        );
        fixture.file(
            PathBuf::from("main/src/lib.rs"),
            r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                // Import the `window.alert` function from the Web.
                #[wasm_bindgen]
                extern {
                    fn alert(s: &str);
                }

                // Export a `greet` function from Rust to JavaScript, that alerts a
                // hello message.
                #[wasm_bindgen]
                pub fn greet(name: &str) {
                    alert(&format!("Hello, {}!", name));
                }
            "#,
        );
    }

    fn project_a_fixture(fixture: &mut Fixture) {
        fixture.file(
            PathBuf::from("project_a/README"),
            "# Project Alpha Fixture\n",
        );
        fixture.file(
            PathBuf::from("project_a/Cargo.toml"),
            r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "project_a"
            repository = "https://github.com/wasm-bindgen/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"
            project_b = { path = "../project_b" }

            [dev-dependencies]
            wasm-bindgen-test = "0.3"
        "#,
        );
        fixture.file(
            PathBuf::from("project_a/src/lib.rs"),
            r#"
                extern crate wasm_bindgen;
                // extern crate project_b;
                use wasm_bindgen::prelude::*;

                // Import the `window.alert` function from the Web.
                #[wasm_bindgen]
                extern {
                    fn alert(s: &str);
                }

                // Export a `greet` function from Rust to JavaScript, that alerts a
                // hello message.
                #[wasm_bindgen]
                pub fn greet(name: &str) {
                    alert(&format!("Hello, {}!", name));
                }
            "#,
        );
    }

    fn project_b_fixture(fixture: &mut Fixture) {
        fixture.file(
            PathBuf::from("project_b/README"),
            "# Project Beta Fixture\n",
        );
        fixture.file(
            PathBuf::from("project_b/Cargo.toml"),
            r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "project_b"
            repository = "https://github.com/wasm-bindgen/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"

            [dev-dependencies]
            wasm-bindgen-test = "0.3"
        "#,
        );
        fixture.file(
            PathBuf::from("project_b/src/lib.rs"),
            r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                // Import the `window.alert` function from the Web.
                #[wasm_bindgen]
                extern {
                    fn alert(s: &str);
                }

                // Export a `greet` function from Rust to JavaScript, that alerts a
                // hello message.
                #[wasm_bindgen]
                pub fn greet(name: &str) {
                    alert(&format!("Hello, {}!", name));
                }
            "#,
        );
    }

    let mut fixture = Fixture::new();
    project_b_fixture(&mut fixture);
    project_a_fixture(&mut fixture);
    project_main_fixture(&mut fixture);
    fixture
}

pub fn single_license() -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml("single_license")
        .license()
        .hello_world_src_lib();
    fixture
}

pub fn dual_license() -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml("dual_license")
        .wtfpl_license()
        .mit_license()
        .hello_world_src_lib();
    fixture
}

pub fn non_standard_license(license_file: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture
        .readme()
        .cargo_toml_with_license_file("dual_license", license_file)
        .file(license_file, "license file for test")
        .hello_world_src_lib();
    fixture
}
