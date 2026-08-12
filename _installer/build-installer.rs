use std::fs;

fn main() {
    fs::create_dir_all("docs/installer").unwrap();
    fs::copy(
        "docs/_installer/wasm-pack.js",
        "docs/installer/wasm-pack.js",
    ).unwrap();
    let index = fs::read_to_string("docs/_installer/index.html").unwrap();
    fs::write(
        "docs/installer/index.html",
        fixup(&index),
    ).unwrap();

    let init = fs::read_to_string("docs/_installer/init.sh").unwrap();
    fs::write(
        "docs/installer/init.sh",
        fixup(&init),
    ).unwrap();
}

fn fixup(input: &str) -> String {
    let manifest = fs::read_to_string("Cargo.toml").unwrap();
    let version = manifest.lines()
        .find(|line| line.starts_with("version ="))
        .unwrap();
    let version = &version[version.find('"').unwrap() + 1..version.rfind('"').unwrap()];

    input.replace("@@WASM_PACK_VERSION@@", &format!("v{}", version))
}
