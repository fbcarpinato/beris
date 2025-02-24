use clap::Parser;
use std::{io::{self, Read, Write}, net::TcpStream, str};

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

    let mut stream = TcpStream::connect(args.url)?;

    let _ = stream.write_all(b"+PING\r\n");

    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer)?;

    let response = str::from_utf8(&buffer[..bytes_read]).unwrap();

    println!("{}", response);

    Ok(())
}
