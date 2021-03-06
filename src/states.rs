use teloxide_macros::Transition;

use serde::{Deserialize, Serialize};

#[derive(Transition, derive_more::From, Serialize, Deserialize)]
pub enum Dialogue {
    Ready(ReadyState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Ready(ReadyState)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReadyState;
