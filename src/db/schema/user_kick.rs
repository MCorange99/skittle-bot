use crate::db::*;

/// A kick is when a user is removed from the server.
/// This struct purely exists to log the kick.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Kick {
    pub user_id: usize,
    pub modeartor_id: usize,
    pub reason: String,
    pub timestamp: i64,
}