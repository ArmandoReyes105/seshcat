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

    /// `true` when the current directory has no parent (filesystem root).
    /// Single source of truth reused by both the header and the
    /// parent-preview pane.
    pub fn is_at_root(&self) -> bool {
        is_filesystem_root(&self.current_path)
    }

    pub fn go_to(&mut self, path: &Path) -> std::io::Result<()> {
        let entries = filesystem::list_dir(path)?;

        self.current_path = path.to_path_buf();
        self.entries = entries;
        self.selected = 0;

        Ok(())
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

/// Note shown when the current directory has no parent (filesystem root).
/// Single shared source of truth so the header and the parent-preview
/// pane can never disagree or duplicate this indication.
pub const ROOT_NOTE: &str = "(raíz del filesystem - no hay padre)";

/// Returns `true` when `path` has no parent, i.e. it is a filesystem root
/// (e.g. `C:\` on Windows or `/` on Unix). Pure, no IO.
pub(crate) fn is_filesystem_root(path: &Path) -> bool {
    path.parent().is_none()
}

#[cfg(test)]
mod root_tests {
    use super::*;

    #[test]
    fn detects_filesystem_root() {
        #[cfg(windows)]
        let root = PathBuf::from("C:\\");
        #[cfg(not(windows))]
        let root = PathBuf::from("/");

        assert!(is_filesystem_root(&root));
    }

    #[test]
    fn detects_non_root_path() {
        #[cfg(windows)]
        let non_root = PathBuf::from("C:\\Users");
        #[cfg(not(windows))]
        let non_root = PathBuf::from("/home");

        assert!(!is_filesystem_root(&non_root));
    }
}
