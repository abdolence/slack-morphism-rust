use crate::errors::*;
use crate::{ClientResult, SlackClientSession};

use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt, TryFutureExt};

pub trait SlackApiResponseScroller {
    type ResponseType;
    type CursorType;

    fn has_next(&self) -> bool;

    fn next_mut<'a, 's>(
        &'a mut self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>>;

    fn to_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxStream<'a, ClientResult<Self::ResponseType>>;
}

pub trait SlackApiScrollableRequest {
    type ResponseType;
    type CursorType;

    fn scroller<'a,'b>(
        &'a self,
    ) -> Box<
        dyn SlackApiResponseScroller<
            ResponseType = Self::ResponseType,
            CursorType = Self::CursorType,
        > +'b
    > where Self : Send + Clone + Sync  +'b,
             Self::ResponseType : Send + Clone + Sync + SlackApiScrollableResponse<CursorType = Self::CursorType>  +'b,
             Self::CursorType : Send + Clone + Sync  +'b {
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

    fn next_cursor(&self) -> Option<&Self::CursorType>;
}

#[derive(Debug, Clone)]
pub struct SlackApiResponseScrollerState<RQ, RS, CT>
where
    RQ: SlackApiScrollableRequest<ResponseType = RS, CursorType = CT> + Send + Sync + Clone,
    RS: SlackApiScrollableResponse<CursorType = CT> + Send + Sync + Clone,
    CT: Send + Sync + Clone,
{
    pub request: RQ,
    pub last_response: Option<RS>,
    pub last_cursor: Option<CT>,
}

impl<RQ, RS, CT> SlackApiResponseScrollerState<RQ, RS, CT>
where
    RQ: SlackApiScrollableRequest<ResponseType = RS, CursorType = CT> + Send + Sync + Clone,
    RS: SlackApiScrollableResponse<CursorType = CT> + Send + Sync + Clone,
    CT: Send + Sync + Clone,
{
    pub fn new(request: &RQ) -> Self {
        Self {
            request: request.clone(),
            last_cursor: None,
            last_response: None,
        }
    }
}

impl<RQ, RS, CT> SlackApiResponseScroller for SlackApiResponseScrollerState<RQ, RS, CT>
where
    RQ: SlackApiScrollableRequest<ResponseType = RS, CursorType = CT> + Send + Sync + Clone,
    RS: SlackApiScrollableResponse<CursorType = CT> + Send + Sync + Clone,
    CT: Send + Sync + Clone,
{
    type ResponseType = RS;
    type CursorType = CT;

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
    ) -> BoxStream<'a, ClientResult<Self::ResponseType>>
    where
        Self: std::marker::Sized,
    {
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
}
