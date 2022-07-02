pub mod manifest;

pub use manifest::*;

use crate::{SlackClientId, SlackClientSecret, SlackSigningSecret, SlackVerificationToken};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppCredentials {
    pub client_id: SlackClientId,
    pub client_secret: SlackClientSecret,
    pub verification_token: SlackVerificationToken,
    pub signing_secret: SlackSigningSecret,
}
