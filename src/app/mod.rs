use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::config::{self, Favorites};
use crate::contracts::{FeatureOutcome, InputHandler};
use crate::features::open;
use crate::features::{navigation::NavigationState, rename::RenameState};

use self::action::Action;

mod action;
mod keymap;

pub enum AppMode {
    Normal,
    Rename(RenameState),
    FavoritesLeader,
    GotoLeader,
}

pub struct App {
    pub navigation: NavigationState,
    pub mode: AppMode,
    pub favorites: Favorites,
    pub(crate) config_dir: Option<PathBuf>,
    pub status: Option<String>,
}

impl App {
    pub fn new(start_path: PathBuf) -> std::io::Result<Self> {
        let state = NavigationState::new(&start_path)?;
        let favorites = config::load_favorites();

        Ok(Self {
            navigation: state,
            mode: AppMode::Normal,
            favorites,
            config_dir: config::config_dir(),
            status: None,
        })
    }

    pub fn is_capturing_keys(&self) -> bool {
        !matches!(self.mode, AppMode::Normal)
    }

    pub fn handle_key(&mut self, key: KeyCode) -> std::io::Result<()> {
        self.status = None;

        let action = match &mut self.mode {
            AppMode::Rename(state) => {
                let outcome = state.handle_key(key)?;
                self.handle_rename_outcome(outcome)?;
                None
            }

            AppMode::FavoritesLeader => {
                self.handle_leader_key(key);
                None
            }

            AppMode::GotoLeader => {
                self.mode = AppMode::Normal;
                keymap::goto_action(key)
            }

            AppMode::Normal => {
                let action = keymap::normal_action(key);
                if action.is_none() {
                    self.navigation.handle_key(key)?;
                }
                action
            }
        };

        if let Some(action) = action {
            self.dispatch(action)?;
        }

        Ok(())
    }

    fn dispatch(&mut self, action: Action) -> std::io::Result<()> {
        match action {
            Action::OpenSelected => {
                if let Some(entry) = self.navigation.selected_entry() {
                    open::open_file(&entry.path)?;
                }
            }
            Action::OpenCurrentDir => open::open_file(self.navigation.current_path())?,
            Action::EnterFavoritesLeader => self.mode = AppMode::FavoritesLeader,
            Action::EnterGotoLeader => self.mode = AppMode::GotoLeader,
            Action::StartRename => self.start_rename(),
            Action::GoToConfig => self.go_to_config(),
        }

        Ok(())
    }

    fn go_to_config(&mut self) {
        let Some(dir) = self.config_dir.clone() else {
            self.status = Some("Config dir not available".to_string());
            return;
        };

        let keymap = match config::ensure_keymap_file_in(&dir) {
            Ok(path) => path,
            Err(e) => {
                self.status = Some(format!("Could not prepare keymap file: {e}"));
                return;
            }
        };

        if let Err(e) = self.navigation.go_to(&dir) {
            self.status = Some(format!("Could not open config dir: {e}"));
            return;
        }

        self.navigation.select_path(&keymap);
    }

    fn handle_leader_key(&mut self, key: KeyCode) {
        self.mode = AppMode::Normal;

        if let KeyCode::Char(c) = key
            && let Some(path) = self.favorites.map.get(&c).cloned()
        {
            let _ = self.navigation.go_to(&path);
        }
    }

    fn handle_rename_outcome(&mut self, outcome: FeatureOutcome) -> std::io::Result<()> {
        match outcome {
            FeatureOutcome::Continue => {}
            FeatureOutcome::Cancel => self.mode = AppMode::Normal,
            FeatureOutcome::Reload => {
                self.mode = AppMode::Normal;
                self.navigation.reload_dir()?;
            }
        }

        Ok(())
    }

    fn start_rename(&mut self) {
        if let Some(entry) = self.navigation.selected_entry() {
            self.mode = AppMode::Rename(RenameState::new(entry));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn g_then_c_goes_to_config_dir_and_selects_keymap() -> Result<(), Box<dyn std::error::Error>> {
        let start = TempDir::new()?;
        let cfg_root = TempDir::new()?;
        // Does not exist yet: also exercises create_dir_all.
        let cfg = cfg_root.path().join("seshcat");

        let mut app = App::new(start.path().to_path_buf())?;
        // Never touch the real %APPDATA% in tests.
        app.config_dir = Some(cfg.clone());

        app.handle_key(KeyCode::Char('g'))?;
        assert!(matches!(app.mode, AppMode::GotoLeader));

        app.handle_key(KeyCode::Char('c'))?;

        assert!(matches!(app.mode, AppMode::Normal));
        assert_eq!(app.navigation.current_path(), cfg);
        let selected = app
            .navigation
            .selected_entry()
            .expect("an entry is selected");
        assert_eq!(selected.name, "keymap.toml");

        Ok(())
    }

    #[test]
    fn g_then_unknown_key_returns_to_normal_without_moving()
    -> Result<(), Box<dyn std::error::Error>> {
        let start = TempDir::new()?;
        let cfg_root = TempDir::new()?;
        let cfg = cfg_root.path().join("seshcat");

        let mut app = App::new(start.path().to_path_buf())?;
        app.config_dir = Some(cfg.clone());

        app.handle_key(KeyCode::Char('g'))?;
        app.handle_key(KeyCode::Char('x'))?;

        assert!(matches!(app.mode, AppMode::Normal));
        assert_eq!(app.navigation.current_path(), start.path());
        assert!(!cfg.exists(), "a cancelled leader must not create anything");

        Ok(())
    }

    #[test]
    fn g_then_esc_cancels_the_leader() -> Result<(), Box<dyn std::error::Error>> {
        let start = TempDir::new()?;
        let mut app = App::new(start.path().to_path_buf())?;

        app.handle_key(KeyCode::Char('g'))?;
        app.handle_key(KeyCode::Esc)?;

        assert!(matches!(app.mode, AppMode::Normal));

        Ok(())
    }

    #[test]
    fn go_to_config_without_config_dir_sets_status_and_does_not_error()
    -> Result<(), Box<dyn std::error::Error>> {
        let start = TempDir::new()?;
        let mut app = App::new(start.path().to_path_buf())?;
        app.config_dir = None;

        app.handle_key(KeyCode::Char('g'))?;
        // The assertion that matters: this must be Ok, not an Err.
        app.handle_key(KeyCode::Char('c'))?;

        assert!(app.status.is_some(), "the user must be told what happened");
        assert!(matches!(app.mode, AppMode::Normal));
        assert_eq!(app.navigation.current_path(), start.path());

        Ok(())
    }

    #[test]
    fn status_is_cleared_on_the_next_key() -> Result<(), Box<dyn std::error::Error>> {
        let start = TempDir::new()?;
        let mut app = App::new(start.path().to_path_buf())?;
        app.config_dir = None;
        app.handle_key(KeyCode::Char('g'))?;
        app.handle_key(KeyCode::Char('c'))?;
        assert!(app.status.is_some());

        app.handle_key(KeyCode::Char('j'))?;

        assert!(app.status.is_none());

        Ok(())
    }
}
