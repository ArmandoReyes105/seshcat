mod input;
mod view;

use std::path::PathBuf;

use crate::contracts::FeatureOutcome;
use crate::models::FsEntry;
use crate::services::filesystem;
use crate::widgets::text_input::TextInput;

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

    fn confirm(&mut self) -> FeatureOutcome {
        match filesystem::rename(&self.target_path, self.input.value()) {
            Ok(()) => FeatureOutcome::Reload,
            Err(e) => {
                self.error = Some(e.to_string());
                FeatureOutcome::Continue
            }
        }
    }
}
