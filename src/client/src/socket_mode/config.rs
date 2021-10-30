use rsb_derive::Builder;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackClientSocketModeConfig {
    #[default = "SlackClientSocketModeConfig::DEFAULT_CONNECTIONS_COUNT"]
    pub max_connections_count: u32,

    #[default = "SlackClientSocketModeConfig::DEFAULT_DEBUG_CONNECTIONS"]
    pub debug_connections: bool,

    #[default = "SlackClientSocketModeConfig::DEFAULT_INITIAL_BACKOFF_IN_SECONDS"]
    pub initial_backoff_in_seconds: u64,

    #[default = "SlackClientSocketModeConfig::DEFAULT_RECONNECT_TIMEOUT_IN_SECONDS"]
    pub reconnect_timeout_in_seconds: u64,
}

impl SlackClientSocketModeConfig {
    pub const DEFAULT_CONNECTIONS_COUNT: u32 = 2;

    pub const DEFAULT_DEBUG_CONNECTIONS: bool = false;

    pub const DEFAULT_INITIAL_BACKOFF_IN_SECONDS: u64 = 5;

    pub const DEFAULT_RECONNECT_TIMEOUT_IN_SECONDS: u64 = 30;
}
