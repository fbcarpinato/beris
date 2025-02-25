use clap::Parser;
use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use widgets::command_input_widget::CommandInputWidget;
use std::{io::{self, Read, Result, Write}, net::TcpStream, str};

mod widgets;

use crate::widgets::command_input_widget;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "localhost:6379")]
    url: String,

    #[arg(short, long, default_value_t = 10)]
    clients: usize,

    #[arg(short, long, default_value_t = 100)]
    requests: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match color_eyre::install() {
        Ok(_) => {
            //
        }
        Err(_err) => {
            //
        }
    }

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    let mut stream = TcpStream::connect(args.url)?;

    let _ = stream.write_all(b"+PING\r\n");

    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer)?;

    let response = str::from_utf8(&buffer[..bytes_read]).unwrap();

    println!("{}", response);


    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;

        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[1]);

    frame.render_widget(
        Paragraph::new("outer 0").block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );
    frame.render_widget(
        Paragraph::new("inner 0").block(Block::new().borders(Borders::ALL)),
        inner_layout[0],
    );
    frame.render_widget(
        CommandInputWidget::new(),
        inner_layout[1],
    );
}
