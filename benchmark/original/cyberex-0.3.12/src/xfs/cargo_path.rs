use std::path::PathBuf;

pub fn get_project_root_path() -> Result<PathBuf, std::env::VarError> {
    std::env::var("CARGO_MANIFEST_DIR").map(|pa| PathBuf::from(&pa))
}
