# wasm-pack new

The `wasm-pack new` command creates a new RustWasm project for you,
using [`cargo-generate`] under the hood.

It takes 3 parameters, name, template, and mode:

```
wasm-pack new <name> --template <template> --mode <normal|noinstall|force>
```

The default template is [`wasm-bindgen/wasm-pack`](https://github.com/wasm-bindgen/wasm-pack),
which includes the `wasm-pack-template` cargo-generate template.

## Name

The `wasm-pack new` command must be given a name argument, e.g.:

```
wasm-pack new myproject
```

## Template

The `wasm-pack new` command can be given an optional template argument, e.g.:

```
wasm-pack new myproject --template https://github.com/wasm-bindgen/wasm-pack
```

The template can be an address to a git repo that contains a [`cargo-generate`]
template.

[`cargo-generate`]: https://github.com/ashleygwilliams/cargo-generate

## Mode

The `wasm-pack new` command can be given an optional mode argument, e.g.:

```
wasm-pack new myproject --mode noinstall
```

The mode passed can be either "normal", "noinstall", or "force". "normal" is passed by
default.

`noinstall` means that wasm-pack should not attempt to install any underlying tools.
If a necessary tool cannot be found, the command will error.

`force` means that wasm-pack should not check the local Rust version. If a local Rust
is an unacceptable Rust version, the command will error.
