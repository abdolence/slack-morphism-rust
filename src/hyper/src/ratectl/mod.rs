use slack_morphism::prelude::SlackApiRateControlConfig;

#[derive(Clone, Debug)]
pub struct SlackTokioRateController {}

impl SlackTokioRateController {
    pub fn new(rate_control_config: SlackApiRateControlConfig) -> Self {
        Self {}
    }
}
