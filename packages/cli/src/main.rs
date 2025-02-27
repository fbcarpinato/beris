mod app;

use app::App;
use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{
    io::{self},
    net::TcpStream,
};

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

    color_eyre::install().expect("Error while installing color_eyre");
    enable_raw_mode()?;

    let stream = TcpStream::connect(args.url)?;

    let terminal = ratatui::init();
    let result = App::default().run(terminal, stream);

    ratatui::restore();
    disable_raw_mode()?;

    result
}
