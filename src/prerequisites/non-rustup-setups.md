# Non-Rustup setups
`wasm-pack` compiles your code using the `wasm32-unknown-unknown` target. `wasm-pack` will automatically add this target for Rustup setups if you don't already have it installed by doing `rustup target add wasm32-unknown-unknown`. However, if you're not using Rustup, then we won't be able to do this automatically, and you'll have to do this yourself.

## Manually add wasm32-unknown-unknown
*Disclaimer: This is not guaranteed to work for every setup. These instructions below are specific for setups that match the exact rustc release, which means that the downloaded wasm32 target can be incompatible.*

To manually add the `wasm32-unknown-unknown` target you will need to download it from the rust-lang website and put the contents in the correct folder.

All the targets for all the different `rustc` versions are not presented in a human way on a website (yet) for you to just select the one you want and download it, one reason for this is that Rustup handles all of this for you and the packaging of targets was mainly built for tools. However, the following steps will walk through how to do this.

First, check what version of `rustc` you're using by running `rustc --version`. This should display something like: `rustc 1.33.0 (2aa4c46cf 2019-02-28)`. Then you need to download the correct wasm32 target for your rustc version. The rustc version is part of the url, which means for `rustc 1.33.0` the url will look like this: `https://static.rust-lang.org/dist/rust-std-1.33.0-wasm32-unknown-unknown.tar.gz`.

Here's some examples of urls for different rustc versions:
 * Nightly https://static.rust-lang.org/dist/rust-std-nightly-wasm32-unknown-unknown.tar.gz
 * Specific date nightly (2019-03-10) https://static.rust-lang.org/dist/2019-03-10/rust-std-nightly-wasm32-unknown-unknown.tar.gz
 * Beta https://static.rust-lang.org/dist/rust-std-beta-wasm32-unknown-unknown.tar.gz

You should be able to download this either by doing `wget https://static.rust-lang.org/dist/rust-std-1.33.0-wasm32-unknown-unknown.tar.gz` or by just visiting the url in a web browser.

After you have downloaded this tarball at a location of your choice, you should unpack it. This should result in a folder named `rust-std-1.33.0-wasm32-unknown-unknown` that contains some folders and files, but the interesting one is a folder called `rust-std-wasm32-unknown-unknown` which contains a `lib` and that should contain a `rustlib` folder and in that, a folder called `wasm32-unknown-unknown`. This is the folder we want to move.

Here's how the structure should look like for rustc 1.33.0:
```
rust-std-1.33.0-wasm32-unknown-unknown
├── components
├── install.sh
├── rust-installer-version
└── rust-std-wasm32-unknown-unknown
    ├── lib
    │   └── rustlib
    │       └── wasm32-unknown-unknown
```

To know where we should move this `wasm32-unknown-unknown` folder we need to run `rustc --print sysroot` which should print a path that looks something like this (this will vary on different operating systems): `/home/user/rust/rust-1.33.0-2019-02-28-2aa4c46cf`. That folder should contain a `lib` folder that contains a `rustlib` folder. We should move the `wasm32-unknown-unknown` to this folder.

On unix-like operating systems we can do that with the following command:
`mv rust-std-1.33.0-wasm32-unknown-unknown/rust-std-wasm32-unknown-unknown/lib/rustlib/wasm32-unknown-unknown /home/user/rust/rust-1.33.0-2019-02-28-2aa4c46cf/lib/rustlib/` and that should be it!
