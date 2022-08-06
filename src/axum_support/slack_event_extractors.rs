use crate::errors::*;
use crate::events::{SlackCommandEvent, SlackInteractionEvent, SlackPushEvent};
use crate::AnyStdResult;
use http::Extensions;
use std::collections::HashMap;

pub trait SlackEventExtractor {
    fn extract(&self, verified_body: &String, extensions: &mut Extensions) -> AnyStdResult<()>;
}

pub struct SlackEventExtractors;

impl SlackEventExtractors {
    pub fn empty() -> SlackEventEmptyExtractor {
        SlackEventEmptyExtractor::new()
    }

    pub fn push_event() -> SlackPushEventExtractor {
        SlackPushEventExtractor::new()
    }

    pub fn command_event() -> SlackCommandEventExtractor {
        SlackCommandEventExtractor::new()
    }

    pub fn interaction_event() -> SlackInteractionEventExtractor {
        SlackInteractionEventExtractor::new()
    }
}

#[derive(Clone)]
pub struct SlackEventEmptyExtractor;

impl SlackEventEmptyExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackEventExtractor for SlackEventEmptyExtractor {
    fn extract(&self, _verified_body: &String, _extensions: &mut Extensions) -> AnyStdResult<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct SlackPushEventExtractor;

impl SlackPushEventExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackEventExtractor for SlackPushEventExtractor {
    fn extract(&self, verified_body: &String, extensions: &mut Extensions) -> AnyStdResult<()> {
        let event = serde_json::from_str::<SlackPushEvent>(verified_body.as_str())
            .map_err(|e| SlackClientProtocolError::new(e).with_json_body(verified_body.clone()))?;

        extensions.insert(event);

        Ok(())
    }
}

#[derive(Clone)]
pub struct SlackCommandEventExtractor;

impl SlackCommandEventExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackEventExtractor for SlackCommandEventExtractor {
    fn extract(&self, verified_body: &String, extensions: &mut Extensions) -> AnyStdResult<()> {
        let body_params: HashMap<String, String> =
            url::form_urlencoded::parse(verified_body.as_bytes())
                .into_owned()
                .collect();

        let event: SlackCommandEvent = match (
            body_params.get("team_id"),
            body_params.get("channel_id"),
            body_params.get("user_id"),
            body_params.get("command"),
            body_params.get("text"),
            body_params.get("response_url"),
            body_params.get("trigger_id"),
        ) {
            (
                Some(team_id),
                Some(channel_id),
                Some(user_id),
                Some(command),
                text,
                Some(response_url),
                Some(trigger_id),
            ) => Ok(SlackCommandEvent::new(
                team_id.into(),
                channel_id.into(),
                user_id.into(),
                command.into(),
                url::Url::parse(response_url)?.into(),
                trigger_id.into(),
            )
            .opt_text(text.cloned())),
            _ => Err(SlackClientError::SystemError(
                SlackClientSystemError::new()
                    .with_message("Absent payload in the request from Slack".into()),
            )),
        }?;

        extensions.insert(event);

        Ok(())
    }
}

#[derive(Clone)]
pub struct SlackInteractionEventExtractor;

impl SlackInteractionEventExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackEventExtractor for SlackInteractionEventExtractor {
    fn extract(&self, verified_body: &String, extensions: &mut Extensions) -> AnyStdResult<()> {
        let body_params: HashMap<String, String> =
            url::form_urlencoded::parse(verified_body.as_bytes())
                .into_owned()
                .collect();

        let payload = body_params.get("payload").ok_or_else(|| {
            SlackClientError::SystemError(
                SlackClientSystemError::new()
                    .with_message("Absent payload in the request from Slack".into()),
            )
        })?;

        let event: SlackInteractionEvent =
            serde_json::from_str::<SlackInteractionEvent>(payload)
                .map_err(|e| SlackClientProtocolError::new(e).with_json_body(payload.clone()))?;

        extensions.insert(event);

        Ok(())
    }
}
