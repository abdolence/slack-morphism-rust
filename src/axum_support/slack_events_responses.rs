use crate::events::SlackInteractionResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

impl IntoResponse for SlackInteractionResponse {
    fn into_response(self) -> Response {
        match self {
            SlackInteractionResponse::Empty => StatusCode::OK.into_response(),
            payload => axum::Json(payload).into_response(),
        }
    }
}
