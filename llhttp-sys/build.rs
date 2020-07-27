use std::env;
use std::path::{Path, PathBuf};

extern crate bindgen;

use anyhow::{anyhow, bail, Context, Result};

fn find_llhttp() -> Result<PathBuf> {
    println!("cargo:rerun-if-env-changed=LLHTTP_ROOT");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=LD_LIBRARY_PATH");

    let link_kind = if cfg!(feature = "static") {
        "static"
    } else {
        "dylib"
    };

    if let Ok(prefix) = env::var("LLHTTP_ROOT") {
        let prefix = Path::new(&prefix);
        if !prefix.exists() || !prefix.is_dir() {
            bail!("LLHTTP_ROOT should point to a directory that exists.");
        }

        let inc_path = prefix.join("include");
        let link_path = prefix.join("lib");
        if link_path.exists() && link_path.is_dir() {
            println!(
                "cargo:rustc-link-search=native={}",
                link_path.to_string_lossy()
            );
        } else {
            bail!("`$LLHTTP_ROOT/lib` subdirectory not found.");
        }

        let mut link_libs = vec![];
        link_libs.push(format!("{}=llhttp", link_kind));

        println!(
            "cargo:warning=building with llhttp using environment variable with {} library @ {:?}, libs={:?}, link_paths=[{:?}], include_paths=[{:?}]",
            link_kind,
            prefix,
            link_libs,
            link_path,
            inc_path
        );

        for lib in link_libs {
            println!("cargo:rustc-link-lib={}", lib);
        }

        Ok(inc_path)
    } else {
        let libllhttp = pkg_config::Config::new()
            .statik(cfg!(feature = "static"))
            .cargo_metadata(true)
            .env_metadata(true)
            .probe("llhttp")?;

        println!(
            "cargo:warning=building with llhttp using pkgconfig {} with {} library, libs={:?}, link_paths={:?}, include_paths={:?}",
            libllhttp.version, link_kind, libllhttp.libs, libllhttp.link_paths, libllhttp.include_paths
        );

        libllhttp
            .include_paths
            .first()
            .cloned()
            .ok_or_else(|| anyhow!("missing include path"))
    }
}

fn main() -> Result<()> {
    let _ = find_llhttp().with_context(|| {
        anyhow!("please download and install llhttp from https://github.com/nodejs/llhttp or https://github.com/JackLiar/llhttp-cmake")
    })?;

    let llhttp_bindings = bindgen::Builder::default().header("src/llhttp.h");

    #[cfg(target_os = "macos")]
    let llhttp_bindings = llhttp_bindings
        .blacklist_type("^__darwin_.*")
        .blacklist_type("^_opaque_.*");

    let llhttp_bindings = llhttp_bindings
        .size_t_is_usize(true)
        .rust_target(bindgen::LATEST_STABLE_RUST)
        .generate()
        .expect("Unable to generate llhttp bindings");

    llhttp_bindings
        .write_to_file("src/raw.rs")
        .expect("Unable to generate llhttp bindings");

    Ok(())
}
