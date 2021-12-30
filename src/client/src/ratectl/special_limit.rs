use crate::ratectl::SlackApiRateControlLimit;
use rsb_derive::Builder;

/**
 * Some Slack Web API methods have special rating limits (e.g. chat.postMessage allowed up to 1rps per workspace
 * channel)
 */
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackApiRateControlSpecialLimit {
    key: String,
    limit: SlackApiRateControlLimit,
}
