use std::io;
use std::io::Write;
use std::net;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = net::TcpListener::bind("127.0.0.1:6379")?;

        println!("Server running at 127.0.0.1:6379");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => match stream.write_all(b"+PONG\r\n") {
                    Ok(()) => {
                        println!("Sended PONG message to client");
                    }
                    Err(error) => {
                        println!("Error while accepting the connection {}", error);
                    }
                },
                Err(error) => {
                    println!("Error while accepting the connection {}", error);
                }
            }
        }

        Ok(())
    }
}
