use crossterm::event::KeyCode;

use crate::contracts::{FeatureOutcome, InputHandler};
use crate::features::navigation::NavigationState;

impl InputHandler for NavigationState {
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
}
