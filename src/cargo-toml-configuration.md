# `Cargo.toml` Configuration

`wasm-pack` can be configured via the `package.metadata.wasm-pack` key in
`Cargo.toml`. Every option has a default, and is not required.

There are three profiles: `dev`, `profiling`, and `release`. These correspond to
the `--dev`, `--profiling`, and `--release` flags passed to `wasm-pack build`.

The available configuration options and their default values are shown below:

```toml
[package.metadata.wasm-pack.profile.dev]
# Should `wasm-opt` be used to further optimize the wasm binary generated after
# the Rust compiler has finished? Using `wasm-opt` can often further decrease
# binary size or do clever tricks that haven't made their way into LLVM yet.
#
# Configuration is set to `false` by default for the dev profile, but it can
# be set to an array of strings which are explicit arguments to pass to
# `wasm-opt`. For example `['-Os']` would optimize for size while `['-O4']`
# would execute very expensive optimizations passes
#
# In most cases, the `-O[X]` flag is enough. However, if you require extreme
# optimizations, see the full list of `wasm-opt` optimization flags
# https://github.com/WebAssembly/binaryen/blob/version_117/test/lit/help/wasm-opt.test
wasm-opt = ['-O']

[package.metadata.wasm-pack.profile.dev.wasm-bindgen]
# Should we enable wasm-bindgen's debug assertions in its generated JS glue?
debug-js-glue = true
# Should wasm-bindgen demangle the symbols in the "name" custom section?
demangle-name-section = true
# Should we emit the DWARF debug info custom sections?
dwarf-debug-info = false
# Should we omit the default import path?
omit-default-module-path = false
# Controls whether wasm-bindgen will split linked modules out into their own files. Enabling this is recommended, because it allows lazy-loading the linked modules and setting a stricter Content Security Policy. Only available in wasm-bindgen 0.2.95 and later.
split-linked-modules = false

[package.metadata.wasm-pack.profile.profiling]
wasm-opt = ['-O']

[package.metadata.wasm-pack.profile.profiling.wasm-bindgen]
debug-js-glue = false
demangle-name-section = true
dwarf-debug-info = false
omit-default-module-path = false

# `wasm-opt` is on by default in for the release profile, but it can be
# disabled by setting it to `false`
[package.metadata.wasm-pack.profile.release]
wasm-opt = false

[package.metadata.wasm-pack.profile.release.wasm-bindgen]
debug-js-glue = false
demangle-name-section = true
dwarf-debug-info = false
omit-default-module-path = false
```
