pub trait SlackTextFormat {
    fn to_slack_format(&self) -> String;
}
