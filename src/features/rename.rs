use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::{
    features::{Feature, FeatureOutcome},
    fs_entry::{self, FsEntry},
    widgets::text_input::TextInput,
};

pub struct RenameState {
    target_path: PathBuf,
    pub input: TextInput,
    pub error: Option<String>,
}

impl RenameState {
    pub fn new(target: &FsEntry) -> Self {
        Self {
            target_path: target.path.clone(),
            input: TextInput::new(&target.name),
            error: None,
        }
    }

    pub fn cursor_col(&self) -> usize {
        self.input.cursor_display_col()
    }
}

impl Feature for RenameState {
    fn handle_key(&mut self, key: KeyCode) -> std::io::Result<FeatureOutcome> {
        match key {
            KeyCode::Esc => Ok(FeatureOutcome::Cancel),
            KeyCode::Enter => Ok(self.confirm()),
            KeyCode::Backspace => {
                self.input.delete_back();
                self.error = None;
                Ok(FeatureOutcome::Continue)
            }
            KeyCode::Delete => {
                self.input.delete_forward();
                self.error = None;
                Ok(FeatureOutcome::Continue)
            }
            KeyCode::Left => {
                self.input.move_left();
                Ok(FeatureOutcome::Continue)
            }
            KeyCode::Right => {
                self.input.move_right();
                Ok(FeatureOutcome::Continue)
            }
            KeyCode::Home => {
                self.input.home();
                Ok(FeatureOutcome::Continue)
            }
            KeyCode::End => {
                self.input.end();
                Ok(FeatureOutcome::Continue)
            }

            KeyCode::Char(c) => {
                self.input.insert(c);
                self.error = None;
                Ok(FeatureOutcome::Continue)
            }
            _ => Ok(FeatureOutcome::Continue),
        }
    }

    fn view(&self) -> String {
        let prompt = format!("renombrar: {}", self.input.value());

        match &self.error {
            Some(err) => format!("{} | Error: {}", prompt, err),
            None => prompt,
        }
    }
}

impl RenameState {
    fn confirm(&mut self) -> FeatureOutcome {
        match fs_entry::rename(&self.target_path, &self.input.value()) {
            Ok(()) => FeatureOutcome::Reload,
            Err(e) => {
                self.error = Some(e.to_string());
                FeatureOutcome::Continue
            }
        }
    }
}
