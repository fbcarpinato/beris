mod event_loop;

use std::{
    cell::RefCell,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    rc::Rc,
    thread,
    time::Duration,
};

use event_loop::{Event, EventLoop};
use slab::Slab;

#[derive(Clone)]
pub struct SharedState {
    pub connected_clients: Rc<RefCell<Slab<Rc<RefCell<TcpStream>>>>>,
    pub listener: Rc<TcpListener>,
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
            connected_clients: Rc::new(RefCell::new(Slab::new())),
            listener: Rc::new(listener),
        };

        Ok(Server {
            event_loop: EventLoop::new(),
            state,
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        println!("Server running at 127.0.0.1:6379");

        self.event_loop.push(Self::create_accept_event(self.state.clone()));

        loop {
            self.event_loop.run();

            thread::sleep(Duration::from_millis(50));
        }
    }

    fn create_accept_event(state: SharedState) -> Event {
        Event::new(move || {
            let mut follow_up_events = Vec::new();

            match state.listener.accept() {
                Ok((stream, addr)) => {
                    println!("Accepted connection from {:?}", addr);
                    if let Err(e) = stream.set_nonblocking(true) {
                        eprintln!("Failed to set stream nonblocking: {:?}", e);
                    } else {
                        let stream_rc = Rc::new(RefCell::new(stream));
                        let client_id = state
                            .connected_clients
                            .borrow_mut()
                            .insert(stream_rc.clone());

                        follow_up_events.push(Self::create_read_event(client_id, stream_rc));
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {:?}", e);
                }
            }

            follow_up_events.push(Self::create_accept_event(state.clone()));

            follow_up_events
        })
    }

    fn create_read_event(client_id: usize, stream_rc: Rc<RefCell<TcpStream>>) -> Event {
        Event::new(move || {
            let mut buffer = [0u8; 1024];
            let mut follow_up_events = Vec::new();

            let read_result = {
                let mut stream = stream_rc.borrow_mut();
                stream.read(&mut buffer)
            };

            match read_result {
                Ok(0) => {
                    println!("Client {} disconnected", client_id);
                }
                Ok(n) => {
                    println!(
                        "Read {} bytes from client {}: {:?}",
                        n,
                        client_id,
                        &buffer[..n]
                    );


                    let write_result = {
                        let mut stream = stream_rc.borrow_mut();
                        stream.write_all(&mut buffer)
                    };

                    match write_result {
                        Ok(_) => {},
                        Err(_) => {}
                    }

                    follow_up_events.push(Self::create_read_event(client_id, stream_rc.clone()));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    follow_up_events.push(Self::create_read_event(client_id, stream_rc.clone()));
                }
                Err(e) => {
                    eprintln!("Error reading from client {}: {:?}", client_id, e);
                }
            }

            follow_up_events
        })
    }
}
