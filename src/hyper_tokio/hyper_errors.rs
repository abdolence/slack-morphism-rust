use crate::errors::*;

impl From<hyper::Error> for SlackClientError {
    fn from(hyper_err: hyper::Error) -> Self {
        SlackClientError::HttpProtocolError(
            SlackClientHttpProtocolError::new().with_cause(Box::new(hyper_err)),
        )
    }
}

impl From<hyper::http::Error> for SlackClientError {
    fn from(hyper_err: hyper::http::Error) -> Self {
        SlackClientError::HttpProtocolError(
            SlackClientHttpProtocolError::new().with_cause(Box::new(hyper_err)),
        )
    }
}
