use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::fs_entry::{self, FsEntry};

pub struct App {
    pub current_path: PathBuf,
    pub entries: Vec<FsEntry>,
    pub selected: usize,
}

impl App {
    pub fn new(start_path: PathBuf) -> std::io::Result<Self> {
        let entries = fs_entry::list_dir(&start_path)?;

        Ok(Self {
            current_path: start_path,
            entries,
            selected: 0,
        })
    }

    pub fn handle_key(&mut self, key: KeyCode) -> std::io::Result<()> {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('l') | KeyCode::Enter => self.enter_selected()?,
            KeyCode::Char('h') => self.go_to_parent()?,
            _ => {}
        }

        Ok(())
    }

    fn move_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected < self.entries.len() - 1 {
            self.selected += 1;
        }
    }

    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
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
