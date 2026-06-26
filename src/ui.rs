use ratatui::{
    Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let items: Vec<ListItem> = app
        .entries
        .iter()
        .map(|entry| {
            let kind = if entry.is_dir { "carpeta" } else { "archivo" };
            ListItem::new(format!("[{kind}] {}", entry.name))
        })
        .collect();

    let title = format!(" {} ", app.current_path.display());

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(list, frame.area(), &mut state);
}
