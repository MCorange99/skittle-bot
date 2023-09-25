use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ReportID(usize);
/// Report is a report that has been made against a user.
/// It could be a warning form a moderator, or a report from a user.
#[derive(Serialize, Deserialize)]
pub struct Report {
    pub user_id: usize,
    pub modeartor_id: usize,
    pub title: String,
    pub description: String,
    pub timestamp: i64,
}
