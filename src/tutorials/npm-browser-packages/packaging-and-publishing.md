# Package Code for npm

We've made our code so now we need to package it all up. In your project directory run the following
command:

```bash
$ wasm-pack build --scope MYSCOPE
```

where `MYSCOPE` is your npm username. Normally you could just type `wasm-pack build` but since
other people are doing this tutorial as well we don't want conflicts with the `wasm-add` package
name! This command when run does a few things:

1. It'll compile your code to wasm if you haven't already
2. It'll generate a pkg folder with the wasm file, a JS wrapper file around the wasm, your README,
   and a `package.json` file.

This is everything you need to upload your code to npm! Let's do just that!

First off you'll need to login to npm with the account you made earlier if you didn't already have
one:

```bash
$ wasm-pack login
```

Next you'll need to go into the `pkg` directory and actually upload the package:

```bash
$ cd pkg
$ npm publish --access=public
```

Now normally if things are not scoped you can just do `npm publish` but if you give it a scope
you'll need to tell npm that this is actually public so it can publish it. We need to do that here
since we gave our packages a scope to avoid conflicting with each other! Next up is actually running
the code and verifying we got it from npm and how we can use that code.
