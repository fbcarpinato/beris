mod event_loop;

use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream, Shutdown},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use event_loop::{Event, EventLoop};
use slab::Slab;
use smallvec::SmallVec;

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
            .push(Self::create_accept_event(self.state.clone()));

        loop {
            self.event_loop.run();
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn create_accept_event(state: SharedState) -> Event {
        Event::new(move || {
            let mut follow_up_events: SmallVec<[Event; 4]> = SmallVec::new();

            match state.listener.accept() {
                Ok((stream, addr)) => {
                    println!("Accepted connection from {:?}", addr);

                    if let Err(e) = stream.set_nonblocking(true) {
                        eprintln!("Failed to set stream nonblocking: {:?}", e);
                    } else {
                        let stream_arc = Arc::new(Mutex::new(stream));
                        let client_id = state
                            .connected_clients
                            .lock()
                            .unwrap()
                            .insert(stream_arc.clone());

                        follow_up_events.push(Self::create_read_event(client_id, stream_arc));
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

    fn create_read_event(client_id: usize, stream_arc: Arc<Mutex<TcpStream>>) -> Event {
        Event::new(move || {
            let mut buffer = [0u8; 1024];
            let mut follow_up_events: SmallVec<[Event; 4]> = SmallVec::new();

            let read_result = {
                let mut stream = stream_arc.lock().unwrap();
                stream.read(&mut buffer)
            };

            match read_result {
                Ok(0) => {
                    println!("Client {} disconnected", client_id);
                    let _ = stream_arc.lock().unwrap().shutdown(Shutdown::Both);
                }
                Ok(n) => {
                    println!(
                        "Read {} bytes from client {}: {:?}",
                        n, client_id, &buffer[..n]
                    );

                    let write_result = {
                        let mut stream = stream_arc.lock().unwrap();
                        stream.write_all(&buffer[..n])
                    };

                    match write_result {
                        Ok(_) => {
                            println!("Echoed {} bytes back to client {}", n, client_id);
                        }
                        Err(e) => {
                            eprintln!("Error writing to client {}: {:?}", client_id, e);
                        }
                    }

                    follow_up_events.push(Self::create_read_event(client_id, stream_arc.clone()));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    follow_up_events.push(Self::create_read_event(client_id, stream_arc.clone()));
                }
                Err(e) => {
                    eprintln!("Error reading from client {}: {:?}", client_id, e);
                }
            }

            follow_up_events
        })
    }
}

