use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::{
    features::{Feature, FeatureOutcome, rename::RenameState},
    fs_entry::{self, FsEntry},
};

pub enum AppMode {
    Normal,
    Rename(RenameState),
}

pub struct App {
    pub current_path: PathBuf,
    pub entries: Vec<FsEntry>,
    pub selected: usize,
    pub mode: AppMode,
}

impl App {
    pub fn new(start_path: PathBuf) -> std::io::Result<Self> {
        let entries = fs_entry::list_dir(&start_path)?;

        Ok(Self {
            current_path: start_path,
            entries,
            selected: 0,
            mode: AppMode::Normal,
        })
    }

    pub fn is_capturing_keys(&self) -> bool {
        !matches!(self.mode, AppMode::Normal)
    }

    pub fn handle_key(&mut self, key: KeyCode) -> std::io::Result<()> {
        match &mut self.mode {
            AppMode::Rename(state) => match state.handle_key(key) {
                FeatureOutcome::Continue => {}
                FeatureOutcome::Reload => {
                    self.mode = AppMode::Normal;
                    self.reload_dir()?;
                }
                FeatureOutcome::Cancel => self.mode = AppMode::Normal,
            },

            AppMode::Normal => {
                self.handle_normal_key(key)?;
            }
        }

        Ok(())
    }

    fn handle_normal_key(&mut self, key: KeyCode) -> std::io::Result<()> {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('l') | KeyCode::Enter | KeyCode::Right => self.enter_selected()?,
            KeyCode::Char('h') | KeyCode::Left => self.go_to_parent()?,
            KeyCode::Char('r') => self.start_rename(),
            _ => {}
        }
        Ok(())
    }

    fn start_rename(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            self.mode = AppMode::Rename(RenameState::new(entry));
        }
    }

    fn reload_dir(&mut self) -> std::io::Result<()> {
        self.entries = fs_entry::list_dir(&self.current_path)?;

        if !self.entries.is_empty() && self.selected >= self.entries.len() {
            self.selected = self.entries.len().saturating_sub(1);
        }
        if self.entries.is_empty() {
            self.selected = 0;
        }
        Ok(())
    }

    fn move_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.selected = (self.selected + 1) % self.entries.len();
    }

    fn move_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.selected = (self.selected + self.entries.len() - 1) % self.entries.len();
    }

    fn enter_selected(&mut self) -> std::io::Result<()> {
        let Some(entry) = self.entries.get(self.selected) else {
            return Ok(());
        };

        if !entry.is_dir {
            return Ok(());
        }

        let next_path = entry.path.clone();
        self.entries = fs_entry::list_dir(&next_path)?;
        self.current_path = next_path;
        self.selected = 0;

        Ok(())
    }

    fn go_to_parent(&mut self) -> std::io::Result<()> {
        let Some(parent) = self.current_path.parent() else {
            return Ok(());
        };

        let parent_path = parent.to_path_buf();
        self.entries = fs_entry::list_dir(&parent_path)?;
        self.current_path = parent_path;
        self.selected = 0;

        Ok(())
    }
}
