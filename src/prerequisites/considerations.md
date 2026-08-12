# nodejs

Currently, `wasm-pack` generated npm modules require you to have the [fetch] polyfill in your node project.

If there is a module from `wasm-pack build --target nodejs` you may encounter some errors regarding global `Headers`, `Request`, `Response` and `fetch` Web APIs.

## Common errors:

```js
ReqwestError(reqwest::Error { kind: Builder, source: "JsValue(ReferenceError: Headers is not defined
ReqwestError(reqwest::Error { kind: Builder, source: "JsValue(ReferenceError: Request is not defined

    var ret = getObject(arg0) instanceof Response;
ReferenceError: Response is not defined
```

## Workaround
Import or declare fetch and objects: Headers, Request, Response

```ts
// CommonJS
const fetch = require('node-fetch');

// ES Module
import fetch from 'node-fetch';

// @ts-ignore
global.fetch = fetch;
// @ts-ignore
global.Headers = fetch.Headers;
// @ts-ignore
global.Request = fetch.Request;
// @ts-ignore
global.Response = fetch.Response;
```

[fetch]: https://github.com/node-fetch/node-fetch

