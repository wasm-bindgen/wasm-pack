# Contributing

## Prerequisites

The technical prerequisites for contributing to this project are the same as for
using it. You can find them documented [here][1].

You'll also want to check out the contributing [guidelines].

[1]: ./prerequisites/index.html
[guidelines]: https://github.com/wasm-bindgen/wasm-pack/blob/master/CONTRIBUTING.md

## 🏃‍♀️ Up and Running

1. fork and clone the `wasm-bindgen/wasm-pack` repository
2. install [node/npm]
3. `cd wasm-pack`
4. `cargo run`. To test command line arguments you can run `cargo run -- <args>`.

## Documentation

Documentation lives in the `/docs` directory. Each command has its own page.
Additionally there are extra pages explaining the prerequisites, setup, and how to
contribute (which you are reading now!).

## Tests

Tests live in the `/tests` directory. To run the tests you can run:

```
cargo test
```

You can also manually test the CLI tool by running:

```
cargo run -- <args>
```

...for example:

```
cargo run -- init /tests/fixtures/js-hello-world --scope=ag_dubs
```
