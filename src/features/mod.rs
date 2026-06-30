use crossterm::event::KeyCode;

pub mod rename;

pub enum FeatureOutcome {
    Continue,
    Reload,
    Cancel,
}

pub trait Feature {
    fn handle_key(&mut self, key: KeyCode) -> FeatureOutcome;
    fn view(&self) -> String;
}
