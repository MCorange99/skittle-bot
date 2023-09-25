use crate::db::*;

/// Message reference contains a collection of messages that may be relevant to something.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReference {
    pub messages: Vec<usize>,
}
