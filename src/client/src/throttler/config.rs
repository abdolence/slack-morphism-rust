use crate::throttler::{SlackApiMethodRateTier, SlackApiRateControlSpecialLimit};
use rsb_derive::Builder;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackApiMethodRateControlConfig {
    tier: Option<SlackApiMethodRateTier>,
    special_rate_limit: Option<SlackApiRateControlSpecialLimit>,
}
