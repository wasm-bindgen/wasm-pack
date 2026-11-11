# Cargo.toml

`Cargo.toml` is the manifest file for Rust's package manager, [`cargo`]. This file contains
metadata such as name, version, and dependencies for packages, which are call "crates" in Rust.

[`cargo`]: https://doc.rust-lang.org/cargo/

There's a bunch of metadata that the template gives us, but there are three key parts to discuss:

1. [`crate-type`](#a1-crate-type)
2. [`wasm-bindgen` dependency](#a2-wasm-bindgen-dependency)
3. [`[features]` and `wee_alloc`, `console_error_panic_hook` dependencies](#a3-features-and-wee_alloc-console_error_panic_hook-dependencies)

<hr/>

## 1. `crate-type`

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

A Rust-`wasm` crate is a bit different from a normal crate, and as a result, we need to note
this in our `Cargo.toml`.

This `[lib]` annotation is typically not needed in Cargo projects, and if you're
familiar with other Rust crates you'll remember that the most common crate types
are `rlib` (the default) or `bin` for binaries (which don't need a `crate-type`
annotation).

Here though `crate-type = ["cdylib"]` typically signifies that you'd like the
compiler to create a dynamic system library, but for WebAssembly target it
simply means "create a `*.wasm` file without a `start` function". On other
platforms this output type will create `*.so` file on Linux, `*.dylib` on
macOS, and `*.dll` Windows.

We also specify `crate-type = ["rlib"]` to ensure that our library can be unit
tested with `wasm-pack test` (which we'll see later). Without this we wouldn't
be able to test our library because the `cdylib` crate type is incompatible with
`wasm-pack`'s style of unit tests.

You can read more about linking and crate types, [here](https://doc.rust-lang.org/reference/linkage.html).

## 2. `wasm-bindgen` dependency

[`wasm-bindgen`] is our most important dependency. This package allows us to use the
`#[wasm-bindgen]` attribute to tag code that represents the interface we want between
our JavaScript and Rust-generated `wasm`. We can import JS and export Rust by using this
attribute.

[`wasm-bindgen`]: https://rustwasm.github.io/docs/wasm-bindgen/

```toml
wasm-bindgen = "0.2"
```

We'll see more about how to use this library when we discuss what has been generated in `lib.rs`.

⚠️ If you are coming from JavaScript, you might note that when we add the dependency
there is no `^` or `~` symbol- it looks like we're locking to the `0.2` version.
However, that's not the case! In Rust, the `^` is implied. You can read more about this in the
[cargo documentation on specifying dependencies].

[cargo documentation on specifying dependencies]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html

## 3. `[features]` and [`wee_alloc`], [`console_error_panic_hook`] dependencies

[`wee_alloc`]: https://crates.io/crates/wee_alloc
[`console_error_panic_hook`]: https://crates.io/crates/console_error_panic_hook

As part of our effort to design a template that helps people discover useful crates
for their particular use case, this template includes two dependencies that can be
very useful for folks developing Rust-`wasm` crates:[ `console_error_panic_hook`] and
[`wee_alloc`].

Because these dependencies are useful primarily in a specific portion of the Rust-`wasm`
crate development workflow, we've also set up a bit of glue code that allows us to include
them both as dependencies, but also allows them to be optionally included.

```toml
[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = "0.2"

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.1", optional = true }

# `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
# compared to the default allocator's ~10K. It is slower than the default
# allocator, however.
#
# Unfortunately, `wee_alloc` requires nightly Rust when targeting wasm for now.
wee_alloc = { version = "0.4.2", optional = true }
```

In our code, we'll mark certain parts of code as running only if certain `[features]`
are enabled, specifically, `console_error_panic_hook` and `wee_alloc`. By default,
only `console_error_panic_hook` is enabled. To disable or enable either feature, by
default, we can edit the `default` vector under `[features]`.

To learn more about these features, we discuss them in-depth in the [`src/lib.rs`] and
[`src/utils.rs`] sections.

[`src/lib.rs`]: src-lib-rs.html
[`src/utils.rs`]: src-utils-rs.html

Briefly, they include:

+ **console_error_panic_hook** for logging panic messages to the developer console.
+ **wee_alloc**, an allocator optimized for small code size.
