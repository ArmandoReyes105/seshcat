use ratatui::{Frame, layout::Rect};

use crate::contracts::View;
use crate::features::navigation::NavigationState;
use crate::ui::theme;
use crate::widgets::entry_list::render_entry_list;

impl View for NavigationState {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let (list, mut state) = render_entry_list(
            self.entries(),
            Some(self.selected()),
            theme::selected_style(),
        );

        frame.render_stateful_widget(list, area, &mut state);
    }
}
