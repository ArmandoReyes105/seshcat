use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{List, ListItem, ListState},
};

use crate::contracts::View;
use crate::features::navigation::NavigationState;
use crate::ui::theme;

impl View for NavigationState {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .entries()
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let tag = if entry.is_dir { "carpeta" } else { "archivo" };
                let tag_style = if entry.is_dir {
                    theme::dir_style()
                } else {
                    theme::file_style()
                };
                let tag_span = Span::styled(format!("[{:<7}]", tag), tag_style);

                let name_span = Span::raw(format!(" {}", entry.name));
                let is_selected = i == self.selected();
                let cursor_span = if is_selected {
                    Span::styled("▎", theme::cursor_bar_style())
                } else {
                    Span::raw(" ")
                };

                let line = Line::from(vec![cursor_span, tag_span, name_span]);

                if is_selected {
                    ListItem::new(Text::from(line).style(theme::selected_style()))
                } else {
                    ListItem::new(line)
                }
            })
            .collect();

        let list = List::new(items);

        let mut state = ListState::default();
        state.select(Some(self.selected()));

        frame.render_stateful_widget(list, area, &mut state);
    }
}
