use std::io;

use server::config::Config;
use server::server::Server;

fn main() -> io::Result<()> {
    let config = Config::new("127.0.0.1:6379");
    let server = Server::new(config);

    server.run()?;

    Ok(())
}
