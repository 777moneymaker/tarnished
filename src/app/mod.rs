use std::cell::Cell;
use std::fmt::{Formatter, write};
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{BarChart, Block, Borders, List, ListItem, ListState, Paragraph};
use anyhow::Result;
use crossterm::event::Event::Key;
use tui::text::Span;

use crate::utils::LOGO;

/// Application.
pub struct App<'a> {
    /// Is the a'application running?
    pub parts: AppParts,
    pub widgets: AppWidgets<'a>,
    pub running: bool
}

/// Describes the three parts of the application:
/// Top bar, Left block and right block
#[derive(Debug)]
pub struct AppParts {
    pub top_bar: Rect,
    pub left_block: Rect,
    pub right_block: Rect,
}

pub struct AppWidgets<'a> {
    pub paragraph: Paragraph<'a>,
    pub file_list: List<'a>,
    pub bar_chart: BarChart<'a>,
    pub file_list_state: Cell<ListState>
}

impl<'a> AppWidgets<'a> {
    pub fn generate_widgets() -> Self {
        let app_name = Paragraph::new(Span::from(LOGO))
            .block(Block::default().title("Lorem ipsum title").borders(Borders::ALL))
            .style(Style::default())
            .alignment(Alignment::Center);

        let files = vec![ListItem::new("Test1"), ListItem::new("Test2")];
        let file_list: List = List::new(files)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("--> ");

        let chart: BarChart = BarChart::default()
            .block(Block::default().title("Test chart").borders(Borders::ALL))
            .style(Style::default());

        let mut file_list_state = ListState::default();
        file_list_state.select(Some(0usize));

        AppWidgets {
            paragraph: app_name,
            file_list: file_list,
            bar_chart: chart,
            file_list_state: Cell::new(file_list_state)
        }
    }
}

impl AppParts {
    /// From a given terminal size generates valid application parts
    /// Top Bar, Left block and right block
    fn generate_parts(frame_size: Rect) -> AppParts {
        let splitted = Layout::default()
            .margin(2)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(85)
            ].as_ref())
            .split(frame_size);

        let top_bar = splitted[0];
        let mid_canvas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref())
            .split(splitted[1]);

        let (left_block, right_block) = (mid_canvas[0], mid_canvas[1]);

        AppParts {
            top_bar,
            left_block,
            right_block
        }
    }
}


impl App<'_> {
    /// Constructs a new instance of [`App`] with their parts and running state
    pub fn new(frame_size: Rect) -> Self {
        App {
            parts: AppParts::generate_parts(frame_size),
            widgets: AppWidgets::generate_widgets(),
            running: false,
        }
    }

    /// From given frame takes it's size and uses it to compute new size off App's parts
    pub fn update_sizes(&mut self, frame_size: Rect) {
        self.parts = AppParts::generate_parts(frame_size);
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let frame_size = frame.size();
        self.update_sizes(frame_size);

        let app_widgets = AppWidgets::generate_widgets();


        frame.render_widget(app_widgets.paragraph, self.parts.top_bar);
        frame.render_stateful_widget(app_widgets.file_list, self.parts.left_block, self.widgets.file_list_state.get_mut());
        frame.render_widget(app_widgets.bar_chart, self.parts.right_block);

    }

    /// Gets action to be performed from pressed key
    pub fn handle_key_event(&mut self, event: KeyEvent) -> TarnishedAction {
        match event.code {
            KeyCode::Esc => TarnishedAction::Quit,
            key @ (KeyCode::Down | KeyCode::Up) => {
                let state = self.widgets.file_list_state.get_mut();
                let selected_idx = state.selected().unwrap_or(0);
                let new_idx = if key == KeyCode::Down {selected_idx + 1} else {selected_idx - 1};
                state.select(Some(new_idx));

                TarnishedAction::Continue
            }
            _ => TarnishedAction::Continue
        }
    }
}

pub enum TarnishedAction {
    Quit,
    Continue,
    // TODO: more
}