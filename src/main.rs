mod app;
mod features;
mod fs_entry;
mod theme;
mod ui;
mod widgets;

use std::env;
use std::io::{self, Stdout};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::app::App;

fn main() -> io::Result<()> {
    let target = resolve_target_path()?;

    let mut app = App::new(target)?;

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;

    result
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            let is_ctrl_c =
                key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL);

            if is_ctrl_c {
                break;
            }

            if !app.is_capturing_keys() && key.code == KeyCode::Char('q') {
                break;
            }

            app.handle_key(key.code)?;
        }
    }

    Ok(())
}

fn resolve_target_path() -> io::Result<PathBuf> {
    let raw = match env::args().nth(1) {
        Some(path_str) => PathBuf::from(path_str),
        None => PathBuf::from("."),
    };

    std::path::absolute(raw)
}
