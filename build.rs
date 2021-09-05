use std::{env, fs, path::PathBuf};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "windows" | "linux" | "macos" => {},
        _ => panic!("Unknown target operating system {:?}.", target_os),
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir.clone());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = PathBuf::from(manifest_dir);

    match target_os.as_str() {
        "windows" => { fs::copy(manifest_path.join("lib/bass.dll"), out_path.join("bass.dll")).unwrap(); },
        "linux" => { fs::copy(manifest_path.join("lib/libbass.so"), out_path.join("libbass.so")).unwrap(); },
        "macos" => { fs::copy(manifest_path.join("lib/libbass.dylib"), out_path.join("libbass.dylib")).unwrap(); },
        _ => unreachable!(),
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=bass");
}
