fn main() {
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_lib_dir = format!("{}/mylibrary/lib", cargo_dir);
    println!("cargo:rustc-link-search={}", cargo_lib_dir);
}