use crate::listener::UserCallbackFunctionType;
use crate::prelude::UserCallbackFunction;
use crate::socket_mode_connector::*;
use crate::{ClientResult, SlackClientHttpConnector};
use slack_morphism_models::events::{SlackCommandEvent, SlackCommandEventResponse};
use std::future::Future;
use std::marker::PhantomData;

pub struct SlackClientSocketModeCallbacks<SCHC, SCWSS>
where
    SCHC: SlackClientHttpConnector + SlackClientSocketModeConnector<SCWSS> + Send + Clone + Sync,
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    command_callback:
        Option<Box<UserCallbackFunctionType<SlackCommandEvent, SlackCommandEventResponse, SCHC>>>,
    _pd: PhantomData<SCWSS>,
}

impl<SCHC, SCWSS> SlackClientSocketModeCallbacks<SCHC, SCWSS>
where
    SCHC: SlackClientHttpConnector
        + SlackClientSocketModeConnector<SCWSS>
        + Send
        + Clone
        + Sync
        + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            command_callback: None,
            _pd: PhantomData::default(),
        }
    }
    pub fn with_command_events<F>(
        mut self,
        command_events_fn: UserCallbackFunction<SlackCommandEvent, F, SCHC>,
    ) -> Self
    where
        F: Future<Output = ClientResult<SlackCommandEventResponse>> + Send + Sync + 'static,
    {
        self.command_callback = Some(Box::new(move |e, c, s| {
            Box::pin(command_events_fn(e, c, s))
        }));

        self
    }
}
