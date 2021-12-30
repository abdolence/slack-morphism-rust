/**
 * A rate limit definition
*/
#[derive(Debug, PartialEq, Clone)]
pub struct SlackApiRateControlLimit {
    pub value: usize,
    pub per: chrono::Duration,
}

impl SlackApiRateControlLimit {
    pub fn new(value: usize, per: chrono::Duration) -> Self {
        assert!(value > 0, "Limit value should be more than zero");
        assert!(
            per.num_milliseconds() > 0,
            "Limit duration should be more than zero"
        );

        Self { value, per }
    }

    pub fn to_rate_limit_in_ms(&self) -> u64 {
        self.per.num_milliseconds() as u64 / self.value as u64
    }
}
