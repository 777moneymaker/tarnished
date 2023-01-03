use tui::layout::{Constraint, Direction, Layout, Rect};

/// Describes the three parts of the application:
/// Top bar, Left block and right block
#[derive(Debug)]
pub struct AppParts {
    pub top_bar: Rect,
    pub left_block: Rect,
    pub right_block: Rect,
}

impl AppParts {
    /// From a given terminal size generates valid application parts
    /// Top Bar, Left block and right block
    pub fn generate_parts(frame_size: Rect) -> AppParts {
        let splitted = Layout::default()
            .margin(2)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
            .split(frame_size);

        let top_bar = splitted[0];
        let mid_canvas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(splitted[1]);

        let (left_block, right_block) = (mid_canvas[0], mid_canvas[1]);

        AppParts {
            top_bar,
            left_block,
            right_block,
        }
    }
}
