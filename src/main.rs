use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::event::Event::Key;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders};
use tui_textarea::TextArea;
use tarnished::app::{App, TarnishedAction};

fn set_valid(textarea: &mut TextArea) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));

    let b = textarea
        .block()
        .cloned()
        .unwrap_or_else(|| Block::default().borders(Borders::ALL));
    textarea.set_block(
        b.style(
            Style::default()
            .fg(Color::Green)
        ).title("Valid"),
    );
}

fn set_invalid(textarea: &mut TextArea) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));

    let b = textarea
        .block()
        .cloned()
        .unwrap_or_else(|| Block::default().borders(Borders::ALL));
    textarea.set_block(
        b.style(
            Style::default()
                .fg(Color::Gray)
        ).title("Invalid"),
    );
}


fn main() -> io::Result<()> {
    // Create new app
    let mut application: App = App::new();

    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Outside loop so we can modify it in this place
    let mut textarea = TextArea::default();
    set_valid(&mut textarea);

    loop {
        // Render frame
        terminal.draw(|mut frame| {
            application.render(&mut frame, &textarea);
        })?;

        let event = event::read()?;

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => break,
                _ => {
                    textarea.input(key);
                }
            }
        }

        match textarea.lines().len() {
            1..=2 => set_valid(&mut textarea),
            _ => set_invalid(&mut textarea)
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
