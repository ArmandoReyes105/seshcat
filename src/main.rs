mod app;
mod config;
mod contracts;
mod features;
mod models;
mod services;
mod ui;
mod widgets;

use std::env;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::process::ExitStatus;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::app::{App, PendingCommand};
use crate::services::shell;

fn main() -> io::Result<()> {
    let target = resolve_target_path()?;

    let mut app = App::new(target)?;

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;

    result
}

fn resume_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    terminal.clear()?;
    Ok(())
}

fn run_suspended(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    cmd: &PendingCommand,
) -> io::Result<ExitStatus> {
    restore_terminal(terminal)?;

    let result = shell::run_blocking(&cmd.line, &cmd.cwd);

    match &result {
        Ok(status) => println!("\n[{status}]"),
        Err(e) => println!("\nNo se pudo ejecutar el comando: {e}"),
    }
    println!("Presiona Enter para volver...");

    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    resume_terminal(terminal)?;

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

            if let Some(cmd) = app.take_pending_command() {
                match run_suspended(terminal, &cmd) {
                    Ok(status) if !status.success() => {
                        app.status = Some(format!("`{}` terminó con {status}", cmd.line));
                    }
                    Ok(_) => {}
                    Err(e) => {
                        app.status = Some(format!("No se pudo ejecutar `{}`: {e}", cmd.line));
                    }
                }

                // The command may have created, deleted or renamed files.
                if let Err(e) = app.navigation.reload_dir() {
                    app.status = Some(format!("No se pudo recargar el directorio: {e}"));
                }
            }
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
