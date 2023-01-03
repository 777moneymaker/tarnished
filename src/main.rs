mod app;
mod utils;

use anyhow::Result;
use crossterm::event::Event::Key;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

use app::{App, TarnishedAction};

fn main() -> Result<()> {
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

    let mut application: App = App::new(terminal_size, None);

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
