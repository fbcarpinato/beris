use arraydeque::{ArrayDeque, Wrapping};
use io_uring::{opcode, types, IoUring};
use nix::sys::eventfd::{EfdFlags, EventFd};
use nix::unistd::write;
use smallvec::SmallVec;
use std::os::fd::{AsFd, AsRawFd};
use std::sync::{Arc, Mutex};

const EVENT_QUEUE_SIZE: usize = 1024;

pub struct Event {
    callback: Box<dyn Fn() -> SmallVec<[Event; 4]> + Send + Sync>,
}

impl Event {
    pub fn new<F>(callback: F) -> Self
    where
        F: 'static + Fn() -> SmallVec<[Event; 4]> + Send + Sync,
    {
        Self {
            callback: Box::new(callback),
        }
    }
}

pub struct EventLoop {
    queue: Arc<Mutex<ArrayDeque<Event, EVENT_QUEUE_SIZE, Wrapping>>>,
    event_fd: EventFd,
    io_uring: IoUring,
    event_buffer: [u8; 8],
}

impl EventLoop {
    pub fn new() -> Self {
        let event_fd = EventFd::from_value_and_flags(0, EfdFlags::EFD_NONBLOCK)
            .expect("Failed to create eventfd");
        let io_uring = IoUring::new(32).expect("Failed to create io_uring");

        let mut event_loop = Self {
            queue: Arc::new(Mutex::new(ArrayDeque::new())),
            event_fd,
            io_uring,
            event_buffer: [0u8; 8],
        };

        event_loop.arm_io_uring().expect("Failed to arm io_uring");
        event_loop
    }

    pub fn push(&self, event: Event) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(event);

        let one: u64 = 1;

        let event_fd = self.event_fd.as_fd();

        if let Err(e) = write(event_fd, &one.to_ne_bytes()) {
            eprintln!("Failed to signal eventfd: {:?}", e);
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.io_uring.submit_and_wait(1) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to submit and wait for io_uring events: {:?}", e);
                    break;
                }
            }

            let cqes: Vec<_> = self.io_uring.completion().collect();

            for cqe in cqes {
                if cqe.user_data() == 0 {
                    if cqe.result() < 0 {
                        eprintln!("Error in eventfd read: {}", cqe.result());
                        break;
                    }

                    self.process_events();

                    if let Err(e) = self.arm_io_uring() {
                        eprintln!("Failed to re-arm io_uring: {:?}", e);
                        break;
                    }
                }
            }
        }
    }

    fn process_events(&mut self) {
        let temp_queue = {
            let mut queue = self.queue.lock().unwrap();
            std::mem::replace(&mut *queue, ArrayDeque::new())
        };

        for event in temp_queue {
            let new_events = (event.callback)();
            for event in new_events {
                self.push(event);
            }
        }
    }

    fn arm_io_uring(&mut self) -> Result<usize, String> {
        self.event_buffer = [0u8; 8];

        unsafe {
            let sqe = opcode::Read::new(
                types::Fd(self.event_fd.as_fd().as_raw_fd()),
                self.event_buffer.as_mut_ptr(),
                self.event_buffer.len() as _,
            )
            .build()
            .user_data(0);

            match self.io_uring.submission().push(&sqe) {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to push SQE to io_uring: {:?}", e)),
            }
        }

        self.io_uring
            .submit()
            .map_err(|e| format!("Failed to submit io_uring: {:?}", e))
    }
}
