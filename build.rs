fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-lib=lib/bass");
    }

    #[cfg(not(windows))]
    {
        println!("cargo:rustc-link-search=./lib");
        println!("cargo:rustc-link-lib=bass");
    }
}
