extern crate bindgen;

use std::{env, path::PathBuf};

const LIB_NAME: &str = "maliput";
const MALIPUT_PATH: &str = "dep/maliput";

fn main() {

    let original_dir = env::current_dir().unwrap();

    println!("cargo:rustc-link-lib=dylib={}", "stdc++");
    println!("cargo:rustc-link-arg=-l{}", "stdc++");
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=maliput_wrapper.h");

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
        .arg("-DCMAKE_INSTALL_PREFIX=install")
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

    let out_dir = env::var("OUT_DIR").unwrap();

  // move maliput/math/libmaliput_math.so to where Cargo expects it (OUT_DIR)
    std::fs::copy(
      "install/lib/libmaliput_math.so",
      format!("{}/libmaliput_math.so", out_dir),
  )
  .expect("Failed to copy math so");


  println!("cargo:rustc-link-lib=maliput_math");

  env::set_current_dir(original_dir)
    .unwrap_or_else(|_| panic!("Unable to change directory to original dir"));

  let bindings = bindgen::Builder::default()
    .header("maliput_wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .clang_args(&["-x", "c++"])
    .clang_arg(format!("-I./dep/maliput/include").as_str())
    .generate()
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");

}
