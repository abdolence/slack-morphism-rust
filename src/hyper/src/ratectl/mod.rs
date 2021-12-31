use slack_morphism::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct SlackTokioRateController {
    has_global_limits: bool,
    throttler: Arc<Mutex<SlackRateThrottler>>,
}

impl SlackTokioRateController {
    pub fn new(rate_control_config: SlackApiRateControlConfig) -> Self {
        let has_global_limits = rate_control_config.global_max_rate_limit.is_some();

        Self {
            has_global_limits,
            throttler: Arc::new(Mutex::new(SlackRateThrottler::new(rate_control_config))),
        }
    }

    pub async fn calc_throttle_delay(
        &self,
        method_rate_ctl: &SlackApiMethodRateControlConfig,
        team_id: Option<SlackTeamId>,
    ) -> Option<Duration> {
        if self.has_global_limits || team_id.is_some() {
            let mut throttler = self.throttler.lock().await;
            throttler.calc_throttle_delay(method_rate_ctl, team_id)
        } else {
            None
        }
    }
}
