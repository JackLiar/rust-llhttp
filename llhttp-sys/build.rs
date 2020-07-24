extern crate bindgen;

fn main() {
    let llhttp_bindings = bindgen::Builder::default()
        .header("src/llhttp.h")
        .rust_target(bindgen::LATEST_STABLE_RUST)
        .generate()
        .expect("Unable to generate llhttp bindings");
    llhttp_bindings
        .write_to_file("src/raw.rs")
        .expect("Unable to generate llhttp bindings");

    #[cfg(feature = "static")]
    println!("cargo:rustc-link-lib=static=llhttp");

    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=llhttp");
}
