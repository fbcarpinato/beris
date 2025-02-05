use std::collections::VecDeque;

pub struct Event {
    callback: Box<dyn Fn()>
}


impl Event {
    pub fn new<F: 'static + Fn()>(callback: F) -> Self {
        Self {
            callback: Box::new(callback)
        }
    }
}

pub struct EventLoop {
    queue: VecDeque<Event>
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }

    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }
}
