use std::net;
use std::io;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = net::TcpListener::bind("127.0.0.1:6379")?;

        println!("Server running at 127.0.0.1:6379");

        for _ in listener.incoming() {
            println!("Accepted connection");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = Server::new();
        assert!(server.run().is_ok());
    }
}
