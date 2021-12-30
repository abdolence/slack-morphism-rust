use crate::ratectl::{
    SlackApiMethodRateTier, SlackApiRateControlLimit, SlackApiRateControlSpecialLimit,
};
use lazy_static::lazy_static;
use rsb_derive::Builder;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackApiMethodRateControlConfig {
    pub tier: Option<SlackApiMethodRateTier>,
    pub special_rate_limit: Option<SlackApiRateControlSpecialLimit>,
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackApiRateControlConfig {
    pub global_max_rate_limit: Option<SlackApiRateControlLimit>,
    pub workspace_max_rate_limit: Option<SlackApiRateControlLimit>,

    #[default = "DEFAULT_TIERS_LIMIT_MAP.clone()"]
    pub slack_api_tier_limits: HashMap<SlackApiMethodRateTier, SlackApiRateControlLimit>,

    pub max_delay_timeout: Option<chrono::Duration>,
    pub max_retries: Option<usize>,
}

impl SlackApiRateControlConfig {}

lazy_static! {
    pub static ref DEFAULT_TIERS_LIMIT_MAP: HashMap<SlackApiMethodRateTier, SlackApiRateControlLimit> =
        vec![
            (
                SlackApiMethodRateTier::Tier1,
                SlackApiRateControlLimit::new(1, chrono::Duration::minutes(1))
            ),
            (
                SlackApiMethodRateTier::Tier2,
                SlackApiRateControlLimit::new(20, chrono::Duration::minutes(1))
            ),
            (
                SlackApiMethodRateTier::Tier3,
                SlackApiRateControlLimit::new(50, chrono::Duration::minutes(1))
            ),
            (
                SlackApiMethodRateTier::Tier4,
                SlackApiRateControlLimit::new(100, chrono::Duration::minutes(1))
            )
        ]
        .into_iter()
        .collect();
}
