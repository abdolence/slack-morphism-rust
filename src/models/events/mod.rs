use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

mod authorization;
mod command;
mod interaction;
mod push;
mod view;

pub use authorization::*;
pub use command::*;
pub use interaction::*;
pub use push::*;
pub use view::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEventId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEventContext(pub String);
