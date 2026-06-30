use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::features::{Feature, FeatureOutcome, navigation::NavigationState, rename::RenameState};

pub enum AppMode {
    Normal,
    Rename(RenameState),
}

pub struct App {
    pub navigation: NavigationState,
    pub mode: AppMode,
}

impl App {
    pub fn new(start_path: PathBuf) -> std::io::Result<Self> {
        let state = NavigationState::new(&start_path)?;

        Ok(Self {
            navigation: state,
            mode: AppMode::Normal,
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
                KeyCode::Char('r') => self.start_rename(),
                _ => {
                    self.navigation.handle_key(key)?;
                }
            },
        }

        Ok(())
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
