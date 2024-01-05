use std::env;
use std::path::PathBuf;

// Return the path of the installed library.
pub fn get_plugin_dir() -> PathBuf {
    let install_dir = env::var("INSTALL_DIR").expect("INSTALL_DIR not set");
    let plugin_dir = PathBuf::from(install_dir).join("maliput_plugins").join("libmaliput_malidrive_road_network.so.runfiles").join("_main").join("maliput_plugins");
    PathBuf::from(plugin_dir)
}

fn main() {
    println!("{}", get_plugin_dir().to_str().unwrap());
}
