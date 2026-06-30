mod input;
mod view;

use std::path::{Path, PathBuf};

use crate::models::FsEntry;
use crate::services::filesystem;

pub struct NavigationState {
    current_path: PathBuf,
    entries: Vec<FsEntry>,
    selected: usize,
}

impl NavigationState {
    pub fn new(start_path: &Path) -> std::io::Result<Self> {
        let start_entries = filesystem::list_dir(&start_path)?;

        Ok(Self {
            current_path: start_path.to_path_buf(),
            entries: start_entries,
            selected: 0,
        })
    }

    pub fn reload_dir(&mut self) -> std::io::Result<()> {
        self.entries = filesystem::list_dir(&self.current_path)?;

        if !self.entries.is_empty() && self.selected >= self.entries.len() {
            self.selected = self.entries.len().saturating_sub(1);
        }
        if self.entries.is_empty() {
            self.selected = 0;
        }
        Ok(())
    }

    pub fn current_path(&self) -> &Path {
        &self.current_path
    }

    pub fn entries(&self) -> &[FsEntry] {
        &self.entries
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn selected_entry(&self) -> Option<&FsEntry> {
        self.entries.get(self.selected)
    }
}

impl NavigationState {
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
        self.entries = filesystem::list_dir(&next_path)?;
        self.current_path = next_path;
        self.selected = 0;

        Ok(())
    }

    fn go_to_parent(&mut self) -> std::io::Result<()> {
        let Some(parent) = self.current_path.parent() else {
            return Ok(());
        };

        let parent_path = parent.to_path_buf();
        self.entries = filesystem::list_dir(&parent_path)?;
        self.current_path = parent_path;
        self.selected = 0;

        Ok(())
    }
}
