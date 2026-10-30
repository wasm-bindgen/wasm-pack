# Building your project

We've written our code so now we need to build it.

We are writing a crate that should be used in the browser, so we run this in
our terminal:

```bash
$ wasm-pack build
```

If you were writing a package that should be used in Node.js (with CommonJS
modules, e.g. `require`), you would run this in your terminal:

```bash
$ wasm-pack build --target nodejs
```

This command when run does a few things:

1. It'll compile your code to wasm if you haven't already
2. It'll generate a `pkg` folder with the wasm file, a JS wrapper file around
   the wasm, your README, and a `package.json` file.
