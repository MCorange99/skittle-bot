use std::sync::{Mutex, Arc};

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use color_eyre::Result;
use serenity::prelude::TypeMapKey;

pub mod models;
pub mod schema;

pub struct Database {
    connection: SqliteConnection
}

impl Database {
    pub fn connect(url: String) -> Result<Self> {
        log::info!("Connecting to database");
        let conn = SqliteConnection::establish(&url);

        if conn.is_err() {
            log::error!("Unable to connect to database at {}", url);
        } else {
            log::info!("Connected to database at {}", url);
        }

        Ok(
            Self {
                connection: conn?
            }
        )
    }
}

impl TypeMapKey for Database {
    // While you will be using RwLock or Mutex most of the time you want to modify data,
    // sometimes it's not required; like for example, with static data, or if you are using other
    // kinds of atomic operators.
    //
    // Arc should stay, to allow for the data lock to be closed early.
    type Value = Arc<Mutex<Database>>;
}