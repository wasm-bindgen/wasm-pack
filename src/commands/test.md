# wasm-pack test

The `wasm-pack test` command wraps the [wasm-bindgen-test-runner](https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test)
CLI allowing you to run wasm tests in different browsers without needing to install the different
webdrivers yourself.

```
wasm-pack test --help
```

## Path

The `wasm-pack test` command can be given an optional path argument.

This path should point to a directory that contains a `Cargo.toml` file. If no
path is given, the `test` command will run in the current directory.

```
# Run tests for the current directory's crate
wasm-pack test

# Run tests for a specified crate
wasm-pack test crates/crate-in-my-workspace
```

## Profile

The `test` command accepts an optional profile argument: `--release`.

If none is supplied, then a debug test build will be used.

## Test environment

Choose where to run your tests by passing in any combination of testing environment flags.

`--headless` is useful for running browser tests in a headless browser as part of a CI process.

```
wasm-pack test --node --firefox --chrome --safari --headless
```

## Panic strategy

The `test` command accepts the `--panic-unwind` flag, which builds the test
binary with `panic=unwind` via the nightly toolchain and `-Z build-std`. See
the [`build` command's documentation](./build.md#panic-strategy) for details.

```
wasm-pack test --node --panic-unwind
```

## Extra options

The `test` command can pass extra options straight to `cargo test` even if they are not
supported in wasm-pack.

To use them simply add the extra arguments at the very end of your command, just
as you would for `cargo test`.

`cargo test -h` for a list of all options that you can pass through.

## Running only some tests

When debugging a specific issue, you may find yourself wanting to run a subset of tests, instead of your entire suite of tests.

Here are a few examples of how to run a subset of your tests:

```
# Example directory structure
$ tree crates/foo
├── Cargo.toml
├── README.md
├── src
│   ├── diff
│   │   ├── diff_test_case.rs
│   │   └── mod.rs
│   ├── lib.rs
└── tests
    ├── diff_patch.rs
    └── node.rs
```

```
# Run all tests in tests/diff_patch.rs in Firefox
wasm-pack test crates/foo --firefox --headless --test diff_patch

# Run all tests in tests/diff_patch.rs that contain the word "replace"
wasm-pack test crates/foo --firefox --headless --test diff_patch replace

# Run all tests inside of a `tests` module inside of src/lib/diff.rs
wasm-pack test crates/foo --firefox --headless --lib diff::tests

# Same as the above, but only if they contain the word replace
wasm-pack test crates/foo --firefox --headless --lib diff::tests::replace
```

Note that you can also filter tests by location in which they're supposed to
run. For example:

```
# Run all tests which are intended to execute in Node.js
wasm-pack test --node

# Run all tests which are intended to be executed in a browser
wasm-pack test --firefox --headless
```
