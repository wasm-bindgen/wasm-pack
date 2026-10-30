use wasm_pack::install::{self, Arch, Os, Tool};

#[test]
#[cfg(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(windows, target_arch = "x86_64"),
))]
fn can_download_prebuilt_wasm_bindgen() {
    let dir = tempfile::TempDir::new().unwrap();
    let cache = binary_install::Cache::at(dir.path());
    if let install::Status::Found(dl) =
        install::download_prebuilt(&Tool::WasmBindgen, &cache, "0.2.100", true).unwrap()
    {
        assert!(dl.binary("wasm-bindgen").unwrap().is_file());
        assert!(dl.binary("wasm-bindgen-test-runner").unwrap().is_file())
    } else {
        assert!(false, "Download failed")
    }
}

#[test]
#[cfg(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(windows, target_arch = "x86_64"),
))]
fn downloading_prebuilt_wasm_bindgen_handles_http_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let bad_version = "0.2.95-some-trailing-version-stuff-that-does-not-exist";
    let cache = binary_install::Cache::at(dir.path());
    let result = install::download_prebuilt(&Tool::WasmBindgen, &cache, bad_version, true);
    assert!(result.is_err());
    let error = result.err().unwrap();

    assert!(error.chain().any(|e| e.to_string().contains("404")));
    assert!(error.chain().any(|e| e.to_string().contains(bad_version)));
}

#[test]
#[cfg(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(windows, target_arch = "x86_64"),
))]
fn can_download_prebuilt_cargo_generate() {
    let dir = tempfile::TempDir::new().unwrap();
    let cache = binary_install::Cache::at(dir.path());
    if let install::Status::Found(dl) =
        install::download_prebuilt(&Tool::CargoGenerate, &cache, "latest", true).unwrap()
    {
        assert!(dl.binary("cargo-generate").unwrap().is_file());
    } else {
        assert!(false, "Download Failed");
    }
}

#[test]
#[cfg(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(windows, target_arch = "x86_64"),
    all(windows, target_arch = "aarch64"),
))]
fn can_download_prebuilt_wasm_opt() {
    let dir = tempfile::TempDir::new().unwrap();
    let cache = binary_install::Cache::at(dir.path());
    if let install::Status::Found(dl) =
        install::download_prebuilt(&Tool::WasmOpt, &cache, "latest", true).unwrap()
    {
        assert!(dl.binary("bin/wasm-opt").unwrap().is_file());
    } else {
        assert!(false, "Download Failed");
    }
}

#[test]
fn wasm_opt_prebuilt_url_is_pinned_version() {
    // Ensure the bundled binaryen version is recent enough to handle
    // 64-bit tables (added in v118). See
    // https://github.com/wasm-bindgen/wasm-pack/issues/1576.
    let url =
        install::prebuilt_url_for(&Tool::WasmOpt, "latest", &Arch::X86_64, &Os::Linux).unwrap();
    let version = url
        .split('/')
        .find(|segment| segment.starts_with("version_"))
        .and_then(|s| s.trim_start_matches("version_").parse::<u32>().ok())
        .expect("binaryen prebuilt URL must contain a version_NNN tag");
    assert!(
        version >= 118,
        "bundled binaryen must be >= 118 for wasm64 support, got version_{}",
        version
    );
}

#[test]
fn all_latest_tool_download_urls_valid() {
    let mut errors = Vec::new();

    for (tool, version) in [
        (Tool::CargoGenerate, "latest"),
        (Tool::WasmBindgen, "0.2.92"),
        (Tool::WasmBindgen, "0.2.129"),
        (Tool::WasmOpt, "latest"),
    ] {
        for arch in [Arch::X86_64, Arch::X86, Arch::AArch64, Arch::Riscv64] {
            for os in [Os::Linux, Os::MacOS, Os::Windows] {
                // For all valid tool, arch & os combinations,
                // error out when any of them is a 404 or similar
                if let Ok(url) = install::prebuilt_url_for(&tool, version, &arch, &os) {
                    // Use HTTP HEAD instead of GET to avoid fetching lots of stuff
                    let res = ureq::head(&url).call().unwrap();
                    let status = res.status();
                    if 500 > status && status >= 400 {
                        errors.push(format!(
                            "Can't download URL {} for {} on {}: {}",
                            url,
                            arch,
                            os,
                            res.status()
                        ));
                    }
                }
            }
        }
    }
    if !errors.is_empty() {
        panic!(
            "Some URLs for prebuild tools were unavailable:\n{}",
            errors.join("\n")
        );
    }
}

#[test]
fn wasm_bindgen_prebuilt_targets_follow_release_artifacts() {
    let target = |version, arch, os| {
        install::prebuilt_url_for(&Tool::WasmBindgen, version, &arch, &os)
            .ok()
            .map(|url| {
                url.trim_end_matches(".tar.gz")
                    .rsplit(&format!("{}-", version))
                    .next()
                    .unwrap()
                    .to_string()
            })
    };
    let cases = [
        (
            "0.2.103",
            Arch::AArch64,
            Os::Linux,
            Some("aarch64-unknown-linux-gnu"),
        ),
        (
            "0.2.104",
            Arch::AArch64,
            Os::Linux,
            Some("aarch64-unknown-linux-musl"),
        ),
        ("0.2.126", Arch::Riscv64, Os::Linux, None),
        (
            "0.2.127",
            Arch::Riscv64,
            Os::Linux,
            Some("riscv64gc-unknown-linux-gnu"),
        ),
        (
            "0.2.129",
            Arch::AArch64,
            Os::Windows,
            Some("x86_64-pc-windows-msvc"),
        ),
    ];
    for (version, arch, os, expected) in cases {
        assert_eq!(
            target(version, arch.clone(), os.clone()).as_deref(),
            expected,
            "wasm-bindgen {} on {} {}",
            version,
            os,
            arch
        );
    }
}
