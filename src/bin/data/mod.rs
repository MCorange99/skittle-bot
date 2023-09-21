use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use std::hash::Hash;

#[derive(Serialize, Deserialize)]
pub struct Database {
    /*
    u64 -> User hashmaps are used instead of an array because the users may end up deleted and
    re-added, which would cause the ids to fall out of sync. Hashmaps allow for gaps in the ids
     */
    /// The users hashmap contains all of the users in the database.
    /// It does not contain all the users in the server.
    pub users: HashMap<UserInternalId, User>,
    /// An association is where two accounts belong to the same person.
    /// This is used to prevent people from using multiple accounts to bypass bans.
    /// Or to sockpuppet.
    pub associations: HashMap<UserInternalId, HashSet<UserInternalId>>,
    /// The user_notes hashmap contains all of the notes that have been added to users.
    /// Mainly used for moderation purposes.
    pub user_notes: HashMap<NoteId, UserNote>,
    /// The messages hashmap contains all of the messages that have been added to the database.
    /// It is used to store messages that may be relevant to something.
    /// It does not contain all of the messages in the server.
    pub messages: HashMap<MessageId, Message>,
    /// The message_references hashmap contains arrays of messages that may be relevant to something.
    /// The messages are usually relevant together, and are associated with a user_note.
    pub message_references: HashMap<MessageRefId, MessageReference>,
    /// The reports hashmap contains all of the reports that have been made against users.
    pub reports: HashMap<ReportID, Report>,
    /// The mutes hashmap contains all of the mutes that have been made against users.
    /// Unlike the above, this will use a vector since it's unlikely that an entry will be deleted.
    pub mutes: Vec<Mute>,
    /// The kicks hashmap contains all of the kicks that have been made against users.
    pub kicks: Vec<Kick>,
    /// The bans hashmap contains all of the bans that have been made against users.
    pub bans: Vec<Ban>,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Member,
    Staff,
    Moderator,
    Administator,
    Developer,
    Owner,
    BotOwner,
}

/*
Internal structs are used to clarify the relationship between the structs
Instead of everything being a usize in order to prevent confusion and bugs
However, the internal structs are able to point to nonexistent data
So make sure to check if the data exists before using it, don't just unwrap it
*/
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserInternalId(usize);
/// User is a struct that contains information about a user.
/// The User's internal id is the key in the users hashmap.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub discord_id: u64,
    pub roles: HashSet<Role>,
    pub is_bot_owner: bool,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoteId(usize);
/// UserNote is a simple text note that can be added to a user.
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageId(usize);
/// Message contains a reference to a specific message.
#[derive(Serialize, Deserialize)]
pub struct Message {
    /// See also https://docs.rs/serenity/latest/serenity/model/channel/struct.Message.html#impl-ArgumentConvert-for-Message
    pub message_link: String,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageRefId(MessageId);
/// Message reference contains a collection of messages that may be relevant to something.
#[derive(Serialize, Deserialize)]
pub struct MessageReference {
    pub messages: Vec<usize>,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

/// A kick is when a user is removed from the server.
/// This struct purely exists to log the kick.
#[derive(Serialize, Deserialize)]
pub struct Kick {
    pub user_id: usize,
    pub modeartor_id: usize,
    pub reason: String,
    pub timestamp: i64,
}

/// A ban is when a user is removed from the server and is unable to rejoin.
/// They can be temporary (with a timestamp) or permanent (with None).
#[derive(Serialize, Deserialize)]
pub struct Ban {
    pub user_id: usize,
    pub modeartor_id: usize,
    pub reason: String,
    pub timestamp: i64,
    pub duration: Option<i64>,
}

pub fn get_hello_world() -> String {
    String::from("Hello World!")
}