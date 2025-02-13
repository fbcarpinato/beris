use smallvec::SmallVec;
use arraydeque::{ArrayDeque, Wrapping};

const EVENT_QUEUE_SIZE: usize = 1024;

pub struct Event {
    callback: Box<dyn Fn() -> SmallVec<[Event; 4]>>
}


impl Event {
    pub fn new<F>(callback: F) -> Self
    where
        F: 'static + Fn() -> SmallVec<[Event; 4]>,
    {
        Self {
            callback: Box::new(callback)
        }
    }
}

pub struct EventLoop {
    queue: ArrayDeque<Event, EVENT_QUEUE_SIZE, Wrapping>,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            queue: ArrayDeque::new()
        }
    }

    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    pub fn run(&mut self) {
        let mut temp_queue: ArrayDeque<Event, EVENT_QUEUE_SIZE, Wrapping> = ArrayDeque::new();
        std::mem::swap(&mut self.queue, &mut temp_queue);

        for event in temp_queue {
            let events = (event.callback)();

            self.queue.extend(events);
        }
    }
}
