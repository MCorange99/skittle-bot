mod schema;
mod types;

pub use types::*;
pub use schema::*;
pub use serde::{Serialize, Deserialize};
pub use std::collections::HashSet;
pub use color_eyre::{Result, eyre::bail};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DatabaseData {
    /*
    u64 -> User hashmaps are used instead of an array because the users may end up deleted and
    re-added, which would cause the ids to fall out of sync. Hashmaps allow for gaps in the ids
     */
    /// The users hashmap contains all of the users in the database.
    /// It does not contain all the users in the server.
    pub users: HashMap<UserInternalId, CoreUser>,
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

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Database {
    data: DatabaseData,
    path: String
}

impl Database {
    pub async fn connect(path: String) -> Result<Self>{

        let p = PathBuf::from(&path);
        let mut existed = true;
        if !p.exists() {
            std::fs::File::create(p)?;
            existed = false;
        }

        let mut db = Self {
            data: DatabaseData::default(),
            path,
        };
        if !existed {
            db.save_cached_data().await?;
        } else {
            db.read_db().await?;
        }

        Ok(db)
    }

    pub(crate) fn get_cached_data(&mut self) -> &DatabaseData {
        &self.data
    }


    /// This will read the data from the db file and save it to cache
    /// 
    /// Will not compile if no db is selected
    pub(crate) async fn read_db(&mut self) -> Result<DatabaseData> {
        let bytes = tokio::fs::read(&self.path).await?;

        cfg_if::cfg_if! {
            if #[cfg(feature = "db_bin")] {
                let data = bincode::deserialize::<DatabaseData>(&bytes)?;
            } else 
            if #[cfg(feature = "db_toml")] {
                let data = toml::from_str::<DatabaseData>(&String::from_utf8(bytes)?)?;
            } else {
                compile_error!("No db selected");
            }
        }

        self.data = data.clone();

        Ok(data)
    }

    pub(crate) async fn save_cached_data(&mut self) -> Result<()> {
        
        cfg_if::cfg_if! {
            if #[cfg(feature = "db_bin")] {
                let bytes = bincode::serialize(&self.data)?;
            } else 
            if #[cfg(feature = "db_toml")] {
                let bytes = toml::to_string_pretty(&self.data)?.bytes().collect::<Vec<u8>>();
            } else {
                compile_error!("No db selected");
            }
        }
        
        tokio::fs::write(&self.path, bytes).await?;

        Ok(())
    }

    pub(crate) fn save_to_cache(&mut self, data: DatabaseData) {
        self.data = data;
    }
}