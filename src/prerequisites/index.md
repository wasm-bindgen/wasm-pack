# Prerequisites

First you'll want to [install the `wasm-pack` CLI][wasm-pack], and `wasm-pack
-V` should print the version that you just installed.

[wasm-pack]: https://wasm-bindgen.github.io/wasm-pack/installer/

Next, since `wasm-pack` is a build tool, you'll want to make sure you have
[Rust][rust] installed. Make sure `rustc -V` prints out at least 1.30.0.

[rust]: https://www.rust-lang.org/tools/install

Finally, if you're using `wasm-pack` to publish to NPM, you'll want
to [install and configure `npm`][npm]. In the future, we intend to rewrite the
npm registry client bits so that the need for a Node runtime is eliminated. If
you're excited about that work- you should reach out to the maintainers and get
involved!

[npm]: ./npm.html

Using a non-rustup setup? Learn how to configure it for wasm-pack [here](./non-rustup-setups.html).
