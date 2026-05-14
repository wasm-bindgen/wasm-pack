<div align="center">

  <h1>📦✨ wasm-pack</h1>

  <p>
    <strong>Your favorite Rust → Wasm workflow tool!</strong>
  </p>

  <p>
    <a href="https://github.com/wasm-bindgen/wasm-pack/actions/workflows/test.yml"><img alt="Build Status" src="https://github.com/wasm-bindgen/wasm-pack/actions/workflows/test.yml/badge.svg?branch=master"/></a>
    <a href="https://crates.io/crates/wasm-pack"><img alt="crates.io" src="https://img.shields.io/crates/v/wasm-pack"/></a>
  </p>

  <h3>
    <a href="https://wasm-bindgen.github.io/wasm-pack/book">Docs</a>
    <span> | </span>
    <a href="https://github.com/wasm-bindgen/wasm-pack/blob/master/CONTRIBUTING.md">Contributing</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>


</div>

## About

This tool seeks to be a one-stop shop for building and working with rust-
generated WebAssembly that you would like to interop with JavaScript, in the
browser or with Node.js. `wasm-pack` helps you build rust-generated
WebAssembly packages that you could publish to the npm registry, or otherwise use
alongside any javascript packages in workflows that you already use, such as [webpack].

[webpack]: https://webpack.js.org/

![demo](demo.gif)

## 🔮 Prerequisites

This project requires Rust 1.30.0 or later.

- [Development Environment](https://wasm-bindgen.github.io/wasm-pack/book/prerequisites/index.html)
- [Installation](https://wasm-bindgen.github.io/wasm-pack/installer)

## ⚡ Quickstart Guide

Visit the [quickstart guide] in our documentation.

[quickstart guide]: https://wasm-bindgen.github.io/wasm-pack/book/quickstart.html

## 🎙️ Commands

- [`new`](https://wasm-bindgen.github.io/wasm-pack/book/commands/new.html): Generate a new RustWasm project using a template
- [`build`](https://wasm-bindgen.github.io/wasm-pack/book/commands/build.html): Generate an npm wasm pkg from a rustwasm crate
- [`test`](https://wasm-bindgen.github.io/wasm-pack/book/commands/test.html): Run browser tests
- [`pack` and `publish`](https://wasm-bindgen.github.io/wasm-pack/book/commands/pack-and-publish.html): Create a tarball of your rustwasm pkg and/or publish to a registry

## 📝 Logging

`wasm-pack` uses [`env_logger`] to produce logs when `wasm-pack` runs.

To configure your log level, use the `RUST_LOG` environment variable. For example:

```
RUST_LOG=info wasm-pack build
```

[`env_logger`]: https://crates.io/crates/env_logger

## 👯 Contributing

Read our [guide] on getting up and running for developing `wasm-pack`, and
check out our [contribution policy].

[guide]: https://wasm-bindgen.github.io/wasm-pack/book/contributing.html
[contribution policy]: CONTRIBUTING.md

## 🤹‍♀️ Governance

This project was started by [ashleygwilliams] and is maintained by [drager].

[ashleygwilliams]: https://github.com/ashleygwilliams
[drager]: https://github.com/drager
