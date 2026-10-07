use serde::Deserialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct Favorites {
    pub map: HashMap<char, PathBuf>,
}

impl Favorites {
    fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[derive(Deserialize)]
struct FavoritesFile {
    #[serde(default)]
    favorites: HashMap<String, PathBuf>,
}

const KEYMAP_FILE_NAME: &str = "keymap.toml";

const KEYMAP_TEMPLATE: &str = "\
# seshcat keymap
#
# Press `f` followed by a key to jump to a favorite directory.
# Use forward slashes on Windows, e.g. \"C:/Users/you/Documents\".

[favorites]
# a = \"C:/Users/you/Documents\"
";

pub fn config_dir() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("seshcat"))
}

pub fn keymap_path_in(dir: &Path) -> PathBuf {
    dir.join(KEYMAP_FILE_NAME)
}

pub fn ensure_keymap_file_in(dir: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(dir)?;

    let path = keymap_path_in(dir);

    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(mut file) => file.write_all(KEYMAP_TEMPLATE.as_bytes())?,
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(e),
    }

    Ok(path)
}

pub fn load_favorites() -> Favorites {
    try_load_favorites().unwrap_or_else(Favorites::empty)
}

fn try_load_favorites() -> Option<Favorites> {
    let seshcat_dir = config_dir()?;

    std::fs::create_dir_all(&seshcat_dir).ok()?;

    let favorites_path = keymap_path_in(&seshcat_dir);

    if !favorites_path.exists() {
        return Some(Favorites::empty());
    }

    let content = std::fs::read_to_string(&favorites_path).ok()?;

    let raw: FavoritesFile = toml::from_str(&content).ok()?;

    let map = raw
        .favorites
        .into_iter()
        .filter_map(|(k, v)| {
            let mut chars = k.chars();
            let ch = chars.next()?;

            if chars.next().is_some() {
                return None;
            }
            Some((ch, v))
        })
        .collect();

    Some(Favorites { map })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // App tests override `config_dir`, so this is the only guard that the
    // real path points at the app's own folder, not the bare OS config dir.
    #[test]
    fn config_dir_points_at_seshcat_folder() {
        if let Some(dir) = config_dir() {
            assert!(dir.ends_with("seshcat"), "unexpected config dir: {dir:?}");
        }
    }

    #[test]
    fn ensure_keymap_file_creates_file_when_missing() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let path = ensure_keymap_file_in(dir.path())?;

        assert_eq!(path, keymap_path_in(dir.path()));
        assert!(path.is_file(), "the keymap file should have been created");

        Ok(())
    }

    #[test]
    fn ensure_keymap_file_does_not_overwrite_existing_file()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let path = keymap_path_in(dir.path());
        std::fs::write(&path, "[favorites]\na = \"C:/mine\"\n")?;

        ensure_keymap_file_in(dir.path())?;

        let content = std::fs::read_to_string(&path)?;

        assert_eq!(
            content, "[favorites]\na = \"C:/mine\"\n",
            "ensure_keymap_file_in must never touch user content"
        );

        Ok(())
    }

    #[test]
    fn ensure_keymap_file_creates_missing_directories() -> Result<(), Box<dyn std::error::Error>> {
        let root = TempDir::new()?;
        let nested = root.path().join("does").join("not-exist");
        let path = ensure_keymap_file_in(&nested)?;

        assert!(path.is_file());

        Ok(())
    }

    #[test]
    fn generated_template_is_valid_toml_with_no_favorites() -> Result<(), Box<dyn std::error::Error>>
    {
        let dir = TempDir::new()?;
        let path = ensure_keymap_file_in(dir.path())?;
        let content = std::fs::read_to_string(&path)?;

        let parsed: FavoritesFile = toml::from_str(&content)?;

        assert!(parsed.favorites.is_empty());

        Ok(())
    }
}
