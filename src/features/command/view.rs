use ratatui::{
    layout::Position,
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use super::{CommandState, RunTarget};
use crate::contracts::View;
use crate::ui::theme;

impl CommandState {
    fn prompt(&self) -> &'static str {
        match self.target {
            RunTarget::Here => "ejecutar: ",
            RunTarget::NewTab => "Abrir pestaña: ",
        }
    }
}

impl View for CommandState {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_style());

        let prompt = self.prompt();

        let prompt_cols = prompt.chars().count() as u16;
        let cursor_x = area.x + 1 + prompt_cols + self.input.cursor_display_col() as u16;
        let cursor_y = area.y + 1;
        frame.set_cursor_position(Position::new(cursor_x, cursor_y));

        let content = format!("{}{}", prompt, self.input.value());
        let paragraph = Paragraph::new(Span::styled(content, theme::input_style())).block(block);

        frame.render_widget(paragraph, area);
    }
}
