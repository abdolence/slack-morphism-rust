use rsb_derive::Builder;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackClientSocketModeConfig {
    #[default = "SlackClientSocketModeConfig::DEFAULT_CONNECTIONS_COUNT"]
    pub max_connections_count: u8,
}

impl SlackClientSocketModeConfig {
    pub const DEFAULT_CONNECTIONS_COUNT: u8 = 1;
}
