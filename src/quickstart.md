# Quickstart

1. Install `rust` using [`rustup`].
1. [Install this tool.]
1. Run `wasm-pack new hello-wasm`.
1. `cd hello-wasm`
1. Run `wasm-pack build --target web`.
1. This tool generates files in a `pkg` dir
1. Import it: `import init, { greet } from "./pkg/hello_wasm.js"`, initialize it: `await init()`, and then use it: `greet()`
1. To publish to npm, run `wasm-pack publish`. You may need to login to the
   registry you want to publish to. You can login using `wasm-pack login`.

[`rustup`]: https://rustup.rs/
[Install this tool.]: https://wasm-bindgen.github.io/wasm-pack/installer/
