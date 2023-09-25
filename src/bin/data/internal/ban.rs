use serde::{Deserialize, Serialize};

/// A ban is when a user is removed from the server and is unable to rejoin.
/// They can be temporary (with a timestamp) or permanent (with None).
#[derive(Serialize, Deserialize, Debug)]
pub struct Ban {
    pub user_id: usize,
    pub modeartor_id: usize,
    pub reason: String,
    pub timestamp: i64,
    pub duration: Option<i64>,
}
