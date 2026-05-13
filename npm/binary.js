// Minimal install/run shim for the wasm-pack binary release tarball.
//
// Replaces the deprecated `binary-install` package and its old transitive
// deps (axios, rimraf@3, glob@7, inflight). Uses Node stdlib for HTTPS
// (follows redirects manually) plus the actively maintained `tar` package
// for extraction.

const fs = require("fs");
const os = require("os");
const path = require("path");
const https = require("https");
const { spawnSync } = require("child_process");
const tar = require("tar");

const WINDOWS_TARGET = "x86_64-pc-windows-msvc";

const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  // https://github.com/nodejs/node/blob/c3664227a83cf009e9a2e1ddeadbd09c14ae466f/deps/uv/src/win/util.c#L1566-L1573
  if ((type === "Windows_NT" || type.startsWith("MINGW32_NT-")) && arch === "x64") {
    return WINDOWS_TARGET;
  }
  if (type === "Linux" && arch === "x64") return "x86_64-unknown-linux-musl";
  if (type === "Linux" && arch === "arm64") return "aarch64-unknown-linux-musl";
  if (type === "Darwin" && arch === "x64") return "x86_64-apple-darwin";
  if (type === "Darwin" && arch === "arm64") return "aarch64-apple-darwin";

  throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getConfig = () => {
  const platform = getPlatform();
  const version = require("./package.json").version;
  const binaryName = platform === WINDOWS_TARGET ? "wasm-pack.exe" : "wasm-pack";
  const url = `https://github.com/wasm-bindgen/wasm-pack/releases/download/v${version}/wasm-pack-v${version}-${platform}.tar.gz`;
  const installDirectory = path.join(__dirname, "binary");
  return {
    binaryName,
    binaryPath: path.join(installDirectory, binaryName),
    installDirectory,
    url,
  };
};

// Follow up to a small number of redirects manually. GitHub release asset
// URLs redirect to S3, and `https.get` doesn't follow redirects on its own.
const httpsGetFollow = (url, maxRedirects = 5) => new Promise((resolve, reject) => {
  const attempt = (currentUrl, remaining) => {
    https.get(currentUrl, (res) => {
      const { statusCode, headers } = res;
      if (statusCode >= 300 && statusCode < 400 && headers.location) {
        if (remaining <= 0) {
          res.resume();
          return reject(new Error(`Too many redirects fetching ${url}`));
        }
        res.resume();
        const next = new URL(headers.location, currentUrl).toString();
        return attempt(next, remaining - 1);
      }
      if (statusCode !== 200) {
        res.resume();
        return reject(new Error(`Request failed with status code ${statusCode}`));
      }
      resolve(res);
    }).on("error", reject);
  };
  attempt(url, maxRedirects);
});

const downloadAndExtract = async (url, installDirectory) => {
  const stream = await httpsGetFollow(url);
  await new Promise((resolve, reject) => {
    stream
      .pipe(tar.x({ strip: 1, C: installDirectory }))
      .on("finish", resolve)
      .on("error", reject);
  });
};

const install = async () => {
  const { binaryPath, installDirectory, url } = getConfig();

  if (fs.existsSync(binaryPath)) {
    console.error("wasm-pack is already installed, skipping installation.");
    return;
  }

  fs.rmSync(installDirectory, { recursive: true, force: true });
  fs.mkdirSync(installDirectory, { recursive: true });

  console.error(`Downloading release from ${url}`);
  try {
    await downloadAndExtract(url, installDirectory);
  } catch (e) {
    console.error(`Error fetching release: ${e.message}`);
    process.exit(1);
  }
  console.error("wasm-pack has been installed!");
};

const run = async () => {
  const { binaryPath } = getConfig();

  if (!fs.existsSync(binaryPath)) {
    await install();
  }

  const args = process.argv.slice(2);
  const result = spawnSync(binaryPath, args, { cwd: process.cwd(), stdio: "inherit" });
  if (result.error) {
    console.error(result.error.message || result.error);
    process.exit(1);
  }
  process.exit(result.status ?? 1);
};

module.exports = { install, run };
