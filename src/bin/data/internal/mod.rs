use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use self::{ban::Ban, kick::Kick, mute::Mute, report::{Report, ReportID}, message::{Message, MessageId, MessageRefId, MessageReference}, note::{NoteId, UserNote}, user::{UserInternalId, User}};

mod user;
mod note;
mod message;
mod report;
mod mute;
mod kick;
mod ban;

/*
Internal structs are used to clarify the relationship between the structs
Instead of everything being a usize in order to prevent confusion and bugs
However, the internal structs are able to point to nonexistent data
So make sure to check if the data exists before using it, don't just unwrap it
*/

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