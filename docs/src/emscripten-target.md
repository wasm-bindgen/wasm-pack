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

## How it works

The whole build is a single cargo invocation. rustc drives `emcc` as the
linker for the emscripten target, and wasm-pack injects
`-sWASM_BINDGEN`, which makes emcc run `wasm-bindgen` itself as a
post-link step — emcc detects the marker section wasm-bindgen embeds when a
crate is compiled for emscripten, runs the CLI over the linked wasm, and
integrates the generated bindings into its own JS output.

wasm-pack's role reduces to:

1. Installing the `wasm-bindgen` CLI matching your `Cargo.lock` and putting
   it on `PATH` for emcc to find. For a git `wasm-bindgen` dependency the
   CLI is built from the same revision.
2. Running `cargo rustc` with the emcc output-shape settings injected as
   final-crate link args:
   `-sWASM_BINDGEN -sMODULARIZE=instance -sEXPORT_ES6 -sAUTO_INIT`
   (plus per-`--target` additions, below), and
   `-sWASM_LEGACY_EXCEPTIONS=0 -sBINARYEN_EXTRA_PASSES=--translate-to-exnref`
   so the output uses standard (exnref) exception handling rather than the
   legacy instructions rustc's LLVM still emits.
3. Copying the emitted `.js` + `.wasm` into `pkg/` and writing the
   `package.json`.

There is no separate wasm-opt step: emcc runs its own optimization pipeline
at link time, driven by the rustc opt-level.

Because the settings are passed as `cargo rustc` trailing args, they
*compose* with any rustflags you configure in `.cargo/config.toml` — add your
own emcc settings (e.g. `-sSTACK_SIZE=8MB`, `-sALLOW_MEMORY_GROWTH`) under
`[target.wasm32-unknown-emscripten] rustflags` as
`"-Clink-arg=-s..."` entries and both sets apply.

> **Toolchain status:** the post-link `-sWASM_BINDGEN` support the
> cargo-driven flow relies on ([emscripten#27208]) has landed on emscripten
> `main` and will ship in emscripten 6.0.10. Until then, wasm-pack's
> auto-installed toolchain is a pinned emsdk tip-of-tree build.

[emscripten#27208]: https://github.com/emscripten-core/emscripten/pull/27208

## Prerequisites

None beyond `git` and `python3` on `PATH`: if no Emscripten toolchain is
found, wasm-pack offers to install one into its cache (a one-time ~1.3 GB
download) and applies it process-scoped to the build — no shell activation
needed. An existing toolchain is preferred when present, checked in order:

1. `emcc` on `PATH`
2. an activated emsdk via the `EMSDK` environment variable
3. an activated emsdk at `~/emsdk`
4. the wasm-pack-managed install

Note that until emscripten 6.0.10 ships, a user-provided toolchain (options
1–3) needs to be a tip-of-tree build (`./emsdk install tot`).

Your crate needs `wasm-bindgen >= 0.2.122`, which ships the emscripten
output mode and the marker section.

## Quick start

```sh
wasm-pack new my-pkg --emscripten
cd my-pkg
wasm-pack build
```

The result in `pkg/` differs from the default wasm-pack layout:

- `<name>.js` — a self-initializing ES module. The wasm-bindgen API surface
  is exported directly; no init call or factory invocation is needed:

  ```js
  import { greet } from "./pkg/my_pkg.js";
  greet("world");
  ```

- `<name>.wasm` — the emscripten-linked wasm, referenced by the JS by
  filename. (There is no `<name>_bg.wasm`.)

## What the template generates

`wasm-pack new --emscripten <name>` produces a crate with this shape:

```
<name>/
  Cargo.toml          # a bin crate (src/main.rs)
  .cargo/
    config.toml       # target = "wasm32-unknown-emscripten" + rustflags
  src/
    main.rs           # fn main() {} + a tiny `greet()` example
  README.md
```

Two things are notable compared to the default template:

1. **It's a `bin` crate.** rustc links emscripten executables through emcc
   as self-contained main modules. A `cdylib` would instead be linked as an
   emscripten *side module* (`-sSIDE_MODULE=2`) — a relocatable object for
   emscripten's dynamic linking, not a usable package. `main()` runs
   automatically when the module initializes (the emscripten idiom) and may
   be empty; the package API is the `#[wasm_bindgen]` exports.

2. **`.cargo/config.toml`** — selects the emscripten target and sets
   codegen rustflags:
   - `-Cpanic=abort` — `panic=unwind` isn't supported across the
     wasm-bindgen boundary on emscripten yet
     ([wasm-bindgen#5165](https://github.com/wasm-bindgen/wasm-bindgen/issues/5165)).
   - `-Crelocation-model=static` — PIC isn't needed for a statically linked
     main module.
   - `-Cllvm-args=-enable-emscripten-cxx-exceptions=0` — avoids pulling in
     a C++ exception runtime, required with `panic=abort`.

If you build your own crate (without using the template), configure both of
these.

## Build targets

All emscripten output is ESM; the `--target` flag tweaks the emcc settings:

| `--target` | Effect |
|---|---|
| `bundler` (default) | baseline settings |
| `web` | same as `bundler` |
| `deno` | same as `bundler` (no `package.json`, as for other targets) |
| `nodejs` | adds `-sENVIRONMENT=node` (drops browser-detection probes) |
| `module` | adds `-sSOURCE_PHASE_IMPORTS` (`import source` for the wasm) |
| `no-modules` | not supported — emcc emits ES modules |

The `module` target uses [source-phase imports][src-phase] for the wasm:

```js
import source wasmModule from './<name>.wasm';
```

This requires a host that understands the proposal.

[src-phase]: https://github.com/tc39/proposal-source-phase-imports

## Limitations

- **`wasm-pack test` is not supported.** The `wasm-bindgen-test` runner
  isn't currently wired up for emscripten; run tests via `cargo test`
  directly.
- **Optimized (release) builds need
  [wasm-bindgen#5270](https://github.com/wasm-bindgen/wasm-bindgen/pull/5270).**
  With earlier wasm-bindgen releases, emcc's meta-DCE at `-O2` strips glue
  internals the output reads dynamically; use `--dev` builds until a release
  containing it ships.
- **No TypeScript declarations yet.** The `.d.ts` story runs through emcc's
  `--emit-tsd`, which needs upstream support for wasm-bindgen's
  multi-value-return exports before wasm-pack can enable it.
- **`panic=unwind` is not supported yet** — tracked in
  [wasm-bindgen#5165](https://github.com/wasm-bindgen/wasm-bindgen/issues/5165).
