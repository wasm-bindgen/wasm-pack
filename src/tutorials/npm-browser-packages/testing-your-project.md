# Testing your project

Now after writing and building code, let's actually execute it! You can execute
tests with:

```bash
$ wasm-pack test --firefox
[INFO]: Checking for the Wasm target...
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running target/wasm32-unknown-unknown/debug/deps/web-9e7d380f8600b08e.wasm
Interactive browsers tests are now available at http://127.0.0.1:8000

Note that interactive mode is enabled because `NO_HEADLESS`
is specified in the environment of this process. Once you're
done with testing you'll need to kill this server with
Ctrl-C.
```

The console won't finish just yet, but as indicated you can visit
http://127.0.0.1:8000 in your web browser to see the test output:

```
running 1 test

test web::pass ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

and we've now executed our first tests in a web browser!

If you'd like to execute tests in a headless web browser (you don't need to
manually visit a page) you can do:

```bash
$ wasm-pack test --headless --firefox
```

and similarly if you're developing a project for Node.js you can also execute
`wasm-pack test --nodejs` to run tests in Node.

Be sure to see the [testing reference documentation][testing-reference] for
other supported features as well!

[testing-reference]: https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html
