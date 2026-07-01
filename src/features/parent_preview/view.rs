use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

use crate::features::navigation::{self, NavigationState};
use crate::ui::theme;
use crate::widgets::entry_list::render_entry_list;

use super::{ParentView, build_view};

/// Explicit placeholder for a parent directory that could not be listed
/// (e.g. permission denied). Distinct from `navigation::ROOT_NOTE` so the
/// two situations never look the same.
const PARENT_ERROR_NOTE: &str = "(no se puede acceder al directorio padre)";

/// Renders the read-only parent-directory pane into `area`. View-only: no
/// input handling, no filesystem access - it only maps the already-resolved
/// `ParentView` (built from `nav`'s cache) onto widgets. Infallible.
pub fn render(frame: &mut Frame, area: Rect, nav: &NavigationState) {
    let view = build_view(nav.parent_snapshot(), nav.current_path());

    match view {
        ParentView::Root => render_placeholder(frame, area, navigation::ROOT_NOTE),
        ParentView::Error(message) => {
            let text = format!("{PARENT_ERROR_NOTE}: {message}");
            render_placeholder(frame, area, &text);
        }
        ParentView::Listed { entries, highlight } => {
            let (list, mut state) =
                render_entry_list(entries, highlight, theme::parent_highlight_style());
            frame.render_stateful_widget(list, area, &mut state);
        }
    }
}

fn render_placeholder(frame: &mut Frame, area: Rect, text: &str) {
    let paragraph = Paragraph::new(Line::from(text)).style(theme::help_style());
    frame.render_widget(paragraph, area);
}
