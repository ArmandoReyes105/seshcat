use ratatui::style::{Color, Modifier, Style};

const COLOR_PRIMARY: Color = Color::Rgb(165, 180, 252);
const COLOR_ACCENT: Color = Color::Rgb(250, 191, 36);
const COLOR_MUTED: Color = Color::Rgb(168, 162, 158);
const COLOR_BORDER: Color = Color::Rgb(67, 56, 202);
const COLOR_BG_SELECTED: Color = Color::Rgb(49, 46, 129);
const COLOR_FG_SELECTED: Color = Color::Rgb(238, 242, 255);

pub fn cursor_bar_style() -> Style {
    Style::new().fg(COLOR_PRIMARY).add_modifier(Modifier::BOLD)
}

pub fn dir_style() -> Style {
    Style::new().fg(COLOR_ACCENT).add_modifier(Modifier::BOLD)
}

pub fn file_style() -> Style {
    Style::new().fg(COLOR_MUTED)
}

pub fn selected_style() -> Style {
    Style::new()
        .bg(COLOR_BG_SELECTED)
        .fg(COLOR_FG_SELECTED)
        .add_modifier(Modifier::BOLD)
}

pub fn title_style() -> Style {
    Style::new().fg(COLOR_PRIMARY).add_modifier(Modifier::BOLD)
}

pub fn path_style() -> Style {
    Style::new().add_modifier(Modifier::BOLD)
}

pub fn border_style() -> Style {
    Style::new().fg(COLOR_BORDER)
}

pub fn help_style() -> Style {
    Style::new().fg(COLOR_MUTED)
}

pub fn input_style() -> Style {
    Style::new()
        .fg(COLOR_FG_SELECTED)
        .add_modifier(Modifier::BOLD)
}

/// Attenuated highlight for the current-directory entry shown inside the
/// parent-preview pane. Distinct from `selected_style` (no background fill,
/// no bold) so it reads as "informational" rather than "actionable".
pub fn parent_highlight_style() -> Style {
    Style::new().fg(COLOR_PRIMARY).add_modifier(Modifier::DIM)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_highlight_style_is_dim_with_no_background() {
        let style = parent_highlight_style();

        assert!(style.add_modifier.contains(Modifier::DIM));
        assert_eq!(style.bg, None);
    }

    #[test]
    fn parent_highlight_style_is_distinct_from_selected_style() {
        assert_ne!(parent_highlight_style(), selected_style());
    }
}
