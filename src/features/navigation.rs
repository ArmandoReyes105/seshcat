use std::path::{Path, PathBuf};

use crossterm::event::KeyCode;

use crate::{
    features::{Feature, FeatureOutcome},
    fs_entry::{self, FsEntry},
};

pub struct NavigationState {
    current_path: PathBuf,
    entries: Vec<FsEntry>,
    selected: usize,
}

impl NavigationState {
    pub fn new(start_path: &Path) -> std::io::Result<Self> {
        let start_entries = fs_entry::list_dir(&start_path)?;

        Ok(Self {
            current_path: start_path.to_path_buf(),
            entries: start_entries,
            selected: 0,
        })
    }

    pub fn reload_dir(&mut self) -> std::io::Result<()> {
        self.entries = fs_entry::list_dir(&self.current_path)?;

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

impl Feature for NavigationState {
    fn handle_key(&mut self, key: crossterm::event::KeyCode) -> std::io::Result<FeatureOutcome> {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('l') | KeyCode::Enter | KeyCode::Right => self.enter_selected()?,
            KeyCode::Char('h') | KeyCode::Left => self.go_to_parent()?,
            _ => {}
        }

        Ok(FeatureOutcome::Continue)
    }

    fn view(&self) -> String {
        String::new()
    }
}
