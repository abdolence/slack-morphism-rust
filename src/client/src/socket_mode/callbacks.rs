use crate::errors::*;
use crate::listener::SlackClientEventsUserState;
use crate::prelude::{SlackInteractionEvent, SlackPushEventCallback, UserCallbackFunction};
use crate::{SlackClient, SlackClientHttpConnector, UserCallbackResult};
use futures::future::BoxFuture;
use slack_morphism_models::events::{SlackCommandEvent, SlackCommandEventResponse};
use slack_morphism_models::socket_mode::SlackSocketModeHelloEvent;
use std::future::Future;
use std::sync::Arc;
use tracing::*;

pub trait SlackSocketModeListenerCallback<SCHC, RQ, RS>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
    RQ: Send + Sync + 'static,
    RS: Send + Sync + 'static,
{
    fn call(
        &self,
        ev: RQ,
        client: Arc<SlackClient<SCHC>>,
        state_storage: SlackClientEventsUserState,
    ) -> BoxFuture<'static, RS>;
}

impl<T, F, SCHC, RQ, RS> SlackSocketModeListenerCallback<SCHC, RQ, RS> for T
where
    T: Send + Sync + Fn(RQ, Arc<SlackClient<SCHC>>, SlackClientEventsUserState) -> F,
    F: Future<Output = RS> + Send + 'static,
    SCHC: SlackClientHttpConnector + Send + Sync,
    RQ: Send + Sync + 'static,
    RS: Send + Sync + 'static,
{
    fn call(
        &self,
        ev: RQ,
        client: Arc<SlackClient<SCHC>>,
        state_storage: SlackClientEventsUserState,
    ) -> BoxFuture<'static, RS> {
        Box::pin(self(ev, client, state_storage))
    }
}

pub struct SlackSocketModeListenerCallbacks<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    pub hello_callback:
        Box<dyn SlackSocketModeListenerCallback<SCHC, SlackSocketModeHelloEvent, ()> + Send + Sync>,

    pub command_callback: Box<
        dyn SlackSocketModeListenerCallback<
                SCHC,
                SlackCommandEvent,
                UserCallbackResult<SlackCommandEventResponse>,
            > + Send
            + Sync,
    >,
    pub interaction_callback: Box<
        dyn SlackSocketModeListenerCallback<SCHC, SlackInteractionEvent, UserCallbackResult<()>>
            + Send
            + Sync,
    >,
    pub push_events_callback: Box<
        dyn SlackSocketModeListenerCallback<SCHC, SlackPushEventCallback, UserCallbackResult<()>>
            + Send
            + Sync,
    >,
}

impl<SCHC> SlackSocketModeListenerCallbacks<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            hello_callback: Box::new(Self::empty_hello_callback),
            command_callback: Box::new(Self::empty_command_events_callback),
            interaction_callback: Box::new(Self::empty_interaction_events_callback),
            push_events_callback: Box::new(Self::empty_push_events_callback),
        }
    }

    pub fn with_hello_events<F>(
        mut self,
        hello_events_fn: UserCallbackFunction<SlackSocketModeHelloEvent, F, SCHC>,
    ) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.hello_callback = Box::new(hello_events_fn);
        self
    }

    async fn empty_hello_callback(
        event: SlackSocketModeHelloEvent,
        _client: Arc<SlackClient<SCHC>>,
        _states: SlackClientEventsUserState,
    ) {
        debug!("Received Slack hello for socket mode: {:?}", event);
    }

    pub fn with_command_events<F>(
        mut self,
        command_events_fn: UserCallbackFunction<SlackCommandEvent, F, SCHC>,
    ) -> Self
    where
        F: Future<Output = UserCallbackResult<SlackCommandEventResponse>> + Send + 'static,
    {
        self.command_callback = Box::new(command_events_fn);
        self
    }

    async fn empty_command_events_callback(
        event: SlackCommandEvent,
        _client: Arc<SlackClient<SCHC>>,
        _states: SlackClientEventsUserState,
    ) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
        warn!("No callback is specified for a command event: {:?}", event);
        Err(Box::new(SlackClientError::SystemError(
            SlackClientSystemError::new()
                .with_message("No callback is specified for a command event".to_string()),
        )))
    }

    pub fn with_interaction_events<F>(
        mut self,
        interaction_events_fn: UserCallbackFunction<SlackInteractionEvent, F, SCHC>,
    ) -> Self
    where
        F: Future<Output = UserCallbackResult<()>> + Send + 'static,
    {
        self.interaction_callback = Box::new(interaction_events_fn);
        self
    }

    async fn empty_interaction_events_callback(
        event: SlackInteractionEvent,
        _client: Arc<SlackClient<SCHC>>,
        _states: SlackClientEventsUserState,
    ) -> UserCallbackResult<()> {
        warn!(
            "No callback is specified for interactive events: {:?}",
            event
        );
        Err(Box::new(SlackClientError::SystemError(
            SlackClientSystemError::new()
                .with_message("No callback is specified for interactive events".to_string()),
        )))
    }

    pub fn with_push_events<F>(
        mut self,
        push_events_fn: UserCallbackFunction<SlackPushEventCallback, F, SCHC>,
    ) -> Self
    where
        F: Future<Output = UserCallbackResult<()>> + Send + 'static,
    {
        self.push_events_callback = Box::new(push_events_fn);
        self
    }

    async fn empty_push_events_callback(
        event: SlackPushEventCallback,
        _client: Arc<SlackClient<SCHC>>,
        _states: SlackClientEventsUserState,
    ) -> UserCallbackResult<()> {
        warn!("No callback is specified for a push event: {:?}", event);

        Err(Box::new(SlackClientError::SystemError(
            SlackClientSystemError::new()
                .with_message("No callback is specified for push events".to_string()),
        )))
    }
}
