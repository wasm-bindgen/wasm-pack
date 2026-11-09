//! Integration tests for `wasm-pack build` against the
//! `wasm32-unknown-emscripten` target.
//!
//! Strategy:
//!   1. Run only when `WASM_PACK_TEST_EMSCRIPTEN=1` (skip with an
//!      explanatory message otherwise): with no toolchain present these
//!      tests trigger wasm-pack's Emscripten auto-install (~1.3 GB into the
//!      test cache), which contributors running the general suite should
//!      not pay for implicitly. CI sets the variable in a dedicated
//!      single-threaded job on runners without emcc, exercising the
//!      auto-install path itself.
//!   2. For each supported wasm-pack `--target` value build the
//!      `emscripten_hello_world` fixture and exercise the full
//!      #[wasm_bindgen] surface from Node by importing the produced ES
//!      module (it self-initializes via `-sAUTO_INIT`) and calling every
//!      export.
//!   3. Verify `--target no-modules` is rejected with a clear message.
//!   4. Verify `wasm-pack test` rejects the emscripten target.

use crate::utils;
use assert_cmd::prelude::*;
use std::path::Path;
use std::process::Command;

/// Skip the calling test unless emscripten testing is opted into via
/// `WASM_PACK_TEST_EMSCRIPTEN=1`.
macro_rules! skip_unless_emscripten_tests {
    () => {
        if std::env::var("WASM_PACK_TEST_EMSCRIPTEN").as_deref() != Ok("1") {
            eprintln!("skipping: set WASM_PACK_TEST_EMSCRIPTEN=1 to enable emscripten tests.");
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
fn make_node_driver(js_path: &Path) -> String {
    // JSON-encode the path so backslashes (Windows) and unusual characters
    // can't break out of the JS string literal.
    let js_json = serde_json::to_string(&format!("file://{}", js_path.display()))
        .expect("path should serialise");
    format!(
        r#"
        globalThis.rs_test_doubler = (n) => n * 2;
        // The module self-initializes on import (-sAUTO_INIT); the
        // wasm-bindgen API surface is exported directly.
        const m = await import({js});

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

        // `rs_hostname` exercises `#[wasm_bindgen(module = "node:os")]` ESM
        // imports. We don't assert the exact value (hostnames vary across CI
        // runners), just that the call returns a non-empty string.
        const hn = m.rs_hostname();
        expect('rs_hostname returns string', typeof hn,                        'string');
        expect('rs_hostname non-empty',      hn.length > 0,                    true);

        // `js_namespace = console` — calling a method on a global namespace.
        // We stub `console.log` to capture the call and restore it after.
        const origLog = console.log;
        let captured = null;
        console.log = (s) => {{ captured = s; }};
        m.rs_log('namespaced log');
        console.log = origLog;
        expect('rs_log via console.log',     captured,                          'namespaced log');

        // `js_namespace = posix` + `module = "node:path"` — namespace on an
        // ESM-imported binding, not on globalThis.
        expect('rs_path_posix_join',         m.rs_path_posix_join('a', 'b'),    'a/b');

        // Class export with `js_namespace = ["app", "math"]` — the class
        // attaches under the `app` export rather than as a bare `Calc`.
        expect('Calc lives under app.math',  typeof m.app?.math?.Calc,          'function');
        const calc = new m.app.math.Calc(21);
        expect('Calc.double',                calc.double(),                      42);
        "#,
        js = js_json,
    )
}

/// Drive the test's Node smoke check against the produced ES module.
fn assert_module_runs_in_node(pkg_dir: &Path, module_name: &str) {
    let js = pkg_dir.join(format!("{module_name}.js"));
    assert!(js.exists(), "expected {js:?} to exist");

    let output = Command::new("node")
        .arg("--input-type=module")
        .arg("-e")
        .arg(make_node_driver(&js))
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
/// fixture and runs the comprehensive Node driver. `wasm-pack` reads the
/// fixture's lockfile, downloads the matching `wasm-bindgen` CLI from
/// crates.io's release artifacts, and puts it on `PATH` for emcc.
fn run_build_and_smoke(target: &str) {
    skip_unless_emscripten_tests!();
    let fixture = utils::fixture::emscripten_hello_world();
    // TODO: drop `--dev` (and bump the fixture pin) once a wasm-bindgen
    // release ships wasm-bindgen#5270. Before it, the emscripten glue reads
    // `wasmExports['name']` inline, which emcc's meta-DCE/minifier at -O2
    // doesn't track — internal exports like `__wbindgen_start` get stripped
    // and the glue fails at runtime.
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--dev")
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
fn emscripten_build_nodejs() {
    run_build_and_smoke("nodejs");
}

#[test]
fn emscripten_build_deno() {
    run_build_and_smoke("deno");
}

#[test]
fn emscripten_build_module_uses_source_phase_imports() {
    skip_unless_emscripten_tests!();
    let fixture = utils::fixture::emscripten_hello_world();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--dev")
        .arg("--target")
        .arg("module")
        .assert()
        .success();

    let pkg = fixture.path.join("pkg");
    let js = pkg.join("em_hello_world.js");
    let body = std::fs::read_to_string(&js).unwrap();
    // The `module` target is the one that motivates source-phase imports.
    // We don't execute it here: `import source` requires source-phase
    // support in the JS engine (pre-release node).
    assert!(
        body.contains("import source"),
        "--target module output should use `import source` for the wasm; got:\n{}",
        &body[..body.len().min(2000)],
    );
}

#[test]
fn emscripten_build_no_modules_is_rejected() {
    skip_unless_emscripten_tests!();
    let fixture = utils::fixture::emscripten_hello_world();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--dev")
        .arg("--target")
        .arg("no-modules")
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "--target no-modules is not supported for the emscripten target",
        ));
}

#[test]
fn emscripten_package_json_points_at_js() {
    skip_unless_emscripten_tests!();
    let fixture = utils::fixture::emscripten_hello_world();
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--dev")
        .arg("--target")
        .arg("web")
        .assert()
        .success();

    let pkg_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(fixture.path.join("pkg/package.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(pkg_json["main"], "em_hello_world.js");
    assert_eq!(pkg_json["type"], "module");
    let files = pkg_json["files"].as_array().unwrap();
    let names: Vec<&str> = files.iter().map(|v| v.as_str().unwrap()).collect();
    assert!(
        names.contains(&"em_hello_world.js"),
        "expected .js in files: {names:?}"
    );
    assert!(
        names.contains(&"em_hello_world.wasm"),
        "expected .wasm in files: {names:?}"
    );
    assert!(
        !names
            .iter()
            .any(|n| n.ends_with("_bg.wasm") || n.ends_with("_bg.js")),
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
