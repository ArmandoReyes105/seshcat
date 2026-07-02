use std::path::Path;

use crate::services::filesystem;

pub fn open_file(path: &Path) -> std::io::Result<()> {
    filesystem::open_file(path)?;
    Ok(())
}
