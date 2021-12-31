use crate::ratectl::*;
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
    pub team_max_rate_limit: Option<SlackApiRateControlLimit>,

    #[default = "SLACK_TIERS_DEFAULT_LIMITS_MAP.clone()"]
    pub tiers_limits: HashMap<SlackApiMethodRateTier, SlackApiRateControlLimit>,

    pub max_delay_timeout: Option<std::time::Duration>,
    pub max_retries: Option<usize>,
}

impl SlackApiRateControlConfig {}

lazy_static! {
    pub static ref SLACK_TIER1_METHOD_CONFIG: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_tier(SlackApiMethodRateTier::Tier1);
    pub static ref SLACK_TIER2_METHOD_CONFIG: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_tier(SlackApiMethodRateTier::Tier1);
    pub static ref SLACK_TIER3_METHOD_CONFIG: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_tier(SlackApiMethodRateTier::Tier1);
    pub static ref SLACK_TIER4_METHOD_CONFIG: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_tier(SlackApiMethodRateTier::Tier1);
}
