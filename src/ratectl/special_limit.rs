use crate::ratectl::SlackApiRateControlLimit;
use rsb_derive::Builder;
use rvstruct::*;

/**
 * Some Slack Web API methods have special rating limits (e.g. chat.postMessage allowed up to 1rps per workspace
 * channel)
 */
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackApiRateControlSpecialLimit {
    pub key: SlackApiRateControlSpecialLimitKey,
    pub limit: SlackApiRateControlLimit,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, ValueStruct)]
pub struct SlackApiRateControlSpecialLimitKey(pub String);
