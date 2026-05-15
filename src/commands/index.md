# Commands

`wasm-pack` has several commands to help you during the process of building
a Rust-generated WebAssembly project.

- `new`: This command generates a new project for you using a template. [Learn more][new]
- `build`: This command builds a `pkg` directory for you with compiled wasm and generated JS. [Learn more][build]
- `pack` and `publish`: These commands will create a tarball, and optionally publish it to a registry, such as npm. [Learn more][pack-pub]

### Deprecated Commands

- `init`: This command has been deprecated in favor of `build`.

[new]: ./new.html
[build]: ./build.html
[pack-pub]: ./pack-and-publish.html

### Log levels

By default `wasm-pack` displays a lot of useful information.

You can cause it to display even *more* information by using `--verbose`, or you can silence *all* stdout by using `--quiet`.

You can also use `--log-level` to have fine-grained control over wasm-pack's log output:

* `--log-level info` is the default, it causes all messages to be logged.
* `--log-level warn` causes warnings and errors to be displayed, but not info.
* `--log-level error` causes only errors to be displayed.

These flags are global flags, so they can be used with every command, and they must come *before* the command:

```sh
wasm-pack --log-level error build
wasm-pack --quiet build
wasm-pack --verbose build
```
