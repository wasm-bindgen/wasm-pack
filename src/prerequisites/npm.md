# npm

Currently, `wasm-pack` requires that you have npm installed to pack and publish your
package. Longterm, this will be replaced by a Rust only version.

If you would rather use another package manager that interfaces with the npm registry
you may, however, the `pack`, `publish`, and `login` commands wrap the npm CLI interface
and as a result require that npm be installed.

You can install [npm] by following [these instructions][npm-install-info].

[npm]: https://www.npmjs.com

### npm Account

Part of the `wasm-pack` workflow is to publish your package to the npm Registry.

Regardless of which package manager CLI tool you prefer, if you wish to publish
your package to the npm registry you'll need an npm account.

You can find information about signing up for npm [here][npm-signup-info].

[`npm link`]: https://docs.npmjs.com/cli/link
[npm-install-info]: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
[npm-signup-info]: https://www.npmjs.com/signup
