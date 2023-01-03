mod app_parts;
mod app_widgets;

use anyhow::Result;
use crossterm::event::Event::Key;
use crossterm::event::{KeyCode, KeyEvent};
use std::cmp::Ordering;
use std::fmt::write;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::Span;

use app_parts::AppParts;
use app_widgets::AppWidgets;

/// Application.
pub struct App<'a> {
    /// Is the a'application running?
    pub parts: AppParts,
    pub widgets: AppWidgets<'a>,
    pub files: Vec<String>,
    pub running: bool,
}

impl App<'_> {
    /// Constructs a new instance of [`App`] with their parts and running state
    pub fn new(frame_size: Rect, files: Option<Vec<String>>) -> Self {
        let default_items = vec!["Test1", "Test2", "Test3"]
            .iter()
            .map(|file| file.to_string())
            .collect();
        let items: Vec<String> = files.unwrap_or(default_items);
        App {
            parts: AppParts::generate_parts(frame_size),
            widgets: AppWidgets::generate_widgets(&items),
            files: items,
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

        let app_widgets = AppWidgets::generate_widgets(&self.files);

        frame.render_widget(app_widgets.paragraph, self.parts.top_bar);
        frame.render_stateful_widget(
            app_widgets.file_list,
            self.parts.left_block,
            self.widgets.file_list_state.get_mut(),
        );
        frame.render_widget(app_widgets.bar_chart, self.parts.right_block);
    }

    /// Gets action to be performed from pressed key
    pub fn handle_key_event(&mut self, event: KeyEvent) -> TarnishedAction {
        match event.code {
            KeyCode::Esc => TarnishedAction::Quit,
            key @ (KeyCode::Down | KeyCode::Up) => {
                self.change_selected_item(key);
                TarnishedAction::Continue
            }
            KeyCode::Enter => {
                // TODO: Implement plot drawing on this event
                TarnishedAction::Continue
            }
            _ => TarnishedAction::Continue,
        }
    }

    fn change_selected_item(&mut self, key: KeyCode) {
        let state = self.widgets.file_list_state.get_mut();
        let selected_idx = state
            .selected()
            .expect("Couldn't get the selected item index") as isize;
        let len = self.files.len() as isize;

        let new_idx: isize = match key {
            KeyCode::Down => selected_idx + 1,
            KeyCode::Up => selected_idx - 1,
            _ => 0isize,
        };

        let updated_idx: isize = if new_idx < 0 {
            len - 1
        } else if new_idx > (len as isize - 1) {
            0
        } else {
            new_idx
        };

        state.select(Some(updated_idx as usize));
    }
}

pub enum TarnishedAction {
    Quit,
    Continue,
    // TODO: more
}
