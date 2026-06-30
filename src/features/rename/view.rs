use ratatui::{
    layout::Position,
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::contracts::View;
use crate::features::rename::RenameState;
use crate::ui::theme;

impl RenameState {
    fn content(&self) -> String {
        let prompt = format!("renombrar: {}", self.input.value());

        match &self.error {
            Some(err) => format!("{} | Error: {}", prompt, err),
            None => prompt,
        }
    }

    fn cursor_col(&self) -> usize {
        self.input.cursor_display_col()
    }
}

impl View for RenameState {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_style());

        const PROMPT_COLS: u16 = 11;

        let cursor_x = area.x + 1 + PROMPT_COLS + self.cursor_col() as u16;
        let cursor_y = area.y + 1;

        frame.set_cursor_position(Position::new(cursor_x, cursor_y));

        let content = self.content();
        let paragraph = Paragraph::new(Span::styled(content, theme::input_style())).block(block);

        frame.render_widget(paragraph, area);
    }
}
