use crate::db::*;

/// Message contains a reference to a specific message.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    /// See also https://docs.rs/serenity/latest/serenity/model/channel/struct.Message.html#impl-ArgumentConvert-for-Message
    pub message_link: String,
}