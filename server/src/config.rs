use std::net::{SocketAddr, ToSocketAddrs};

pub struct Config {
    addr: SocketAddr,
}

impl Config {
    pub fn new<A>(addr: A) -> Self
    where
        A: ToSocketAddrs,
    {
        let addr = addr.to_socket_addrs()
            .expect("Failed to resolve address")
            .next()
            .expect("No valid address found");

        Config { addr }
    }

    pub fn get_addr(&self) -> &SocketAddr {
       &self.addr
    }
}
