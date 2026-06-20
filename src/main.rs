use std::io::{self, Stdout};
use std::path::{Path, PathBuf};
use std::{env, fs};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::{Frame, Terminal};

fn main() -> io::Result<()> {
    let target = resolve_target_path();

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal, &target);
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

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, path: &Path) -> io::Result<()> {
    let entries = list_dir(path)?;

    loop {
        terminal.draw(|frame| ui(frame, &entries))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }

            if key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('c')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                break;
            }
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, entries: &[PathBuf]) {
    let items: Vec<ListItem> = entries
        .iter()
        .map(|path| {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_else(|| path.to_string_lossy());

            let kind = if path.is_dir() {
                "dir"
            } else if path.is_file() {
                "file"
            } else {
                "other"
            };

            ListItem::new(format!("[{kind}] {name}"))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Seshcat"));

    frame.render_widget(list, frame.area());
}

fn resolve_target_path() -> PathBuf {
    match env::args().nth(1) {
        Some(path_str) => PathBuf::from(path_str),
        None => PathBuf::from("."),
    }
}

fn list_dir(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let dir_entries: Vec<fs::DirEntry> = entries.collect::<io::Result<Vec<_>>>()?;

    let paths: Vec<PathBuf> = dir_entries.iter().map(|entry| entry.path()).collect();

    Ok(paths)
}
