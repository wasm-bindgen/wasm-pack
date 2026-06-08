# wasm-pack build

The `wasm-pack build` command creates the files necessary for JavaScript
interoperability and for publishing a package to npm. This involves compiling
your code to wasm and generating a pkg folder. This pkg folder will contain the
wasm binary, a JS wrapper file, your `README`, and a `package.json` file.

The `pkg` directory is automatically `.gitignore`d by default, since it contains
build artifacts which are not intended to be checked into version
control.<sup>[0](#footnote-0)</sup> You can disable this with the
[`--no-gitignore` flag](#skip-gitignore).

## Path

The `wasm-pack build` command can be given an optional path argument, e.g.:

```
wasm-pack build examples/js-hello-world
```

This path should point to a directory that contains a `Cargo.toml` file. If no
path is given, the `build` command will run in the current directory.

## Output Directory

By default, `wasm-pack` will generate a directory for its build output called `pkg`.
If you'd like to customize this you can use the `--out-dir` flag.

```
wasm-pack build --out-dir out
```

The above command will put your build artifacts in a directory called `out`, instead
of the default `pkg`.

## Generated file names

Flag `--out-name` sets the prefix for output file names. If not provided, package name is used instead.

Usage examples, assuming our crate is named `dom`:

```
wasm-pack build
# will produce files
# dom.d.ts  dom.js  dom_bg.d.ts  dom_bg.wasm  package.json  README.md

wasm-pack build --out-name index
# will produce files
# index.d.ts  index.js  index_bg.d.ts  index_bg.wasm  package.json  README.md
```


## Profile

The `build` command accepts an optional profile argument: one of `--dev`,
`--profiling`, or `--release`. If none is supplied, then `--release` is used.

This controls whether debug assertions are enabled, debug info is generated, and
which (if any) optimizations are enabled.

| Profile       | Debug Assertions | Debug Info | Optimizations | Notes                                 |
|---------------|------------------|------------|---------------|---------------------------------------|
| `--dev`       | Yes              | Yes        | No            | Useful for development and debugging. |
| `--profiling` | No               | Yes        | Yes           | Useful when profiling and investigating performance issues. |
| `--release`   | No               | No         | Yes           | Useful for shipping to production.    |

The `--dev` profile will build the output package using cargo's [default
non-release profile][cargo-profile-sections-documentation]. Building this way is
faster but applies few optimizations to the output, and enables debug assertions
and other runtime correctness checks. The `--profiling` and `--release` profiles
use cargo's release profile, but the former enables debug info as well, which
helps when investigating performance issues in a profiler.

The exact meaning of the profile flags may evolve as the platform matures.

[cargo-profile-sections-documentation]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections

## Target

The `build` command accepts a `--target` argument. This will customize the JS
that is emitted and how the WebAssembly files are instantiated and loaded. For
more documentation on the various strategies here, see the [documentation on
using the compiled output][deploy].

```
wasm-pack build --target nodejs
```

| Option    | Usage | Description                                                                                                     |
|-----------|------------|-----------------------------------------------------------------------------------------------------|
| *not specified* or `bundler` | [Bundler][bundlers] | Outputs JS that is suitable for interoperation with a Bundler like Webpack. You'll `import` the JS and the `module` key is specified in `package.json`. `sideEffects: false` is by default. |
| `nodejs`  | [Node.js][deploy-nodejs] | Outputs JS that uses CommonJS modules, for use with a `require` statement. `main` key in `package.json`. |
| `web` | [Native in browser][deploy-web] | Outputs JS that can be natively imported as an ES module in a browser, but the WebAssembly must be manually instantiated and loaded. |
| `no-modules` | [Native in browser][deploy-web] | Same as `web`, except the JS is included on a page and modifies global state, and doesn't support as many `wasm-bindgen` features as `web` |
| `deno` | [Deno][deploy-deno] | Outputs JS that can be natively imported as an ES module in deno. |

[deploy]: https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html
[bundlers]: https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html#bundlers
[deploy-nodejs]: https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html#nodejs
[deploy-web]: https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html#without-a-bundler
[deploy-deno]: https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html#deno

## Scope

The `build` command also accepts an optional `--scope` argument. This will scope
your package name, which is useful if your package name might conflict with
something in the public registry. For example:

```
wasm-pack build examples/js-hello-world --scope test
```

This command would create a `package.json` file for a package called
`@test/js-hello-world`. For more information about scoping, you can refer to
the npm documentation [here][npm-scope-documentation].

[npm-scope-documentation]: https://docs.npmjs.com/misc/scope

## Mode

The `build` command accepts an optional `--mode` argument.
```
wasm-pack build examples/js-hello-world --mode no-install
```

| Option        | Description                                                                              |
|---------------|------------------------------------------------------------------------------------------|
| `no-install`  | `wasm-pack build` implicitly and create wasm binding without installing `wasm-bindgen`.  |
| `normal`      | do all the stuffs of `no-install` with installed `wasm-bindgen`.                         |

## Extra options

The `build` command can pass extra options straight to `cargo build` even if
they are not supported in wasm-pack. To use them simply add the extra arguments
at the very end of your command, just as you would for `cargo build`. For
example, to build the previous example using cargo's offline feature:

```
wasm-pack build examples/js-hello-world --mode no-install -- --offline
```

## Skip .gitignore

By default, `wasm-pack` creates a `.gitignore` file in the output directory
containing `*`, which prevents the build artifacts from being checked into
version control. If you want to commit the `pkg` directory to your repository
(e.g. for GitHub Pages, Deno packages, or monorepo setups), you can use the
`--no-gitignore` flag to skip generating the `.gitignore` file:

```
wasm-pack build --no-gitignore
```

This is also available with `--target web` and other targets.

## Panic strategy

By default, Rust panics in WebAssembly compile with `panic=abort`, which aborts
the WebAssembly instance on panic. The `--panic-unwind` flag changes this so
panics can be caught at FFI boundaries and converted to JavaScript exceptions
by tools like [`wasm-bindgen`'s catch-unwind support][wbg-catch-unwind].

```
wasm-pack build --panic-unwind
```

This flag:

- Invokes `cargo` with the **nightly** toolchain (`cargo +nightly build`).
- Adds `-Z build-std=std,panic_unwind` to rebuild `std` with unwinding
  support.
- Sets `RUSTFLAGS=-Cpanic=unwind` (preserving any user-provided `RUSTFLAGS`).

The first time you use `--panic-unwind`, `wasm-pack` will install any missing
prerequisites via `rustup`:

- The nightly toolchain
- The `rust-src` component for nightly
- The `wasm32-unknown-unknown` target for nightly

If you are not using `rustup` you must install these prerequisites manually.
See [Non-`rustup` setups][non-rustup].

> **Note:** `wasm-pack` only handles producing the `.wasm`. The actual
> "panic = recoverable JavaScript exception" behaviour requires runtime glue
> from your bindings layer (e.g. `wasm-bindgen`'s catch-unwind feature). With
> just `--panic-unwind` and no runtime glue, panics still terminate the
> instance — they are merely *unwound* rather than *aborted*.

`--panic-unwind` is also available for [`wasm-pack test`](./test.md).

## 64-bit WebAssembly (`wasm64-unknown-unknown`)

The cargo target triple is the source of truth for which WebAssembly ABI
`wasm-pack` builds. To produce a `memory64` binary, declare the target the
cargo-native way — either in `.cargo/config.toml`:

```toml
# .cargo/config.toml
[build]
target = "wasm64-unknown-unknown"
```

or as an extra cargo argument:

```
wasm-pack build -- --target wasm64-unknown-unknown
```

or via `CARGO_BUILD_TARGET=wasm64-unknown-unknown` in the environment.

`wasm64-unknown-unknown` is a [tier-3 Rust target][tier-3], so `rustup`
has no prebuilt artifacts for it. You need to provide two pieces yourself
via cargo's native config:

1. **A nightly toolchain** — `rust-toolchain.toml` is the cargo-native
   way to pin one to your project:

   ```toml
   # rust-toolchain.toml
   [toolchain]
   channel = "nightly"
   components = ["rust-src"]
   ```

   Or set `RUSTUP_TOOLCHAIN=nightly` for one-off invocations.

2. **`-Z build-std` to build `std` from source**, since there is no
   prebuilt one. Add to your `.cargo/config.toml`:

   ```toml
   [unstable]
   build-std = ["std", "panic_abort"]
   ```

   Or pass `-Z build-std=std,panic_abort` as an extra cargo argument.

`wasm-pack` itself stays out of the cargo invocation — it does not inject
`+nightly` or `-Z build-std` (those would override your toolchain pin or
surprise users who hadn't intended a nightly build). What it does do when
it sees a `wasm64-*` triple:

- Verifies the active toolchain is nightly, with a helpful error pointing
  at the config above if it isn't.
- Installs the `rust-src` component for the active toolchain via `rustup`
  if missing.
- Does **not** attempt `rustup target add wasm64-*` (which would always
  fail for a tier-3 target).
- Passes `--enable-memory64` to `wasm-opt` so the optimiser accepts
  64-bit memories and tables.

[tier-3]: https://doc.rust-lang.org/nightly/rustc/platform-support.html

[wbg-catch-unwind]: https://wasm-bindgen.github.io/wasm-bindgen/reference/catch-unwind.html
[non-rustup]: ../prerequisites/non-rustup-setups.md

<hr style="font-size: 1.5em; margin-top: 2.5em"/>

<sup id="footnote-0">0</sup> If you need to include additional assets in the pkg
directory and your NPM package, we intend to have a solution for your use case
soon. You can use `--no-gitignore` to omit the `.gitignore` file in the
meantime. [↩](#wasm-pack-build)
