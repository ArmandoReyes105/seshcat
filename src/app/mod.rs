use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::config::{self, Favorites};
use crate::contracts::{FeatureOutcome, InputHandler};
use crate::features::open;
use crate::features::{navigation::NavigationState, rename::RenameState};

pub enum AppMode {
    Normal,
    Rename(RenameState),
    Leader,
}

pub struct App {
    pub navigation: NavigationState,
    pub mode: AppMode,
    pub favorites: Favorites,
}

impl App {
    pub fn new(start_path: PathBuf) -> std::io::Result<Self> {
        let state = NavigationState::new(&start_path)?;
        let favorites = config::load_favorites();

        Ok(Self {
            navigation: state,
            mode: AppMode::Normal,
            favorites,
        })
    }

    pub fn is_capturing_keys(&self) -> bool {
        !matches!(self.mode, AppMode::Normal)
    }

    pub fn handle_key(&mut self, key: KeyCode) -> std::io::Result<()> {
        match &mut self.mode {
            AppMode::Rename(state) => {
                let outcome = state.handle_key(key)?;
                self.handle_rename_outcome(outcome)?;
            }

            AppMode::Normal => match key {
                KeyCode::Char('o') => {
                    if let Some(entry) = self.navigation.selected_entry() {
                        open::open_file(&entry.path)?;
                    }
                }
                KeyCode::Char('O') => open::open_file(&self.navigation.current_path())?,
                KeyCode::Char('f') => self.mode = AppMode::Leader,
                KeyCode::Char('r') => self.start_rename(),
                _ => {
                    self.navigation.handle_key(key)?;
                }
            },

            AppMode::Leader => self.handle_leader_key(key),
        }

        Ok(())
    }

    fn handle_leader_key(&mut self, key: KeyCode) {
        self.mode = AppMode::Normal;

        if let KeyCode::Char(c) = key {
            if let Some(path) = self.favorites.map.get(&c).cloned() {
                let _ = self.navigation.go_to(&path);
            }
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
