use crate::errors::*;
use crate::{ClientResult, SlackClientSession};

use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use std::time::Duration;

pub trait SlackApiResponseScroller {
    type ResponseType;
    type CursorType;
    type ResponseItemType;

    fn has_next(&self) -> bool;

    fn next_mut<'a, 's>(
        &'a mut self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>>;

    fn to_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxStream<'a, ClientResult<Self::ResponseType>>;

    fn to_items_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxStream<'a, ClientResult<Vec<Self::ResponseItemType>>>;

    fn collect_items_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
        throttle_duration: Duration,
    ) -> BoxFuture<'a, ClientResult<Vec<Self::ResponseItemType>>>;
}

pub trait SlackApiScrollableRequest {
    type ResponseType;
    type CursorType;
    type ResponseItemType;

    fn scroller<'a, 'b>(
        &'a self,
    ) -> Box<
        dyn SlackApiResponseScroller<
                ResponseType = Self::ResponseType,
                CursorType = Self::CursorType,
                ResponseItemType = Self::ResponseItemType,
            > + 'b,
    >
    where
        Self: Send + Clone + Sync + 'b,
        Self::ResponseType: Send
            + Clone
            + Sync
            + SlackApiScrollableResponse<
                CursorType = Self::CursorType,
                ResponseItemType = Self::ResponseItemType,
            > + 'b,
        Self::CursorType: Send + Clone + Sync + 'b,
        Self::ResponseItemType: Send + Clone + Sync + 'b,
    {
        Box::new(SlackApiResponseScrollerState::new(self))
    }

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self;

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>>;
}

pub trait SlackApiScrollableResponse {
    type CursorType;
    type ResponseItemType;

    fn next_cursor(&self) -> Option<&Self::CursorType>;
    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a>;
}

#[derive(Debug, Clone)]
pub struct SlackApiResponseScrollerState<RQ, RS, CT, RIT>
where
    RQ: SlackApiScrollableRequest<ResponseType = RS, CursorType = CT, ResponseItemType = RIT>
        + Send
        + Sync
        + Clone,
    RS: SlackApiScrollableResponse<CursorType = CT, ResponseItemType = RIT> + Send + Sync + Clone,
    CT: Send + Sync + Clone,
    RIT: Send + Sync + Clone,
{
    pub request: RQ,
    pub last_response: Option<RS>,
    pub last_cursor: Option<CT>,
}

impl<RQ, RS, CT, RIT> SlackApiResponseScrollerState<RQ, RS, CT, RIT>
where
    RQ: SlackApiScrollableRequest<ResponseType = RS, CursorType = CT, ResponseItemType = RIT>
        + Send
        + Sync
        + Clone,
    RS: SlackApiScrollableResponse<CursorType = CT, ResponseItemType = RIT> + Send + Sync + Clone,
    CT: Send + Sync + Clone,
    RIT: Send + Sync + Clone,
{
    pub fn new(request: &RQ) -> Self {
        Self {
            request: request.clone(),
            last_cursor: None,
            last_response: None,
        }
    }
}

impl<RQ, RS, CT, RIT> SlackApiResponseScroller for SlackApiResponseScrollerState<RQ, RS, CT, RIT>
where
    RQ: SlackApiScrollableRequest<ResponseType = RS, CursorType = CT, ResponseItemType = RIT>
        + Send
        + Sync
        + Clone,
    RS: SlackApiScrollableResponse<CursorType = CT, ResponseItemType = RIT> + Send + Sync + Clone,
    CT: Send + Sync + Clone,
    RIT: Send + Sync + Clone,
{
    type ResponseType = RS;
    type CursorType = CT;
    type ResponseItemType = RIT;

    fn has_next(&self) -> bool {
        self.last_response.is_none() || (self.last_response.is_some() && self.last_cursor.is_some())
    }

    fn next_mut<'a, 's>(
        &'a mut self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        let cursor = &self.last_cursor;

        if !&self.has_next() {
            async {
                Err(Box::new(SlackClientError::EndOfStream(
                    SlackClientEndOfStreamError::new(),
                ))
                .into())
            }
            .boxed()
        } else {
            let updated_request = self.request.with_new_cursor(cursor.as_ref());

            async move {
                updated_request
                    .scroll(&session)
                    .map_ok(|res| {
                        self.last_response = Some(res.clone());
                        self.last_cursor = res.next_cursor().cloned();
                        res
                    })
                    .await
            }
            .boxed()
        }
    }

    fn to_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxStream<'a, ClientResult<Self::ResponseType>> {
        let init_state = self.clone();
        let stream = futures_util::stream::unfold(init_state, move |mut state| async move {
            if state.has_next() {
                let res = state.next_mut(session).await;
                Some((res, state))
            } else {
                None
            }
        });

        stream.boxed()
    }

    fn to_items_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxStream<'a, ClientResult<Vec<Self::ResponseItemType>>> {
        self.to_stream(session)
            .map_ok(|rs| {
                rs.scrollable_items()
                    .cloned()
                    .collect::<Vec<Self::ResponseItemType>>()
            })
            .boxed()
    }

    fn collect_items_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
        throttle_duration: Duration,
    ) -> BoxFuture<'a, ClientResult<Vec<Self::ResponseItemType>>> {
        use tokio_stream::StreamExt;
        self.to_stream(session)
            .throttle(throttle_duration)
            .map_ok(|rs| {
                rs.scrollable_items()
                    .cloned()
                    .collect::<Vec<Self::ResponseItemType>>()
            })
            .try_concat()
            .boxed()
    }
}
