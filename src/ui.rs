use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

use crate::{
    app::App,
    theme::{self, path_style},
};

pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::vertical([
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    render_header(frame, app, areas[0]);
    render_list(frame, app, areas[1]);
    render_footer(frame, areas[2]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let note = if app.current_path.parent().is_none() {
        "(raíz del filesystem - no hay padre)"
    } else {
        ""
    };

    let path_str = app.current_path.display().to_string();
    let text = Text::from(vec![
        Line::from(Span::styled("Seshcat", theme::title_style())),
        Line::from(Span::styled(path_str, path_style())),
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
    let items: Vec<ListItem> = app
        .entries
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
            let is_selected = i == app.selected;
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
    state.select(Some(app.selected));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let help_text = Span::styled(
        "j/k mover ·  h subir  ·  l/enter entrar  ·  q salir",
        theme::help_style(),
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_style());

    let paragraph = Paragraph::new(Line::from(vec![help_text])).block(block);
    frame.render_widget(paragraph, area);
}
