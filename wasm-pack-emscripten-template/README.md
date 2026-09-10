<div align="center">

  <h1><code>{{project-name}}</code></h1>

  <strong>A wasm-bindgen package targeting <code>wasm32-unknown-emscripten</code>.</strong>

</div>

## Prerequisites

This template targets the emscripten toolchain in addition to Rust. Install
[emsdk](https://emscripten.org/docs/getting_started/downloads.html) and run
`source <emsdk>/emsdk_env.sh` before building.

## Build

```sh
wasm-pack build
```

The whole build is a single cargo invocation: rustc drives `emcc` as the
linker, and emcc runs `wasm-bindgen` itself as a post-link step
(`-sWASM_BINDGEN`). That produces a `pkg/` directory containing:

- `{{project-name}}.js` — self-initializing ES module
- `{{project-name}}.wasm` — emscripten-linked wasm
- `package.json` — npm package metadata

## Use

The module self-initializes on import; the wasm-bindgen API surface is
exported directly:

```js
import { greet } from "./pkg/{{project-name}}.js";

greet("world");
```

## Why emscripten?

The emscripten target unlocks standard-library APIs that aren't available
under `wasm32-unknown-unknown`:

- `std::time::{Instant, SystemTime}`
- `std::env::{current_dir, vars}`
- `std::fs` (in-memory MEMFS)
- `std::collections::HashMap` with default random state
- `rand::random()` via emscripten's `getentropy`
- POSIX-style I/O and many libc APIs

The cost is a heavier runtime (~10-50 KB JS overhead). If you only need
pure Rust + `js-sys`/`web-sys`, the default `wasm32-unknown-unknown` target
remains the right choice — use the regular `wasm-pack-template` instead.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT)
at your option.
