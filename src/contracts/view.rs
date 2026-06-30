use ratatui::{Frame, layout::Rect};

pub trait View {
    fn render(&self, frame: &mut Frame, area: Rect);
}
