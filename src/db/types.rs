use std::sync::Arc;

use serde::{Serialize, Deserialize};
use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

use super::Database;

#[derive(Serialize, Deserialize, Hash,
    PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct UserInternalId(String);

impl UserInternalId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

#[derive(Serialize, Deserialize, Hash,
    PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct NoteId(String);

impl NoteId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}


#[derive(Serialize, Deserialize, Hash,
    PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct MessageId(String);

impl MessageId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

#[derive(Serialize, Deserialize, Hash,
    PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct MessageRefId(String);

impl MessageRefId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

#[derive(Serialize, Deserialize, Hash,
    PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct ReportID(String);

impl ReportID {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

#[derive(Serialize, Deserialize, Hash,
    PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum UserRole {
    Member,
    Staff,
    Moderator,
    Administator,
    Developer,
    Owner,
    BotOwner,
}

impl TypeMapKey for Database {
    // While you will be using RwLock or Mutex most of the time you want to modify data,
    // sometimes it's not required; like for example, with static data, or if you are using other
    // kinds of atomic operators.
    //
    // Arc should stay, to allow for the data lock to be closed early.
    type Value = Arc<RwLock<Database>>;
}