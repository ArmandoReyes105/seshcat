use crossterm::event::KeyCode;

use crate::contracts::{FeatureOutcome, InputHandler};
use crate::features::rename::RenameState;

impl InputHandler for RenameState {
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
}
