use crossterm::event::KeyCode;

pub mod navigation;
pub mod rename;

pub enum FeatureOutcome {
    Continue,
    Reload,
    Cancel,
}

pub trait Feature {
    fn handle_key(&mut self, key: KeyCode) -> std::io::Result<FeatureOutcome>;
    fn view(&self) -> String;
}
