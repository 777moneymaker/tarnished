use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use tarnished::app::{App, TarnishedAction};

fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut application: App = App::new();

    loop {
        // Render frame
        terminal.draw(|mut frame| {
            application.render(&mut frame)
        })?;


        // Read event
        let app_event = event::read()?;

        if let Event::Key(key_event) = app_event {
            match App::get_action(key_event) {
                TarnishedAction::Quit => break,
                TarnishedAction::Continue => {}
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
