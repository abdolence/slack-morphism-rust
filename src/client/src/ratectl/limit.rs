use crate::prelude::ThrottlingCounter;

/**
 * A rate limit definition
*/
#[derive(Debug, PartialEq, Clone)]
pub struct SlackApiRateControlLimit {
    pub value: usize,
    pub per: std::time::Duration,
}

impl SlackApiRateControlLimit {
    pub fn new(value: usize, per: std::time::Duration) -> Self {
        assert!(value > 0, "Limit value should be more than zero");
        assert!(
            per.as_millis() > 0,
            "Limit duration should be more than zero"
        );

        Self { value, per }
    }

    pub fn to_rate_limit_in_ms(&self) -> u64 {
        self.per.as_millis() as u64 / self.value as u64
    }

    pub fn to_rate_limit_capacity(&self) -> usize {
        self.per.as_millis() as usize / self.to_rate_limit_in_ms() as usize
    }

    pub fn to_throttling_counter(&self) -> ThrottlingCounter {
        ThrottlingCounter::new(self.to_rate_limit_capacity(), self.to_rate_limit_in_ms())
    }
}
