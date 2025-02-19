use std::io;

use server::Server;

fn main() -> io::Result<()> {
    let server = Server::new()?;

    server.run();

    Ok(())
}
