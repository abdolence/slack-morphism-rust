use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeWssClientId {
    pub initial_index: u32,
    pub token_index: u32,
    pub reconnected: u64,
}

impl SlackSocketModeWssClientId {
    pub fn new_reconnected_id(&self) -> Self {
        if self.reconnected < 64 {
            Self {
                reconnected: self.reconnected + 1,
                ..self.clone()
            }
        } else {
            Self {
                reconnected: 0,
                ..self.clone()
            }
        }
    }
}

impl ToString for SlackSocketModeWssClientId {
    fn to_string(&self) -> String {
        format!(
            "{}/{}/{}",
            self.initial_index, self.token_index, self.reconnected
        )
    }
}
