use rsb_derive::Builder;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::time::Duration;
use url::ParseError;

#[derive(Debug)]
pub enum SlackClientError {
    ApiError(SlackClientApiError),
    HttpError(SlackClientHttpError),
    HttpProtocolError(SlackClientHttpProtocolError),
    EndOfStream(SlackClientEndOfStreamError),
    SystemError(SlackClientSystemError),
    ProtocolError(SlackClientProtocolError),
    SocketModeProtocolError(SlackClientSocketModeProtocolError),
    RateLimitError(SlackRateLimitError),
}

impl SlackClientError {
    fn option_to_string<T: ToString>(value: &Option<T>) -> String {
        value
            .as_ref()
            .map_or_else(|| "-".to_string(), |v| v.to_string())
    }
}

impl Display for SlackClientError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            SlackClientError::ApiError(ref err) => err.fmt(f),
            SlackClientError::HttpError(ref err) => err.fmt(f),
            SlackClientError::HttpProtocolError(ref err) => err.fmt(f),
            SlackClientError::EndOfStream(ref err) => err.fmt(f),
            SlackClientError::ProtocolError(ref err) => err.fmt(f),
            SlackClientError::SocketModeProtocolError(ref err) => err.fmt(f),
            SlackClientError::SystemError(ref err) => err.fmt(f),
            SlackClientError::RateLimitError(ref err) => err.fmt(f),
        }
    }
}

impl Error for SlackClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SlackClientError::ApiError(ref err) => Some(err),
            SlackClientError::HttpError(ref err) => Some(err),
            SlackClientError::HttpProtocolError(ref err) => Some(err),
            SlackClientError::EndOfStream(ref err) => Some(err),
            SlackClientError::ProtocolError(ref err) => Some(err),
            SlackClientError::SocketModeProtocolError(ref err) => Some(err),
            SlackClientError::SystemError(ref err) => Some(err),
            SlackClientError::RateLimitError(ref err) => Some(err),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackClientApiError {
    pub code: String,
    pub errors: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

impl Display for SlackClientApiError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack API error: {}\nBody: '{}'",
            self.code,
            SlackClientError::option_to_string(&self.http_response_body)
        )
    }
}

impl Error for SlackClientApiError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackClientHttpError {
    pub status_code: http::StatusCode,
    pub http_response_body: Option<String>,
}

impl Display for SlackClientHttpError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack HTTP error status: {}. Body: '{}'",
            self.status_code,
            SlackClientError::option_to_string(&self.http_response_body)
        )
    }
}

impl std::error::Error for SlackClientHttpError {}

#[derive(Debug, Builder)]
pub struct SlackClientHttpProtocolError {
    pub cause: Option<Box<dyn std::error::Error + Sync + Send>>,
}

impl Display for SlackClientHttpProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Slack http protocol error: {:?}", self.cause)
    }
}

impl std::error::Error for SlackClientHttpProtocolError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackClientEndOfStreamError {}

impl Display for SlackClientEndOfStreamError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Slack end of stream error")
    }
}

impl std::error::Error for SlackClientEndOfStreamError {}

#[derive(Debug, Builder)]
pub struct SlackClientProtocolError {
    pub json_error: serde_json::Error,
    pub json_body: Option<String>,
}

impl Display for SlackClientProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack JSON protocol error: {}. Body: '{}'",
            self.json_error,
            SlackClientError::option_to_string(&self.json_body)
        )
    }
}

impl std::error::Error for SlackClientProtocolError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackClientSocketModeProtocolError {
    pub message: String,
}

impl Display for SlackClientSocketModeProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Slack socket mode protocol error: {}", self.message)
    }
}

impl std::error::Error for SlackClientSocketModeProtocolError {}

#[derive(Debug, Builder)]
pub struct SlackClientSystemError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error + Sync + Send + 'static>>,
}

impl Display for SlackClientSystemError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack system protocol error. {}{:?}",
            self.message.as_ref().unwrap_or(&"".to_string()),
            self.cause
        )
    }
}

impl std::error::Error for SlackClientSystemError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackRateLimitError {
    pub retry_after: Option<Duration>,
    pub code: Option<String>,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

impl Display for SlackRateLimitError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack API rate limit error: {}\nBody: '{}'. Retry after: `{:?}`",
            SlackClientError::option_to_string(&self.code),
            SlackClientError::option_to_string(&self.http_response_body),
            self.retry_after,
        )
    }
}

impl Error for SlackRateLimitError {}

impl From<url::ParseError> for SlackClientError {
    fn from(url_parse_error: ParseError) -> Self {
        SlackClientError::HttpProtocolError(
            SlackClientHttpProtocolError::new().with_cause(Box::new(url_parse_error)),
        )
    }
}

impl From<Box<dyn std::error::Error + Sync + Send>> for SlackClientError {
    fn from(err: Box<dyn Error + Sync + Send>) -> Self {
        SlackClientError::SystemError(SlackClientSystemError::new().with_cause(err))
    }
}

pub fn map_serde_error(err: serde_json::Error, tried_to_parse: Option<&str>) -> SlackClientError {
    SlackClientError::ProtocolError(
        SlackClientProtocolError::new(err).opt_json_body(tried_to_parse.map(|s| s.to_string())),
    )
}
