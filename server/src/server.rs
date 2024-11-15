use std::io;
use std::net;

use crate::config::Config;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = net::TcpListener::bind(self.config.get_addr())?;

        println!("Server running at {}", self.config.get_addr());

        for _ in listener.incoming() {
            println!("Accepted connection");
        }

        Ok(())
    }
}
