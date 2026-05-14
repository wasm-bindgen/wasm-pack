# Changelog

## 🤍 Unreleased

## 🌷 0.15.0

- ### ✨ Features

  - **Add wasm64-unknown-unknown target support - [audreyt], [pull/1553]**

    Parameterizes the target triple in `cargo_build_wasm()` and `cargo_build_wasm_tests()`, extracts `--target` from extra options, adds `--enable-memory64` to wasm-opt args for wasm64 targets, and dynamically constructs the `CARGO_TARGET_*_RUNNER` env var from the target triple.

    [pull/1553]: https://github.com/wasm-bindgen/wasm-pack/pull/1553
    [audreyt]: https://github.com/audreyt

  - **Add `--panic-unwind` flag to build and test commands - [guybedford], [pull/1572]**

    [pull/1572]: https://github.com/wasm-bindgen/wasm-pack/pull/1572

  - **Move `wasm-pack-template` into the repo - [guybedford], [pull/1573]**

    Vendors the project template directly into this repository so `wasm-pack new` no longer depends on a separate external template repo.

    [pull/1573]: https://github.com/wasm-bindgen/wasm-pack/pull/1573

- ### 🤕 Fixes

  - **Replace deprecated `binary-install` and add release auto-publish - [guybedford], [pull/1579]**

    The 0.14.0 npm package shipped with the old `drager/wasm-pack` release URL and was never republished after the repository moved, so `npm install -g wasm-pack` failed with a 404. This inlines the install/run logic into `npm/binary.js`, drops the deprecated `binary-install` dependency (which pulled in unmaintained `rimraf@3`, `glob@7`, `inflight`, and `tar@6`), promotes `tar` to a real dependency, and adds release workflow automation so the npm package no longer requires a manual `npm publish`.

    [pull/1579]: https://github.com/wasm-bindgen/wasm-pack/pull/1579

  - **Make `init.sh` POSIX-compatible and stop mangling `$VERSION` - [guybedford], [pull/1578]**

    [pull/1578]: https://github.com/wasm-bindgen/wasm-pack/pull/1578
    [guybedford]: https://github.com/guybedford

  - **Add trailing newline in `package.json` - [joaofreires], [pull/1162]**

    [pull/1162]: https://github.com/wasm-bindgen/wasm-pack/pull/1162
    [joaofreires]: https://github.com/joaofreires

  - **Fix `drager` → `wasm-bindgen` in URLs - [akesling], [pull/1571]**

    [pull/1571]: https://github.com/wasm-bindgen/wasm-pack/pull/1571
    [akesling]: https://github.com/akesling

- ### 🛠️ Maintenance

  - **Bump follow-redirects from 1.15.11 to 1.16.0 in /npm - [dependabot], [pull/1565]**

    [pull/1565]: https://github.com/wasm-bindgen/wasm-pack/pull/1565

  - **Bump rustls-webpki from 0.103.9 to 0.103.13 - [dependabot], [pull/1570]**

    [pull/1570]: https://github.com/wasm-bindgen/wasm-pack/pull/1570

  - **Bump brace-expansion from 1.1.12 to 1.1.14 in /npm - [dependabot], [pull/1574]**

    [pull/1574]: https://github.com/wasm-bindgen/wasm-pack/pull/1574

  - **Bump tar from 0.4.44 to 0.4.45 - [dependabot], [pull/1559]**

    [pull/1559]: https://github.com/wasm-bindgen/wasm-pack/pull/1559

  - **Bump time from 0.3.45 to 0.3.47 - [dependabot], [pull/1549]**

    [pull/1549]: https://github.com/wasm-bindgen/wasm-pack/pull/1549

  - **Bump minimatch from 3.1.2 to 3.1.5 in /npm - [dependabot], [pull/1555]**

    [pull/1555]: https://github.com/wasm-bindgen/wasm-pack/pull/1555

- ### 📖 Documentation

  - **Update documentation and references to the new GitHub repo - [qinyuhang], [pull/1567]**

    [pull/1567]: https://github.com/wasm-bindgen/wasm-pack/pull/1567

  - **Fix typo in prerequisites documentation - [pRizz], [pull/1545]**

    [pull/1545]: https://github.com/wasm-bindgen/wasm-pack/pull/1545
    [pRizz]: https://github.com/pRizz

## 🌻 0.14.0

- ### ✨ Features

  - **Support arbitrary wasm targets (WASI support) - [RReverser], [pull/1524]**

    Allows building for targets other than wasm32-unknown-unknown, enabling WASI and other custom wasm targets.

    [pull/1524]: https://github.com/drager/wasm-pack/pull/1524
    [RReverser]: https://github.com/RReverser

  - **macOS ARM (aarch64-apple-darwin) build support - [kaleidawave], [pull/1529]**

    Adds native Apple Silicon support in release builds and NPM package.

    [pull/1529]: https://github.com/drager/wasm-pack/pull/1529
    [kaleidawave]: https://github.com/kaleidawave

  - **Allow `--split-linked-modules` flag for wasm-bindgen - [codeart1st], [pull/1443]**

    [pull/1443]: https://github.com/drager/wasm-pack/pull/1443
    [codeart1st]: https://github.com/codeart1st

  - **Custom build profile support - [rafaelbeckel], [pull/1428]**

    Allows using custom cargo profiles via `--profile`.

    [pull/1428]: https://github.com/drager/wasm-pack/pull/1428
    [rafaelbeckel]: https://github.com/rafaelbeckel

- ### 🤕 Fixes

  - **Fix NPM package download URL - [qinyuhang], [pull/1543]**

    [pull/1543]: https://github.com/drager/wasm-pack/pull/1543
    [qinyuhang]: https://github.com/qinyuhang

  - **Filter build artifacts to only .wasm files - [drager], [pull/1535]**

    [pull/1535]: https://github.com/drager/wasm-pack/pull/1535

  - **Handle undefined VERSION in installer script - [BrianHung], [pull/1512]**

    [pull/1512]: https://github.com/drager/wasm-pack/pull/1512
    [BrianHung]: https://github.com/BrianHung

  - **Fix it_gets_wasm_bindgen_version test - [mshroyer], [pull/1509]**

    [pull/1509]: https://github.com/drager/wasm-pack/pull/1509
    [mshroyer]: https://github.com/mshroyer

- ### 🛠️ Maintenance

  - **Update dependencies to latest versions - [drager], [pull/1536]**

    [pull/1536]: https://github.com/drager/wasm-pack/pull/1536

  - **Security workflow permissions fixes - [drager]**

  - **Bump ring from 0.17.8 to 0.17.14 - [dependabot], [pull/1516]**

    [pull/1516]: https://github.com/drager/wasm-pack/pull/1516

  - **Bump brace-expansion from 1.1.11 to 1.1.12 in /npm - [dependabot], [pull/1515]**

    [pull/1515]: https://github.com/drager/wasm-pack/pull/1515

  - **Bump rustls from 0.23.16 to 0.23.18 - [dependabot], [pull/1451]**

    [pull/1451]: https://github.com/drager/wasm-pack/pull/1451
    [dependabot]: https://github.com/apps/dependabot

  - **Fix tar vulnerability (CVE-2026-23745) in npm package**

    Override tar dependency to ^7.5.3 to fix arbitrary file overwrite and symlink poisoning vulnerability ([GHSA-8qq5-rm4j-mr97]).

    [GHSA-8qq5-rm4j-mr97]: https://github.com/advisories/GHSA-8qq5-rm4j-mr97

  - **Fix axios vulnerabilities in npm package**

    Override axios dependency to ^0.30.0 to fix SSRF/credential leakage via absolute URL and XSRF-TOKEN leakage (CSRF) vulnerabilities.

- ### 📖 Documentation

  - **Update documentation links to drager's repo - [yutannihilation], [pull/1513]**

    [pull/1513]: https://github.com/drager/wasm-pack/pull/1513
    [yutannihilation]: https://github.com/yutannihilation

  - **Document prerequisites for webdriver tests - [mshroyer], [pull/1509]**

## ☀️ 0.13.1

- ### ✨ Features

  - **Requests using proxy settings from env - [jjyr], [pull/1438]**

    This enables ureq to use proxy settings from env, it solves lots of pain in network restricted environments.

    [pull/1438]: https://github.com/rustwasm/wasm-pack/pull/1438
    [jjyr]: https://github.com/jjyr

- ### 🤕 Fixes

  - **Update binary-install to v0.4.1 - [drager], [pull/1407]**

    Release v0.4.0 of binary-install introduced a regression that caused failures on some platforms. This release fixes that regression.

    [pull/1407]: https://github.com/rustwasm/wasm-pack/pull/1407
    [drager]: https://github.com/drager

  - ** Allow npm binary upgrades - [net], [pull/1439]**

    Fixes an issue where upgrading `wasm-pack` via NPM would not update the underlying binary.
    Previously, the binary was stored in the `binary-install` package's directory without versioning, causing version upgrades to silently fail as the old binary continued to be used.
    The binary is now stored in `node_modules/wasm-pack/binary/`, ensuring proper version updates when upgrading the package.

    **Before:** Upgrading from `0.12.1` to `0.13.0` would continue using the `0.12.1` binary
    **After:** Each `wasm-pack` version manages its own binary, enabling proper version upgrades

    [pull/1439]: https://github.com/rustwasm/wasm-pack/pull/1439
    [net]: https://github.com/net

- ### 🛠️ Maintenance

  - ** Remove unmaintained dependency atty in favor of stdlib - [mariusvniekerk], [pull/1436]**

    [pull/1436]: https://github.com/rustwasm/wasm-pack/pull/1436
    [mariusvniekerk]: https://github.com/mariusvniekerk

## ☀️ 0.13.0

- ### ✨ Features

  - **Add option to skip optimization with wasm-opt - [sisou], [pull/1321]**

    This feature introduces the `--no-opt` option to wasm-pack, providing a significant improvement in build efficiency for projects requiring multiple wasm-pack executions.

    [pull/1321]: https://github.com/rustwasm/wasm-pack/pull/1321
    [sisou]: https://github.com/sisou

  - **Add support geckodriver for linux-aarch64 - [EstebanBorai], [pull/1371]**

    Introduces support to download Geckodriver in Linux aarch64.

    [pull/1371]: https://github.com/rustwasm/wasm-pack/pull/1371
    [EstebanBorai]: https://github.com/EstebanBorai

  - **Add wasm-opt linux aarch64 condition - [dkristia], [issue/1392], [pull/1393]**

    A linux aarch64 build for wasm-opt exists in the newest binaryen versions.

    [issue/1392]: https://github.com/rustwasm/wasm-pack/issues/1392
    [pull/1393]: https://github.com/rustwasm/wasm-pack/pull/1393
    [dkristia]: https://github.com/dkristia

- ### 🤕 Fixes

  - **Fix passing relative paths to cargo - [dfaust], [issue/704], [issue/1156], [issue/1252], [pull/1331]**

    When building a crate located in a sub-directory, relative paths, passed as extra options to cargo (like `--target-dir`), are now handled correctly.

    [issue/704]: https://github.com/rustwasm/wasm-pack/issues/704
    [issue/1156]: https://github.com/rustwasm/wasm-pack/issues/1156
    [issue/1252]: https://github.com/rustwasm/wasm-pack/issues/1252
    [pull/1331]: https://github.com/rustwasm/wasm-pack/pull/1331
    [dfaust]: https://github.com/dfaust

  - **Rewrite wasm_target to use target-libdir - [daidoji], [issue/1342], [pull/1343]**

    Rewritten wasm_target to use target libdir from the rustc tool rather than looking through sysroot. This is to accomodate non-rustup installations.

    [issue/1342]: https://github.com/rustwasm/wasm-pack/issues/1342
    [pull/1343]: https://github.com/rustwasm/wasm-pack/pull/1343
    [daidoji]: https://github.com/daidoji

  - **Declare ES module in package.json - [gthb], [issue/1039], [pull/1061]**

    In bundler mode, generate package.json with "type": "module" and use the "main" attribute instead of the "module" attribute.

    This change makes the built ES module palatable to Node.js (when run with --experimental-vm-modules --experimental-wasm-modules),
    while it remains also palatable to webpack as illustrated in webpack/webpack#14313
    (where the pkg subfolder is generated with wasm-pack built with this change).
    This resolves the headache of using a wasm-pack-built package in a library that one needs to both run directly in Node and include in a webpack build.

    [issue/1039]: https://github.com/rustwasm/wasm-pack/issues/1039
    [pull/1061]: https://github.com/rustwasm/wasm-pack/pull/1061
    [gthb]: https://github.com/gthb

  - **Use new chromdriver endpoint and fix CI - [Myriad-Dreamin], [kade-robertson], [issue/1315], [issue/1390], [pull/1325], [pull/1391]**

    [issue/1315]: https://github.com/rustwasm/wasm-pack/issues/1315
    [issue/1390]: https://github.com/rustwasm/wasm-pack/issues/1390
    [pull/1325]: https://github.com/rustwasm/wasm-pack/pull/1325
    [pull/1391]: https://github.com/rustwasm/wasm-pack/pull/1391
    [Myriad-Dreamin]: https://github.com/Myriad-Dreamin
    [kade-robertson]: https://github.com/kade-robertson

  - **Add mingw support to npm package - [nathaniel-daniel], [issue/1354], [issue/1359], [pull/1363]**

    Fixes the NPM package's platform detection for mingw.

    [issue/1354]: https://github.com/rustwasm/wasm-pack/issues/1354
    [issue/1359]: https://github.com/rustwasm/wasm-pack/issues/1359
    [pull/1363]: https://github.com/rustwasm/wasm-pack/pull/1363
    [nathaniel-daniel]: https://github.com/nathaniel-daniel

  - **pkg-dir option for pack and publish commands - [danielronnkvist], [issue/1369], [pull/1370]**

    To be able to use these commands when the output directory option to the build command isn't the default pkg.

    [issue/1369]: https://github.com/rustwasm/wasm-pack/issues/1369
    [pull/1370]: https://github.com/rustwasm/wasm-pack/pull/1370
    [danielronnkvist]: https://github.com/danielronnkvist

  - **Optimize out-dir display - [ahaoboy], [issue/1395], [pull/1396]**

    Optimize out-dir display.

    from:

    `[INFO]: 📦 Your wasm pkg is ready to publish at /root/code/fib-wasm/fib-rs/../fib-wasm/wasm.`

    to:

    `[INFO]: 📦 Your wasm pkg is ready to publish at /root/code/fib-wasm/fib-wasm/wasm.`

    [issue/1395]: https://github.com/rustwasm/wasm-pack/issues/1395
    [pull/1396]: https://github.com/rustwasm/wasm-pack/pull/1396
    [ahaoboy]: https://github.com/ahaoboy

- ### 🛠️ Maintenance

  - **Fix error and warnings in install script - [lucashorward], [issue/1159], [issue/1217], [issue/1283], [pull/1320]**

    [issue/1159]: https://github.com/rustwasm/wasm-pack/issues/1159
    [issue/1217]: https://github.com/rustwasm/wasm-pack/issues/1217
    [issue/1283]: https://github.com/rustwasm/wasm-pack/issues/1283
    [pull/1320]: https://github.com/rustwasm/wasm-pack/pull/1320
    [lucashorward]: https://github.com/lucashorward

  - **Bump follow-redirects from 1.14.9 to 1.15.6 in /npm - [dependabot], [pull/1375]**

    [pull/1375]: https://github.com/rustwasm/wasm-pack/pull/1375

  - **Bump rustls-webpki from 0.100.1 to 0.100.2 - [dependabot], [pull/1323]**

    [pull/1341]: https://github.com/rustwasm/wasm-pack/pull/1341

  - **Bump rustix from 0.37.20 to 0.37.25 - [dependabot], [pull/1341]**

    [pull/1323]: https://github.com/rustwasm/wasm-pack/pull/1323
    [dependabot]: https://github.com/apps/dependabot

  - **Bump rustls from 0.21.9 to 0.21.11 - [dependabot], [pull/1385]**

    [pull/1385]: https://github.com/rustwasm/wasm-pack/pull/1385
    [dependabot]: https://github.com/apps/dependabot

  - **Bump tar from 6.1.11 to 6.2.1 in /npm - [dependabot], [pull/1379]**

    [pull/1379]: https://github.com/rustwasm/wasm-pack/pull/1379
    [dependabot]: https://github.com/apps/dependabot

- ### 📖 Documentation

  - **Fix typo in README - [Lionelf329], [pull/1368]**

    [pull/1268]: https://github.com/rustwasm/wasm-pack/pull/1368
    [Lionelf329]: https://github.com/Lionelf329

  - **Add a description of build --target deno - [puxiao], [pull/1344]**

    [pull/1344]: https://github.com/rustwasm/wasm-pack/pull/1344
    [puxiao]: https://github.com/puxiao

  - **Document deno in build target - [sigmaSd], [pull/1348]**

    [pull/1348]: https://github.com/rustwasm/wasm-pack/pull/1348
    [sigmaSd]: https://github.com/sigmaSd

  - **Fix local navigation backing one step too far in docs - [SamuSoft], [pull/1387]**

    [pull/1387]: https://github.com/rustwasm/wasm-pack/pull/1387
    [SamuSoft]: https://github.com/SamuSoft

  - **Add --target web to quick start build command - [josephrocca], [pull/1367]**

    [pull/1367]: https://github.com/rustwasm/wasm-pack/pull/1367
    [josephrocca]: https://github.com/josephrocca

## ☀️ 0.12.1

- ### 🤕 Fixes

  - **Restore --version command - [lynn], [issue/1301], [pull/1305]**

    The --version command got lost in space in v0.12.0. It's now brought back!

    [issue/1301]: https://github.com/rustwasm/wasm-pack/issues/1301
    [pull/1305]: https://github.com/rustwasm/wasm-pack/pull/1305
    [lynn]: https://github.com/lynn

  - **Fix value parser for Option<PathBuf> - [Myriad-Dreamin], [issue/1304], [pull/1307]**

    A value parser for OsString cannot parse a command line argument for Option<PathBuf>,
    which let it failed to specify paths for pack, publish and test commands, this faulty behavior
    was introduced in v0.12.0.

    [issue/1304]: https://github.com/rustwasm/wasm-pack/issues/1304
    [pull/1307]: https://github.com/rustwasm/wasm-pack/pull/1307
    [Myriad-Dreamin]: https://github.com/Myriad-Dreamin

## ☀️ 0.12.0

- ### ✨ Features

  - **Add --no-pack flag to build command - [hamza1311], [ashleygwilliams], [issue/691], [issue/811], [pull/695], [pull/1291]**

    When calling wasm-pack build a user can optionally pass --no-pack and wasm-pack will build your wasm, generate js, and not build a package.json.

    [issue/691]: https://github.com/rustwasm/wasm-pack/issues/691
    [issue/811]: https://github.com/rustwasm/wasm-pack/issues/811
    [pull/695]: https://github.com/rustwasm/wasm-pack/pull/695
    [pull/1291]: https://github.com/rustwasm/wasm-pack/pull/1291
    [ashleygwilliams]: https://github.com/ashleygwilliams

  - **Add wasmbindgen option: omit_default_module_path - [matthiasgeihs], [pull/1272]**

    Adds an option to call wasm-bindgen with --omit_default_module_path.

    [pull/1272]: https://github.com/rustwasm/wasm-pack/pull/1272
    [matthiasgeihs]: https://github.com/matthiasgeihs

- ### 🤕 Fixes

  - **Add HTTP header USER-AGENT - [LeviticusNelson], [issue/1266], [pull/1285]**

    We encountered some issues when we didn't send an User-Agent. This is now fixed.

    [issue/1266]: https://github.com/rustwasm/wasm-pack/issues/1266
    [pull/1285]: https://github.com/rustwasm/wasm-pack/pull/1285
    [LeviticusNelson]: https://github.com/LeviticusNelson

  - **Replace curl with ureq - [hamza1311], [issue/650], [issue/823], [issue/997], [issue/1079], [issue/1203], [issue/1234], [issue/1281], [pull/1290]**

    The HTTP client is now pure Rust. Removes the dependency of openssl which have caused a lot of issues for people using wasm-pack on various distributions.

    [issue/650]: https://github.com/rustwasm/wasm-pack/issues/650
    [issue/823]: https://github.com/rustwasm/wasm-pack/issues/823
    [issue/997]: https://github.com/rustwasm/wasm-pack/issues/997
    [issue/1079]: https://github.com/rustwasm/wasm-pack/issues/1079
    [issue/1203]: https://github.com/rustwasm/wasm-pack/issues/1203
    [issue/1234]: https://github.com/rustwasm/wasm-pack/issues/1234
    [issue/1281]: https://github.com/rustwasm/wasm-pack/issues/1281
    [pull/1290]: https://github.com/rustwasm/wasm-pack/pull/1290
    [hamza1311]: https://github.com/hamza1311

  - **Update binary-install to 0.2.0. binary-install replaced curl with ureq - [drager]**

    See [PR](https://github.com/rustwasm/binary-install/pull/24) in binary-install repo for more information.

    [drager]: https://github.com/drager

  - **Remove --always-auth from npm login - [EstebanBorai], [pull/1288]**

    npm login doesn't support --always-auth anymore, instead it is under the adduser subcommand.

    [pull/1288]: https://github.com/rustwasm/wasm-pack/pull/1288
    [EstebanBorai]: https://github.com/EstebanBorai

  - **Turn off cargo colors during log level test - [dtolnay], [pull/1294]**

    [pull/1294]: https://github.com/rustwasm/wasm-pack/pull/1294
    [dtolnay]: https://github.com/dtolnay

  - **Fix getting the target-dir in wasm_bindgen_build - [tomasol], [issue/1278], [pull/1279]**

    Fixes a wasm-pack panic if --target-dir was supplied (and arguments are not sorted).

    [issue/1278]: https://github.com/rustwasm/wasm-pack/issues/1278
    [pull/1279]: https://github.com/rustwasm/wasm-pack/pull/1279
    [tomasol]: https://github.com/tomasol

  - **Respect package.readme in Cargo.toml - [heaths], [issue/1215], [pull/1298], [pull/1216]**

    wasm-pack now respects specifying readme=false:

    ```toml
    [package]
    readme = false
    ```

    [issue/1215]: https://github.com/rustwasm/wasm-pack/issues/1215
    [pull/1298]: https://github.com/rustwasm/wasm-pack/pull/1298
    [pull/1216]: https://github.com/rustwasm/wasm-pack/pull/1216
    [heaths]: https://github.com/heaths

- ### 📖 Documentation

  - **Don't hide install options behind link - [oyamauchi], [issue/355], [pull/1242]**

    [issue/355]: https://github.com/rustwasm/wasm-pack/issues/355
    [pull/1242]: https://github.com/rustwasm/wasm-pack/issues/1242
    [oyamauchi]: https://github.com/oyamauchi

- ### 🛠️ Maintenance

  - **Bump cargo-generate version to 0.18.2 - [sassman], [issue/1245] [pull/1269]**

    [issue/1245]: https://github.com/rustwasm/wasm-pack/issues/1245
    [pull/1269]: https://github.com/rustwasm/wasm-pack/pull/1269
    [sassman]: https://github.com/sassman

  - **Replace unmaintained actions-rs/toolchain action in CI workflows - [striezel], [pull/1246]**

    Now we are using https://github.com/dtolnay/rust-toolchain instead.

    [pull/1246]: https://github.com/rustwasm/wasm-pack/pull/1246
    [striezel]: https://github.com/striezel

  - **Update several dependencies - [hamza1311], [pull/1292]**

    Updated clap, toml, predicates and serial_test to their latest versions.

    [pull/1292]: https://github.com/rustwasm/wasm-pack/pull/1292

## 🌦️ 0.11.1

- ### 🤕 Fixes

  - **Fix discovery of locally installed `wasm-opt` - [Liamolucko], [issue/1247], [pull/1257]**

    [issue/1247]: https://github.com/rustwasm/wasm-pack/issues/1247
    [pull/1257]: https://github.com/rustwasm/wasm-pack/pull/1257
    [Liamolucko]: https://github.com/Liamolucko

  - **Fix wasm-pack bin script entry - [ahippler], [issue/1248], [pull/1250]**

    [issue/1248]: https://github.com/rustwasm/wasm-pack/issues/1248
    [pull/1250]: https://github.com/rustwasm/wasm-pack/pull/1250
    [ahippler]: https://github.com/ahippler

- ### 🛠️ Maintenance

  - **bump openssl from 0.10.46 to 0.10.48 - [pull/1254]**

    [pull/1254]: https://github.com/rustwasm/wasm-pack/pull/1254

## 🌦️ 0.11.0

- ### ✨ Features

  - **Make Deno target available - [egfx-notifications], [issue/672], [issue/879], [pull/1117]**

    [issue/672]: https://github.com/rustwasm/wasm-pack/issues/672
    [issue/879]: https://github.com/rustwasm/wasm-pack/issues/879
    [pull/1117]: https://github.com/rustwasm/wasm-pack/pull/1117
    [egfx-notifications]: https://github.com/egfx-notifications

  - **Add support for more platforms to installer script - [omninonsense], [issue/1064], [issue/952], [issue/1125], [pull/1122]**

    This makes the installation script work on M1 macs, as well as inside docker (especially when combined with buildx) for aarch64/arm64 architectures.

    [issue/1064]: https://github.com/rustwasm/wasm-pack/issues/1064
    [issue/952]: https://github.com/rustwasm/wasm-pack/issues/952
    [issue/1125]: https://github.com/rustwasm/wasm-pack/issues/1125
    [pull/1122]: https://github.com/rustwasm/wasm-pack/pull/1122
    [omninonsense]: https://github.com/omninonsense

  - **Add Linux arm64 support - [nnelgxorz], [issue/1169], [pull/1170]**

    [issue/1169]: https://github.com/rustwasm/wasm-pack/issues/1169
    [pull/1170]: https://github.com/rustwasm/wasm-pack/pull/1170
    [nnelgxorz]: https://github.com/nnelgxorz

  - **Add support for workspace inheritance - [printfn], [issue/1180], [pull/1185]**

    [issue/1180]: https://github.com/rustwasm/wasm-pack/issues/1180
    [pull/1185]: https://github.com/rustwasm/wasm-pack/pull/1185

- ### 🤕 Fixes

  - **--target-dir as extra option is now considered as expected - [sassman], [issue/1076], [pull/1082]**

    [issue/1076]: https://github.com/rustwasm/wasm-pack/issues/1076
    [pull/1082]: https://github.com/rustwasm/wasm-pack/pull/1082
    [sassman]: https://github.com/sassman

  - **Pass through --weak-refs --reference-types flags to bindgen - [serprex], [issue/930], [pull/937]**

    [issue/930]: https://github.com/rustwasm/wasm-pack/issues/930
    [pull/937]: https://github.com/rustwasm/wasm-pack/pull/937
    [serprex]: https://github.com/serprex

  - **Fix binaryen URL and use updated binary-install to fix installation on macOS - [matheus23], [printfn], [pull/1188]**

    Use the updated binary-install crate (rustwasm/binary-install#21), switches from failure to anyhow to match what binary-install uses, and fixes wasm-opt installation on macOS.

    [pull/1188]: https://github.com/rustwasm/wasm-pack/pull/1188
    [matheus23]: https://github.com/matheus23
    [printfn]: https://github.com/printfn
    [rustwasm/binary-install#21]: https://github.com/rustwasm/binary-install/pull/21

  - **Mark snippets and the bundler target's main file as having side effects - [Liamolucko], [issue/972], [rustwasm/wasm-bindgen/3276], [pull/1224]**

    [issue/972]: https://github.com/rustwasm/wasm-pack/issues/972
    [rustwasm/wasm-bindgen/3276]: https://github.com/rustwasm/wasm-bindgen/issues/3276
    [pull/1224]: https://github.com/rustwasm/wasm-pack/pull/1224
    [Liamolucko]: https://github.com/Liamolucko

- ### 📖 Documentation

  - **Fix typos in non-rustup-setups.md - [dallasbrittany], [issue/1141], [pull/1142]**

    [issue/1141]: https://github.com/rustwasm/wasm-pack/issues/1141
    [pull/1142]: https://github.com/rustwasm/wasm-pack/issues/1141
    [dallasbrittany]: https://github.com/dallasbrittany

  - **Fix typos in considerations.md - [lhjt], [pull/1066]**

    [pull/1066]: https://github.com/rustwasm/wasm-pack/pull/1066
    [lhjt]: https://github.com/lhjt

  - **Grammar and typo fixes - [helixbass], [pull/1143]**

    [pull/1143]: https://github.com/rustwasm/wasm-pack/pull/1143
    [helixbass]: https://github.com/helixbass

  - **Replace two mentions of wasm-pack init with wasm-pack build in the docs - [mstange], [pull/1086]**

    [pull/1086]: https://github.com/rustwasm/wasm-pack/pull/1086
    [mstange]: https://github.com/mstange

  - **Update npm installation link - [benediktwerner], [pull/1227]**

    [pull/1227]: https://github.com/rustwasm/wasm-pack/pull/1227
    [benediktwerner]: https://github.com/benediktwerner

- ### 🛠️ Maintenance

  - **Bump wasm-opt to version 108 - [MichaelMauderer], [issue/1135] [pull/1136]**

    [pull/1136]: https://github.com/rustwasm/wasm-pack/pull/1136
    [issue/1135]: https://github.com/rustwasm/wasm-pack/issues/1135
    [MichaelMauderer]: https://github.com/MichaelMauderer

  - **Update binary-install to v1.0.1 - [EverlastingBugstopper], [pull/1130]**

    [pull/1130]: https://github.com/rustwasm/wasm-pack/pull/1130

  - **Add back run.js to npm installer - [EverlastingBugstopper], [pull/1149]**

    [pull/1149]: https://github.com/rustwasm/wasm-pack/pull/1149

  - **Fix some typos in the codebase - [striezel], [pull/1220]**

    [pull/1220]: https://github.com/rustwasm/wasm-pack/pull/1220
    [striezel]: https://github.com/striezel

  - **Update actions/checkout in GitHub Actions workflows to v3 - [striezel], [pull/1221]**

    [pull/1221]: https://github.com/rustwasm/wasm-pack/pull/1221

  - **Update actions/cache in GitHub Actions workflows to v3 - [striezel], [pull/1222]**

    [pull/1222]: https://github.com/rustwasm/wasm-pack/pull/1222

  - **Update JamesIves/github-pages-deploy-action in GHA workflow to v4.4.1 - [striezel], [pull/1223]**

    [pull/1223]: https://github.com/rustwasm/wasm-pack/pull/1223

## 🌦️ 0.10.3

- ### 🤕 Fixes

  - **Use bash to create release tarballs - [nasso], [issue/1097] [pull/1144]**

    Fixes Windows installer failure due to malformatted tar.

    [pull/1144]: https://github.com/rustwasm/wasm-pack/pull/1144
    [issue/1097]: https://github.com/rustwasm/wasm-pack/issues/1097
    [nasso]: https://github.com/nasso

  - **Clean up package.json from previous runs - [main--], [issue/1110-comment] [pull/1119]**

    Remove the package.json file from previous runs to avoid crashes.

    [pull/1119]: https://github.com/rustwasm/wasm-pack/pull/1119
    [issue/1110-comment]: https://github.com/rustwasm/wasm-pack/pull/1110#issuecomment-1059008962
    [main--]: https://github.com/main--

  - **Do not remove the pkg directory - [huntc], [issue/1099] [pull/1110]**

    A recent change ensured that the pkg directory was removed as the first step of attempting to create it.
    Unfortunately, this caused a problem for webpack when watching the pkg directory.
    Webpack was unable to recover its watching and so any watch server must be restarted,
    which is a blocker when using it. This PR and release fixes this.

    [pull/1110]: https://github.com/rustwasm/wasm-pack/pull/1110
    [issue/1099]: https://github.com/rustwasm/wasm-pack/issues/1099
    [huntc]: https://github.com/huntc

  - **Bump regex from 1.5.4 to 1.5.6 - [dependabot], [pull/1147]**

    Version 1.5.5 of the regex crate fixed a security bug in the regex compiler.

    [pull/1147]: https://github.com/rustwasm/wasm-pack/pull/1147

  - **Bump openssl-src from 111.17.0+1.1.1m to 111.20.0+1.1.1o - [dependabot], [pull/1146]**

    Bring in bug fixes from the new version of openssl-src.

    [pull/1146]: https://github.com/rustwasm/wasm-pack/pull/1146
    [dependabot]: https://github.com/apps/dependabot

## 🌦️ 0.10.2

- ### ✨ Features

  - **Implement support for RFC 8, transitive NPM dependencies - [jpgneves], [issue/606] [pull/986]**

    [pull/986]: https://github.com/rustwasm/wasm-pack/pull/986
    [issue/606]: https://github.com/rustwasm/wasm-pack/issues/606
    [jpgneves]: https://github.com/jpgneves

- ### 🤕 Fixes

  - **Add support for macos aarch64 - [d3lm], [issue/913] [pull/1088]**

    This fixes aarch64 for MacOS and will download x86_64-apple-darwin.

    [pull/1088]: https://github.com/rustwasm/wasm-pack/pull/1088
    [issue/913]: https://github.com/rustwasm/wasm-pack/issues/913
    [d3lm]: https://github.com/d3lm

  - **Add linux/arm64 to release workflow - [nacardin], [issue/1064] [pull/1065]**

    [pull/1065]: https://github.com/rustwasm/wasm-pack/pull/1065
    [issue/1064]: https://github.com/rustwasm/wasm-pack/issues/1064
    [nacardin]: https://github.com/nacardin

  - **Force axios version - [drager], [pull/1094]**

    Forces npm package `axios` to version `0.21.2` in order to get security fix for a security vulnerability present in axios
    before version `0.21.2`.

    [pull/1094]: https://github.com/rustwasm/wasm-pack/pull/1094

- ### 📖 Documentation

  - **Update docs for how to pass extra options to cargo - [FrankenApps], [issue/1059] [pull/1073]**

    [FrankenApps]: https://github.com/FrankenApps
    [pull/1073]: https://github.com/rustwasm/wasm-pack/pull/1073
    [issue/1059]: https://github.com/rustwasm/wasm-pack/issues/1059

## 🌦️ 0.10.1

- ### 🤕 Fixes

  - **Add exe to binary name if windows - [drager], [issue/1038] [pull/1055]**

    [pull/1055]: https://github.com/rustwasm/wasm-pack/pull/1055
    [issue/1038]: https://github.com/rustwasm/wasm-pack/issues/1038

## 🌦️ 0.10.0

- ### ✨ Features

  - **Added keywords - [lucashorward], [issue/707] [pull/838]**

    `package.json` files usually contain a keywords array so that npm can make searching easier.
    This PR extracts keywords from `Cargo.toml` and puts them into `package.json`.

    [lucashorward]: https://github.com/lucashorward
    [pull/838]: https://github.com/rustwasm/wasm-pack/pull/838
    [issue/707]: https://github.com/rustwasm/wasm-pack/issues/707

- ### 🤕 Fixes

  - **Update binary-install to get fix for axios security vulnerability - [simlay], [Rizary], [issue/958] [pull/973] [pull/1012]**

    Updates `binary-install` npm package to version `^0.1.0` in order to get security fix for a security vulnerability in axios.

    [simlay]: https://github.com/simlay
    [rizary]: https://github.com/Rizary
    [pull/973]: https://github.com/rustwasm/wasm-pack/pull/973
    [pull/1012]: https://github.com/rustwasm/wasm-pack/pull/1012
    [issue/958]: https://github.com/rustwasm/wasm-pack/issues/958

  - **Fix cargo-generate installation - [bradyjoslin], [issue/975] [issue/907] [pull/983]**

    `wasm-pack new hello-wasm` didn't work due to a bad link when trying to install `cargo-generate`.

    This PR points the installation to the correct place and makes `wasm-pack new` working again!

    [bradyjoslin]: https://github.com/bradyjoslin
    [pull/983]: https://github.com/rustwasm/wasm-pack/pull/983
    [issue/975]: https://github.com/rustwasm/wasm-pack/issues/975
    [issue/907]: https://github.com/rustwasm/wasm-pack/issues/907

  - **Pass through extra options when building tests - [azriel91], [issue/698] [pull/851]**

    `wasm-pack test` accepts extra options to pass through to `cargo` when running tests.
    Under the hood, this runs `cargo build` before `cargo test`, and the additional options were only passed through to the `test` command. This meant that crates that enabled native features by default could not be built using `wasm-pack`, as it would attempt to build tests for the `wasm32-unknown-unknown` target with the native features enabled.

    This PR passes through the extra options to `cargo` when building the tests as well.

    [azriel91]: https://github.com/azriel91
    [pull/851]: https://github.com/rustwasm/wasm-pack/pull/851
    [issue/698]: https://github.com/rustwasm/wasm-pack/issues/698

  - **Corrected files included in package.json for bundler / no target - [lucashorward], [issue/837] [pull/839]**

    `wasm-pack build` and `wasm-pack build --target bundler` generates a \_bg.js file, but it was not added to the `package.json`.
    The file that is added, \*.js will however reference the \_bg.js, so when the package was distributed (both through pack or publish) it is not usable.

    This PR includes that \_bg.js file in `package.json`.

    [pull/839]: https://github.com/rustwasm/wasm-pack/pull/839
    [issue/837]: https://github.com/rustwasm/wasm-pack/issues/837

  - **Find the main package if multiple packages have the same name - [ghost], [pull/830]**

    If there were 2 packages with the same name, `wasm-pack` would sometimes use the wrong one and errored.

    [ghost]: https://github.com/ghost
    [pull/830]: https://github.com/rustwasm/wasm-pack/pull/830
    [issue/829]: https://github.com/rustwasm/wasm-pack/issues/829

- ### 📖 Documentation

  - **Remove duplicated "is" in the wee_alloc tutorial- [pione30], [issue/1003] [pull/1004]**

    [pione30]: https://github.com/pione30
    [pull/1004]: https://github.com/rustwasm/wasm-pack/pull/1004
    [issue/1003]: https://github.com/rustwasm/wasm-pack/issues/1003

  - **Fix TOC links - [Swaagie], [pull/1007]**

    [swaagie]: https://github.com/Swaagie
    [pull/1007]: https://github.com/rustwasm/wasm-pack/pull/1007

  - **Remove outdated TOC heading- [gthb], [pull/1011]**

    [gthb]: https://github.com/gthb
    [pull/1011]: https://github.com/rustwasm/wasm-pack/pull/1011

  - **Add link to template repo - [milahu], [pull/942]**

    [milahu]: https://github.com/milahu
    [pull/942]: https://github.com/rustwasm/wasm-pack/pull/942

  - **Remove greenkeeper reference - [cdvv7788], [crotwell], [issue/1001] [pull/844] [pull/1002]**

    [cdvv7788]: https://github.com/cdvv7788
    [crotwell]: https://github.com/crotwell
    [pull/844]: https://github.com/rustwasm/wasm-pack/pull/844
    [pull/1002]: https://github.com/rustwasm/wasm-pack/pull/1002
    [issue/1001]: https://github.com/rustwasm/wasm-pack/issues/1001

- ### 🛠️ Maintenance

  - **Fix CI. Remove appveyor and travis and use Github actions - [ashleygwilliams], [drager], [issue/594] [issue/979] [pull/947]**

    [pull/947]: https://github.com/rustwasm/wasm-pack/pull/947
    [issue/594]: https://github.com/rustwasm/wasm-pack/issues/594
    [issue/979]: https://github.com/rustwasm/wasm-pack/issues/979

  - **Cargo update - [ashleygwilliams], [pull/800]**

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [pull/800]: https://github.com/rustwasm/wasm-pack/pull/800

  - **Remove dirs dependency - [brightly-salty], [issue/943] [pull/944]**

    [brightly-salty]: https://github.com/brightly-salty
    [pull/944]: https://github.com/rustwasm/wasm-pack/pull/944
    [issue/943]: https://github.com/rustwasm/wasm-pack/issues/943

  - **Fix logs for uniformity - [petosorus], [issue/716] [pull/723]**

    [petosorus]: https://github.com/petosorus
    [pull/723]: https://github.com/rustwasm/wasm-pack/pull/723
    [issue/716]: https://github.com/rustwasm/wasm-pack/issues/716

  - **Fixing build error - [Pauan], [pull/841]**

    [pull/841]: https://github.com/rustwasm/wasm-pack/pull/841

## ☁️ 0.9.1

- ### 🤕 Fixes

  - **Bump binaryen to version_90 - [ashleygwilliams], [issue/781] [issue/782] [pull/687]**

    Previously, wasm-pack was hardcoded to install and attempt to execute wasm-opt on every build
    using binaryen version 78. This version had various issues on Unix/Linux and caused broken CI
    builds for many folks (we're so sorry!).

    This PR updates the binaryen version to 90, which should fix the issues folks were having.

    Long-term, we'd like to create an auto-updating mechanism so that we can install and use the
    latest release of binaryen as we do for other binaries we orchestrate.

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [pull/687]: https://github.com/rustwasm/wasm-pack/pull/687
    [issue/782]: https://github.com/rustwasm/wasm-pack/issues/782
    [issue/781]: https://github.com/rustwasm/wasm-pack/issues/781

- ### 🛠️ Maintenance

  - **Consolidate wasm-opt installation into existing binary install logic - [ashleygwilliams], [issue/685] [pull/687]**

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [pull/687]: https://github.com/rustwasm/wasm-pack/pull/687
    [issue/685]: https://github.com/rustwasm/wasm-pack/issues/685

## 🌥️ 0.9.0

- ### ✨ Features

  - **Adding in `--quiet` and `--log-level` flags to control the console output - [Pauan], [pull/694]**

    The `--verbose` flag has long existed as a way to get more console output, but now there are two flags to get _less_ console output:

    - `--quiet` will silence _all_ stdout, so only errors will be displayed.
    - `--log-level` can be used to silence `[INFO]` or `[WARN]` output from wasm-pack.

    You can cause it to display even _more_ information by using `--verbose`, or you can silence _all_ stdout by using `--quiet`.

    You can also use `--log-level` to have fine-grained control over wasm-pack's log output:

    - `--log-level info` is the default, it causes all messages to be logged.
    - `--log-level warn` causes warnings and errors to be displayed, but not info.
    - `--log-level error` causes only errors to be displayed.

    These flags are global flags, so they can be used with every command, and they must come _before_ the command:

    ```sh
    wasm-pack --log-level error build
    wasm-pack --quiet build
    ```

    [Pauan]: https://github.com/Pauan
    [pull/694]: https://github.com/rustwasm/wasm-pack/pull/694

  - **Wrap `cargo-generate` with `wasm-pack new` - [ashleygwilliams], [issue/373] [pull/623]**

    One of the first steps in getting started with `wasm-pack` is to `cargo install cargo-generate` to bootstrap some project templates. This can take a while and is an extra burden on users just getting started with `wasm-pack`. `wasm-pack new` uses `cargo-generate` to bootstrap new projects, removing the need to install the tool on your own. You can read more about this feature [here](https://github.com/rustwasm/wasm-pack/blob/master/docs/src/commands/new.md).

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [pull/623]: https://github.com/rustwasm/wasm-pack/pull/623
    [issue/373]: https://github.com/rustwasm/wasm-pack/issues/373

  - **Allow `wasm-pack` to be run from subdirectories - [gameldar], [issue/620] [pull/624]**

    If a crate path is not specified when running `wasm-pack` and there is no `Cargo.toml` in the current working directory, `wasm-pack` will walk up the directory structure to find a `Cargo.toml`.

    [gameldar]: https://github.com/gameldar
    [pull/624]: https://github.com/rustwasm/wasm-pack/pull/624
    [issue/620]: https://github.com/rustwasm/wasm-pack/issues/620

  - **Automatically execute `wasm-opt` on produced binaries - [alexcrichton], [issue/159] [pull/625]**

    When `wasm-pack` builds binaries in released and profiling modes, it will execute `wasm-opt` on the binary, making the result smaller and more performant.

    [alexcrichton]: https://github.com/alexcrichton
    [pull/625]: https://github.com/rustwasm/wasm-pack/pull/625
    [issue/159]: https://github.com/rustwasm/wasm-pack/issues/159

  - **Helpful error message when wasm-bindgen fails because of an old version - [gameldar], [ashleygwilliams], [issue/627] [pull/633]**

    `wasm-pack` will pass a `--web` flag to `wasm-bindgen` when `wasm-pack build --target web` is run. Before, if the user had an old version of `wasm-bindgen` in their dependencies, they would receive a cryptic error message. Now they will be notified that they need to update their `wasm-bindgen` dependency if they want to build for the `web` target.

    [gameldar]: https://github.com/gameldar
    [pull/633]: https://github.com/rustwasm/wasm-pack/pull/633
    [issue/627]: https://github.com/rustwasm/wasm-pack/issues/627

  - **Publish releases by tag to npm - [Tarnadas], [pull/690]**

    You can now use `wasm-pack publish` to publish tagged releases with the optional `--tag` argument. You can read more about [distribution tags](https://docs.npmjs.com/cli/dist-tag) on NPM, and more about this feature in [our docs](https://github.com/Tarnadas/wasm-pack/blob/master/docs/src/commands/pack-and-publish.md#publishing-tagged-releases).

    [Tarnadas]: https://github.com/Tarnadas
    [pull/690]: https://github.com/rustwasm/wasm-pack/pull/690

- ### 🤕 Fixes

  - **Only use exactly v0.24.0 geckodriver on Windows - [ashleygwilliams], [issue/770] [pull/774]**

    `wasm-pack test` is a great way to test your web Wasm modules- and it very nicely sets up and configures
    the necessary browser engine drivers to do so!

    For the v0.25.0 release of geckodriver, the team switched their build environment- which introduced a new
    surprise runtime dependency, Visual C++ redistributable package, to their windows binaries. You can read
    more about the issue here, [mozilla/geckodriver/issue/1617].

    Becuase the introduction of this runtime dependency is considered a bug, and should be eventually fixed,
    the team decided that the least invasive solution would be to hold geckodriver binaries, on Windows, at
    v0.24.0, and to disable the auto-update logic, until the bug is fixed.

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [issue/770]: https://github.com/rustwasm/wasm-pack/issues/770
    [pull/774]: https://github.com/rustwasm/wasm-pack/pull/774
    [mozilla/geckodriver/issue/1617]: https://github.com/mozilla/geckodriver/issues/1617#issuecomment-532168958

  - **Handle version check failures - [drager], [issue/652], [issue/653] [pull/660]**

    Every day, `wasm-pack` checks the crates.io API for the latest version number and lets the user know if their installation is out of date. Now, when these API calls fail, `wasm-pack` alerts the user of the failure and waits until the next day to make another call to crates.io.

    [drager]: https://github.com/drager
    [pull/660]: https://github.com/rustwasm/wasm-pack/pull/660
    [issue/652]: https://github.com/rustwasm/wasm-pack/issues/652
    [issue/653]: https://github.com/rustwasm/wasm-pack/issues/653

  - **Add user agent for version check - [drager], [issue/651] [pull/658]**

    crates.io requires tools to set a version check `User-Agent` header when requesting the latest version. Now, when `wasm-pack` performs an API request to crates.io, it sends `User-Agent: wasm-pack/0.9.0`.

    [drager]: https://github.com/drager
    [pull/658]: https://github.com/rustwasm/wasm-pack/pull/658
    [issue/651]: https://github.com/rustwasm/wasm-pack/issues/651

  - **Remove broken link from the README - [drager], [pull/635]**

    [drager]: https://github.com/drager
    [pull/635]: https://github.com/rustwasm/wasm-pack/pull/635

  - **Make `sideEffects` in generated `package.json` a boolean instead of a string - [rhysd], [pull/649]**

    [rhysd]: https://github.com/rhysd
    [pull/649]: https://github.com/rustwasm/wasm-pack/pull/649

  - **Don't warn if license-file is present - [ashleygwilliams], [issue/692] [pull/693]**

    Previously, `wasm-pack` would warn that the `license` field was missing if the `license-file` field was used instead. This warning is now only surfaced if both `license` and `license-field` are absent from a `Cargo.toml`.

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [pull/693]: https://github.com/rustwasm/wasm-pack/pull/693
    [issue/692]: https://github.com/rustwasm/wasm-pack/issues/692

  - **Select correct webdriver version - [MartinKavik], [issue/611] [pull/706]**

    `wasm-pack` used to install a pinned version of the Chrome, Gecko, and Safari drivers. Now when a driver needs to be installed, `wasm-pack` will pull the latest version from the API and install that instead.

    [MartinKavik]: https://github.com/MartinKavik
    [pull/706]: https://github.com/rustwasm/wasm-pack/pull/706
    [issue/611]: https://github.com/rustwasm/wasm-pack/issues/611

  - **Only run node tests on `wasm-pack test --node` - [alexcrichton], [pull/630]**

    [alexcrichton]: https://github.com/alexcrichton
    [pull/630]: https://github.com/rustwasm/wasm-pack/pull/630

  - **Fix npm installs for Windows Users - [EverlastingBugstopper], [issue/757] [pull/759]**

    We recently published `wasm-pack` on the npm registry but forgot to test on Windows! `npm install -g wasm-pack` now works on Windows machines.

    [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
    [pull/759]: https://github.com/rustwasm/wasm-pack/pull/759
    [issue/757]: https://github.com/rustwasm/wasm-pack/issues/757

  - **Clean up `cargo test` warnings - [ashleygwilliams], [issue/752] [pull/753]**

    Tests now use `std::sync::Once::new()` instead of the deprecated `std::sync::ONCE_INIT`

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [pull/753]: https://github.com/rustwasm/wasm-pack/pull/753
    [issue/752]: https://github.com/rustwasm/wasm-pack/issues/752

- ### 📖 Documentation

  - **Document npm installer - [drager], [issue/751] [pull/767]**

    [drager]: https://github.com/drager
    [issue/751]: https://github.com/rustwasm/wasm-pack/issues/751
    [pull/767]: https://github.com/rustwasm/wasm-pack/pull/767

  - **Update help message for `build` and `publish` subcommands - [ibaryshnikov], [issue/636] [pull/640]**

    `wasm-bindgen` recently changed the default target from `browser` to `bundler` and deprecated `browser`. This change is now reflected in the help message for `wasm-pack build`.

    [ibaryshnikov]: https://github.com/ibaryshnikov
    [pull/640]: https://github.com/rustwasm/wasm-pack/pull/640
    [issue/636]: https://github.com/rustwasm/wasm-pack/issues/636

  - **Add Release Checklist - [ashleygwilliams], [issue/370] [pull/626]**

    While we try to automate releases of `wasm-pack` as much as possible, there are still some manual steps that need to be completed when releasing a new version (like writing a changelog 😉). These steps now live in [`RELEASE_CHECKLIST.md`](https://github.com/rustwasm/wasm-pack/blob/master/RELEASE_CHECKLIST.md).

    [ashleygwilliams]: https://github.com/ashleygwilliams
    [pull/626]: https://github.com/rustwasm/wasm-pack/pull/626
    [issue/370]: https://github.com/rustwasm/wasm-pack/issues/370

- ### 🛠️ Maintenance

  - **Ensure that `wasm-bindgen` generates move assertions - [fitzgen], [issue/677] [pull/683]**

    `wasm-pack` now creates `wasm-bindgen` test fixtures that must generate move assertions for both free functions and methods.

    [fitzgen]: https://github.com/fitzgen
    [pull/683]: https://github.com/rustwasm/wasm-pack/pull/683
    [issue/677]: https://github.com/rustwasm/wasm-pack/issues/677

  - **Update `cargo_metadata` to v0.8.0 - [ThomasdenH], [pull/670]**

    [ThomasdenH]: https://github.com/ThomasdenH
    [pull/670]: https://github.com/rustwasm/wasm-pack/pull/670

  - **Update `rustfmt` install snippet in PR template` - [data-pup], [issue/639] [pull/664]**

    `rustfmt` is now available on Rust's stable channel so now the `wasm-pack` PR template recommends installing the stable version instead of the nightly version.

    [data-pup]: https://github.com/data-pup
    [pull/664]: https://github.com/rustwasm/wasm-pack/pull/664
    [issue/639]: https://github.com/rustwasm/wasm-pack/issues/639

## 🛠️ 0.8.1

- ### 🤕 Fixes

  - **Check for "rustup" rather than ".rustup" when checking for wasm32 - [drager], [issue/613][pull/616]**

    When we introduced support for non-rustup setups we did a check if the user was
    using rustup or not. However, this check was too constrained and only covered
    the most common cases, but it did not work for Docker setups.

    This PR addresses that and it now covers Docker setups as well!
    When doing this fix we also found two other small issues which this PR also addresses.
    The first is that we did not print the helpful error message when the wasm32 target
    was not found and the other one was that it linked to the wrong section of the documentation.

    [issue/613]: https://github.com/rustwasm/wasm-pack/issues/613
    [pull/616]: https://github.com/rustwasm/wasm-pack/pull/616

## 🌤️ 0.8.0

- ### ✨ Features

  - **Give user's ability to customize generated filenames with `--out-name` flag - [ibaryshnikov], [issue/596] [pull/599]**

    When running `wasm-pack build`, several files are generated. These files
    are named based on the name of the crate, as per your `Cargo.toml` file.
    Sometimes- that's not the name you'd like your files to have!

    You can now specify a custom name for the generated files using a new
    flag, `--out-name`. Given a project called `dom`, here's a comparison of
    the default and custom generated filenames:

    ```
    wasm-pack build
    # will produce files
    # dom.d.ts  dom.js  dom_bg.d.ts  dom_bg.wasm  package.json  README.md

     wasm-pack build --out-name index
    # will produce files
    # index.d.ts  index.js  index_bg.d.ts  index_bg.wasm  package.json  README.md
    ```

    [ibaryshnikov]: https://github.com/ibaryshnikov
    [issue/596]: https://github.com/rustwasm/wasm-pack/issues/596
    [pull/599]: https://github.com/rustwasm/wasm-pack/pull/599

- ### 🤕 Fixes

  - **Fix panics in `build mode --no-install` - [alexcrichton], [pull/598]**

    This commit fixes the `wasm-pack build --mode no-install` command from
    unconditionally panicking as well as `--mode force`. These steps were
    calling an `unwrap()` on an internal `Option<T>` which was supposed to
    be set during `step_install_wasm_bindgen`, but that step wasn't run in
    these modes. The mode configuration of steps has been refactored
    slightly to ensure that more steps are shared between these modes to
    reduce duplication.

    [pull/598]: https://github.com/rustwasm/wasm-pack/pull/598

  - **Print unexpected panics to standard error - [drager], [issue/562] [pull/601]**

    Unexpected panics are unfortunate but they're currently covered up and written
    out to an auxiliary file. This makes panics in CI difficult to debug,
    especially at a glance, as CI builders are likely not uploading those files.

    This PR will print to standard error for unexpected panics and then let
    `human_panic` handle panics, just like before.

    [issue/562]: https://github.com/rustwasm/wasm-pack/issues/562
    [pull/601]: https://github.com/rustwasm/wasm-pack/pull/601

  - **Improve error message when `wasm32-unknown-unknown` is missing - [drager], [issue/579] [pull/602]**

    For folks with non-rustup environments (which we only started supporting in
    0.7.0!), we were giving a missing target error that was not helpful!

    We've updated the error message to include more information, and we've added
    some documentation to help explain how you can remedy the error by manually
    installing the target on your specific rust setup- including the fact that
    it may _not_ be possible to add the target to some setups.

    Check out the docs [here](https://wasm-bindgen.github.io/wasm-pack/book/prerequisites/non-rustup-setups.html).

    [issue/579]: https://github.com/rustwasm/wasm-pack/issues/579
    [pull/602]: https://github.com/rustwasm/wasm-pack/pull/602

- ### 📖 Documentation

  - **Document `--out-dir` flag - [ashleygwilliams], [issue/592] [pull/593]**

    Recently, someone asked on Discord about customizing the name of the directory
    that contains the assets built by `wasm-pack`. We've had the `out-dir` flag for
    a while, but it wasn't documented! Now it is.

    [issue/592]: https://github.com/rustwasm/wasm-pack/issues/592
    [pull/593]: https://github.com/rustwasm/wasm-pack/pull/593

  - **Fix broken links in docs and update for template changes - [drager], [ashleygwilliams], [issue/609] [pull/612] [pull/614]**

    Recently, some improvements were made to the [`wasmpack-template`]. Additionally,
    there were some broken links in the documentation. We've updated the docs for the
    new template and fixed the broken links!

    [issue/609]: https://github.com/rustwasm/wasm-pack/issues/609
    [pull/612]: https://github.com/rustwasm/wasm-pack/pull/612
    [pull/614]: https://github.com/rustwasm/wasm-pack/pull/614

- ### 🛠️ Maintenance

  - **Move `binary-install` to its own repo - [drager], [issue/500] [pull/600]**

    `binary-install` is a crate that holds the abstractions for how `wasm-pack` downloads
    and caches pre-built binaries for the tools it wraps. It was originally part of the
    `wasm-pack` code, then moved into a workspace as an independent crate. Now that we
    believe it's stable, we've moved it into its own [repo](https://github.com/rustwasm/binary-install)!

    [issue/500]: https://github.com/rustwasm/wasm-pack/issues/500
    [pull/600]: https://github.com/rustwasm/wasm-pack/pull/600

## 🌤️ 0.7.0

- ### ✨ Features

  - **Non `rustup` environment support - [drager], [pull/552]**

    Before now, `wasm-pack` had a hard requirement that `rustup` had to be in the PATH. While most Rust users use
    `rustup` there are variety reasons to have an environment that doesn't use `rustup`. With this PR, we'll now
    support folks who are using a non-`rustup` environment!

    [pull/552]: https://github.com/rustwasm/wasm-pack/pull/552

  - **Improved CLI Output - [alexcrichton], [pull/547]**

    It's hard to decide if this is a fix or a feature, but let's keep it positive! This PR moves `wasm-pack`'s CLI
    output strategy closer to the desired standard we have for 1.0. This is important as it fixes many small bugs
    that are distributed across a diveristy of terminals and difficult to test for locally.

    This strategy was first introduced as a mini RFC in [issue/298], and then was discussed in a session at the Rust
    All Hands ([notes](https://gist.github.com/fitzgen/23a62ebbd67574b9f6f72e5ac8eaeb67#file-road-to-wasm-pack-1-0-md)).

    You'll notice that the spinner is gone- we eventually want to have one, but we'd like one that doesn't cause bugs!
    If you have feedback about terminal support or an output bug, please [file an issue]! We want to hear from you!

    Check out the new output in the `README` demo- or update your `wasm-pack` and take it for a spin!

    [file an issue]: https://github.com/rustwasm/wasm-pack/issues/new/choose
    [pull/547]: https://github.com/rustwasm/wasm-pack/pull/547
    [issue/298]: https://github.com/rustwasm/wasm-pack/issues/298

  - **Add support for `--target web` - [alexcrichton], [pull/567]**

    Recently, `wasm-bindgen` add a new target- `web`. This new target is similar to the `no-modules` target, in that
    it is designed to generate code that should be loaded directly in a browser, without the need of a bundler. As
    opposed to the `no-modules` target, which produces an IIFE (Immediately Invoked Function Expression), this target
    produces code that is an ES6 module.

    You can use this target by running:

    ```
    wasm-pack build --target web
    ```

    Learn more about how to use this target by [checking out the docs!](https://wasm-bindgen.github.io/wasm-pack/book/commands/build.html#target)

    [pull/567]: https://github.com/rustwasm/wasm-pack/pull/567

  - **Support passing arbitrary arguments to `cargo test` via `wasm-pack test` - [chinedufn], [issue/525] [pull/530]**

    `wasm-pack test` is an awesome command that wraps `cargo test` in a way that helps provide you some nice out of the
    box configuration and setup. However, you may find yourself wanting to leverage the full funcationality of `cargo test`
    by passing arguments that haven't been re-exported by the `wasm-pack test` interface.

    For example, if you have a large test suite, it can be nice to simply run one test, or a subset of your tests.
    `cargo test` supports this, however up until now, the `wasm-pack test` interface did not!

    `wasm-pack test` now accepts passing and arbitrary set of arguments that it will forward along to its `cargo test` call
    by allowing users to use `--` after any `wasm-pack test` arguments, followed by the set of arguments you'd like to pass
    to `cargo test`.

    For example:

    ```
    # Anything after `--` gets passed to the `cargo test`
    wasm-pack test --firefox --headless -- --package my-workspace-crate my_test_name --color=always
    ```

    This will just run the `my_test_name` test and will output using color!

    [See the `test` docs here!](https://rustwasm.github.io/docs/wasm-pack/commands/test.html)

    [chinedufn]: https://github.com/chinedufn
    [issue/525]: https://github.com/rustwasm/wasm-pack/issues/525
    [pull/530]: https://github.com/rustwasm/wasm-pack/pull/530

  - **Support `homepage` field of `Cargo.toml` and `package.json` - [rhysd], [pull/531]**

    Both `Cargo.toml` and `package.json` support a `homepage` field that allow you to specify a website for
    your project. We didn't support it previously (purely an accidental omission) - but now we do!

    [pull/531]: https://github.com/rustwasm/wasm-pack/pull/531

  - **Support `license-file` field in `Cargo.toml` - [rhysd], [pull/527]**

    Sometimes, you want to provide a custom license, or specific license file that doesn't map to SPDX standard
    licenses. In Rust/Cargo, you accomplish this by omitting the `license` field and including a `license-file`
    field instead. You can read more about this in the [`cargo` manifest documentation].

    In an npm package, this translates to `"license": "SEE LICENSE IN <filename>"` in your `package.json`. You can
    read more about this in the [npm `package.json` documentation].

    We previously only supported using SPDX standard licenses, by only supporting the `"license"` key in your
    `Cargo.toml`- but now we'll allow you to leverage the `license-file` key as well, and will translate it
    correctly into your `package.json`!

    [`cargo` manifest documentation]: https://doc.rust-lang.org/cargo/reference/manifest.html
    [npm `package.json` documentation]: https://docs.npmjs.com/files/package.json#license
    [rhysd]: https://github.com/rhysd
    [pull/527]: https://github.com/rustwasm/wasm-pack/pull/527

- ### 🤕 Fixes

  - **`wasm-pack-init (1).exe` should work - [ashleygwilliams], [issue/518] [pull/550]**

    Several users noted that when downloading a new version of `wasm-pack` their browser named the executable
    file `wasm-pack-init (1).exe`. When named this way, the file didn't show the init instructions on execution.
    This happened because the installation logic was requiring an exact match on filename. We've loosened that
    restriction so that the filename must _start_ with `wasm-pack-init` and will still execute files with these
    additional, extraneous charaters in the filename. Thanks so much to [Mblkolo] and [danwilhelm] for filing the
    issue and the excellent discussion!

    [issue/518]: https://github.com/rustwasm/wasm-pack/issues/518
    [pull/550]: https://github.com/rustwasm/wasm-pack/pull/550
    [Mblkolo]: https://github.com/Mblkolo

  - **Fix chromedriver error and message on Windows for `wasm-pack test` - [jscheffner], [issue/535] [pull/537]**

    When running `wasm-pack test` on a 64-bit Windows machine, users would receive an error:
    `geckodriver binaries are unavailable for this target`. This error message had two issues- firstly, it accidentally
    said "geckodriver" instead of "chromedriver", secondly, it threw an error instead of using the available 32-bit
    chromedriver distribution. Chromedriver does not do a specific disribution for Windows 64-bit!

    We've fixed the error message and have also ensured that 64-bit Windows users won't encounter an error, and will
    appropriately fallback to the 32-bit Windows chromedriver.

    [jscheffner]: https://github.com/jscheffner
    [issue/535]: https://github.com/rustwasm/wasm-pack/issues/535
    [pull/537]: https://github.com/rustwasm/wasm-pack/pull/537

  - **Correct look up location for `wasm-bindgen` when it's installed via `cargo install` - [fitzgen], [pull/504]**

    Sometimes, when a `wasm-bindgen` binary is not available, or if `wasm-pack` is being run on an architecture that
    `wasm-bindgen` doesn't produce binaries for, instead of downloading a pre-built binary, `wasm-pack` will install
    `wasm-bindgen` using `cargo install`. This is a great and flexible back up!

    However, due to the last release's recent refactor to use a global cache, we overlooked the `cargo install` case
    and did not look for `wasm-bindgen` in the appropriate location. As a result, this led to a bug where `wasm-pack`
    would panic.

    We've fixed the lookup for the `cargo install`'d `wasm-bindgen` by moving the `cargo-install`'d version to global
    cache location for `wasm-pack` once it's successfully built. We also eliminated the panic in favor of
    propagating an error. Thanks for your bug reports and sorry about the mistake!

    [pull/504]: https://github.com/rustwasm/wasm-pack/pull/504

  - **Only print `cargo test` output the once - [fitzgen], [issue/511] [pull/521]**

    Due to some technical debt and churn in the part of the codebase that handles output, we were accidentally
    printing the output of `cargo test` twice. Now we ensure that we print it only one time!

    [issue/511]: https://github.com/rustwasm/wasm-pack/issues/511
    [pull/521]: https://github.com/rustwasm/wasm-pack/pull/521

- ### 🛠️ Maintenance

  - **Fix `clippy` warnings - [mstallmo], [issue/477] [pull/478]**

    [`clippy`] is an awesome utilty that helps lint your Rust code for common optimizations and idioms. at the
    beginning of `wasm-pack` development, `clippy` had not yet stablized, but it has since 1.0'd and it was
    high time we leveraged it in `wasm-pack`. We still aren't _completely_ fixed, but we're working on it, and
    we've already dervived a ton of value from the tool!

    [`clippy`]: https://github.com/rust-lang/rust-clippy
    [issue/477]: https://github.com/rustwasm/wasm-pack/issues/477
    [pull/478]: https://github.com/rustwasm/wasm-pack/pull/478

  - **Run `clippy` check on Travis - [drager], [pull/502]**

    Now that `wasm-pack` has been clippified- we want to keep it that way! Now in addition to `cargo fmt` and
    `cargo test`, we'll also run `cargo clippy` on all incoming PRs!

    [pull/502]: https://github.com/rustwasm/wasm-pack/pull/502

  - **Port tests to use `assert-cmd` - [fitzgen], [pull/522]**

    [`assert_cmd`] is a great utility for testing CLI applications that is supported by the [CLI WG]. `wasm-pack`
    development began before this library existed- so we were using a much less pleasant and efficient strategy
    to test the CLI functionality of `wasm-pack`. Now we've ported over to using this great library!

    [CLI WG]: https://www.rust-lang.org/what/cli
    [`assert_cmd`]: https://crates.io/crates/assert_cmd
    [pull/522]: https://github.com/rustwasm/wasm-pack/pull/522

  - **Add initial tests for `binary-install` crate - [drager], [pull/517]**

    In the last release, we separated some of our binary install logic into a new crate, `binary-install`.
    However, that's about all we did... move the logic! In an effort to move the crate into true open source
    status, [drager] has done some excellent work adding tests to the crate. This was trickier than it looked
    and involved creating a test server! Thanks for all the efforts [drager], and the great review work [fitzgen]
    and [lfairy]!

    [pull/517]: https://github.com/rustwasm/wasm-pack/pull/517
    [lfairy]: https://github.com/lfairy

  - **Update tests `wasm-bindgen` version - [huangjj27], [issue/519] [issue/417] [pull/526]**

    Our tests use fixtures that reference `wasm-bindgen` often, but the versions were not consistent or up to
    date. As a result, the test suite leverage many version of `wasm-bindgen` which meant that they took a while
    to run as they couldn't use the cached version of `wasm-bindgen` because the cached versions we slightly
    different! Now they are up to date and consistent so the tests can perform better!

    [pull/526]: https://github.com/rustwasm/wasm-pack/pull/526
    [issue/519]: https://github.com/rustwasm/wasm-pack/issues/519
    [issue/417]: https://github.com/rustwasm/wasm-pack/issues/417

- ### 📖 Documentation

  - **Flag gh-pages docs as unpublished - [alexcrichton] [pull/565]**

    Recently, [DebugSteven] made a PR to merge all the documentation for the rustwasm toolchain into a
    [single location]. This is going to make discovering and using tools from the entire organization easier
    for new and seasoned folks alike. This also has the feature of displaying documentation that is related
    to the current published version of each tool- unlike before, where the only accessible documentation was
    for the tools at current master (which may or may not be currently published!)

    If you like reading the current master's documentation- fear not, each tool will still publish the
    documentation generated from the master branch on their individual `gh-pages`
    ([See `wasm-pack's` master docs here]). To avoid confusion, we've added a flash message that let's you know
    which documentation you are reading- and provides a link to documentation of the published version- just
    in case that's what you're looking for!

    [DebugSteve]: https://github.com/DebugSteven
    [single location]: https://rustwasm.github.io/docs.html
    [See `wasm-pack's` master docs here]: https://wasm-bindgen.github.io/wasm-pack/book/
    [pull/565]: https://github.com/rustwasm/wasm-pack/pull/565

  - **Add new QuickStart guide for "Hybrid Applications with Webpack" - [DebugSteven] [pull/536]**

    Since `wasm-pack` was first published, we've focused on a workflow where a user writes a library and then
    publishes it to npm, where anyone can use it like any npm package in their JavaScript or Node.js application.

    Shortly after `wasm-pack` appeared, some RustWASM teammates created a template for a similar workflow- building
    a RustWASM package _alongside_ an application. They did this by leveraging Webpack plugins, and it's a really
    lovely user experience!

    [This template] hasn't gotten as much attention because we've lacked a quickstart guide for folks to discover
    and follow- now we've got one!

    Check out the guide [here](https://wasm-bindgen.github.io/wasm-pack/book/tutorials/hybrid-applications-with-webpack/index.html)!

    [This temaplte]: https://github.com/rustwasm/rust-webpack-template
    [DebugSteven]: https://github.com/DebugSteven
    [pull/536]: https://github.com/rustwasm/wasm-pack/pull/536

  - **Add `wee_alloc` deepdive - [surma], [pull/542]**

    `wee_alloc` is a useful utility that deserved more attention and explanation than our previous docs addressed.
    This was partially because the `wasm-pack` template has an explanatory comment that helps explain its use.
    However, for folks who don't use the template, `wee_alloc` is something important to know about- so now we have
    given it its own section!

    Check out the deepdive [here](https://wasm-bindgen.github.io/wasm-pack/book/tutorials/npm-browser-packages/template-deep-dive/wee_alloc.html)!

    [surma]: https://github.com/surma
    [pull/542]: https://github.com/rustwasm/wasm-pack/pull/542

  - **Update prerequisite documentation - [alexcrichton], [pull/569]**

    Many folks are using `wasm-pack` without publishing to npm- as a result, we've updated the documentation to
    clearly indicate that npm is an optional requirement, only required for specific targets and workflows.
    Additionally, since the 2018 Edition landed, `nightly` Rust is no longer a requirement. We've removed those
    instructions and have consolidated the documentation so it is shorter and more efficient at getting you
    started!

    [pull/569]: https://github.com/rustwasm/wasm-pack/pull/569

  - **Clarify what kind of account `login` adds - [killercup], [pull/539]**

    Previously, when view `--help`, the command description for `login` showed:
    `👤  Add a registry user account!` This could be confusing for folks, so now it's been updated to read:
    `👤  Add an npm registry user account!`, which is much clearer!

    [killercup]: https://github.com/killercup
    [pull/539]: https://github.com/rustwasm/wasm-pack/pull/539

  - **Wasm is a contraction, not an acronym - [fitzgen], [pull/555]**

    Ever wonder how you're _actually_ supposed to refer to WebAssembly in short-form? WASM? wasm? For the pedants
    out there, the correct usage is "Wasm" because Wasm is a _contraction_ of the words Web and Assembly. We've
    updated our doucmentation to consistently refer to WebAssembly as Wasm in the shortform.

    _The more you know!_

    [pull/555]: https://github.com/rustwasm/wasm-pack/pull/555

  - **Fix links and Rust highlightning - [drager], [issue/513] [pull/514] [pull/516]**

    We had some broken links and missing Rust syntax highlighting in a few sections of the docs. This fixes that!

    [issue/513]: https://github.com/rustwasm/wasm-pack/issues/513
    [pull/514]: https://github.com/rustwasm/wasm-pack/pull/514
    [pull/516]: https://github.com/rustwasm/wasm-pack/pull/516

## 🌅 0.6.0

- ### ✨ Features

  - **Add three build profiles and infrastructure for their toml config - [fitzgen], [issue/153] [issue/160] [pull/440]**

    When originally conceived, `wasm-pack` was exclusively a packaging and publishing tool, which naively assumed
    that the crate author would simply run `wasm-pack` when they were ready to publish a wasm package. As a result,
    `wasm-pack` always ran `cargo build` in `--release` mode. Since then, `wasm-pack` has grown into an integrated build
    tool used at all stages of development, from idea conception to publishing, and as such has developed new needs.

    In previous releases, we've supported a flag called `--debug` which will run `cargo build` in `dev` mode, which
    trades faster compilation speed for a lack of optimizations. We've renamed this flag to `--dev` to match `cargo`
    and added an additional flag, representing a third, intermediary, build profile, called `--profiling` which
    is useful for investigating performance issues. You can see all three flags and their uses in the table below:

    | Profile       | Debug Assertions | Debug Info | Optimizations | Notes                                                       |
    | ------------- | ---------------- | ---------- | ------------- | ----------------------------------------------------------- |
    | `--dev`       | Yes              | Yes        | No            | Useful for development and debugging.                       |
    | `--profiling` | No               | Yes        | Yes           | Useful when profiling and investigating performance issues. |
    | `--release`   | No               | No         | Yes           | Useful for shipping to production.                          |

    The meaning of these flags will evolve as the platform grows, and always be tied to the behavior of these flags
    in `cargo`. You can learn more about these in the [`cargo profile` documentation].

    This PR also introduces a way to configure `wasm-pack` in your `Cargo.toml` file that we intend to use much more
    in the future. As a largely convention-based tool, `wasm-pack` will never require that you configure it manually,
    however, as our community and their projects mature alongside the tool, it became clear that allowing folks the
    ability to drop down and configure things was something we needed to do to meet their needs.

    Currently, you can only configure things related to the above-mentioned build profiles. To learn more,
    [check out the documentation][profile-config-docs]. It leverages the `package.metadata.wasm-pack` key in your
    `Cargo.toml`, and looks like this:

    ```toml
    # Cargo.toml

    [package.metadata.wasm-pack.profile.dev.wasm-bindgen]
    # Should we enable wasm-bindgen's debug assertions in its generated JS glue?
    debug-js-glue = true
    # Should wasm-bindgen demangle the symbols in the "name" custom section?
    demangle-name-section = true
    # Should we emit the DWARF debug info custom sections?
    dwarf-debug-info = false
    ```

    As always- there are defaults for you to use, but if you love to configure (or have a project that requires it),
    get excited, as your options have grown now and will continue to!

    [profile-config-docs]: https://wasm-bindgen.github.io/wasm-pack/book/cargo-toml-configuration.html
    [`cargo profile` documentation]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections
    [issue/153]: https://github.com/rustwasm/wasm-pack/issues/153
    [issue/160]: https://github.com/rustwasm/wasm-pack/issues/160
    [pull/440]: https://github.com/rustwasm/wasm-pack/pull/440

  - **DEPRECATION: Rename `--debug` to `--dev` to match `cargo` - [fitzgen], [pull/439]**

    See the discussion of the build profiles feature above. This is a strict renaming of the previous `--debug` flag,
    which will now warn as deprecated.

    [pull/439]: https://github.com/rustwasm/wasm-pack/pull/439

  - **Add an option to pass an arbitrary set of arguments to `cargo build` - [torkve], [issue/455] [pull/461]**

    As an integrated build tool, `wasm-pack` orchestrates many secondary command line tools to build your package
    in a single command. Notably, one of these tools is `cargo`. `cargo` has a wide array of features and flags, and
    we couldn't reasonably expect to implement them all as first class features of `wasm-pack`. As a result, we've
    created the option to allow users to pass an arbitrary number of additional flags to `wasm-pack` by appending them
    to the `wasm-pack build` command, after passing `--`. For example:

    ```
    wasm-pack build examples/js-hello-world --mode no-install -- -Z offline
    ```

    In the above example, the flag `-Z offline` will be passed to `cargo build`. This feature is documented
    [here][cargo opts docs].

    [cargo opts docs]: https://wasm-bindgen.github.io/wasm-pack/book/commands/build.html#extra-options
    [torkve]: https://github.com/torkve
    [issue/455]: https://github.com/rustwasm/wasm-pack/issues/455
    [pull/461]: https://github.com/rustwasm/wasm-pack/pull/461

  - **Pre-build before wasm-pack publish - [csmoe], [issue/438] [pull/444]**

    Previously, if you ran `wasm-pack publish` before you had successfully run `wasm-pack build`,
    you'd receive an error that a package could not be found- because there would be no `pkg` or
    out-directory containing a `package.json`.

    In this situation, you would hope that `wasm-pack` would build your package for you when you
    ran `wasm-pack publish`. This is slightly complicated by the fact that not everyone wants to
    build their package to the default target or to a directory named `pkg`.

    To solve this, running `wasm-pack publish` before a successful build will give you an interactive
    prompt to build your package- allowing you to specify your out directory as well as the target you'd
    like to build to. Check it out in the gif below:

    ![pre-build publish workflow](https://user-images.githubusercontent.com/35686186/50500909-5984fe80-0a8f-11e9-9de6-43d1423b2969.gif)

    [issue/438]: https://github.com/rustwasm/wasm-pack/issues/438
    [pull/444]: https://github.com/rustwasm/wasm-pack/pull/444

  - **Generate self-.gitignore as part of pkg folder - [RReverser], [pull/453]**

    Since `wasm-pack` was first published, the `pkg` directory was intended to be treated as a
    build artifact, and as such should never be published to version control. This was
    never enforced by any assets generated by `wasm-pack`, however.

    Now, when building your package, `wasm-pack` will also generate a `.gitignore` file so that the
    `pkg`, or out-directory, will be ignored.

    If you use another version control tool, you'll need to still create or edit your own ignore file-
    pull requests to support other version control tools are welcome!

    If you require editing of the generated `package.json` or add additonal assets to your package
    before publishing, you'll want to remove the `.gitignore` file and commit to version control. We
    intend to have a solution that makes this workflow significantly easier in upcoming releases!

    [RReverser]: https://github.com/RReverser
    [pull/453]: https://github.com/rustwasm/wasm-pack/pull/453

  - **Support cargo workspaces - [fitzgen], [issue/252] [issue/305] [pull/430]**

    Workspaces are a well-liked and used feature of cargo that allow you to build multiple crates
    in a single cargo project. Because of how `wasm-pack` handled paths for `target` and out-directories,
    we did not support cargo workspaces out of the box. Now they should work well and the feature is
    well guarded by tests!

    [issue/252]: https://github.com/rustwasm/wasm-pack/issues/252
    [issue/305]: https://github.com/rustwasm/wasm-pack/issues/305
    [pull/430]: https://github.com/rustwasm/wasm-pack/pull/430

  - **Use a global cache for all downloaded binaries - [alexcrichton], [pull/426]**

    `wasm-pack` is an integrated build tool that orchestrates several other command line tools to build
    your wasm project for you. How `wasm-pack` does this has evolved significantly since it's early versions.
    In the last version, a `bin` directory was created to house the tool binaries that `wasm-pack` needed to
    build our project, but this had several limitations. Firstly, it created a `bin` directory in your project's
    root, which could be confusing. Secondly, it meant that sharing these tools across multiple projects was
    not possible. We did this because it gaves us the fine-grained control over the version of these tools that
    you used.

    Now, `wasm-pack` will not generate a `bin` directory, but rather will use a global cache. We retain the
    fine-grained control over the versions of these tools that are used, but allow multiple projects that use
    the same tools at the same versions to share the already installed asset. Your global cache will generally
    be in your user's home directory- we use the [`dirs` crate] to determine where to place this global cache.
    This is not currently customizable but is something we intend to look into doing!

    This feature ensures that `wasm-pack` users are downloading a minimal number of binaries from the network,
    which, for `wasm-pack` users with multiple projects, should speed up build times.

    [`dirs` crate]: https://docs.rs/dirs/1.0.4/dirs/fn.cache_dir.html
    [pull/426]: https://github.com/rustwasm/wasm-pack/pull/426

- ### 🤕 Fixes

  - **Fix `pack`, `login`, and `publish` for Windows users - [danwilhelm], [issue/277] [pull/489]**

    Rust's behavior for spawning processes on some Windows targets introduced an interesting case where
    Rust would fail unless the command was explicitly spawned with a prepended `cmd /c`. This failure
    of `wasm-pack` was well noticed by our community - and thanks to the efforts of `danwilhelm` is now
    fixed! You can read more on the background of this issue in [rust-lang/rust issue/44542].

    [rust-lang/rust issue/44542]: https://github.com/rust-lang/rust/pull/44542
    [issue/277]: https://github.com/rustwasm/wasm-pack/issues/277
    [pull/489]: https://github.com/rustwasm/wasm-pack/pull/489

  - **Validate `--target` argument - [csmoe], [issue/483] [pull/484]**

    For a few releases now, `wasm-pack` has supported allowing users to specifying the target module system
    they'd like their package built for- `browser`, `nodejs`, and `no-modules`. We did not however, validate
    this input, and so if a user made even a slight mistake, e.g. `node`, `wasm-pack` would not catch the
    error and would build your project using the default, `browser`. This is of course, surprising, and
    unpleasant behavior and so now we'll error out with a message containing the supported target names.

    [issue/483]: https://github.com/rustwasm/wasm-pack/issues/483
    [pull/484]: https://github.com/rustwasm/wasm-pack/pull/484

  - **Fix login - [danwilhelm], [issue/486] [pull/487]**

    [danwilhelm]: https://github.com/danwilhelm
    [issue/486]: https://github.com/rustwasm/wasm-pack/issues/486
    [pull/487]: https://github.com/rustwasm/wasm-pack/pull/487

  - **Eliminate unecessary escaping in build success terminal output - [huangjj27], [issue/390] [pull/396]**

    Previously, on some systems, a successful `wasm-pack build` would print a unfortunate looking string:

    ```
    | :-) Your wasm pkg is ready to publish at "\\\\?\\C:\\Users\\Ferris\\tmp\\wasm-bug\\pkg".
    ```

    We've updated this to make sure the path to your project is well-formed, and most importantly,
    human-readable.

    [issue/390]: https://github.com/rustwasm/wasm-pack/issues/390
    [pull/396]: https://github.com/rustwasm/wasm-pack/pull/396

  - **Copy license file(s) to out directory - [mstallmo], [issue/407] [pull/411]**

    Since `wasm-pack` was first published, we've copied over your `Cargo.toml` license definition over to
    your `package.json`. However, we overlooked copying the actual `LICENSE` files over! Now we do!

    [issue/407]: https://github.com/rustwasm/wasm-pack/issues/407
    [pull/411]: https://github.com/rustwasm/wasm-pack/pull/411

  - **Don't require cdylib crate-type for testing - [alexcrichton], [pull/442]**

    `wasm-pack` was unecssarily checking `Cargo.toml` for the `cdylib` crate type during calls to `wasm-pack test`.
    The `cdylib` output isn't necessary for the `wasm-pack test` stage because `wasm-bindgen` isn't being run over
    a wasm file during testing. This check is now removed!

    [pull/442]: https://github.com/rustwasm/wasm-pack/pull/442

  - **Fix wasm-bindgen if lib is renamed via `lib.name` - [alexcrichton], [issue/339] [pull/435]**

    In some circumstances, a library author may wish to specify a `name` in the `[package]` portion of their
    `Cargo.toml`, as well as a different `name` in the `[lib]` portion, e.g.:

    ```toml
    [package]
    name = "hello-wasm"

    [lib]
    name = "wasm-lib"
    ```

    This would cause the `wasm-bindgen` build stage of `wasm-pack` to error out because `wasm-pack` would attempt
    to run `wasm-bindgen-cli` on a path using the `[package]` name, which wouldn't exist (because it would be using
    the `[lib]` name). Now it works- thanks to more usage of [`cargo_metadata`] in `wasm-pack` internals!

    [`cargo_metadata`]: https://crates.io/crates/cargo_metadata
    [issue/339]: https://github.com/rustwasm/wasm-pack/issues/339
    [pull/435]: https://github.com/rustwasm/wasm-pack/pull/435

  - **Print standard error only once for failing commands - [fitzgen], [issue/422] [pull/424]**

    Previously, `wasm-pack` may have printed `stderr` twice in some circumstances. This was both confusing and not
    a pleasant experience, so now we've ensued that `wasm-pack` prints `stderr` exactly once! (It's hard enough to have
    errors, you don't want `wasm-pack` rubbing it in, right?)

    [issue/422]: https://github.com/rustwasm/wasm-pack/issues/422
    [pull/424]: https://github.com/rustwasm/wasm-pack/pull/424

  - **Add no-modules to --target flag's help text - [fitzgen], [issue/416] [pull/417]**

    This is an interesting one! `fitzgen` very reasonably filed an issue asking to add `wasm-bindgen`'s
    `--target no-modules` feature to `wasm-pack`. This was confusing as this feature was indeed already implemented,
    and documented- BUT, notably missing from the `wasm-pack --help` text. We've fixed that now- and it was an omission
    so glaring we definitely considered it a bug!

    [issue/416]: https://github.com/rustwasm/wasm-pack/issues/416
    [pull/417]: https://github.com/rustwasm/wasm-pack/pull/417

- ### 🛠️ Maintenance

  - **Replace `slog` with `log` - [alexcrichton], [issue/425] [pull/434]**

    For internal maintenance reasons, as well as several end-user ones, we've migrated away from the `slog` family
    of crates, and are now using the `log` crate plus `env_logger`. Now, `wasm-pack` won't create a `wasm-pack.log`.
    Additionally, enabling logging will now be done through `RUST_LOG=wasm_pack` instead of `-v` flags.

    [issue/425]: https://github.com/rustwasm/wasm-pack/issues/425
    [pull/434]: https://github.com/rustwasm/wasm-pack/pull/434

  - **Move binary installation to its own crate - [drager], [issue/384] [pull/415]**

    In `wasm-pack 0.5.0`, we move away from `cargo install`ing many of the tools that `wasm-pack` orchestrates. Because
    we used `cargo install`, this required an end user to sit through the compilation of each tool, which was a
    prohibitively long time. We moved, instead, to building, and then installing, binaries of the tools. This sped up
    build times dramatically!

    This pattern has been very beneficial to `wasm-pack` and is potentially something that could be beneficial to other
    projects! As a result, we've refactored it out into a crate and have published it as it's own crate, [`binary-install`].

    [`binary-install`]: https://crates.io/crates/binary-install
    [drager]: https://github.com/drager
    [issue/384]: https://github.com/rustwasm/wasm-pack/issues/384
    [pull/415]: https://github.com/rustwasm/wasm-pack/pull/415

  - **Replace internal `Error` with `failure::Error` - [alexcrichton], [pull/436]**

    The story of error message handling in `wasm-pack` has not been the prettiest. We originally were manually implementing
    errors, adding the [`failure` crate] at one point, but not fully updating the entire codebase. With this PR, we are
    nearly completely handling errors with `failure`, bringing the code into a much more maintainable and
    pleasant-to-work-on place.

    [`failure` crate]: https://crates.io/crates/failure
    [pull/436]: https://github.com/rustwasm/wasm-pack/pull/436

  - **Update `mdbook` version used by Travis - [fitzgen], [pull/433]**

    [pull/433]: https://github.com/rustwasm/wasm-pack/pull/433

  - **Read the `Cargo.toml` file only once - [fitzgen], [issue/25] [pull/431]**

    This is a very fun one since it fixes one of the original issues filed by `ag_dubs` at the very beginning of `wasm-pack`
    development. In a rush to implement a POC tool, `ag_dubs` noted for posterity that the `Cargo.toml` was being read
    multiple times (twice), when it did not need to be. Thanks to `fitzgen` now it's read only once! A minor performance
    improvement in the scheme of things, but a nice one :)

    [issue/25]: https://github.com/rustwasm/wasm-pack/issues/25
    [pull/431]: https://github.com/rustwasm/wasm-pack/pull/431

  - **Use `name` field for Travis CI jobs - [fitzgen], [pull/432]**

    [pull/432]: https://github.com/rustwasm/wasm-pack/pull/432

  - **Add a test for build command - [huangjj27], [pull/408]**

    [huangjj27]: https://github.com/huangjj27
    [pull/408]: https://github.com/rustwasm/wasm-pack/pull/408

  - **Test paths on Windows - [xmclark], [issue/380] [pull/389]**

    [xmclark]: https://github.com/xmclark
    [issue/380]: https://github.com/rustwasm/wasm-pack/issues/380
    [pull/389]: https://github.com/rustwasm/wasm-pack/pull/389

  - **Fix typo in test function name for copying the README - [mstallmo], [pull/412]**

    [pull/412]: https://github.com/rustwasm/wasm-pack/pull/412

- ### 📖 Documentation

  - **Complete template deep dive docs - [danwilhelm], [issue/345] [issue/346] [pull/490]**

    In a rush to publish a release, `ag_dubs` left some "Coming soon!" comments on most pages
    of the "Template Deep Dive" docs. These docs help walk new users through the boilerplate
    that using the `wasm-pack` template generates for you. Thanks so much to `danwilhem` for
    picking this up and doing an excellent job!

    [issue/345]: https://github.com/rustwasm/wasm-pack/issues/345
    [issue/346]: https://github.com/rustwasm/wasm-pack/issues/346
    [pull/490]: https://github.com/rustwasm/wasm-pack/pull/490

  - **Minor docs updates - [fitzgen], [issue/473] [pull/485]**

    [issue/473]: https://github.com/rustwasm/wasm-pack/issues/473
    [pull/485]: https://github.com/rustwasm/wasm-pack/pull/485

## 🌄 0.5.1

- ### 🤕 Fixes

  - **Child Process and output management - [fitzgen], [issue/287] [pull/392]**

    Not exactly a "fix", but definitely a huge improvment in how child processes and their
    output are handled by `wasm-pack`. Ever sat at a long prompt from `wasm-pack` and
    wondered what was happening? No longer! Did `wasm-pack` eat your test output- no more!

    [issue/287]: https://github.com/rustwasm/wasm-pack/issues/287
    [pull/392]: https://github.com/rustwasm/wasm-pack/pull/392

  - **Less scary missing field messages - [mstallmo], [issue/393] [pull/394]**

    After watching a livestream of someone using `wasm-pack`, [fitzgen] noted that folks
    seemed pretty alarmed by the loud warning about missing optional manifest fields.
    As a result, we are now downgrading those messages from WARN to INFO, and consolidating
    them on a single line.

    [issue/393]: https://github.com/rustwasm/wasm-pack/issues/393
    [pull/394]: https://github.com/rustwasm/wasm-pack/pull/394

  - **Add `exit_status` to CLI errors - [konstin], [issue/291] [pull/387]**

    We'd been hiding these- but we shouldn't have been!

    [konstin]: https://github.com/konstin
    [issue/291]: https://github.com/rustwasm/wasm-pack/issues/291
    [pull/387]: https://github.com/rustwasm/wasm-pack/pull/387

  - **Remove lingering forced nightly usage - [alexcrichton], [pull/383]**

    In 0.5.0 we removed all forced nightly usage as we depend on `~1.30` which is now
    available on both nightly and beta channels! We had a bit of a race condition with
    that PR and the `wasm-pack test` PR, and missed a few as a result! This removes all
    lingering forced nightly, which only affected the `wasm-pack test` command.

    [pull/383]: https://github.com/rustwasm/wasm-pack/pull/383

  - **Fix `wasm-bindgen-test` dependency error message - [fitzgen], [issue/377] [pull/378]**

    The error message about missing the `wasm-bindgen-test` dependency errantly stated
    that the user was missing a `wasm-bindgen` dependency! We've fixed it to correctly
    state the missing dependency now.

    [issue/377]: https://github.com/rustwasm/wasm-pack/issues/377
    [pull/378]: https://github.com/rustwasm/wasm-pack/pull/378

  - **Fix prerequisites links in docs - [fitzgen], [pull/376]**

    [pull/376]: https://github.com/rustwasm/wasm-pack/pull/376

- ### 🛠️ Maintenance

  - **Leverage `failure::Error` consistently - [drager], [issue/280] [pull/401]**

    This PR finally makes it so that `wasm-pack` is handling errors in a consistent
    way across the codebase.

    [drager]: https://github.com/drager
    [issue/280]: https://github.com/rustwasm/wasm-pack/issues/280
    [pull/401]: https://github.com/rustwasm/wasm-pack/pull/401

## ☀️ 0.5.0

- ### ✨ Features

  - #### **Website!** - [ashleygwilliams], [pull/246]

    We have a website now. It has the installer and links to documentation. In the future,
    we hope to have calls to action for folks first coming to the site who are looking to
    do specific things- these will help them find the docs and tutorials they need to.

    This PR also has a complete rework of our documentation.

    Check it out [here](https://wasm-bindgen.github.io/wasm-pack/)!

  - #### 🍱 Module Support

    - **BREAKING: use correct `package.json` keys for generated JavaScript - [ashleygwilliams], [issue/309] [pull/312]**

      This is marked as potentially breaking because it changes the `package.json` keys that
      are generated by the project.

      Previously, we generated a JavaScript file and placed it in the `main` key, regardless
      of what you were targeting, ES6 and Node.js alike.

      We have received a lot of requests for `wasm-pack` to generate "isomorphic" packages,
      that contain assets that could work on both Node.js and ES6, and this led to us
      looking more closely at how we are using `package.json`.

      With this release, we will do the following:

      - `--target browser`: By default, we generate JS that is an ES6 module. We used to put
        this in the `main` field. Now we put it in the `module` field. We also add
        `sideEffects: false` so that bundlers that want to tree shake can.

      - `--target nodejs`: This target doesn't change. We put generated JS that is a
        CommonJS module in the `main` key.

      - `--target no-modules`: This is a new target. For this target we generate bare JavaScript.
        This code is put in a `browser` field.

      You can see the structs that represent each target's expected `package.json` [here](https://github.com/rustwasm/wasm-pack/tree/master/src/manifest/npm).

      Thanks so much to [bterlson] for his help in sorting this out for us!

      [bterlson]: https://github.com/bterlson
      [issue/309]: https://github.com/rustwasm/wasm-pack/issues/309
      [pull/312]: https://github.com/rustwasm/wasm-pack/pull/312

  - #### 🛠️ New Commands

    - **`wasm-pack init` is now `wasm-pack build` - [csmoe], [issue/188] [pull/216]**

      When this project was first conceived, we imagined it would be simply a way to package
      up generate wasm and js and publish it to npm. Here we are at version `0.5.0` and we
      have become much more- an integrated build tool!

      As a result, the original command `init` does a lot more than that these days. We've
      renamed the command to better reflect the work it's actually doing. `init` will still
      work, but is deprecated now, and we will eventually remove it.

      [csmoe]: https://github.com/csmoe
      [issue/188]: https://github.com/rustwasm/wasm-pack/issues/188
      [pull/216]: https://github.com/rustwasm/wasm-pack/pull/216

    - **add new command: `wasm-pack test` - [fitzgen], [pull/271]**

      This is an experimental new command that will run your tests in Node.js or a headless
      browser using `wasm-pack test`. Check out this [tutorial](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
      to learn more!

      [pull/271]: https://github.com/rustwasm/wasm-pack/pull/271

    - **add 2FA support to `wasm-pack publish` - [mstallmo], [issue/257] [pull/282]**

      We've been wrapping the `npm login` and `npm publish` commands as `wasm-pack login`
      and `wasm-pack publish` for a while now- but we didn't fully support two factor
      authentication. Now we do! (Be safe out there! 2FA is good for everyone!)

      [issue/257]: https://github.com/rustwasm/wasm-pack/issues/257
      [pull/282]: https://github.com/rustwasm/wasm-pack/pull/282

  - #### 🎏 New Flags

    - **New target, bare JavaScript: `--target no-modules` - [ashleygwilliams], [issue/317] [pull/327]**

      `wasm-bindgen` offers a `no-modules` flag that until now, we didn't support. This flag
      produces bare, no modules JavaScript. So if that's your thing, this target is for you!

      [issue/317]: https://github.com/rustwasm/wasm-pack/issues/317
      [pull/327]: https://github.com/rustwasm/wasm-pack/pull/327

    - **`--access` flag for `wasm-pack` publish - [ashleygwilliams], [issue/297] [pull/299]**

      Many of our tutorials use scopes to help prevent folks from attempting to publish
      packages that will lead to npm Registry errors because the package name already exists.

      However, by default, scoped packages are assumed by the npm registry to be private, and
      the ability to publish private packages to the npm registry is a paid feature. Worry not!
      Now you can pass `--access public` to `wasm-pack publish` and publish scoped packages
      publicly.

      [issue/297]: https://github.com/rustwasm/wasm-pack/issues/297
      [pull/299]: https://github.com/rustwasm/wasm-pack/pull/299

  - #### ✅ New Checks

    - **rustc version check - [ashleygwilliams], [issue/351] [pull/353]**

      Now that we have a new fangled installer, there's a chance that folks might install `wasm-pack`
      and not have Rust installed. Additionally, now that the features we required from the `nightly`
      channel of Rust have moved to `beta`- we don't need to enforce `nightly`.

      As of this release, we will check that your Rust version is above `1.30.0`. You can be on
      either the `nightly` or `beta` channel and all of `wasm-pack`s calls to `cargo` will
      respect that.

      Really hate this? You can pass `--mode force` to `wasm-pack` to skip this check. I hope you know
      what you're doing!

    - **coordinating wasm-bindgen versions and installing from binaries for improved speed - [datapup], [issue/146] [pull/244] [pull/324]**

      This is the true gem of this release. Have you been frustrated by how long `wasm-pack` takes to
      run? Overusing `--mode no-install`? This is the release you're looking for.

      Many releases back we realized that folks were struggling to keep the `wasm-bindgen` library
      that their project used in sync with the `wasm-bindgen` CLI application which `wasm-pack`
      runs for you. This became such an issue that we opted to force install `wasm-bindgen` to ensure
      that every `wasm-pack` user had the latest version.

      Like many technical solutions, this solved our original problem, but caused a new one. Now, we
      we are forcing a `cargo install` of `wasm-bindgen` on every run, and that means downloading
      and compiling `wasm-bindgen` everytime you want to run `wasm-pack`. That's unacceptable!

      We're happy to announce that we have a pretty great solution, and several more planned for
      future releases. As of this release, we will read your `Cargo.lock` to find the version of
      `wasm-bindgen` you are using in your local project. We will attempt to fetch a binary version
      of `wasm-bindgen` that matches your local version. We place that binary local to your project,
      and use it when you run `wasm-pack build`. The next time you run `wasm-pack build` we'll use
      that binary, instead of fetching a new one. We still fall back to `cargo install` for
      less common architectures but this is a huge speed improvement. Check out these benchmarks!

      ##### `wasm-pack` v0.4.2

      ```
      $ time wasm-pack init                   # fresh build
      real    1m58.802s
      user    14m49.679s
      sys     0m24.957s

      $ time wasm-pack init                   # re-build
      real    0m56.953s
      user    11m12.075s
      sys     0m18.835s

      $ time wasm-pack init -m no-install     # re-build with no-install
      real    0m0.091s
      user    0m0.052s
      sys     0m0.042s
      ```

      ##### `wasm-pack` v0.5.0

      ```
      $ time wasm-pack build                  # fresh build
      real    1m3.350s
      user    3m46.912s
      sys     0m6.057s

      $ time wasm-pack build                  # re-build
      real    0m0.230s
      user    0m0.185s
      sys     0m0.047s

      $ time wasm-pack build -m no-install    # re-build with no-install
      real    0m0.104s
      user    0m0.066s
      sys     0m0.041s
      ```

      [datapup]: https://github.com/datapup
      [issue/146]: https://github.com/rustwasm/wasm-pack/issues/146
      [pull/244]: https://github.com/rustwasm/wasm-pack/pull/244
      [pull/324]: https://github.com/rustwasm/wasm-pack/pull/324

    - **enforce `cargo build` with `--lib` - [ashleygwilliams], [issue/303] [pull/330]**

      Right now, `wasm-pack` only works on Rust library projects. But sometimes, if you're
      new to Rust, you might end up having a `main.rs` in your project, just by mistake.
      Some folks ran into this and realized that it can cause issues!

      As a result, we are enforcing that `cargo build` only build the library at this time.

      Want to use `wasm-pack` on a binary application? We're interested in hearing from you!
      Checkout [issue/326] and please comment! We want to support binary applicaitons in
      the future and are always happy and curious to hear about how folks use `wasm-pack`!

      [issue/326]: https://github.com/rustwasm/wasm-pack/issues/326
      [issue/303]: https://github.com/rustwasm/wasm-pack/issues/303
      [pull/330]: https://github.com/rustwasm/wasm-pack/pull/330

  - #### Installers and Releases

    - **Appveyor Windows Pre-Built binaries - [alexcrichton], [issue/147] [pull/301]**

      We finally got Appveyor to publish pre-built binaries to GitHub releases.
      Aside: I really wish there were an easier way to test and debug this stuff.

      [alexcrichton]: https://github.com/alexcrichton
      [issue/147]: https://github.com/rustwasm/wasm-pack/issues/147
      [pull/301]: https://github.com/rustwasm/wasm-pack/pull/301

    - **new experimental installer - [alexcrichton], [pull/307]**

      Whew, this one is exciting. Up until now, `wasm-pack` has been distributed using
      `cargo install`. This is not ideal for several reasons. Updating is confusing,
      and every time it's installed the user has to wait for it to compile- right at the
      moment they just want to hurry up and use it already.

      Say hello to the new `wasm-pack` installer- we have an executable for Windows
      and a `curl` script for \*nix users. Not pleased with that? File an issue for your
      preferred distribution method and we'll do our best to get it working!

      This is experimental- so please try it out and file issues as you run into things!
      You'll always be able to use `cargo install` as a backup.

      Checkout the new installer [here](https://wasm-bindgen.github.io/wasm-pack/installer/)!

      [pull/307]: https://github.com/rustwasm/wasm-pack/pull/307

- ### 🛠️ Maintenance

  - **testing fixture strategy improvements - [fitzgen], [pull/211] [pull/323]**

    [pull/211]: https://github.com/rustwasm/wasm-pack/pull/211
    [pull/323]: https://github.com/rustwasm/wasm-pack/pull/323

  - **split testing utils into separate files - [csmoe], [issue/231] [pull/216]**

    [issue/231]: https://github.com/rustwasm/wasm-pack/issues/231
    [pull/216]: https://github.com/rustwasm/wasm-pack/pull/216

  - **update dependencies - [ashleygwilliams], [issue/319] [pull/320]**

    [issue/319]: https://github.com/rustwasm/wasm-pack/issues/319
    [pull/320]: https://github.com/rustwasm/wasm-pack/pull/320

- ### 📖 Documentation

  - **improve readability of warnings about missing optional fields - [twilco], [pull/296]**

    A little punctuation goes a long way. Error message improvement PRs are the best.

    [twilco]: https://github.com/twilco
    [pull/296]: https://github.com/rustwasm/wasm-pack/pull/296

  - **update links in README - [alexcrichton], [pull/300]**

    We had a real dicey documentation situation for a while. Sorry about that, and thank
    you SO MUCH to all the folks who filed PRs to fix it.

    [pull/300]: https://github.com/rustwasm/wasm-pack/pull/300

  - **fix broken links in book by using relative paths - [mstallmo], [issue/325] [pull/328]**

    [mstallmo]: https://github.com/mstallmo
    [issue/325]: https://github.com/rustwasm/wasm-pack/issues/325
    [pull/328]: https://github.com/rustwasm/wasm-pack/pull/328

## ✨ 0.4.2

- #### 🤕 Fixes

  - **recognize `[dependencies.wasm-bindgen]` during dep check in `init` - [ashleygwilliams], [issue/221] [pull/224]**

    When we originally implemented the dependency check in `wasm-pack init` we naively only checked for the
    "simple" dependency declaration, `[dependencies] wasm-bindgen="0.2"`. However! This is not the only way
    to declare this dependency, and it's not the ideal way to do it if you want to specify features from the
    crate. Now that a bunch of folks want to use `features = ["serde-serialize"]` we ran into a bunch of folks
    having issues with our naive dependency checker! Thanks so much to [turboladen] for filing the very detailed
    issue that helped us solve this quickly!

    PSSSST! Curious what `features = ["serde-serialize"]` with `wasm-bindgen` actually does? It's awesome:

    > It's possible to pass data from Rust to JS not explicitly supported in the [Feature Reference](./feature-reference.md) by serializing via [Serde](https://github.com/serde-rs/serde).

    Read the [Passing arbitrary data to JS docs] to learn more!

    [Passing arbitrary data to JS docs]: https://github.com/rustwasm/wasm-bindgen/blob/master/guide/src/reference/arbitrary-data-with-serde.md
    [turboladen]: https://github.com/turboladen
    [issue/221]: https://github.com/rustwasm/wasm-pack/issues/221
    [pull/224]: https://github.com/rustwasm/wasm-pack/pull/224

  - **improve UX of publish and pack commands - [Mackiovello], [pull/198]**

    Previous to this fix, you would need to be in the parent directory of the `/pkg` dir to successfully run
    `pack` or `publish`. This was pretty crummy! Thankfully, [Mackiovello] swooped in with a fix, that you can
    find documented in the [pack and publish docs]!

    [Mackiovello]: https://github.com/Mackiovello
    [pull/198]: https://github.com/rustwasm/wasm-pack/pull/198
    [pack and publish docs]: https://github.com/rustwasm/wasm-pack/blob/05e4743c22b57f4c4a1bfff1df1d2cc1a595f523/docs/pack-and-publish.md

  - **use `PathBuf` instead of `String` for paths - [Mackiovello], [pull/220]**

    This is mostly a maintenance PR but does fix one very small bug- depending on if you add a trailing slash to
    a path that you pass to `init`, you might have seen an extra `/`! Now that we're using a proper Type to
    handle this, that's much better, and in general, all the operations using paths are more robust now.

    [pull/220]: https://github.com/rustwasm/wasm-pack/pull/220

- #### 📖 Documentation

  - **update docs and tests to eliminate no longer necessary feature flags - [ashleygwilliams], [pull/226]**

    The Rust 2018 edition marches on and we are seeing feature flags drop like flies :) Instead of a whole slew
    of feature flags, we now only need one, `#![feature(use_extern_macros)]`, and that one is also not long for
    this world :)

    [pull/226]: https://github.com/rustwasm/wasm-pack/pull/226

## ⭐ 0.4.1

- #### 🤕 Fixes

  - **fix `files` key value for projects build for `nodejs` target - [ashleygwilliams], [issue/199] [pull/205]**

    We became aware that the `files` key in `package.json` did not include the additional `_bg.js` file that
    `wasm-bindgen` generates for projects being built for the `nodejs` target. This resulted in the file not
    being included in the published package and resulted in a `Module Not Found` error for folks.

    This was a group effort from [mciantyre] with [pull/200] and [Brooooooklyn] with [pull/197]. Thank you so
    much for your diligence and patience while we sorted through it.

    [mciantyre]: https://github.com/mciantyre
    [Brooooooklyn]: https://github.com/Brooooooklyn
    [issue/199]: https://github.com/rustwasm/wasm-pack/issues/199
    [pull/205]: https://github.com/rustwasm/wasm-pack/pull/205
    [pull/197]: https://github.com/rustwasm/wasm-pack/pull/197
    [pull/200]: https://github.com/rustwasm/wasm-pack/pull/200

- #### 🛠️ Maintenance

  - **clean up `quicli` remnants - [SoryRawyer], [pull/193]**

    In [v0.3.0] we removed the `quicli` dependency, however there were a few remnants
    left behind. They are now removed!

    [SoryRawyer]: https://github.com/SoryRawyer
    [pull/193]: https://github.com/rustwasm/wasm-pack/pull/193
    [v0.3.0]: https://github.com/rustwasm/wasm-pack/blob/master/CHANGELOG.md#-030

- #### 📖 Documentation

  - **DOCUMENT EVERYTHING!! and deny missing docs for all future development - [fitzgen], [pull/208]**

    The `wasm-pack` team has worked hard on tutorial documentation and keeping the codebase as self-explanatory
    as possible, but we have been slowly accruing a documentation debt. This amazing PR, landed just moments
    before this point release and was just too good not to include. Thank you so much, [fitzgen]!

    [fitzgen]: https://github.com/fitzgen
    [pull/208]: https://github.com/rustwasm/wasm-pack/pull/208

  - **fix README code example - [steveklabnik], [pull/195]**

    The code example in our `README.md` was missing a critical `pub`. It's there now!

    [pull/195]: https://github.com/rustwasm/wasm-pack/pull/195/files

  - **fix README markup - [Hywan], [pull/202]**

    There was an errant `` ` `` - it's gone now!

    [Hywan]: https://github.com/Hywan
    [pull/202]: https://github.com/rustwasm/wasm-pack/pull/202

## 🌟 0.4.0

This release has a ton of awesome things in it, but the best thing is that
almost all of this awesome work is brought to you by a **new** contributor
to `wasm-pack`. Welcome ya'll! We're so glad to have you!

### ✨ Features

- #### 🎏 New Flags

  - **`--mode` flag for skipping steps when calling `init` - [ashleygwilliams], [pull/186]**

    After teaching and working with `wasm-pack` for some time, it's clear that people would
    like the flexibility to run some of the steps included in the `init` command and not others.
    This release introduces a `--mode` flag that you can pass to `init`. The two modes currently
    available are `skip-build` and `no-installs` and they are explained below. In the future,
    we are looking to change the `init` interface, and potentially to split it into two commands.
    If you have thoughts or opinions on this, please weigh in on [issue/188]!

    [issue/188]: https://github.com/ashleygwilliams/wasm-pack/issues/188
    [pull/186]: https://github.com/ashleygwilliams/wasm-pack/pull/186

    - **`skip-build` mode - [kohensu], [pull/151]**

      ```
      wasm-pack init --mode skip-build
      ```

      Sometimes you want to run some of the shorter meta-data steps that
      `wasm-pack init` does for you without all the longer build steps. Now
      you can! Additionally, this PR was a fantastic refactor that allows even
      more custom build configurations will be simple to implement!

      [kohensu]: https://github.com/kohensu
      [pull/151]: https://github.com/ashleygwilliams/wasm-pack/pull/151

    - **`no-installs` mode - [ashleygwilliams], [pull/186]**

      ```
      wasm-pack init --mode no-installs
      ```

      Sometimes you want to run `wasm-pack` and not have it modify your global
      env by installing stuff! Or maybe you are just in a hurry and trust your
      env is set up correctly- now the `--mode no-install` option allows you to
      do this.

  - **`--debug` - [clanehin], [pull/127]**

    ```
    wasm-pack init --debug
    ```

    Find yourself needing to compile your Rust in `development` mode? You can now
    pass the `--debug` flag to do so! Thanks so much to [clanehin] for filing
    [issue/126] for this feature... and then implementing it!

    [pull/127]: https://github.com/ashleygwilliams/wasm-pack/pull/127
    [issue/126]: https://github.com/ashleygwilliams/wasm-pack/issues/126
    [clanehin]: https://github.com/clanehin

- #### ✅ New Checks

  - **ensure you have `cdylib` crate type - [kendromelon], [pull/150]**

    One of the biggest mistakes we've seen beginners make is forgetting to declare
    the `cdylib` crate type in their `Cargo.toml` before running `wasm-pack init`.
    This PR fixes that, and comes from someone who ran into this exact issue learning
    about `wasm-pack` at [JSConfEU]! Love when it works out like this.

    [JSConfEU]: https://2018.jsconf.eu/
    [kendromelon]: https://github.com/kedromelon
    [pull/150]: https://github.com/ashleygwilliams/wasm-pack/pull/150

  - **ensure you have declared wasm-bindgen as a dep - [robertohuertasm], [pull/162]**

    Another easy mistake to make is to forget to declare `wasm-bindgen` as a
    dependency in your `Cargo.toml`. Now `wasm-pack` will check and make sure you
    have it set before doing a bunch of long build steps :)

    [robertohuertasm]: https://github.com/robertohuertasm
    [pull/162]: https://github.com/ashleygwilliams/wasm-pack/pull/162

  - **ensure you are running `nightly` - [FreeMasen], [pull/172]**

    `wasm-pack` currently requires that you run it with `nightly` Rust. Now, `wasm-pack`
    will make sure you have `nightly` installed and will ensure that `cargo build` is run
    with `nightly`. Thanks so much to [FreeMasen] for filing [issue/171] and fixing it!

    [FreeMasen]: https://github.com/FreeMasen
    [issue/171]: https://github.com/ashleygwilliams/wasm-pack/issues/171
    [pull/172]: https://github.com/ashleygwilliams/wasm-pack/pull/172

### 🤕 Fixes

- **fixed broken progress bar spinner - [migerh], [pull/164]**

  Oh no! We broke the progress bar spinner in version 0.3.0. Thankfully, it's
  fixed now- with a thoughtful refactor that also makes the underlying code
  sounder overall.

[migerh]: https://github.com/migerh
[pull/164]: https://github.com/ashleygwilliams/wasm-pack/pull/164

### 🛠️ Maintenance

- **WIP bot - [ashleygwilliams] & [mgattozzi], [issue/170]**

  We've got a lot of work happening on `wasm-pack` so it's good to have a bit
  of protection from accidentally merging a Work In Progress. As a result, we
  now have the [WIP Github App] set up on `wasm-pack`. Great suggestion [mgattozzi]!

  [WIP Github App]: https://github.com/wip/app
  [issue/170]: https://github.com/ashleygwilliams/wasm-pack/issues/170

- **modularize `command.rs` - [ashleygwilliams], [pull/182]**

  Thanks to the growth of `wasm-pack`, `command.rs` was getting pretty long.
  We've broken it out into per command modules now, to help make it easier to
  read and maintain!

  [pull/182]: https://github.com/ashleygwilliams/wasm-pack/pull/182

- **improve PoisonError conversion - [migerh], [pull/187]**

  As part of the awesome progress bar spinner fix in [pull/164], [migerh] introduced
  a small concern with an `unwrap` due to an outstanding need to convert `PoisonError`
  into `wasm-pack`'s custom `Error`. Though not a critical concern, [migerh] mitigated
  this right away by replacing `std::sync::RwLock` with the [`parking_lot` crate]!
  This cleaned up the code even more than the previous patch!

  [`parking_lot` crate]: https://github.com/Amanieu/parking_lot
  [pull/187]: https://github.com/ashleygwilliams/wasm-pack/pull/187

- **wasm category for crates.io discovery- [TomasHubelbauer], [pull/149]**

  [crates.io] has [categories] to help folks discover crates, be we weren't
  leveraging it! Now- if you explore the [`wasm` category] on [crates.io]
  you'll see `wasm-pack`!

[crates.io]: https://crates.io/
[categories]: https://crates.io/categories
[`wasm` category]: https://crates.io/categories/wasm
[TomasHubelbauer]: https://github.com/TomasHubelbauer
[pull/149]: https://github.com/ashleygwilliams/wasm-pack/pull/149

- **human panic is now 1.0.0 - [spacekookie], [pull/156]**

  Congrats friends! We like what you do.

[pull/156]: https://github.com/ashleygwilliams/wasm-pack/pull/156
[spacekookie]: https://github.com/spacekookie

### 📖 Documentation

- **cleaned up the README - [ashleygwilliams], [pull/155]**

  Our `README` was struggling with a common problem- doing too much at once.
  More specifically, it wasn't clear who the audience was, contributers or
  end users? We've cleaned up our README and created a document specifically
  to help contributors get up and running.

[pull/155]: https://github.com/ashleygwilliams/wasm-pack/pull/155

## 🌠 0.3.1

Babby's first point release! Are we a real project now?

### 🤕 Fixes

- **fixed `init` `Is a Directory` error - [ashleygwilliams], [pull/139]**

  Our new logging feature accidentally introduced a regression into 0.3.0. When
  calling `wasm-pack init`, if a directory was not passed, a user would receive
  a "Is a Directory" Error. Sorry about that! Thanks to [jbolila] for filing
  [issue/136]!

[pull/139]: https://github.com/ashleygwilliams/wasm-pack/pull/139
[issue/136]: https://github.com/ashleygwilliams/wasm-pack/issues/136
[jbolila]: https://github.com/jbolila

- **typescript files were not included in published package - [danreeves], [pull/138]**

  Generating Typescript type files by default was a pretty rad feature in
  0.3.0 but we accidentally forgot to ensure they were included in the
  published package. Thanks so much to [danreeves] for catching this issue
  and fixing it for us!

[danreeves]: https://github.com/danreeves
[pull/138]: https://github.com/ashleygwilliams/wasm-pack/pull/138

## 💫 0.3.0

### ✨ Features

- **Logging - [mgattozzi], [pull/134]**

  Up until now, we've forced folks to rely on emoji-jammed console output to debug
  errors. While emojis are fun, this is often not the most pleasant experience. Now
  we'll generate a `wasm-pack.log` file if `wasm-pack` errors on you, and you can
  customize the log verbosity using the (previously unimplemented) verbosity flag.

[pull/134]: https://github.com/ashleygwilliams/wasm-pack/pull/134

- **`--target` flag - [djfarly], [pull/132]**

  `wasm-bindgen-cli` is able to generate a JS module wrapper for generated wasm files
  for both ES6 modules and CommonJS. Up until now, we only used wasm-bindgen's default
  behavior, ES6 modules. You can now pass a `--target` flag with either `nodejs` or
  `browser` to generate the type of module you want to use. Defaults to `browser` if not
  passed.

[djfarly]: https://github.com/djfarly
[pull/132]: https://github.com/ashleygwilliams/wasm-pack/pull/132

- **human readable panics - [yoshuawuyts], [pull/118]**

  Panics aren't always the most friendly situation ever. While we never want to panic on ya,
  if we do- we'll do it in a way that's a little more readable now.

[pull/118]: https://github.com/ashleygwilliams/wasm-pack/pull/118

- **typescript support by default - [kwonoj], [pull/109]**

  `wasm-bindgen` now generates typescript type files by default. To suppress generating
  the type file you can pass the `--no-typescript` flag. The type file is useful for more
  than just typescript folks- many IDEs use it for completion!

[kwonoj]: https://github.com/kwonoj
[pull/109]: https://github.com/ashleygwilliams/wasm-pack/pull/109

- **wrap `npm login` command - [djfarly], [pull/100]**

  In order to publish a package to npm, you need to be logged in. You can now use
  `wasm-pack login` to login to the npm (or any other) registry.

[pull/100]: https://github.com/ashleygwilliams/wasm-pack/pull/100

- **exit early on failure - [mgattozzi], [pull/90]**

  Until now, `wasm-pack` would continue to run tasks, even if a task failed. Now- if something
  fails, we'll exit so you don't have to wait to fix the error.

[pull/90]: https://github.com/ashleygwilliams/wasm-pack/pull/90

### 🤕 Fixes

- **force install wasm-bindgen - [ashleygwilliams], [pull/133]**

  Using an out of date version of `wasm-bindgen` can run you into a bunch of trouble. This
  very small change should fix the large number of bug reports we received from users using
  an out of date `wasm-bindgen-cli` by force installing `wasm-bindgen-cli` to ensure the user
  always has the latest version. We don't expect this to be a forever solution (it's a bit
  slow!) but it should help those who are getting started have a less rough time.

[pull/133]: https://github.com/ashleygwilliams/wasm-pack/pull/133

- **fix CI release builds - [ashleygwilliams], [pull/135]**

  This was not working! But now it is! You can always use `cargo install` to install
  wasm-pack, but now you can find pre-built Linux and Mac binaries in the [Releases]
  tab of our GitHub repo.

[Releases]: https://github.com/ashleygwilliams/wasm-pack/releases
[pull/135]: https://github.com/ashleygwilliams/wasm-pack/pull/135

### 🛠️ Maintenance

- **remove `quicli` dependency - [mgattozzi], [pull/131]**

  While `quicli` is a great way to get started writing a CLI app in Rust- it's not meant for
  large, mature applications. Now that `wasm-pack` is bigger and has many active users, we've
  removed this dependency to unblock further development on the tool.

[pull/131]: https://github.com/ashleygwilliams/wasm-pack/pull/131

- **update rustfmt CI test - [djfarly], [pull/128]**

  Since 0.2.0 how one should call `rustfmt` changed! We've kept it up to date so we can continue
  to maintain conventional style in the codebase.

[pull/128]: https://github.com/ashleygwilliams/wasm-pack/pull/128

- **custom module for errors - [mgattozzi], [pull/120]**

  Thanks to the `failure` crate, we've been playing fast and loose with errors for a bit. We're
  finally getting serious about error handling - by organizing all of our specific errors in a
  specific module. This will make it easier to communicate these errors out and handle new error
  cases from future features.

[pull/120]: https://github.com/ashleygwilliams/wasm-pack/pull/120

### 📖 Documentation

Special thanks to [data-pup] who continues to be our documentation champion! In case you missed it,
check out the guides in the [docs directory!](docs)!

## 🌌 0.2.0

This release focuses on filling out all commands and improving stderr/out
handling for improved user experience!

### ✨ Features

- **`pack` and `publish` - [jamiebuilds], [pull/67]**
  You can now run `wasm-pack pack` to generate a tarball of your generated package,
  as well as run `wasm-pack publish` to publish your package to the npm registry.
  Both commands require that you have npm installed, and the `publish` command requires
  that you be logged in to the npm client. We're working on wrapping the `npm login`
  command so that you can also login directly from `wasm-pack`, see [pull/100] for more
  details.

[jamiebuilds]: https://github.com/jamiebuilds
[pull/67]: https://github.com/ashleygwilliams/wasm-pack/pull/67
[pull/100]: https://github.com/ashleygwilliams/wasm-pack/pull/100

- **`package.json` is pretty printed now - [yoshuawuyts], [pull/70]**

  Previously, `package.json` was not very human readable. Now it is pretty printed!

- **`collaborators` - [yoshuawuyts], [pull/70]**

  `wasm-pack` now will fill out the `collaborators` field in your `package.json` for
  you based on your `Cargo.toml` `authors` data. For more discussion on how we decided
  on this v.s. other types of `author` fields in `package.json`, see [issues/2].

[yoshuawuyts]: https://github.com/yoshuawuyts
[pull/70]: https://github.com/ashleygwilliams/wasm-pack/pull/70
[issues/2]: https://github.com/ashleygwilliams/wasm-pack/issues/2

- **Release binaries built with CI - [ashleygwilliams], [pull/103]**

[ashleygwilliams]: https://github.com/ashleygwilliams
[pull/103]: https://github.com/ashleygwilliams/wasm-pack/pull/103

### 🤕 Fixes

- **Optional `package.json` fields warn instead of failing - [mgattozzi], [pull/65]**

[pull/65]: https://github.com/ashleygwilliams/wasm-pack/pull/65

- **Program doesn't swallow stout and sterr - [mgattozzi], [pull/90]**

[mgattozzi]: https://github.com/mgattozzi
[pull/90]: https://github.com/ashleygwilliams/wasm-pack/pull/90

### 🛠️ Maintenance and 📖 Documentation

Thanks so much to [mgattozzi], [data-pup], [sendilkumarn], [Andy-Bell],
[steveklabnik], [jasondavies], and [edsrzf] for all the awesome refactoring,
documentation, typo-fixing, and testing work. We appreciate it so much!

[data-pup]: https://github.com/data-pup
[sendilkumarn]: https://github.com/sendilkumarn
[Andy-Bell]: https://github.com/Andy-Bell
[steveklabnik]: https://github.com/steveklabnik
[jasondavies]: https://github.com/jasondavies
[edsrzf]: https://github.com/edsrzf

## 💥 0.1.0

- First release!
