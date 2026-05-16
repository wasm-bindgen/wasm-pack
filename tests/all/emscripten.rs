//! Integration tests for `wasm-pack build` against the
//! `wasm32-unknown-emscripten` target.
//!
//! Strategy:
//!   1. Detect whether `emcc` is reachable. If not, skip with an explanatory
//!      message — contributors without emsdk can still run the rest of the
//!      suite. CI installs emsdk via `setup-emsdk`.
//!   2. For each supported wasm-pack `--target` value (bundler / web /
//!      module / nodejs / deno) build the `emscripten_hello_world` fixture
//!      and exercise the full #[wasm_bindgen] surface from Node by:
//!        - importing the produced `.mjs` ESM
//!        - installing a `globalThis.rs_test_doubler` for the Rust→JS path
//!        - awaiting the emscripten factory
//!        - calling every exported function/class and asserting results
//!   3. Verify `--target no-modules` is rejected with a clear message.
//!   4. Verify `wasm-pack test` rejects the emscripten target.

use crate::utils;
use assert_cmd::prelude::*;
use std::path::Path;
use std::process::Command;

/// Returns true if `emcc` is reachable (either on PATH or via `$EMSDK`).
fn emcc_available() -> bool {
    if which::which("emcc").is_ok() {
        return true;
    }
    if let Ok(emsdk) = std::env::var("EMSDK") {
        return Path::new(&emsdk).join("upstream/emscripten/emcc").exists();
    }
    false
}

/// Locate the wasm-bindgen binary that the tests should drive.
///
/// Resolution order:
///   1. `$WASM_BINDGEN_BIN` env var (set by `cargo test` from a sibling
///      checkout with local fixes)
///   2. `../wasm-bindgen/target/release/wasm-bindgen` (default sibling
///      checkout path used by the project's contributors)
///   3. `None` — let wasm-pack fall back to cache/install
fn local_wasm_bindgen_bin() -> Option<std::path::PathBuf> {
    if let Ok(p) = std::env::var("WASM_BINDGEN_BIN") {
        let p = std::path::PathBuf::from(p);
        if p.is_file() {
            return Some(p);
        }
    }
    for candidate in [
        "../wasm-bindgen/target/release/wasm-bindgen",
        "../wasm-bindgen/target/debug/wasm-bindgen",
    ] {
        let p = Path::new(env!("CARGO_MANIFEST_DIR")).join(candidate);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

/// Skip the calling test (with an explanatory message) if emcc isn't
/// available. CI is expected to install emsdk before running tests.
macro_rules! skip_without_emcc {
    () => {
        if !emcc_available() {
            eprintln!(
                "skipping: emcc not found on PATH and $EMSDK is unset. \
                 Install emsdk and `source emsdk_env.sh` to enable these tests."
            );
            return;
        }
    };
}

/// Node driver that exercises every export of the fixture. Returns the
/// driver as a JS source string ready to feed to `node --input-type=module`.
///
/// Each check prints `PASS <name>` or `FAIL <name>: got <x>, want <y>` and
/// sets `process.exitCode = 1` on any failure. The full surface is driven in
/// one node invocation so we catch interactions between different codegen
/// paths (e.g. heap reallocation invalidating cached views).
fn make_node_driver(mjs_path: &Path) -> String {
    format!(
        r#"
        globalThis.rs_test_doubler = (n) => n * 2;
        const {{ default: M }} = await import('file://{mjs}');
        const m = await M();

        function expect(name, got, want) {{
            const ok = JSON.stringify(got) === JSON.stringify(want);
            console.log(`${{ok ? 'PASS' : 'FAIL'}} ${{name}}: got ${{JSON.stringify(got)}}${{ok ? '' : `, want ${{JSON.stringify(want)}}`}}`);
            if (!ok) process.exitCode = 1;
        }}

        expect('rs_add',          m.rs_add(17, 25),                            42);
        expect('rs_greet',        m.rs_greet('world'),                         'hello, world!');
        expect('rs_make_adder',   m.rs_make_adder(5)(10),                      15);
        expect('rs_sum',          m.rs_sum(Float64Array.from([1, 2, 3, 4])),   10);
        expect('rs_xor',          m.rs_xor(Uint8Array.from([1, 2, 4])),        7);
        expect('rs_divide ok',    m.rs_divide(20, 4),                          5);

        try {{
            m.rs_divide(20, 0);
            expect('rs_divide throws', 'did not throw', 'threw');
        }} catch (e) {{
            const msg = String(e.message || e);
            expect('rs_divide throws', msg.includes('division by zero') ? 'threw' : msg, 'threw');
        }}

        const c = new m.Counter(10);
        expect('Counter ctor',    c.value,                                     10);
        expect('Counter.increment', c.increment(5),                            15);
        expect('Counter.value',   c.value,                                     15);

        expect('rs_double_via_js', m.rs_double_via_js(21),                     42);
        "#,
        mjs = mjs_path.display(),
    )
}

/// Drive the test's Node smoke check against the produced `.mjs`.
fn assert_module_runs_in_node(pkg_dir: &Path, module_name: &str) {
    let mjs = pkg_dir.join(format!("{module_name}.mjs"));
    assert!(mjs.exists(), "expected {mjs:?} to exist");

    let output = Command::new("node")
        .arg("--input-type=module")
        .arg("-e")
        .arg(make_node_driver(&mjs))
        .output()
        .expect("failed to spawn node");

    assert!(
        output.status.success(),
        "node test failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

/// Helper that does a full `wasm-pack build --target <variant>` against the
/// fixture and runs the comprehensive Node driver. If a local wasm-bindgen
/// binary is available (typically a sibling checkout with fixes for
/// emscripten output mode), it's used in preference to the
/// `install_local_wasm_bindgen` cached version.
fn run_build_and_smoke(target: &str) {
    skip_without_emcc!();
    let fixture = utils::fixture::emscripten_hello_world();
    let mut cmd = fixture.wasm_pack();
    if let Some(bin) = local_wasm_bindgen_bin() {
        cmd.env("WASM_BINDGEN_BIN", bin);
    } else {
        fixture.install_local_wasm_bindgen();
    }
    cmd.arg("build")
        .arg("--target")
        .arg(target)
        .assert()
        .success();
    assert_module_runs_in_node(&fixture.path.join("pkg"), "em_hello_world");
}

#[test]
fn emscripten_build_bundler() {
    run_build_and_smoke("bundler");
}

#[test]
fn emscripten_build_web() {
    run_build_and_smoke("web");
}

#[test]
fn emscripten_build_module_uses_source_phase_imports() {
    skip_without_emcc!();
    let fixture = utils::fixture::emscripten_hello_world();
    let mut cmd = fixture.wasm_pack();
    if let Some(bin) = local_wasm_bindgen_bin() {
        cmd.env("WASM_BINDGEN_BIN", bin);
    } else {
        fixture.install_local_wasm_bindgen();
    }
    // `--no-opt` works around an emcc limitation: its bundled acorn JS
    // optimizer doesn't yet parse `import source` syntax at -O2+ (the
    // emitted JS crashes its own optimizer). Drop --no-opt once the
    // `acorn-import-phases` plugin lands upstream in emscripten.
    cmd.arg("build")
        .arg("--target")
        .arg("module")
        .arg("--no-opt")
        .assert()
        .success();

    let pkg = fixture.path.join("pkg");
    let mjs = pkg.join("em_hello_world.mjs");
    let body = std::fs::read_to_string(&mjs).unwrap();
    // The `module` target is the one that motivates source-phase imports.
    assert!(
        body.contains("import source"),
        "--target module output should use `import source` for the wasm; got:\n{}",
        &body[..body.len().min(2000)],
    );

    assert_module_runs_in_node(&pkg, "em_hello_world");
}

#[test]
fn emscripten_build_nodejs() {
    run_build_and_smoke("nodejs");
}

#[test]
fn emscripten_build_deno() {
    run_build_and_smoke("deno");
}

#[test]
fn emscripten_build_no_modules_is_rejected() {
    skip_without_emcc!();
    let fixture = utils::fixture::emscripten_hello_world();
    let mut cmd = fixture.wasm_pack();
    if let Some(bin) = local_wasm_bindgen_bin() {
        cmd.env("WASM_BINDGEN_BIN", bin);
    } else {
        fixture.install_local_wasm_bindgen();
    }
    cmd.arg("build")
        .arg("--target")
        .arg("no-modules")
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "`--target no-modules` is not supported for wasm32-unknown-emscripten",
        ));
}

#[test]
fn emscripten_package_json_points_at_mjs() {
    skip_without_emcc!();
    let fixture = utils::fixture::emscripten_hello_world();
    let mut cmd = fixture.wasm_pack();
    if let Some(bin) = local_wasm_bindgen_bin() {
        cmd.env("WASM_BINDGEN_BIN", bin);
    } else {
        fixture.install_local_wasm_bindgen();
    }
    cmd.arg("build").arg("--target").arg("web").assert().success();

    let pkg_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(fixture.path.join("pkg/package.json")).unwrap()).unwrap();
    assert_eq!(pkg_json["main"], "em_hello_world.mjs");
    assert_eq!(pkg_json["type"], "module");
    assert_eq!(pkg_json["types"], "em_hello_world.d.ts");
    let files = pkg_json["files"].as_array().unwrap();
    let names: Vec<&str> = files.iter().map(|v| v.as_str().unwrap()).collect();
    assert!(names.contains(&"em_hello_world.mjs"), "expected .mjs in files: {names:?}");
    assert!(names.contains(&"em_hello_world.wasm"), "expected .wasm in files: {names:?}");
    assert!(
        !names.iter().any(|n| n.ends_with("_bg.wasm") || n.ends_with("_bg.js")),
        "emscripten pkg should not list `_bg.*` artifacts: {names:?}"
    );
}

#[test]
fn emscripten_test_command_is_rejected() {
    // No emcc gating — this path doesn't actually invoke emcc, just
    // verifies the early-rejection logic in `wasm-pack test`.
    let fixture = utils::fixture::emscripten_hello_world();
    fixture
        .wasm_pack()
        .arg("test")
        .arg("--node")
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "does not currently support the wasm32-unknown-emscripten target",
        ));
}
