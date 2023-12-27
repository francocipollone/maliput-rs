use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const LIB_NAME: &str = "maliput";
const MALIPUT_PATH: &str = "dep/maliput";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/lib.rs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let install_dir = out_dir.join("install");
    fs::create_dir_all(&install_dir)?;

    let original_dir = env::current_dir().unwrap();

    println!("cargo:root={}", install_dir.display());

    println!("cargo:rustc-link-lib=dylib={}", "stdc++");
    println!("cargo:rustc-link-arg=-l{}", "stdc++");
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());

    // build lib
    env::set_current_dir(MALIPUT_PATH)
        .unwrap_or_else(|_| panic!("Unable to change directory to {}", MALIPUT_PATH));
    _ = std::fs::remove_dir_all("build");
    _ = std::fs::create_dir("build");
    env::set_current_dir("build")
        .unwrap_or_else(|_| panic!("Unable to change directory to {} build", LIB_NAME));

    // TODO(francocipollone): Try to te place custom build script with cmake crate
    let code = std::process::Command::new("cmake")
        .arg("..")
        // .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", install_dir.display()))
        .arg("-DBUILD_TESTING=OFF")
        .status()
        .expect("Failed to generate build script");
    if code.code() != Some(0) {
        panic!("Failed to generate build script");
    }

    let code = std::process::Command::new("cmake")
        .arg("--build")
        .arg(".")
        .status()
        .expect("Failed to build lib");
    if code.code() != Some(0) {
        panic!("Failed to build lib");
    }

    let code = std::process::Command::new("cmake")
        .arg("--install")
        .arg(".")
        .status()
        .expect("Failed to install lib");
    if code.code() != Some(0) {
        panic!("Failed to install lib");
    }

    env::set_current_dir(original_dir)
        .unwrap_or_else(|_| panic!("Unable to change directory to original dir"));



    println!("cargo:rustc-env=INSTALL_DIR={}", install_dir.display());
    println!("cargo:CXXBRIDGE_DIR0={}/include", install_dir.display());

    Ok(())
}
