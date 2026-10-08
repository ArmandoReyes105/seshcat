mod input;
mod view;

use crate::widgets::text_input::TextInput;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunTarget {
    Here,
    NewTab,
}

pub struct CommandState {
    pub input: TextInput,
    pub target: RunTarget,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandOutcome {
    Continue,
    Cancel,
    Submit(String),
}

impl CommandState {
    pub fn new(target: RunTarget) -> Self {
        Self {
            input: TextInput::default(),
            target,
        }
    }
}
