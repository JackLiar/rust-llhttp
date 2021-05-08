use std::env;
use std::path::{Path, PathBuf};

#[cfg(feature = "gen")]
extern crate bindgen;

use anyhow::{anyhow, bail, Context, Error, Result};

fn find_llhttp() -> Result<PathBuf> {
    println!("cargo:rerun-if-env-changed=LLHTTP_ROOT");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=LD_LIBRARY_PATH");

    let link_kind = if cfg!(feature = "static") { "static" } else { "dylib" };

    if let Ok(prefix) = env::var("LLHTTP_ROOT") {
        let prefix = Path::new(&prefix);
        if !prefix.exists() || !prefix.is_dir() {
            bail!("LLHTTP_ROOT should point to a directory that exists.");
        }

        let inc_path = prefix.join("include");
        let link_path = prefix.join("lib");
        if link_path.exists() && link_path.is_dir() {
            println!("cargo:rustc-link-search=native={}", link_path.to_string_lossy());
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

#[cfg(feature = "gen")]
fn generate_binding(inc_dir: &Path, out_dir: &Path) -> Result<()> {
    let out_file = out_dir.join("llhttp.rs");
    let inc_file = inc_dir.join("llhttp.h");
    let inc_file = inc_file.to_str().expect("header file");

    println!(
        "cargo:warning=generating raw llhttp binding file @ {} from {}",
        out_file.display(),
        inc_file,
    );

    println!("cargo:rerun-if-changed={}", inc_file);

    let llhttp_bindings = bindgen::Builder::default().header(inc_file);

    #[cfg(target_os = "macos")]
    let llhttp_bindings = llhttp_bindings
        .blocklist_type("^__darwin_.*")
        .blocklist_type("^_opaque_.*");

    llhttp_bindings
        .use_core()
        .ctypes_prefix("::libc")
        .allowlist_var("^llhttp_.*")
        .allowlist_type("^llhttp_.*")
        .allowlist_function("^llhttp_.*")
        .size_t_is_usize(true)
        .rust_target(bindgen::LATEST_STABLE_RUST)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .newtype_enum("llhttp_errno")
        .newtype_enum("llhttp_flags")
        .newtype_enum("llhttp_lenient_flags")
        .newtype_enum("llhttp_type")
        .newtype_enum("llhttp_method")
        .generate()
        .map_err(|_| Error::msg("generate binding files"))?
        .write_to_file(out_file)
        .with_context(|| "write wrapper")?;

    Ok(())
}

#[cfg(not(feature = "gen"))]
fn generate_binding(_: &Path, out_dir: &Path) -> Result<()> {
    std::fs::copy("src/llhttp.rs", out_dir.join("llhttp.rs"))
        .map(|_| ())
        .with_context(|| "copy binding file")
}

fn main() -> Result<()> {
    let inc_dir = find_llhttp().with_context(|| {
        anyhow!("please download and install llhttp from https://github.com/nodejs/llhttp or https://github.com/JackLiar/llhttp-cmake")
    })?;
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    generate_binding(&inc_dir, &out_dir)?;

    Ok(())
}
