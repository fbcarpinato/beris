use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://localhost:6379")]
    url: String,

    #[arg(short, long, default_value_t = 10)]
    clients: usize,

    #[arg(short, long, default_value_t = 100)]
    requests: usize,
}


fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Configuration: ");
    println!("\t- Server url: {}", args.url);
    println!("\t- Clients: {}", args.clients);
    println!("\t- Requests: {}", args.requests);

    Ok(())
}
