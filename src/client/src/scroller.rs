use crate::{ClientResult, SlackClientSession};
use crate::errors::*;

use futures::{TryFutureExt, FutureExt, StreamExt};
use futures::future::BoxFuture;
use futures::stream::BoxStream;

pub trait SlackApiResponseScroller {
    type ResponseType;
    type CursorType;

    fn has_next(&self) -> bool;

    fn next_mut<'a, 's>(&'a mut self, session: &'a SlackClientSession<'s>) -> BoxFuture<'a,ClientResult<Self::ResponseType>>;
    fn next<'a, 's>(&'a self, session: &'a SlackClientSession<'s>) -> BoxFuture<'a,(ClientResult<Self::ResponseType>, Self)>
        where Self : std::marker::Sized;

    fn to_stream<'a,'s>(&'a self, session: &'a SlackClientSession<'s>) -> BoxStream<'a,ClientResult<Self::ResponseType>> where Self : std::marker::Sized;
}

pub trait SlackApiScrollableRequest {
    type ResponseType;
    type CursorType;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self;

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a,ClientResult<Self::ResponseType>>;
}

pub trait SlackApiScrollableResponse {
    type CursorType;

    fn next_cursor(&self) -> Option<&Self::CursorType>;
}

pub trait SlackApiResponseScrollerFactory {
    type ResponseType;
    type CursorType;

    fn scroller(
        &self,
    ) -> Box<
        dyn SlackApiResponseScroller<
            ResponseType=Self::ResponseType,
            CursorType=Self::CursorType
        >
    >;
}

#[derive(Debug, Clone)]
pub struct SlackApiResponseScrollerState<RQ, RS, CT>
    where
        RQ: SlackApiScrollableRequest<ResponseType=RS, CursorType = CT> + Send + Sync + Clone,
        RS: SlackApiScrollableResponse<CursorType = CT> + Send + Sync + Clone,
        CT: Send + Sync + Clone {
    pub request: RQ,
    pub last_response: Option<RS>,
    pub last_cursor: Option<CT>,
}

impl<RQ, RS, CT> SlackApiResponseScroller for SlackApiResponseScrollerState<RQ, RS, CT>
    where
        RQ: SlackApiScrollableRequest<ResponseType=RS, CursorType = CT> + Send + Sync + Clone,
        RS: SlackApiScrollableResponse<CursorType = CT> + Send + Sync + Clone,
        CT: Send + Sync + Clone
{
    type ResponseType = RS;
    type CursorType = CT;

    fn has_next(&self) -> bool {
        self.last_response.is_none() || (self.last_response.is_some() && self.last_cursor.is_some())
    }

    fn next_mut<'a, 's>(&'a mut self, session: &'a SlackClientSession<'s>) -> BoxFuture<'a,ClientResult<Self::ResponseType>> {

        let cursor = &self.last_cursor;

        if !&self.has_next() {
            async {
                Err(Box::new(SlackClientError::EndOfStream(SlackClientEndOfStreamError::new())).into())
            }.boxed()
        }
        else {
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
            }.boxed()
        }

    }

    fn next<'a, 's>(&'a self, session: &'a SlackClientSession<'s>) -> BoxFuture<'a,(ClientResult<Self::ResponseType>,Self)> {

        let self_clone = self.clone();

        if !&self.has_next() {
            async {
                ( Err(Box::new(SlackClientError::EndOfStream(SlackClientEndOfStreamError::new())).into()), self_clone )
            }.boxed()
        }
        else {
            let cursor = &self.last_cursor;
            let updated_request = self.request.with_new_cursor(cursor.as_ref());

            async move {
                updated_request
                    .scroll(&session)
                    .map(|res| {
                        let updated_self =
                            match &res {
                                Ok(ok_resp) => {
                                    Self {
                                        last_response : Some(ok_resp.clone()),
                                        last_cursor : ok_resp.next_cursor().cloned(),
                                        .. self_clone
                                    }
                                }
                                Err(_) => self_clone
                            };

                        (res, updated_self)
                    })
                    .await
            }.boxed()
        }
    }

    fn to_stream<'a,'s>(&'a self, session: &'a SlackClientSession<'s>) -> BoxStream<'a,ClientResult<Self::ResponseType>> where Self : std::marker::Sized {

        let stream = futures_util::stream::unfold(self.clone(), move |state| async move {
            if state.has_next() {
                let (res,updated_state) = state.next(session).await;
                Some( (res, updated_state) )
            }
            else {
                None
            }
        });

        stream.boxed()
    }
}
