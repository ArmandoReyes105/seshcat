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

/*pub fn error_style() -> Style {
    Style::new().fg(Color::Red).add_modifier(Modifier::BOLD)
}*/
