use slack_morphism::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct SlackTokioRateController {
    pub config: SlackApiRateControlConfig,
    throttler: Arc<Mutex<SlackRateThrottler>>,
}

impl SlackTokioRateController {
    pub fn new(rate_control_config: SlackApiRateControlConfig) -> Self {
        Self {
            config: rate_control_config.clone(),
            throttler: Arc::new(Mutex::new(SlackRateThrottler::new(rate_control_config))),
        }
    }

    pub async fn calc_throttle_delay(
        &self,
        method_rate_ctl: Option<&SlackApiMethodRateControlConfig>,
        team_id: Option<SlackTeamId>,
        delayed: Option<Duration>,
    ) -> Option<Duration> {
        let has_global_limits = self.config.global_max_rate_limit.is_some();

        let delayed_by_state = if has_global_limits || team_id.is_some() {
            if let Some(exist_method_rate_ctl) = method_rate_ctl {
                let mut throttler = self.throttler.lock().await;
                throttler.calc_throttle_delay(exist_method_rate_ctl, team_id)
            } else {
                None
            }
        } else {
            None
        };

        delayed_by_state.max(delayed)
    }
}
