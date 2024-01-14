use crate::errors::*;

impl From<http::Error> for SlackClientError {
    fn from(hyper_err: http::Error) -> Self {
        SlackClientError::HttpProtocolError(
            SlackClientHttpProtocolError::new().with_cause(Box::new(hyper_err)),
        )
    }
}

impl From<hyper_util::client::legacy::Error> for SlackClientError {
    fn from(hyper_err: hyper_util::client::legacy::Error) -> Self {
        SlackClientError::HttpProtocolError(
            SlackClientHttpProtocolError::new().with_cause(Box::new(hyper_err)),
        )
    }
}
