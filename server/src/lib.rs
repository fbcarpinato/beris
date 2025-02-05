mod event_loop;

use std::io;
use std::net;

use event_loop::{Event, EventLoop};

pub struct Server {
    event_loop: EventLoop
}

impl Server {
    pub fn new() -> Self {
        Server {
            event_loop: EventLoop::new()
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        let listener = net::TcpListener::bind("127.0.0.1:6379")?;

        println!("Server running at 127.0.0.1:6379");

        loop {
            match listener.accept() {
                Ok((_stream, _)) => {
                    self.event_loop.push(
                        Event::new(move || {})
                    );
                },
                Err(_) => {
                    //
                }
            }
        }

        // for stream in listener.incoming() {
        //     match stream {
        //         Ok(mut stream) => match stream.write_all(b"+PONG\r\n") {
        //             Ok(()) => {
        //                 println!("Sended PONG message to client");
        //             }
        //             Err(error) => {
        //                 println!("Error while accepting the connection {}", error);
        //             }
        //         },
        //         Err(error) => {
        //             println!("Error while accepting the connection {}", error);
        //         }
        //     }
        // }
    }
}
