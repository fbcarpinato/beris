use bumpalo::Bump;
use io_uring::{opcode, types, IoUring};
use nix::unistd::close;
use std::os::fd::AsRawFd;
use std::ptr;

use crate::state::State;

const EVENT_QUEUE_SIZE: u32 = 1024;

enum Operation {
    Accept,

    Read { client_id: usize, client_fd: i32 },

    Write { client_id: usize, client_fd: i32 },
}

pub struct EventLoop {
    io_uring: IoUring,
    state: State,
    alloc: Bump,
}

impl EventLoop {
    pub fn new(state: State) -> Self {
        let io_uring = IoUring::new(EVENT_QUEUE_SIZE).expect("Failed to create io_uring");

        Self {
            io_uring,
            state,
            alloc: Bump::with_capacity(1024 * 1000),
        }
    }

    pub fn run(&mut self) {
        self.submit_accept_operation();

        loop {
            match self.io_uring.submit_and_wait(1) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to submit and wait for io_uring events: {:?}", e);
                    break;
                }
            }

            let completions: Vec<_> = self.io_uring.completion().collect();

            for cqe in completions {
                let res = cqe.result();

                let op_ptr = cqe.user_data() as *mut Operation;
                let op = unsafe { Box::from_raw(op_ptr) };

                match *op {
                    Operation::Accept => {
                        if res < 0 {
                            eprintln!("Accept error: {}", res);
                        } else {
                            let client_fd = res;
                            println!("Accepted client with fd: {}", client_fd);

                            let client_id = self.state.add_client(client_fd);

                            self.submit_read_operation(client_id);
                        }

                        self.submit_accept_operation();
                    }
                    Operation::Read {
                        client_id,
                        client_fd,
                    } => {
                        if res <= 0 {
                            if res == 0 {
                                println!("Client {} disconnected", client_id);
                            } else {
                                eprintln!("Read error on client {}: {}", client_id, res);
                            }
                            let _ = close(client_fd);
                            self.state.remove_client(client_id);
                        } else {
                            let n = res as usize;
                            println!("Read {} bytes from client {}", n, client_id);
                            self.submit_write_operation(client_id);
                        }
                    }
                    Operation::Write {
                        client_id,
                        client_fd: _,
                    } => {
                        println!("Write complete for client {}", client_id);
                        self.submit_read_operation(client_id);
                    }
                }
            }
        }
    }

    fn submit_accept_operation(&mut self) {
        let fd = self.state.listener.as_raw_fd();

        let op = Box::new(Operation::Accept);
        let user_data = Box::into_raw(op) as u64;

        let entry = opcode::Accept::new(types::Fd(fd), ptr::null_mut(), ptr::null_mut())
            .build()
            .user_data(user_data);

        unsafe {
            self.io_uring.submission().push(&entry).unwrap();
        }

        self.io_uring.submit().expect("Submit accept failed");
    }

    fn submit_read_operation(&mut self, client_id: usize) {
        let client_fd = self
            .state
            .get_client(client_id)
            .expect(format!("Failed to find client_fd with client_id {}", client_id).as_str());

        let buffer = self.alloc.alloc([0u8; 1024]);
        let buf_ptr = buffer.as_mut_ptr();

        let op = Box::new(Operation::Read {
            client_id,
            client_fd,
        });
        let user_data = Box::into_raw(op) as u64;

        let entry = opcode::Read::new(types::Fd(client_fd), buf_ptr, 1024)
            .build()
            .user_data(user_data);

        unsafe {
            self.io_uring.submission().push(&entry).unwrap();
        }

        self.io_uring.submit().expect("Submit accept failed");
    }

    fn submit_write_operation(&mut self, client_id: usize) {
        let client_fd = self
            .state
            .get_client(client_id)
            .expect(format!("Failed to find client_fd with client_id {}", client_id).as_str());

        let response = self.alloc.alloc_str("+PONG\r\n");

        let op = Box::new(Operation::Write {
            client_id,
            client_fd,
        });
        let user_data = Box::into_raw(op) as u64;

        let entry = opcode::Write::new(
            types::Fd(client_fd),
            response.as_ptr(),
            response.len() as u32,
        )
        .build()
        .user_data(user_data);

        unsafe {
            self.io_uring.submission().push(&entry).unwrap();
        }

        self.io_uring.submit().expect("Submit accept failed");
    }
}
