# wee_alloc

1. [What is `wee_alloc`?](#what-is-wee_alloc)
2. [Enabling `wee_alloc`](#enabling-wee_alloc)

## What is `wee_alloc`?

WebAssembly code is frequently transmitted over the wire to users, so compiled
code size is often important to ensure an application loads quickly and is
responsive.

> `wee_alloc` is a tiny allocator designed for WebAssembly that has a (pre-compression) code-size footprint of only a single kilobyte.

[An analysis](http://fitzgeraldnick.com/2018/02/09/wee-alloc.html) suggests that over half of the bare minimum WebAssembly memory footprint is required by Rust's default memory allocator. Yet, WebAssembly code often does not require a sophisticated allocator, since it often just requests a couple of large initial allocations.

`wee_alloc` trades off size for speed. It has a tiny code-size
footprint, but it is not competitive in terms of performance with the
default global allocator, for example.

For even more details, see the [`wee_alloc`
repository](https://github.com/rustwasm/wee_alloc), or
[general documentation](https://rustwasm.github.io/docs/book/reference/code-size.html) about
shrinking code size of WebAssembly binaries.

## Enabling `wee_alloc`

In `lib.rs`, we have the configuration for `wee_alloc` inside a `cfg_if!` macro:

```rust
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
```

This code block is intended to initialize `wee_alloc` as the global memory
allocator, but only if the `wee_alloc` feature is enabled at compile time. The
feature can be enabled by passing extra options while building:

```
$ wasm-pack build --features wee_alloc
```

or alternatively you could turn it on by default in `Cargo.toml`:

```toml
[features]
default = ["console_error_panic_hook", "wee_alloc"]
```
