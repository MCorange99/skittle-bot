use std::collections::HashSet;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct UserInternalId(usize);
/// User is a struct that contains information about a user.
/// The User's internal id is the key in the users hashmap.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub discord_id: u64,
    pub roles: HashSet<Role>,
    pub is_bot_owner: bool,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Role {
    Member,
    Staff,
    Moderator,
    Administator,
    Developer,
    Owner,
    BotOwner,
}