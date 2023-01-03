mod app;
mod parser;
mod utils;

use anyhow::Result;
use crossterm::event::Event::Key;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::path::Path;
use tui::{backend::CrosstermBackend, Terminal};

use app::{App, TarnishedAction};

fn string_ends_with_any(s: &String, suffixes: Vec<&str>) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}

fn main() -> Result<()> {
    let files: Vec<String> = std::fs::read_dir("./")?
        .map(|item| {
            let entry = item.expect("Couldn't unwrap dir entry");
            entry.path().as_path().to_owned()
        })
        .filter(|file| file.extension().is_some())
        .filter(|file| {
            vec!["fna", "fn", "fasta"].contains(&file.extension().unwrap().to_str().unwrap())
        })
        .map(|file| {
            file.to_owned()
                .into_os_string()
                .into_string()
                .expect("Couldn't unwrap string from filepath")
        })
        .collect();

    if files.len() == 0 {
        eprintln!("Couldn't find any valid fasta. Consider adding only 'fa' or 'fasta' extensions to your files");
        std::process::exit(1);
    }

    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a new app
    let terminal_size = terminal
        .size()
        .expect("Couldn't establish terminal size at init");

    let mut application: App = App::new(terminal_size, Some(files));

    loop {
        // Render frame
        terminal.draw(|mut frame| {
            application.render(&mut frame);
        })?;

        let event = event::read()?;
        if let Key(key_event) = event {
            let action: TarnishedAction = application.handle_key_event(key_event);
            match action {
                TarnishedAction::Quit => break,
                TarnishedAction::Continue => continue,
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
