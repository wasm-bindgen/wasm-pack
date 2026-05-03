# wasm-pack build

The `wasm-pack build` command creates the files necessary for JavaScript
interoperability and for publishing a package to npm. This involves compiling
your code to wasm and generating a pkg folder. This pkg folder will contain the
wasm binary, a JS wrapper file, your `README`, and a `package.json` file.<sup>[0](#footnote-0)</sup>

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

<hr style="font-size: 1.5em; margin-top: 2.5em"/>

<sup id="footnote-0">0</sup> In contrast to previous releases, a `.gitignore`
file is no longer created in the `pkg` directory. [↩](#wasm-pack-build)
