use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

use slab::Slab;

pub struct State {
    pub listener: Arc<TcpListener>,
    pub connected_clients: Arc<Mutex<Slab<i32>>>,
}

impl State {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            connected_clients: Arc::new(Mutex::new(Slab::new())),
            listener: Arc::new(listener),
        }
    }

    pub fn add_client(&self, fd: i32) -> usize {
        let mut clients = self.connected_clients.lock().unwrap();
        clients.insert(fd)
    }

    pub fn remove_client(&mut self, client_id: usize) {
        let mut clients = self.connected_clients.lock().unwrap();
        clients.remove(client_id);
    }

    pub fn get_client(&self, client_id: usize) -> Option<i32> {
        let clients = self.connected_clients.lock().unwrap();
        clients.get(client_id).copied()
    }
}
