use crate::*;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use std::time::Duration;
use tokio_stream::StreamExt;

pub trait SlackApiResponseScrollerExt<SCHC, CT, RT, RIT>:
    SlackApiResponseScroller<SCHC, CursorType = CT, ResponseType = RT, ResponseItemType = RIT>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
    RT: Send + Clone + Sync + SlackApiScrollableResponse<CursorType = CT, ResponseItemType = RIT>,
    RIT: Send + Clone,
{
    fn collect_items_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
        throttle_duration: Duration,
    ) -> BoxFuture<'a, ClientResult<Vec<RIT>>>;

    fn to_items_throttled_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
        throttle_duration: Duration,
    ) -> BoxStream<'a, ClientResult<Vec<Self::ResponseItemType>>>;
}

impl<SCHC, CT, RT, RIT> SlackApiResponseScrollerExt<SCHC, CT, RT, RIT>
    for dyn SlackApiResponseScroller<SCHC, CursorType = CT, ResponseType = RT, ResponseItemType = RIT>
        + Send
        + Sync
where
    SCHC: SlackClientHttpConnector + Send + Sync,
    RT: Send + Clone + Sync + SlackApiScrollableResponse<CursorType = CT, ResponseItemType = RIT>,
    RIT: Send + Clone,
{
    fn collect_items_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
        throttle_duration: Duration,
    ) -> BoxFuture<'a, ClientResult<Vec<RIT>>> {
        Box::pin(
            self.to_items_throttled_stream(session, throttle_duration)
                .try_concat(),
        )
    }

    fn to_items_throttled_stream<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
        throttle_duration: Duration,
    ) -> BoxStream<'a, ClientResult<Vec<Self::ResponseItemType>>> {
        Box::pin(self.to_items_stream(session).throttle(throttle_duration))
    }
}
