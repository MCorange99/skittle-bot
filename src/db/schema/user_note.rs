use crate::db::*;

/// UserNote is a simple text note that can be added to a user.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserNote {
    pub user_id: usize,
    pub modeartor_id: usize,
    pub note: String,
    pub message_reference: Option<MessageRefId>,
    // VERY BAD IDEA but I'm not sure if chrono's timestamp works
    // with discord's, so it's best to use a unix timestamp
    // Not to mention the timestamp can easily be acquired from
    // the command message
    /// See also https://docs.rs/serenity/latest/serenity/model/timestamp/struct.Timestamp.html#method.from_unix_timestamp
    pub created_at: i64,
}