use crossterm::event::KeyCode;

pub enum FeatureOutcome {
    Continue,
    Reload,
    Cancel,
}

pub trait InputHandler {
    fn handle_key(&mut self, key: KeyCode) -> std::io::Result<FeatureOutcome>;
}
