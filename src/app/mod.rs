use std::error;

use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::utils::LOGO;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let frame_size = frame.size();

        // Top bar, that will take 20% of main_canvas
        // Mid canvas for some stats
        // Bottom bar for idk what (TODO)
        let splitted = Layout::default()
            .margin(2)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref())
            .split(frame_size);

        let top_bar = splitted[0];
        let mid = splitted[1];
        let bottom_bar = splitted[2];

        let mid_canvases = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33)
            ]).split(mid);

        let mid_leftmost = mid_canvases[0];
        let mid_middle = mid_canvases[1];
        let mid_rightmost = mid_canvases[2];

        let app_title = Paragraph::new(Text::from(LOGO))
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });
        frame.render_widget(app_title, top_bar);

        frame.render_widget(
            Block::default()
                .borders(Borders::ALL & !Borders::TOP)
                .title("MidBarLeft"),
            mid_leftmost
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL & !Borders::TOP)
                .title("MidBarMid"),
            mid_middle
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL & !Borders::TOP)
                .title("MidBarRight"),
            mid_rightmost
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL & !Borders::TOP)
                .title("BottomBar"),
            bottom_bar
        );
    }

    /// Gets action to be performed from pressed key
    pub fn get_action(event: KeyEvent) -> TarnishedAction {
        match event.code {
            KeyCode::Char('q') => TarnishedAction::Quit,
            _ => TarnishedAction::Continue
        }
    }
}

pub enum TarnishedAction {
    Quit,
    Continue,
    // TODO: more
}