use std::time::Duration;

use tokio::{sync::mpsc, time::Instant};

use crate::network::NetworkRequest;

pub struct Poll<F> {
    last_poll: Instant,
    interval: Duration,
    func: F,
}

impl<F> Poll<F>
where
    F: FnMut(),
{
    pub fn new(interval: Duration, func: F) -> Self {
        Self {
            last_poll: Instant::now(),
            interval,
            func,
        }
    }

    pub fn poll(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_poll) >= self.interval {
            (self.func)();
            self.last_poll = now;
        }
    }
}
