pub mod theme;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, AppMode};
use crate::contracts::View;
use crate::features::navigation;
use crate::features::parent_preview;

pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::vertical([
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    render_header(frame, app, areas[0]);
    render_list(frame, app, areas[1]);
    render_footer(frame, app, areas[2]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let note = if app.navigation.is_at_root() {
        navigation::ROOT_NOTE
    } else {
        ""
    };

    let path_str = app.navigation.current_path().display().to_string();
    let text = Text::from(vec![
        Line::from(Span::styled("Seshcat", theme::title_style())),
        Line::from(Span::styled(path_str, theme::path_style())),
        Line::from(Span::raw(note)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_style());

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_list(frame: &mut Frame, app: &App, area: Rect) {
    let columns =
        Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]).split(area);

    parent_preview::view::render(frame, columns[0], &app.navigation);
    app.navigation.render(frame, columns[1]);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_style());

    match &app.mode {
        AppMode::Normal => {
            let help_text = Span::styled(
                "j/k mover ·  h subir  ·  l/enter entrar  ·  q salir",
                theme::help_style(),
            );
            let paragraph = Paragraph::new(Line::from(vec![help_text])).block(block);
            frame.render_widget(paragraph, area);
        }
        AppMode::Rename(state) => state.render(frame, area),
        _ => {}
    }
}
