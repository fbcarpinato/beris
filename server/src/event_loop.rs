use std::collections::VecDeque;

pub struct Event {
    callback: Box<dyn Fn() -> Vec<Event>>
}


impl Event {
    pub fn new<F>(callback: F) -> Self
    where
        F: 'static + Fn() -> Vec<Event>,
    {
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

    pub fn run(&mut self) {
        while !self.queue.is_empty() {
            let task = self.queue.pop_front();

            match task {
                Some(task) => {
                    let events = (task.callback)();

                    self.queue.extend(events);
                },
                None => {}
            }
        }
    }
}
