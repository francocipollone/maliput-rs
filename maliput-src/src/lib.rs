use std::path::PathBuf;

/// Returns the path to the vendored include directory.
pub fn include() -> PathBuf {
    PathBuf::from(env!("INSTALL_DIR")).join("include")
}
