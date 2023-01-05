pub mod app_parts;

use app_parts::AppParts;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::Span;
use tui::widgets::ListState;
use tui::widgets::{BarChart, Block, Borders, List, ListItem, Paragraph};

use crate::parser::FastaRecord;
use crate::utils::LOGO;

/// Application.
pub struct App<'a> {
    /// Is the a'application running?
    pub parts: AppParts,
    pub records: &'a Vec<FastaRecord>,
    pub running: bool,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`] with their parts and running state
    pub fn new(frame_size: Rect, records: &'a Vec<FastaRecord>) -> App<'a> {
        App {
            parts: AppParts::generate_parts(frame_size),
            records: records,
            running: false,
        }
    }

    /// From given frame takes it's size and uses it to compute new size off App's parts
    pub fn update_sizes(&mut self, frame_size: Rect) {
        self.parts = AppParts::generate_parts(frame_size);
    }

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>, selected: usize) {
        let frame_size = frame.size();
        self.update_sizes(frame_size);
        let app_name = Paragraph::new(Span::from(LOGO))
            .block(
                Block::default()
                    .style(Style::default().bg(Color::Black))
                    .borders(Borders::ALL),
            )
            .style(Style::default())
            .alignment(Alignment::Center);

        let files: Vec<ListItem> = self
            .records
            .iter()
            .map(|record| ListItem::new(record))
            .collect();
        let file_list: List = List::new(files)
            .block(
                Block::default()
                    .title("Identified FASTA records")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Rgb(153, 50, 204)),
            )
            .highlight_symbol("--> ");

        let mut file_list_state = ListState::default();
        file_list_state.select(Some(selected));

        let bar_gap = 1;
        let bc = BarChart::default()
            .block(
                Block::default()
                    .title(self.records[selected].id.clone())
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Left),
            )
            .bar_width((self.parts.right_block.width / 4) - bar_gap)
            .bar_gap(bar_gap)
            .bar_style(
                Style::default()
                    .fg(Color::Rgb(153, 50, 204))
                    .bg(Color::Black),
            )
            .value_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(Style::default().fg(Color::White))
            .data(&self.records[selected].nucleotide_counts);

        frame.render_widget(app_name, self.parts.top_bar);
        frame.render_stateful_widget(file_list, self.parts.left_block, &mut file_list_state);
        frame.render_widget(bc, self.parts.right_block);
    }
}
