use crate::ratectl::*;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
pub struct SlackTeamLimits {
    pub team_limit_counter: Option<ThrottlingCounter>,
    pub tier_limits: HashMap<SlackApiMethodRateTier, ThrottlingCounter>,
    pub special_limits: HashMap<SlackApiRateControlSpecialLimitKey, ThrottlingCounter>,
    pub updated: Instant,
}

impl SlackTeamLimits {
    pub fn new(rate_control_config: &SlackApiRateControlConfig) -> Self {
        Self {
            team_limit_counter: rate_control_config
                .team_max_rate_limit
                .clone()
                .map(|rl| rl.to_throttling_counter()),
            tier_limits: HashMap::new(),
            special_limits: HashMap::new(),
            updated: Instant::now(),
        }
    }
}
