use crate::utils;
use assert_cmd::prelude::*;
use std::fs;
use std::path::Path;

#[test]
fn build_in_non_crate_directory_doesnt_panic() {
    let fixture = utils::fixture::not_a_crate();
    fixture
        .wasm_pack()
        .arg("build")
        .arg(".")
        .assert()
        .failure()
        .stderr(predicates::str::contains("missing a `Cargo.toml`"));
}

#[test]
fn it_should_build_js_hello_world_example() {
    let fixture = utils::fixture::js_hello_world();
    fixture.wasm_pack().arg("build").assert().success();
}

#[test]
fn it_should_not_make_a_pkg_json_if_passed_no_pack() {
    let fixture = utils::fixture::js_hello_world();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--no-pack")
        .assert()
        .success();

    let pkg_path = fixture.path.join("pkg");
    assert_eq!(pkg_path.join("package.json").exists(), false);
    assert_eq!(pkg_path.join("README.md").exists(), false);
    assert_eq!(pkg_path.join("licence").exists(), false);
}

#[test]
fn it_should_not_delete_existing_pkg_json_if_passed_no_pack() {
    let fixture = utils::fixture::js_hello_world();
    let package_json_path = fixture.path.join("pkg").join("package.json");
    fixture.file("pkg/package.json", r#"{ "name": "pre-existing" }"#);
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--no-pack")
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&package_json_path).unwrap(),
        r#"{ "name": "pre-existing" }"#
    );
}

#[test]
fn it_should_build_js_hello_world_example_with_custom_target_dir() {
    let fixture = utils::fixture::js_hello_world();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--target-dir")
        .arg("target2")
        .arg("--all-features")
        .arg("--offline")
        .assert()
        .success();
}

#[test]
fn it_should_build_crates_in_a_workspace() {
    let fixture = utils::fixture::Fixture::new();
    fixture
        .file(
            "Cargo.toml",
            r#"
                [workspace]
                members = ["blah"]
            "#,
        )
        .file(
            Path::new("blah").join("Cargo.toml"),
            r#"
                [package]
                authors = ["The wasm-pack developers"]
                description = "so awesome rust+wasm package"
                license = "WTFPL"
                name = "blah"
                repository = "https://github.com/wasm-bindgen/wasm-pack.git"
                version = "0.1.0"

                [lib]
                crate-type = ["cdylib"]

                [dependencies]
                wasm-bindgen = "0.2"
            "#,
        )
        .file(
            Path::new("blah").join("src").join("lib.rs"),
            r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn hello() -> u32 { 42 }
            "#,
        )
        .install_local_wasm_bindgen();
    fixture
        .wasm_pack()
        .current_dir(&fixture.path.join("blah"))
        .arg("build")
        .assert()
        .success();
}

#[test]
fn renamed_crate_name_works() {
    let fixture = utils::fixture::Fixture::new();
    fixture
        .readme()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [lib]
                crate-type = ["cdylib"]
                name = 'bar'

                [dependencies]
                wasm-bindgen = "0.2"
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
        .install_local_wasm_bindgen();
    fixture.wasm_pack().arg("build").assert().success();
}

#[test]
fn dash_dash_web_target_has_error_on_old_bindgen() {
    let fixture = utils::fixture::Fixture::new();
    fixture
        .readme()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [lib]
                crate-type = ["cdylib"]
                name = 'bar'

                [dependencies]
                wasm-bindgen = "=0.2.37"
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
        .install_local_wasm_bindgen();
    let cmd = fixture
        .wasm_pack()
        .arg("build")
        .arg("--target")
        .arg("web")
        .assert()
        .failure();
    let output = String::from_utf8(cmd.get_output().stderr.clone()).unwrap();

    assert!(
        output.contains("Please update your project to wasm-bindgen version >= 0.2.39")
            || output.contains("older versions of the `wasm-bindgen` crate are incompatible with current versions of Rust"),
        "Output did not contain 'Please update your project to wasm-bindgen version >= 0.2.39' or 'older versions of the `wasm-bindgen` crate are incompatible with current versions of Rust', output was {}",
        output
    );
}

#[test]
fn it_should_build_nested_project_with_transitive_dependencies() {
    let fixture = utils::fixture::transitive_dependencies();
    fixture.install_local_wasm_bindgen();
    fixture
        .wasm_pack()
        .current_dir(fixture.path.join("main"))
        .arg("build")
        .assert()
        .success();
}

#[test]
fn build_different_profiles() {
    let fixture = utils::fixture::js_hello_world();
    fixture.install_local_wasm_bindgen();

    for profile in ["--dev", "--debug", "--profiling", "--release"]
        .iter()
        .cloned()
    {
        fixture
            .wasm_pack()
            .arg("build")
            .arg(profile)
            .assert()
            .success();
    }
}

#[test]
fn build_custom_profile() {
    let profile_name = "my-custom-profile";
    let fixture = utils::fixture::js_hello_world_with_custom_profile(profile_name);
    fixture.install_local_wasm_bindgen();

    fixture
        .wasm_pack()
        .arg("build")
        .arg("--profile")
        .arg(profile_name)
        .assert()
        .success();
}

#[test]
fn build_with_and_without_wasm_bindgen_debug() {
    for debug in [true, false].iter().cloned() {
        let fixture = utils::fixture::Fixture::new();
        fixture
            .readme()
            .file(
                "Cargo.toml",
                format!(
                    r#"
                    [package]
                    authors = ["The wasm-pack developers"]
                    description = "so awesome rust+wasm package"
                    license = "WTFPL"
                    name = "whatever"
                    repository = "https://github.com/wasm-bindgen/wasm-pack.git"
                    version = "0.1.0"

                    [lib]
                    crate-type = ["cdylib"]

                    [dependencies]
                    wasm-bindgen = "0.2"

                    [package.metadata.wasm-pack.profile.dev.wasm-bindgen]
                    debug-js-glue = {}
                    "#,
                    debug
                ),
            )
            .file(
                "src/lib.rs",
                r#"
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub struct MyThing {}

                #[wasm_bindgen]
                impl MyThing {
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> MyThing {
                        MyThing {}
                    }
                }

                #[wasm_bindgen]
                pub fn take(foo: MyThing) {
                    drop(foo);
                }
                "#,
            )
            .install_local_wasm_bindgen();

        fixture
            .wasm_pack()
            .arg("build")
            .arg("--dev")
            .assert()
            .success();

        let contents = fs::read_to_string(fixture.path.join("pkg/whatever_bg.js")).unwrap();
        let contains_move_assertions =
            contents.contains("throw new Error('Attempt to use a moved value')");
        assert_eq!(
            contains_move_assertions, debug,
            "Should contain moved value assertions iff debug assertions are enabled. \
             Contains move assertions? {}. \
             Is a debug JS glue build? {}.",
            contains_move_assertions, debug,
        );
    }
}

#[test]
fn build_with_arbitrary_cargo_options() {
    let fixture = utils::fixture::js_hello_world();
    fixture.install_local_wasm_bindgen();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--no-default-features")
        .assert()
        .success();
}

#[test]
fn build_no_install() {
    let fixture = utils::fixture::js_hello_world();
    fixture.install_local_wasm_bindgen();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--mode")
        .arg("no-install")
        .assert()
        .success();
}

#[test]
fn build_force() {
    let fixture = utils::fixture::js_hello_world();
    fixture.install_local_wasm_bindgen();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--mode")
        .arg("force")
        .assert()
        .success();
}

#[test]
fn build_from_new() {
    let fixture = utils::fixture::not_a_crate();
    let name = "generated-project";
    fixture
        .wasm_pack()
        .arg("new")
        .arg(name)
        .arg("--template")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("wasm-pack-template"))
        .assert()
        .success();
    let project_location = fixture.path.join(&name);
    fixture
        .wasm_pack()
        .arg("build")
        .arg(&project_location)
        .assert()
        .success();
}

#[test]
fn build_crates_with_same_names() {
    let fixture = utils::fixture::Fixture::new();
    fixture
        .readme()
        .file(
            "somename1/Cargo.toml",
            r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "somename"
            repository = "https://github.com/wasm-bindgen/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"
            somenameother = { path = "../somename2", package = "somename" }
            "#,
        )
        .file(
            "somename1/src/lib.rs",
            r#"
            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            pub fn method() -> i32 {
                somenameother::method()
            }
            "#,
        )
        .file(
            "somename2/Cargo.toml",
            r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "somename"
            repository = "https://github.com/wasm-bindgen/wasm-pack.git"
            version = "0.1.1"

            [lib]
            crate-type = ["rlib"]
            "#,
        )
        .file(
            "somename2/src/lib.rs",
            r#"
            pub fn method() -> i32 {
                0
            }
            "#,
        );
    fixture.install_local_wasm_bindgen();
    fixture
        .wasm_pack()
        .current_dir(fixture.path.join("somename1"))
        .arg("build")
        .assert()
        .success();
}
