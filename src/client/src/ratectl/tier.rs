use crate::ratectl::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum SlackApiMethodRateTier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

lazy_static! {
    pub static ref SLACK_TIERS_DEFAULT_LIMITS_MAP: HashMap<SlackApiMethodRateTier, SlackApiRateControlLimit> =
        vec![
            (
                SlackApiMethodRateTier::Tier1,
                SlackApiRateControlLimit::new(1, std::time::Duration::from_secs(60))
            ),
            (
                SlackApiMethodRateTier::Tier2,
                SlackApiRateControlLimit::new(20, std::time::Duration::from_secs(60))
            ),
            (
                SlackApiMethodRateTier::Tier3,
                SlackApiRateControlLimit::new(50, std::time::Duration::from_secs(60))
            ),
            (
                SlackApiMethodRateTier::Tier4,
                SlackApiRateControlLimit::new(100, std::time::Duration::from_secs(60))
            )
        ]
        .into_iter()
        .collect();
}
