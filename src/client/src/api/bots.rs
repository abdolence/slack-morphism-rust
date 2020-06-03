use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ClientResult;
use crate::SlackClientSession;
use slack_morphism_models::common::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiBotsInfoRequest {
    pub bot : Option<String>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiBotsInfoResponse {
    pub id: String,
    pub deleted: Option<bool>,
    pub name: String,
    pub updated: Option<SlackDateTime>,
    pub app_id: String,
    pub user_id: String,
    pub icons: Option<HashMap<String, String>>
}

impl<'a> SlackClientSession<'a> {

    pub async fn bots_info(
        &self,
        req: &SlackApiBotsInfoRequest,
    ) -> ClientResult<SlackApiBotsInfoResponse> {
        self.http_get(
            "bots.info",
            vec![
                ("bot", req.bot.as_ref())
            ],
        )
        .await
    }

}
