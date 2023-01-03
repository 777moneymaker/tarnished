use std::cell::Cell;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{BarChart, Block, Borders, List, ListItem, ListState, Paragraph};

use crate::utils::LOGO;

pub struct AppWidgets<'a> {
    pub paragraph: Paragraph<'a>,
    pub file_list: List<'a>,
    pub bar_chart: BarChart<'a>,
    pub file_list_state: Cell<ListState>,
}

impl<'a> AppWidgets<'a> {
    pub fn generate_widgets(files: &Vec<String>) -> Self {
        let app_name = Paragraph::new(Span::from(LOGO))
            .block(
                Block::default()
                    .title("Lorem ipsum title")
                    .borders(Borders::ALL),
            )
            .style(Style::default())
            .alignment(Alignment::Center);

        let files: Vec<ListItem> = files
            .iter()
            .map(|file| ListItem::new(file.clone()))
            .collect();
        let file_list: List = List::new(files)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Rgb(148, 0, 211)),
            )
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
            file_list_state: Cell::new(file_list_state),
        }
    }

    fn generate_bar_chart(files: Option<Vec<String>>) {
        todo!("Implement this");
    }
}
