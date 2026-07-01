use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

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

pub fn load_favorites() -> Favorites {
    try_load_favorites().unwrap_or_else(Favorites::empty)
}

fn try_load_favorites() -> Option<Favorites> {
    let config_dir = dirs::config_dir()?;
    let seshcat_dir = config_dir.join("seshcat");

    std::fs::create_dir_all(&seshcat_dir).ok()?;

    let favorites_path = seshcat_dir.join("keymap.toml");

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
