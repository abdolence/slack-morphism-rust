use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct ThrottlingCounter {
    capacity: i64,
    max_capacity: usize,
    last_arrived: Instant,
    last_updated: Instant,
    rate_limit_in_millis: u64,
    delay: Duration,
}

impl ThrottlingCounter {
    pub fn new(max_capacity: usize, rate_limit_in_millis: u64) -> Self {
        Self {
            capacity: max_capacity as i64,
            max_capacity,
            last_arrived: Instant::now(),
            last_updated: Instant::now(),
            rate_limit_in_millis,
            delay: Duration::from_millis(0),
        }
    }

    pub fn update(&self, now: Instant) -> Self {
        let time_elapsed_millis = now
            .checked_duration_since(self.last_arrived)
            .unwrap_or_else(|| Duration::from_millis(0))
            .as_millis() as u64;

        let (arrived, new_last_arrived) = {
            if time_elapsed_millis >= self.rate_limit_in_millis {
                let arrived_in_time = time_elapsed_millis / self.rate_limit_in_millis;
                let new_last_updated = self.last_arrived.add(Duration::from_millis(
                    arrived_in_time * self.rate_limit_in_millis,
                ));
                (arrived_in_time as usize, new_last_updated)
            } else {
                (0usize, self.last_arrived)
            }
        };

        let new_available_capacity =
            std::cmp::min(self.capacity + arrived as i64, self.max_capacity as i64);

        if new_available_capacity > 0 {
            Self {
                capacity: new_available_capacity - 1,
                last_arrived: new_last_arrived,
                last_updated: now,
                delay: Duration::from_millis(0),
                ..self.clone()
            }
        } else {
            let updated_capacity = new_available_capacity - 1;

            let base_delay_in_millis = now
                .checked_duration_since(self.last_updated)
                .map_or(0u64, |d| d.as_millis() as u64);

            let delay_penalty = (self.rate_limit_in_millis as f64 * self.capacity.abs() as f64
                / self.max_capacity as f64) as u64;

            let delay_in_millis = self
                .rate_limit_in_millis
                .saturating_sub(base_delay_in_millis);
            let delay_with_penalty = Duration::from_millis(delay_in_millis + delay_penalty);

            Self {
                capacity: updated_capacity,
                last_arrived: new_last_arrived,
                last_updated: now,
                delay: delay_with_penalty,
                ..self.clone()
            }
        }
    }

    pub fn delay(&self) -> &Duration {
        &self.delay
    }
}

#[test]
fn check_decreased() {
    use crate::ratectl::*;
    let rate_limit = SlackApiRateControlLimit::new(15, Duration::from_secs(60));
    let rate_limit_in_ms = rate_limit.to_rate_limit_in_ms();
    let rate_limit_capacity = rate_limit.to_rate_limit_capacity();

    let now = Instant::now();
    let counter = ThrottlingCounter::new(rate_limit_capacity, rate_limit_in_ms);
    let updated_counter = counter.update(now.add(Duration::from_millis(rate_limit_in_ms - 1)));

    assert_eq!(updated_counter.last_arrived, counter.last_arrived);
    assert_eq!(updated_counter.delay, Duration::from_millis(0));
    assert_eq!(updated_counter.capacity, counter.capacity - 1);
}

#[test]
fn check_max_available() {
    use crate::ratectl::*;
    let rate_limit = SlackApiRateControlLimit::new(15, Duration::from_secs(60));
    let rate_limit_in_ms = rate_limit.to_rate_limit_in_ms();
    let rate_limit_capacity = rate_limit.to_rate_limit_capacity();

    let now = Instant::now();
    let counter = ThrottlingCounter::new(rate_limit_capacity, rate_limit_in_ms);
    let updated_counter = counter.update(now.add(Duration::from_millis(rate_limit_in_ms + 1)));

    assert_eq!(updated_counter.delay, Duration::from_millis(0));
    assert_eq!(updated_counter.capacity, (counter.max_capacity - 1) as i64);
}

#[test]
fn check_delay() {
    use crate::ratectl::*;
    let counter =
        SlackApiRateControlLimit::new(15, Duration::from_secs(60)).to_throttling_counter();

    let now = Instant::now();

    let updated_counter =
        (0..counter.capacity + 1).fold(counter.clone(), |result, _| result.update(now));

    assert_eq!(
        updated_counter.delay,
        Duration::from_millis(counter.rate_limit_in_millis)
    );
}
