# Building for `wasm32-unknown-emscripten`

`wasm-pack` supports the `wasm32-unknown-emscripten` target as an alternative
to the default `wasm32-unknown-unknown`. The two produce wasm binaries that
behave differently at runtime: emscripten output includes a libc, file system
shims, POSIX-style APIs, and a richer JavaScript runtime around the wasm.

You generally want emscripten when:

- You need `std::time::{Instant, SystemTime}`.
- You need `std::env::{current_dir, vars}`.
- You need `std::fs::*` (backed by an in-memory MEMFS).
- You need `std::collections::HashMap` with default random state.
- You need `rand::random()` to Just Work (via `getentropy`).
- You're linking Rust against C/C++ sources via `bindgen`/`cxx`.

You generally want `wasm32-unknown-unknown` when:

- Your crate is pure Rust + `js-sys`/`web-sys`.
- You want the smallest possible runtime overhead.
- You care about cold-start time on the web.

Most projects don't need emscripten. The default target stays the right choice
for the common case.

## Prerequisites

`wasm-pack` invokes the Emscripten SDK (`emcc`) during the build. Install
[emsdk](https://emscripten.org/docs/getting_started/downloads.html) and
activate it before running `wasm-pack`:

```sh
git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
cd ~/emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

`emcc --version` should print at least 3.1.60. Older versions lack
`-sSOURCE_PHASE_IMPORTS=1` and will fail the build with a clear error.

`wasm-pack` will auto-detect a `~/emsdk` install or honor the `$EMSDK`
environment variable if `emcc` isn't already on `PATH`. CI configurations
should `source emsdk_env.sh` before running `wasm-pack` to make `emcc`
available to child processes.

## Quick start

```sh
wasm-pack new my-pkg --emscripten
cd my-pkg
wasm-pack build
```

The first command scaffolds a project that's pre-configured for the
emscripten target. The second runs the full pipeline:

```
cargo build → emcc link → wasm-bindgen → emcc post-link
```

The result in `pkg/` matches the layout for other `wasm-pack` targets, with
two notable differences:

- The JS module has an `.mjs` extension (always ESM).
- There's no `<name>_bg.wasm`; the wasm file is just `<name>.wasm`.

## What the template generates

`wasm-pack new --emscripten <name>` produces a crate with this shape:

```
<name>/
  Cargo.toml          # crate-type = ["staticlib"]
  .cargo/
    config.toml       # target = "wasm32-unknown-emscripten"
  src/
    lib.rs            # a tiny `greet()` example
  README.md
```

Two things are notable compared to the default template:

1. **`crate-type = ["staticlib"]`** — emcc consumes a static library and
   links it into the final wasm itself. `cdylib` would skip emcc entirely.

2. **`.cargo/config.toml`** — selects the emscripten target and sets the
   rustflags emcc expects:
   - `-Cpanic=abort` — `panic=unwind` isn't supported across the wasm-bindgen
     boundary on emscripten yet.
   - `-Crelocation-model=static` — the staticlib is linked directly; PIC
     isn't needed.
   - `-Cllvm-args=-enable-emscripten-cxx-exceptions=0` — avoids pulling in
     a C++ exception runtime we don't ship.

If you build your own crate (without using the template), make sure to
configure both of these.

## Build targets

The wasm-pack `--target` flag still selects the output shape:

| `--target` | Output |
|---|---|
| `bundler` (default) | `<name>.mjs` ESM async factory, env: `web,node` |
| `web` | same as `bundler` |
| `module` | `<name>.mjs` with `import source` for the wasm |
| `nodejs` | `<name>.mjs`, env: `node` |
| `deno` | `<name>.mjs`, env: `web,node` |
| `no-modules` | not supported — emcc produces module-shaped output only |

All shapes are ESM; emscripten doesn't emit CJS. The `nodejs` target is
distinguished only by the `-sENVIRONMENT=node` setting, which omits
browser-detection probes from the runtime.

The `module` target uses [source-phase imports][src-phase] for the wasm:

```js
import source wasmModule from './<name>.wasm';
```

This requires a host that understands the proposal (modern bundlers, Node 22+,
Deno).

[src-phase]: https://github.com/tc39/proposal-source-phase-imports

## Limitations

A few things differ from the default wasm-pack target:

- **`wasm-pack test` is not supported.** The `wasm-bindgen-test` runner
  isn't currently wired up for emscripten; tests must run via
  `cargo test` directly.
- **TypeScript declarations come from wasm-bindgen only.** emcc's
  `--emit-tsd` mode currently asserts on wasm-bindgen-style multi-value
  returns (any Rust function returning `String`, `Vec`, etc.). The `.d.ts`
  in `pkg/` describes the wasm-bindgen surface but does not type
  emscripten runtime methods.
- **Optimization is capped at `-O2`.** emcc's `-O3` enables wasm-opt's
  `--minify-imports-and-exports` pass, which renames wasm exports to
  single letters — but wasm-bindgen's generated JS glue references the
  original names. `-O2` produces near-equivalent output without the
  renaming.
- **`--target module` builds run unoptimized today.** emcc's bundled JS
  optimizer (`acorn-optimizer`) can't parse `import source` syntax. Pass
  `--no-opt` for `--target module` builds, or wait for the upstream emcc
  fix.

## How the build pipeline works

For curiosity:

1. **`cargo build`** — cargo invokes `rustc --target wasm32-unknown-emscripten`,
   which under the hood links via emcc to produce a `librustworker.a`
   staticlib.

2. **`emcc` link** — `wasm-pack` invokes `emcc --no-entry --oformat=bare`
   to produce a bare `.wasm` from the staticlib. Symbols wasm-bindgen
   needs are preserved via `-sEXPORTED_FUNCTIONS`.

3. **`wasm-bindgen`** — runs over the bare `.wasm` with `--keep-lld-exports`
   (no `--target`; emscripten output mode is auto-detected from the
   `__wasm_bindgen_emscripten_marker` custom section). Produces a
   rewritten `_bg.wasm`, a `library_bindgen.js` (an emscripten JS
   library), and a `.d.ts`.

4. **`emcc --post-link`** — combines the wasm-bindgen-rewritten wasm with
   `library_bindgen.js` to emit the final `<name>.mjs` (ESM factory
   function) and `<name>.wasm` (final, post-linked binary).

Each phase is a separate, self-contained tool invocation. Intermediate
artifacts (`linked.wasm`, `_bg.wasm`, `library_bindgen.js`) are not
shipped in `pkg/`.
