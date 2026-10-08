use crossterm::event::KeyCode;

use super::{CommandOutcome, CommandState};

impl CommandState {
    pub fn handle_key(&mut self, key: KeyCode) -> CommandOutcome {
        match key {
            KeyCode::Esc => CommandOutcome::Cancel,
            KeyCode::Enter => {
                let line = self.input.value().trim();
                if line.is_empty() {
                    CommandOutcome::Continue
                } else {
                    CommandOutcome::Submit(line.to_string())
                }
            }
            KeyCode::Backspace => {
                self.input.delete_back();
                CommandOutcome::Continue
            }
            KeyCode::Delete => {
                self.input.delete_forward();
                CommandOutcome::Continue
            }
            KeyCode::Left => {
                self.input.move_left();
                CommandOutcome::Continue
            }
            KeyCode::Right => {
                self.input.move_right();
                CommandOutcome::Continue
            }
            KeyCode::Home => {
                self.input.home();
                CommandOutcome::Continue
            }
            KeyCode::End => {
                self.input.end();
                CommandOutcome::Continue
            }
            KeyCode::Char(c) => {
                self.input.insert(c);
                CommandOutcome::Continue
            }
            _ => CommandOutcome::Continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::command::RunTarget;

    fn typed(text: &str) -> CommandState {
        let mut state = CommandState::new(RunTarget::Here);
        for c in text.chars() {
            state.handle_key(KeyCode::Char(c));
        }
        state
    }

    #[test]
    fn esc_cancels() {
        let mut state = typed("ls");

        assert_eq!(state.handle_key(KeyCode::Esc), CommandOutcome::Cancel);
    }

    #[test]
    fn enter_on_empty_input_continues() {
        let mut state = CommandState::new(RunTarget::Here);

        assert_eq!(state.handle_key(KeyCode::Enter), CommandOutcome::Continue);
    }

    #[test]
    fn enter_on_whitespace_input_continues() {
        let mut state = typed("   ");

        assert_eq!(state.handle_key(KeyCode::Enter), CommandOutcome::Continue);
    }

    #[test]
    fn enter_trims_and_submits() {
        let mut state = typed("  git status  ");

        assert_eq!(
            state.handle_key(KeyCode::Enter),
            CommandOutcome::Submit("git status".to_string())
        );
    }

    #[test]
    fn typing_then_backspace_edits_the_value() {
        let mut state = typed("lsx");

        state.handle_key(KeyCode::Backspace);

        assert_eq!(state.input.value(), "ls");
    }
}
