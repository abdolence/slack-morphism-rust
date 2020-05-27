use hyper::StatusCode;
use rsb_derive::Builder;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum SlackClientError {
    ApiError(SlackClientApiError),
    HttpError(SlackClientHttpError),
}

impl Display for SlackClientError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            SlackClientError::ApiError(ref err) => err.fmt(f),
            SlackClientError::HttpError(ref err) => err.fmt(f),
        }
    }
}

impl Error for SlackClientError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SlackClientError::ApiError(ref err) => Some(err),
            SlackClientError::HttpError(ref err) => Some(err),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackClientApiError {
    pub code: String,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

impl Display for SlackClientApiError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Slack API error: {}", self)
    }
}

impl Error for SlackClientApiError {}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackClientHttpError {
    pub status_code: StatusCode,
    pub http_response_body: Option<String>,
}

impl Display for SlackClientHttpError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Slack HTTP error: {}", self)
    }
}

impl std::error::Error for SlackClientHttpError {}
