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
///
/// `highlighted` may be out of bounds relative to `entries.len()` (e.g. a
/// stale index computed against a different snapshot). This is safe: no
/// entry index ever equals an out-of-range `highlighted`, so no row is
/// mistakenly highlighted and rendering proceeds normally.
pub fn render_entry_list(
    entries: &[FsEntry],
    highlighted: Option<usize>,
    highlight_style: Style,
) -> (List<'static>, ListState) {
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let tag = if entry.is_dir { "\u{f07b}" } else { "\u{f15b}" };
            let tag_style = if entry.is_dir {
                theme::dir_style()
            } else {
                theme::file_style()
            };
            let tag_span = Span::styled(format!("{:>3} ", tag), tag_style);

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
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Modifier;
    use ratatui::widgets::StatefulWidget;
    use std::path::PathBuf;

    fn entry(name: &str, is_dir: bool) -> FsEntry {
        FsEntry {
            name: name.to_string(),
            path: PathBuf::from(name),
            is_dir,
        }
    }

    /// Renders `entries` into a fresh `Buffer` of `width` columns, one row
    /// per entry, and returns the resulting buffer alongside the state used.
    fn render_to_buffer(
        entries: &[FsEntry],
        highlighted: Option<usize>,
        highlight_style: Style,
        width: u16,
    ) -> Buffer {
        let (list, mut state) = render_entry_list(entries, highlighted, highlight_style);
        let area = Rect::new(0, 0, width, entries.len().max(1) as u16);
        let mut buf = Buffer::empty(area);
        StatefulWidget::render(list, area, &mut buf, &mut state);
        buf
    }

    fn row_text(buf: &Buffer, y: u16) -> String {
        (0..buf.area.width)
            .map(|x| buf[(x, y)].symbol().to_string())
            .collect::<String>()
            .trim_end()
            .to_string()
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

    #[test]
    fn renders_cursor_glyph_and_tag_text_per_row() {
        let entries = vec![entry("alpha", true), entry("beta.txt", false)];
        let buf = render_to_buffer(&entries, Some(0), theme::selected_style(), 30);

        assert_eq!(row_text(&buf, 0), "▎[carpeta] alpha");
        assert_eq!(row_text(&buf, 1), " [archivo] beta.txt");
    }

    #[test]
    fn highlighted_row_gets_the_passed_highlight_style() {
        let entries = vec![entry("alpha", true), entry("beta.txt", false)];
        let highlight = theme::selected_style();
        let buf = render_to_buffer(&entries, Some(0), highlight, 30);

        // Name span carries no explicit per-span style, so it directly
        // reflects the highlight style patched onto the whole row.
        let name_col = "▎[carpeta]".chars().count() as u16;
        let highlighted_cell = &buf[(name_col, 0)];
        assert_eq!(highlighted_cell.bg, highlight.bg.unwrap());
        assert_eq!(highlighted_cell.fg, highlight.fg.unwrap());
        assert!(highlighted_cell.modifier.contains(Modifier::BOLD));

        // The non-highlighted row must NOT carry the highlight background.
        let normal_name_col = " [archivo]".chars().count() as u16;
        let normal_cell = &buf[(normal_name_col, 1)];
        assert_ne!(normal_cell.bg, highlight.bg.unwrap());
    }

    #[test]
    fn highlighted_out_of_bounds_does_not_panic_and_highlights_nothing() {
        let entries = vec![entry("a", true), entry("b", false)];

        // Index 5 is out of range for a 2-item list. Must not panic, and
        // since no real index equals 5, no row should be highlighted.
        let buf = render_to_buffer(&entries, Some(5), theme::selected_style(), 20);

        assert_eq!(row_text(&buf, 0), " [carpeta] a");
        assert_eq!(row_text(&buf, 1), " [archivo] b");

        let cell_a = &buf[(0, 0)];
        let cell_b = &buf[(0, 1)];
        assert_ne!(cell_a.bg, theme::selected_style().bg.unwrap());
        assert_ne!(cell_b.bg, theme::selected_style().bg.unwrap());
    }
}
