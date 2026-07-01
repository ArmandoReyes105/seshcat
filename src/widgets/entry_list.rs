use ratatui::{
    style::Style,
    text::{Line, Span, Text},
    widgets::{List, ListItem, ListState},
};

use crate::models::FsEntry;
use crate::ui::theme;

/// Renders a list of filesystem entries into a ratatui `List` + `ListState`,
/// highlighting the entry at `highlighted` (if any) with `highlight_style`.
///
/// Shared between the current-directory list and the parent-preview pane so
/// both panes stay visually consistent.
pub fn render_entry_list(
    entries: &[FsEntry],
    highlighted: Option<usize>,
    highlight_style: Style,
) -> (List<'static>, ListState) {
    let items: Vec<ListItem> = entries
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
            let is_highlighted = Some(i) == highlighted;
            let cursor_span = if is_highlighted {
                Span::styled("▎", theme::cursor_bar_style())
            } else {
                Span::raw(" ")
            };

            let line = Line::from(vec![cursor_span, tag_span, name_span]);

            if is_highlighted {
                ListItem::new(Text::from(line).style(highlight_style))
            } else {
                ListItem::new(line)
            }
        })
        .collect();

    let list = List::new(items);

    let mut state = ListState::default();
    state.select(highlighted);

    (list, state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn entry(name: &str, is_dir: bool) -> FsEntry {
        FsEntry {
            name: name.to_string(),
            path: PathBuf::from(name),
            is_dir,
        }
    }

    #[test]
    fn selects_highlighted_index_when_some() {
        let entries = vec![entry("a", true), entry("b", false)];
        let (_, state) = render_entry_list(&entries, Some(1), theme::selected_style());

        assert_eq!(state.selected(), Some(1));
    }

    #[test]
    fn selects_none_when_highlighted_is_none() {
        let entries = vec![entry("a", true), entry("b", false)];
        let (_, state) = render_entry_list(&entries, None, theme::selected_style());

        assert_eq!(state.selected(), None);
    }

    #[test]
    fn empty_entries_produce_no_selection() {
        let entries: Vec<FsEntry> = Vec::new();
        let (_, state) = render_entry_list(&entries, None, theme::selected_style());

        assert_eq!(state.selected(), None);
    }
}
