use crate::utils::fixture;
use binary_install::Cache;
use wasm_pack::test::webdriver;

#[test]
#[cfg(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "windows", target_arch = "x86"),
    all(target_os = "windows", target_arch = "x86_64")
))]
fn can_install_chromedriver() {
    let fixture = fixture::js_hello_world();
    let cache = Cache::at(&fixture.path);
    assert!(webdriver::install_chromedriver(&cache, true).is_ok());
}

#[test]
#[cfg(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "aarch64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "windows", target_arch = "x86"),
    all(target_os = "windows", target_arch = "x86_64")
))]
fn can_install_geckodriver() {
    let fixture = fixture::js_hello_world();
    let cache = Cache::at(&fixture.path);
    assert!(webdriver::install_geckodriver(&cache, true).is_ok());
}
