use slack_morphism::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct SlackTokioRateController {
    pub config: SlackApiRateControlConfig,
    state: Arc<Mutex<SlackTokioRateControllerSharedState>>,
}

struct TeamLimits {
    team_limit_counter: ThrottlingCounter,
    special_limits: HashMap<SlackApiRateControlSpecialLimitKey, ThrottlingCounter>,
    updated: Instant,
}

#[derive(Debug)]
struct SlackTokioRateControllerSharedState {
    global_max_rate_limit_counter: Option<ThrottlingCounter>,
    rate_limit_per_team: HashMap<SlackTeamId, ThrottlingCounter>,
}

impl SlackTokioRateController {
    pub fn new(rate_control_config: SlackApiRateControlConfig) -> Self {
        let initial_state = SlackTokioRateControllerSharedState {
            global_max_rate_limit_counter: rate_control_config
                .global_max_rate_limit
                .clone()
                .map(|rl| rl.to_throttling_counter()),
            rate_limit_per_team: HashMap::new(),
        };

        Self {
            config: rate_control_config.clone(),
            state: Arc::new(Mutex::new(initial_state)),
        }
    }

    pub fn calc_throttle_delay(
        &self,
        method_rate_ctl: &SlackApiMethodRateControlConfig,
        team_id: Option<SlackTeamId>,
    ) -> Duration {
        todo!()
    }
}
