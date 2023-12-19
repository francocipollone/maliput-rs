use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/lib.rs");

    let maliput_src_root = PathBuf::from(env::var("DEP_MALIPUT_SRC_ROOT").unwrap());
    println!("maliput_src_root={}", maliput_src_root.display());

    let include_paths = [
        maliput_src_root.join("include"),
    ];

    println!("cargo:rustc-link-search=native={}/lib", maliput_src_root.display());
    // TODO(francocipollone): Extend to all the libraries.
    println!("cargo:rustc-link-lib=maliput_math");

    println!("include_paths={}", include_paths[0].display());

    autocxx_build::Builder::new("src/lib.rs", &include_paths)
        .build()?
        .flag_if_supported("-std=c++17")
        .compile("maliput-sys");

    Ok(())
}
