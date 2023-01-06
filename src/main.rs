mod app;
mod parser;
mod utils;

use anyhow::Result;
use clap::{App as ClapApp, Arg};
use crossterm::event::Event::Key;
use crossterm::event::KeyCode;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use indicatif::{ProgressBar, ProgressStyle};
use parser::FastaRecord;
use std::io;
use std::path::Path;
use tui::layout::Rect;
use tui::{backend::CrosstermBackend, Terminal};

use crate::app::App;

const VALID_EXTS: [&str; 3] = ["fna", "fn", "fasta"];
const MIN_TERMINAL_SIZE: Rect = Rect {
    x: 0,
    y: 0,
    width: 80,
    height: 30,
};

fn main() -> Result<()> {
    // Arg parsing
    let args = ClapApp::new("Tarnished")
        .about("App for testing the nucleotide abundance in fasta files")
        .author("Miłosz Chodkowski")
        .version("0.3.0")
        .arg(
            Arg::with_name("files")
                .multiple(true)
                .takes_value(true)
                .help("files in fasta format ending with [.fa | .fna | .fasta]")
                .required(true),
        )
        .get_matches();

    let matches = args.values_of("files").expect("No files provided");

    let mut files = Vec::<String>::with_capacity(20000);
    let filterd_iterator = matches.filter_map(|file| {
        let path = Path::new(file);
        let extension = path.extension()?.to_str().expect("");
        match VALID_EXTS.contains(&extension) {
            true => Some(path),
            false => None,
        }
    });
    for file in filterd_iterator {
        let fl = file
            .to_owned()
            .into_os_string()
            .into_string()
            .expect("Couldn't unwrap string from filepath");
        files.push(fl);
    }

    if files.is_empty() {
        eprintln!("Couldn't find any valid fasta. Consider adding only 'fa' or 'fasta' extensions to your files");
        std::process::exit(1);
    }

    let bar = ProgressBar::new(files.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "🦀🦀🦀 [{elapsed:.green}] [File: {msg:.yellow}] {bar:60.yellow} | [Done: {percent}%]",
        )
        .unwrap()
        .progress_chars("❚.."),
    );

    let records: Vec<FastaRecord> = files
        .iter()
        .map(|file| {
            bar.set_message(file.clone());
            bar.inc(1);
            FastaRecord::parse((&file).to_string()).expect("Couldn't read a file")
        })
        .collect();
    bar.finish();
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

    match (terminal_size.height, terminal_size.width)
        .cmp(&(MIN_TERMINAL_SIZE.height, MIN_TERMINAL_SIZE.width))
    {
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Greater => {}
        std::cmp::Ordering::Less => {
            // restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            eprintln!(
                "Terminal size is too small: (H: {}, W: {}). Required (H: {}, W: {})",
                terminal_size.height,
                terminal_size.width,
                MIN_TERMINAL_SIZE.height,
                MIN_TERMINAL_SIZE.width
            );
            std::process::exit(1);
        }
    }

    let mut selected_idx: isize = 0;
    let records_len = records.len() as isize;
    let mut application: App = App::new(terminal_size, &records);

    loop {
        // Render frame
        terminal.draw(|frame| {
            application.render(frame, selected_idx as usize);
        })?;

        let event = event::read()?;
        if let Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => break,
                key @ (KeyCode::Down | KeyCode::Up) => {
                    let new_idx: isize = match key {
                        KeyCode::Down => selected_idx + 1,
                        KeyCode::Up => selected_idx - 1,
                        _ => 0isize,
                    };

                    let updated_idx: isize = if new_idx < 0 {
                        records_len - 1
                    } else if new_idx > (records_len as isize - 1) {
                        0
                    } else {
                        new_idx
                    };

                    selected_idx = updated_idx;
                    continue;
                }
                _ => continue,
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
