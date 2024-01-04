use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/create_road_network_wrapper.h");

    let maliput_src_root = PathBuf::from(env::var("DEP_MALIPUT_SRC_ROOT").unwrap());
    println!("maliput_src_root={}", maliput_src_root.display());

    println!("cargo:rustc-link-search=native={}/bazel-bin", maliput_src_root.display());

    // Link to all the libraries in the bazel-bin folder.
    println!("cargo:rustc-link-lib=drake");
    println!("cargo:rustc-link-lib=geometry_base");
    println!("cargo:rustc-link-lib=plugin");
    println!("cargo:rustc-link-lib=common");
    println!("cargo:rustc-link-lib=utility");
    println!("cargo:rustc-link-lib=math");
    println!("cargo:rustc-link-lib=api");

    cxx_build::bridges(["src/api.rs", "src/plugin.rs"])
        .flag_if_supported("-std=c++17")
        .include("src")
        .compile("maliput-sys");

    Ok(())
}
