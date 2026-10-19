wasm-pack builds Rust crates into npm packages. The commands in `src/command`
drive cargo, `src/bindgen.rs` and `src/install` fetch and run wasm-bindgen
and other tools (wasm-opt, browser drivers) per platform, and `src/manifest`
generates `package.json` from `Cargo.toml`. `npm/` is the npm installer that
downloads the wasm-pack binary.

- User-visible changes need an entry under `Unreleased` in `CHANGELOG.md`,
  in the existing format with the author and a PR link.
- New or changed command flags and `Cargo.toml` configuration need matching
  updates in `docs/src/commands` and `docs/src/cargo-toml-configuration.md`.
- Behaviour changes should be covered in `tests/all`, using the fixtures in
  `tests/all/utils/fixture.rs`.
- Tool downloads and installs are platform specific; check that changes keep
  working across the OS and architecture combinations handled in
  `src/install`.
