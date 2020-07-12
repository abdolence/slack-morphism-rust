pub enum SlackDateTimeFormats {
    DateNum,
    Date,
    DateShort,
    DateLong,
    DatePretty,
    DateShortPretty,
    DateLongPretty,
    Time,
    TimeSecs,
}

impl ToString for SlackDateTimeFormats {
    fn to_string(&self) -> String {
        match self {
            SlackDateTimeFormats::DateNum => "{date_num}".into(),
            SlackDateTimeFormats::Date => "{date}".into(),
            SlackDateTimeFormats::DateShort => "{date_short}".into(),
            SlackDateTimeFormats::DateLong => "{date_long}".into(),
            SlackDateTimeFormats::DatePretty => "{date_pretty}".into(),
            SlackDateTimeFormats::DateShortPretty => "{date_short_pretty}".into(),
            SlackDateTimeFormats::DateLongPretty => "{date_long_pretty}".into(),
            SlackDateTimeFormats::Time => "{time}".into(),
            SlackDateTimeFormats::TimeSecs => "{time_secs}".into(),
        }
    }
}
