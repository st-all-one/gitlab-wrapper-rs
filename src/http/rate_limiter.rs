use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub(crate) struct SlidingWindow {
    max_requests: u32,
    window: Duration,
    timestamps: VecDeque<Instant>,
}

impl SlidingWindow {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            timestamps: VecDeque::with_capacity(max_requests as usize),
        }
    }

    pub fn acquire(&mut self) {
        let now = Instant::now();
        let cutoff = now - self.window;

        while let Some(&ts) = self.timestamps.front() {
            if ts < cutoff {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        if self.timestamps.len() >= self.max_requests as usize {
            if let Some(&oldest) = self.timestamps.front() {
                let wait = self.window.saturating_sub(now - oldest);
                if !wait.is_zero() {
                    log::debug!(target: "gitlab_wrapper::rate_limiter", "Rate limit reached, waiting {wait:?}");
                    std::thread::sleep(wait);
                    let now = Instant::now();
                    let cutoff = now - self.window;
                    while let Some(&ts) = self.timestamps.front() {
                        if ts < cutoff {
                            self.timestamps.pop_front();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        self.timestamps.push_back(Instant::now());
    }
}
