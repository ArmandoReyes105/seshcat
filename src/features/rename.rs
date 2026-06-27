use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::{
    features::{Feature, FeatureOutcome},
    fs_entry::{self, FsEntry},
};

pub struct RenameState {
    target_path: PathBuf,
    pub buffer: String,
    pub error: Option<String>,
}

impl RenameState {
    pub fn new(target: &FsEntry) -> Self {
        Self {
            target_path: target.path.clone(),
            buffer: target.name.clone(),
            error: None,
        }
    }
}

impl Feature for RenameState {
    fn handle_key(&mut self, key: KeyCode) -> FeatureOutcome {
        match key {
            KeyCode::Esc => FeatureOutcome::Cancel,
            KeyCode::Enter => self.confirm(),
            KeyCode::Backspace => {
                self.buffer.pop();
                self.error = None;
                FeatureOutcome::Continue
            }
            KeyCode::Char(c) => {
                self.buffer.push(c);
                self.error = None;
                FeatureOutcome::Continue
            }
            _ => FeatureOutcome::Continue,
        }
    }

    fn view(&self) -> String {
        let prompt = format!("renombrar: {}_", self.buffer);

        match &self.error {
            Some(err) => format!("{} | {}", prompt, err),
            None => prompt,
        }
    }
}

impl RenameState {
    fn confirm(&mut self) -> FeatureOutcome {
        match fs_entry::rename(&self.target_path, &self.buffer) {
            Ok(()) => FeatureOutcome::Reaload,
            Err(e) => {
                self.error = Some(e.to_string());
                FeatureOutcome::Continue
            }
        }
    }
}
