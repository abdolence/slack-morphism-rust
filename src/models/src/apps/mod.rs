pub mod manifest;

pub use manifest::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SlackClientId, SlackClientSecret, SlackSigningSecret, SlackVerificationToken};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppCredentials {
    pub client_id: SlackClientId,
    pub client_secret: SlackClientSecret,
    pub verification_token: SlackVerificationToken,
    pub signing_secret: SlackSigningSecret,
}
