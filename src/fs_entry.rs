use std::fs;
use std::path::{Path, PathBuf};

pub struct FsEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

pub fn list_dir(path: &Path) -> std::io::Result<Vec<FsEntry>> {
    let entries = fs::read_dir(path)?;
    let dir_entries: Vec<fs::DirEntry> = entries.collect::<std::io::Result<Vec<_>>>()?;

    let fs_entries: Vec<FsEntry> = dir_entries
        .iter()
        .map(|entry| {
            let path = entry.path();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.to_string_lossy().into_owned());
            let is_dir = path.is_dir();

            FsEntry { name, path, is_dir }
        })
        .collect();

    Ok(fs_entries)
}
