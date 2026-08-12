# src/lib.rs

`lib.rs` is the template's main source file. The name `lib.rs` commonly implies that this Rust project will be compiled as a library.

It contains three key parts:

1. [`#[wasm_bindgen] functions`](#a1-wasm_bindgen-functions)
2. [Crate imports](#a2-crate-imports)
3. [`wee_alloc` optional dependecy](#a3-wee_alloc-optional-dependecy)
	- [What is `wee_alloc`?](#what-is-wee_alloc)

---

We'll start with the most important part of `lib.rs` -- the two `#[wasm_bindgen]` functions (which you can find at the bottom of the file). In many cases, this is the only part of `lib.rs` you will need to modify.

## 1. Using `wasm_bindgen`

To expose functionality from the `wasm-bindgen` crate more conveniently we can use the `use` keyword.
`use` allows us to conveniently refer to parts of a crate or module. You can learn more about how Rust
lets you write modular code in [this chapter of the book](https://doc.rust-lang.org/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html).

```rust
use wasm_bindgen::prelude::*;
```

Many crates contain a prelude, a list of things that are convenient to import
all at once. This allows common features of the module to be conveniently
accessed without a lengthy prefix. For example, in this file we can use
`#[wasm_bindgen]` only because it is brought into scope by the prelude.

The asterisk at the end of this `use` indicates that everything inside the module `wasm_bindgen::prelude` (i.e. the module `prelude` inside the crate `wasm_bindgen`) can be referred to without prefixing it with `wasm_bindgen::prelude`.

For example, `#[wasm_bindgen]` could also be written as `#[wasm_bindgen::prelude::wasm_bindgen]`, although this is not recommended.

## 1. `#[wasm_bindgen]` functions

The `#[wasm_bindgen]` attribute indicates that the function below it will be accessible both in JavaScript and Rust.

```rust
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
```

The `extern` block imports the external JavaScript function `alert` into Rust. This declaration is required to call `alert` from Rust. By declaring it in this way, `wasm-bindgen` will create JavaScript stubs for `alert` which allow us to pass strings back and forth between Rust and JavaScript.

We can see that the `alert` function requires a single parameter `s` of type `&str`, a string. In Rust, any string literal such as `"Hello, test-wasm!"` is of type `&str`. So, `alert` could be called by writing `alert("Hello, test-wasm!");`.

We knew to declare `alert` in this way because it is how we would call `alert` in JavaScript -- by passing it a string argument.

```rust
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test-wasm!");
}
```

If we were to write the `greet` function without the `#[wasm_bindgen]` attribute, then `greet` would not be easily accessible within JavaScript. Furthermore, we wouldn't be able to natively convert certain types such as `&str` between JavaScript and Rust. So, both the `#[wasm_bindgen]` attribute and the prior import of `alert` allow `greet` to be called from JavaScript.

This is all you need to know to interface with JavaScript, at least to start! You can learn a bunch more by reading the
[`wasm-bindgen` documentation]!

[`wasm-bindgen` documentation]: https://rustwasm.github.io/docs/wasm-bindgen/

If you are curious about the rest, read on.

## 2. Crate Organization

```rust
mod utils;
```
This statement declares a new module named `utils` that is defined by the contents of `utils.rs`. Equivalently, we could place the contents of `utils.rs` inside the `utils` declaration, replacing the line with:

```rust
mod utils {
    // contents of utils.rs
}
```

Either way, the contents of `utils.rs` define a single public function `set_panic_hook`. Because we are placing it inside the `utils` module, we will be able to call the function directly by writing `utils::set_panic_hook()`. We will discuss how and why to use this function in `src/utils.rs`.


```rust
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]	static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

At compile time this will test if the `wee_alloc` feature is enabled for this
compilation. If it's enabled we'll configure a global allocator (according to
[`wee_alloc`'s docs][wee-alloc-docs]), otherwise it'll compile to nothing.

[wee-alloc-docs]: https://docs.rs/wee_alloc/0.4.3/wee_alloc/

As we saw earlier, the `default` vector in `[features]` only contains `"console_error_panic_hook"` and not `"wee_alloc"`. So, in this case, this 
block will be replaced by no code at all, and hence the default memory allocator will be used instead of `wee_alloc`.

