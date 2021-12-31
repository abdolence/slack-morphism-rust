use slack_morphism::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct SlackTokioRateController {
    pub config: SlackApiRateControlConfig,
    state: Arc<Mutex<SlackTokioRateControllerSharedState>>,
}

#[derive(Debug)]
struct SlackTeamLimits {
    team_limit_counter: Option<ThrottlingCounter>,
    special_limits: HashMap<SlackApiRateControlSpecialLimitKey, ThrottlingCounter>,
    updated: Instant,
}

impl SlackTeamLimits {
    pub fn new(rate_control_config: &SlackApiRateControlConfig) -> Self {
        Self {
            team_limit_counter: rate_control_config
                .team_max_rate_limit
                .clone()
                .map(|rl| rl.to_throttling_counter()),
            special_limits: HashMap::new(),
            updated: Instant::now(),
        }
    }
}

#[derive(Debug)]
struct SlackTokioRateControllerSharedState {
    global_max_rate_limit_counter: Option<ThrottlingCounter>,
    rate_limit_per_team: HashMap<SlackTeamId, SlackTeamLimits>,
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

    pub async fn calc_throttle_delay(
        &self,
        method_rate_ctl: &SlackApiMethodRateControlConfig,
        team_id: Option<SlackTeamId>,
    ) -> Option<Duration> {
        let mut delays_heap: BinaryHeap<Duration> = BinaryHeap::new();
        let now = Instant::now();

        if self.config.global_max_rate_limit.is_some() || team_id.is_some() {
            let mut state = self.state.lock().await;

            state
                .global_max_rate_limit_counter
                .as_ref()
                .map(|c| c.update(now))
                .into_iter()
                .for_each(|updated_counter| {
                    if !updated_counter.delay().is_zero() {
                        delays_heap.push(updated_counter.delay().clone())
                    }
                    state.global_max_rate_limit_counter = Some(updated_counter);
                });

            match team_id {
                Some(team_id) => {
                    let team_limits = state
                        .rate_limit_per_team
                        .entry(team_id)
                        .or_insert(SlackTeamLimits::new(&self.config));
                    team_limits.updated = now;

                    team_limits
                        .team_limit_counter
                        .as_ref()
                        .map(|c| c.update(now))
                        .into_iter()
                        .for_each(|updated_counter| {
                            if !updated_counter.delay().is_zero() {
                                delays_heap.push(updated_counter.delay().clone())
                            }
                            team_limits.team_limit_counter = Some(updated_counter);
                        });

                    match method_rate_ctl.special_rate_limit {
                        Some(ref special_rate_limit) => {
                            let special_team_limit = team_limits
                                .special_limits
                                .entry(special_rate_limit.key.clone())
                                .or_insert(special_rate_limit.limit.to_throttling_counter());

                            *special_team_limit = special_team_limit.update(now);

                            if !special_team_limit.delay().is_zero() {
                                delays_heap.push(special_team_limit.delay().clone())
                            }
                        }
                        None => {} // no need for special rate limiting
                    }

                    // Clean up old teams limits
                    state
                        .rate_limit_per_team
                        .retain(|_, v| v.updated.duration_since(now).as_secs() < 3600);
                }
                None => {} // Nothing to do if team id isn't specified
            }
        }

        delays_heap.pop()
    }
}
