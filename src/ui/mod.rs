pub mod theme;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, AppMode};
use crate::contracts::View;
use crate::features::parent_preview;

pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    render_header(frame, app, areas[0]);
    render_list(frame, app, areas[1]);
    render_footer(frame, app, areas[2]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let path_str = app.navigation.current_path().display().to_string();
    let text = Text::from(vec![Line::from(Span::styled(
        path_str,
        theme::path_style(),
    ))]);

    let block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_style())
        .title("Seshcat ");

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
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_style())
        .title("Info ");

    match &app.mode {
        AppMode::Normal => {
            let line = match &app.status {
                Some(message) => Span::styled(message.as_str(), theme::status_style()),
                None => Span::styled(
                    "j/k mover ·  h subir  ·  l/enter entrar  ·  q salir",
                    theme::help_style(),
                ),
            };
            let paragraph = Paragraph::new(Line::from(vec![line])).block(block);
            frame.render_widget(paragraph, area);
        }
        AppMode::Rename(state) => state.render(frame, area),
        AppMode::Command(state) => state.render(frame, area),
        AppMode::FavoritesLeader => {
            let text = Span::styled(
                "-- ★ Favorite Jump • Press a key • Esc Cancel --",
                theme::help_style(),
            );
            let paragraph = Paragraph::new(text).block(block);
            frame.render_widget(paragraph, area);
        }
        AppMode::GotoLeader => {
            let text = Span::styled("-- Go to • c Config • Esc Cancel --", theme::help_style());
            let paragraph = Paragraph::new(text).block(block);
            frame.render_widget(paragraph, area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::navigation;
    use ratatui::{Terminal, backend::TestBackend, buffer::Buffer};
    use std::path::PathBuf;

    /// Flattens every cell in `buf` (row-major) into a single string so
    /// substring assertions can span the whole rendered screen regardless
    /// of which pane produced the text.
    fn buffer_text(buf: &Buffer) -> String {
        (0..buf.area.height)
            .map(|y| {
                (0..buf.area.width)
                    .map(|x| buf[(x, y)].symbol().to_string())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn render_to_text(app: &App) -> String {
        let backend = TestBackend::new(150, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(frame, app)).unwrap();
        buffer_text(terminal.backend().buffer())
    }

    #[test]
    fn footer_shows_goto_leader_hint() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.mode = AppMode::GotoLeader;

        let rendered = render_to_text(&app);

        assert!(
            rendered.contains("-- Go to • c Config • Esc Cancel --"),
            "goto leader hint missing:\n{rendered}"
        );
    }

    #[test]
    fn footer_shows_status_in_normal_mode() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.status = Some("Config dir not available".to_string());

        let rendered = render_to_text(&app);

        assert!(rendered.contains("Config dir not available"));
        assert!(
            !rendered.contains("q salir"),
            "status replaces the help text"
        );
    }

    #[test]
    fn footer_shows_help_when_no_status() {
        let dir = tempfile::TempDir::new().unwrap();
        let app = App::new(dir.path().to_path_buf()).unwrap();

        let rendered = render_to_text(&app);

        assert!(rendered.contains("q salir"));
    }

    /// Regression test for the root-indication duplication bug: at the
    /// filesystem root, both `render_header` and the parent-preview pane
    /// used to render `navigation::ROOT_NOTE`, showing it twice in the same
    /// frame. The parent pane is the single owner of this indication.
    #[test]
    fn root_note_appears_exactly_once_at_filesystem_root() {
        #[cfg(windows)]
        let root = PathBuf::from("C:\\");
        #[cfg(not(windows))]
        let root = PathBuf::from("/");

        let app = App::new(root).expect("failed to build App at filesystem root");

        // Wide enough that the 1/3-width parent pane (~50 cols) can fit the
        // full `ROOT_NOTE` text (36 chars) without clipping - a narrower
        // terminal would truncate the pane's copy and hide the duplicate.
        let backend = TestBackend::new(150, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(frame, &app)).unwrap();

        let rendered = buffer_text(terminal.backend().buffer());
        let occurrences = rendered.matches(navigation::ROOT_NOTE).count();

        assert_eq!(
            occurrences, 1,
            "expected exactly one root indication in the rendered screen, found {occurrences}:\n{rendered}"
        );
    }
}
