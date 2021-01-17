use futures::future::BoxFuture;
use futures::{FutureExt, TryStreamExt};
use slack_morphism::{
    ClientResult, SlackApiResponseScroller, SlackApiScrollableResponse, SlackClientHttpConnector,
    SlackClientSession,
};
use std::time::Duration;

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
}

impl<SCHC, CT, RT, RIT> SlackApiResponseScrollerExt<SCHC, CT, RT, RIT>
    for dyn SlackApiResponseScroller<
        SCHC,
        CursorType = CT,
        ResponseType = RT,
        ResponseItemType = RIT,
    >
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
