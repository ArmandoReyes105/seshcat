use std::path::PathBuf;

pub struct FsEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}
