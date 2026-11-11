# src/utils.rs

The purpose of `utils.rs` is to define the `utils` module, which contains a single function `set_panic_hook`. This function becomes part of the `utils` module in `lib.rs`, as described in the preceding section.

If the `console_error_panic_hook` feature is not enabled, then `set_panic_hook` is defined to be an inlined empty function. So, there is no run-time performance or code-size penalty incurred by its use.

We will discuss:
1. [Defining `set_panic_hook`](#1-defining-set_panic_hook)
2. [What is `console_error_panic_hook`?](#2-what-is-console_error_panic_hook)


---

## 1. Defining `set_panic_hook`

```rust
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
```

Here, we define a function that's preceded by a `cfg` attribute. This attribue,
`#[cfg(feature = "console_error_panic_hook")]`, tells Rust to check if the 
`console_error_panic_hook` feature is set at compile time. If it is, it will call
this function. If it isn't- it won't! 

## 2. What is `console_error_panic_hook`?

The [crate `console_error_panic_hook`][ceph] allows debugging Rust panic
messages in a web browser, making it much easier to debug WebAssembly code.

Let's compare what happens when Rust code panics before and after enabling the
feature:

**Before:** `"RuntimeError: Unreachable executed"`

**After:** `"panicked at 'index out of bounds: the len is 3 but the index is 4', libcore/slice/mod.rs:2046:10"`

To do this, a [panic hook] is configured that logs panics to the
developer console via the JavaScript `console.error` function.

Note though that `console_error_panic_hook` is not entirely automatic, so you'll
need to make sure that `utils::set_panic_hook()` is called before any of our
code runs (and it's safe to run `set_panic_hook` many times).

For more details, see the [`console_error_panic_hook`
repository](https://github.com/rustwasm/console_error_panic_hook).

[ceph]: https://crates.io/crates/console_error_panic_hook
[panic hook]: https://doc.rust-lang.org/std/panic/fn.set_hook.html
