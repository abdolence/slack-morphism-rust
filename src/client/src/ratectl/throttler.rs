use crate::ratectl::*;
use slack_morphism_models::SlackTeamId;
use std::collections::{BinaryHeap, HashMap};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct SlackRateThrottler {
    pub config: SlackApiRateControlConfig,
    global_max_rate_limit_counter: Option<ThrottlingCounter>,
    rate_limit_per_team: HashMap<SlackTeamId, SlackTeamLimits>,
}

impl SlackRateThrottler {
    pub fn new(rate_control_config: SlackApiRateControlConfig) -> Self {
        let global_max_rate_limit_counter = rate_control_config
            .global_max_rate_limit
            .clone()
            .map(|rl| rl.to_throttling_counter());
        Self {
            config: rate_control_config,
            global_max_rate_limit_counter,
            rate_limit_per_team: HashMap::new(),
        }
    }

    pub fn calc_throttle_delay(
        &mut self,
        method_rate_ctl: &SlackApiMethodRateControlConfig,
        team_id: Option<SlackTeamId>,
    ) -> Option<Duration> {
        let mut delays_heap: BinaryHeap<Duration> = BinaryHeap::new();
        let now = Instant::now();

        self.global_max_rate_limit_counter
            .as_ref()
            .map(|c| c.update(now))
            .into_iter()
            .for_each(|updated_counter| {
                if !updated_counter.delay().is_zero() {
                    delays_heap.push(updated_counter.delay().clone())
                }
                self.global_max_rate_limit_counter = Some(updated_counter);
            });

        match team_id {
            Some(team_id) => {
                let team_limits = self
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
                self.rate_limit_per_team
                    .retain(|_, v| v.updated.duration_since(now).as_secs() < 3600);
            }
            None => {} // Nothing to do if team id isn't specified
        }

        delays_heap.pop()
    }
}
