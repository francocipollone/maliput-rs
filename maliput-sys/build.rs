use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/create_road_network_wrapper.h");

    let maliput_src_root = PathBuf::from(env::var("DEP_MALIPUT_SRC_ROOT").unwrap());
    println!("maliput_src_root={}", maliput_src_root.display());

    let include_paths = [
        maliput_src_root.join("include"),
        "src".into(),
    ];

    println!("cargo:rustc-link-search=native={}/lib", maliput_src_root.display());
    // TODO(francocipollone): Extend to all the libraries.
    println!("cargo:rustc-link-lib=maliput_api");
    println!("cargo:rustc-link-lib=maliput_common");
    println!("cargo:rustc-link-lib=maliput_math");
    println!("cargo:rustc-link-lib=maliput_plugin");

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .include(&include_paths[0])
        .include(&include_paths[1])
        .compile("maliput-sys");

    Ok(())
}
