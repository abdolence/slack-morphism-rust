use crate::ratectl::SlackApiRateControlLimit;
use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct ThrottlingCounter {
    capacity: usize,
    max_capacity: usize,
    last_updated: Instant,
    rate_limit_in_millis: u64,
    delay: Duration,
}

impl ThrottlingCounter {
    pub fn new(max_capacity: usize, rate_limit_in_millis: u64) -> Self {
        Self {
            capacity: max_capacity,
            max_capacity,
            last_updated: Instant::now(),
            rate_limit_in_millis,
            delay: Duration::from_millis(0),
        }
    }

    pub fn update(&self, now: Instant) -> Self {
        let time_elapsed_millis = now.duration_since(self.last_updated).as_millis() as u64;

        let (arrived, new_last_updated) = {
            if time_elapsed_millis >= self.rate_limit_in_millis {
                let arrived_in_time = time_elapsed_millis / self.rate_limit_in_millis;
                let new_last_updated = self.last_updated.add(Duration::from_millis(
                    arrived_in_time * self.rate_limit_in_millis,
                ));
                (arrived_in_time as usize, new_last_updated)
            } else {
                (0usize, self.last_updated)
            }
        };

        let new_available_capacity = std::cmp::min(self.capacity + arrived, self.max_capacity);

        if new_available_capacity > 0 {
            Self {
                capacity: new_available_capacity - 1,
                last_updated: new_last_updated,
                delay: Duration::from_millis(0),
                ..self.clone()
            }
        } else {
            let updated_time_elapsed_in_millis =
                now.duration_since(new_last_updated).as_millis() as u64;
            let delay_in_millis = self.rate_limit_in_millis - updated_time_elapsed_in_millis;
            let delay = Duration::from_millis(delay_in_millis);

            Self {
                capacity: 0,
                last_updated: now.add(delay),
                delay,
                ..self.clone()
            }
        }
    }
}

#[test]
fn check_decreased() {
    let rate_limit = SlackApiRateControlLimit::new(15, Duration::from_secs(60));
    let rate_limit_in_ms = rate_limit.to_rate_limit_in_ms();
    let rate_limit_capacity = rate_limit.to_rate_limit_capacity();

    let now = Instant::now();
    let counter = ThrottlingCounter::new(rate_limit_capacity, rate_limit_in_ms);
    let updated_counter = counter.update(now.add(Duration::from_millis(rate_limit_in_ms - 1)));

    assert_eq!(updated_counter.last_updated, counter.last_updated);
    assert_eq!(updated_counter.delay, Duration::from_millis(0));
    assert_eq!(updated_counter.capacity, counter.capacity - 1);
}
