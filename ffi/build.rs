fn main() {
    let lib_name = "mylibrary";
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_lib_dir = format!("{}/{}/lib", cargo_dir, lib_name);
    println!("cargo:rustc-link-search={}", cargo_lib_dir);
    println!("cargo:rustc-link-lib=static={}", lib_name);
}
