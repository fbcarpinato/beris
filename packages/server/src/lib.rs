mod event_loop;
mod resp;
mod state;

use std::{io, net::TcpListener};

use event_loop::EventLoop;
use state::State;

pub struct Server {
    event_loop: EventLoop,
}

impl Server {
    pub fn new() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:6379")?;
        listener.set_nonblocking(true)?;

        let state = State::new(listener);

        Ok(Server {
            event_loop: EventLoop::new(state),
        })
    }

    pub fn run(mut self) {
        println!("Server running at 127.0.0.1:6379");

        self.event_loop.run();
    }
}
