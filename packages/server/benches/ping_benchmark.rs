#![feature(test)]
extern crate test;

use std::io::{Read, Write};
use std::net::TcpStream;
use test::Bencher;

#[bench]
fn bench_set_command(b: &mut Bencher) {
    b.iter(|| {
        let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
        let command = "PING";
        stream.write_all(command.as_bytes()).unwrap();

        let mut buffer = [0; 128];
        stream.read(&mut buffer).unwrap();
    });
}
