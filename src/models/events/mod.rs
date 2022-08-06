use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

mod authorization;
mod command;
mod interaction;
mod push;

pub use authorization::*;
pub use command::*;
pub use interaction::*;
pub use push::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEventId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEventContext(pub String);
