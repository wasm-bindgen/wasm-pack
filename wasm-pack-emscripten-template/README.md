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

That produces a `pkg/` directory containing:

- `{{project-name}}.mjs` — ESM module factory
- `{{project-name}}.wasm` — emscripten-linked wasm
- `{{project-name}}.d.ts` — TypeScript declarations
- `package.json` — npm package metadata

## Use

```js
import Module from "./pkg/{{project-name}}.mjs";

const m = await Module();
m.greet("world");
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
