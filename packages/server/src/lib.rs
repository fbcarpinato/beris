mod event_loop;

use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use event_loop::EventLoop;
use slab::Slab;

#[derive(Clone)]
pub struct SharedState {
    pub connected_clients: Arc<Mutex<Slab<Arc<Mutex<TcpStream>>>>>,
    pub listener: Arc<TcpListener>,
}

pub struct Server {
    event_loop: EventLoop,
    state: SharedState,
}

impl Server {
    pub fn new() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:6379")?;
        listener.set_nonblocking(true)?;

        let state = SharedState {
            connected_clients: Arc::new(Mutex::new(Slab::new())),
            listener: Arc::new(listener),
        };

        Ok(Server {
            event_loop: EventLoop::new(),
            state,
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        println!("Server running at 127.0.0.1:6379");

        self.event_loop
            .push(event_loop::EventType::AcceptConnection(self.state));

        loop {
            self.event_loop.run();
            thread::sleep(Duration::from_millis(50));
        }
    }
}
