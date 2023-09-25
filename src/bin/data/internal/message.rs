use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct MessageId(usize);
/// Message contains a reference to a specific message.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// See also https://docs.rs/serenity/latest/serenity/model/channel/struct.Message.html#impl-ArgumentConvert-for-Message
    pub message_link: String,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct MessageRefId(MessageId);
/// Message reference contains a collection of messages that may be relevant to something.
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReference {
    pub messages: Vec<usize>,
}
