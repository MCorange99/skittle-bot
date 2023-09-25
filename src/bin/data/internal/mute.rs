use serde::{Deserialize, Serialize};

/// A mute is a temporary role that is added to a user to prevent them from sending messages.
/// or speaking in voice channels.
#[derive(Serialize, Deserialize)]
pub struct Mute {
    pub user_id: usize,
    pub modeartor_id: usize,
    pub reason: String,
    pub timestamp: i64,
    pub duration: i64,
}
