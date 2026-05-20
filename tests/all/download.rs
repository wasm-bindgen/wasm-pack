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
fn all_latest_tool_download_urls_valid() {
    let mut errors = Vec::new();

    for tool in [Tool::CargoGenerate, Tool::WasmBindgen, Tool::WasmOpt] {
        for arch in [Arch::X86_64, Arch::X86, Arch::AArch64] {
            for os in [Os::Linux, Os::MacOS, Os::Windows] {
                // For all valid tool, arch & os combinations,
                // error out when any of them is a 404 or similar
                if let Ok(url) = install::prebuilt_url_for(&tool, "0.2.92", &arch, &os) {
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
