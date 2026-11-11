# tests/web.rs

`web.rs` is an integration test [defined with Cargo][cargo-tests] that is
intended to be run in a headless web browser via the `wasm-pack test` command.

[cargo-tests]: https://doc.rust-lang.org/cargo/guide/tests.html

It contains three key parts:

1. [`#[wasm_bindgen_test] functions`](#a1-wasm_bindgen_test-functions)
2. [Crate Configuration](#a2-crate-configuration)
3. [`#![cfg]` directives](#a3-cfg-directives)

---

## 1. `#[wasm_bindgen_test]` functions

The `#[wasm_bindgen_test]` is like the [normal Rust `#[test]`
attribute][rust-test], except it defines a test accessible to WebAssembly and
headless web browser testing.

> **Note**: Eventually `#[test]` will work with WebAssembly as well! Currently
> though [custom test frameworks][ctf] are not stable.

[rust-test]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[ctf]: https://github.com/rust-lang/rust/issues/50297

```rust
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
```

Here the `pass` function is a unit test which asserts that arithmetic works in
WebAssembly like we'd expect everywhere else. If the test panics (such as the
`assert_eq!` being false) then the test will fail, otherwise the test will
succeed.

The [reference documentation for `#[wasm_bindgen_test]`][wbg-test] should have
more information about defining these tests.

[wbg-test]: https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html

## 2. Crate Configuration

Other than the test in this module, we'll also see:

```rust
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
```

Like we saw earlier in `src/lib.rs` the `*` import pulls in everything from
`wasm_bindgen_test`, notably the `wasm_bindgen_test_configure` macro and the
`wasm_bindgen_test` attribute.

The `wasm_bindgen_test_configure` macro (denoted by ending in `!`) is used to
indicate that the test is intended to execute in a web browser as opposed to
Node.js, which is the default.

## 3. `#![cfg]` directives

The last part we'll notice about this crate is this statement at the top:

```rust
#![cfg(target_arch = "wasm32")]
```

This statement means that the test is only intended for the `wasm32`
architecture, or the `wasm32-unknown-unknown` target. This enables `cargo test`
to work in your project if the library is also being developed for other
platforms by ensuring that these tests only execute in a web browser.
