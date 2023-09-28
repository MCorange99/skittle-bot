use std::ops::{BitOr, BitAnd, Not};

use diesel::prelude::*;
use color_eyre::{Result, eyre::bail};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::core_users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CoreUsers {
    pub user_id: i64,
    pub user_role: i64,
}




#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UserRole {
    Member         = 1,
    Staff          = 2,
    Moderator      = 4,
    Administrator  = 8,
    Founder        = 16,
    BotOwner       = 32
}

impl UserRole {
    pub fn add(i: i64, role: UserRole) -> i64 {
        i | role
    }

    pub fn has(i: i64, role: UserRole) -> bool {
        i & role == role as i64
    }

    pub fn remove(i: i64, role: UserRole) -> i64 {
        i & !role
    }
}


impl TryInto<UserRole> for String {
    type Error = color_eyre::eyre::Error;

    fn try_into(self) -> Result<UserRole> {
        match self.to_lowercase().as_str() {
            "member" => Ok(UserRole::Member),
            "staff" => Ok(UserRole::Staff),
            "moderator" | "mod" => Ok(UserRole::Moderator),
            "administrator" | "admin" => Ok(UserRole::Administrator),
            "founder" => Ok(UserRole::Founder),
            "botowner" => Ok(UserRole::BotOwner),
            _ => bail!("Role not found")
        }
    }

}

impl BitOr<UserRole> for i64 {
    type Output = i64;

    fn bitor(self, rhs: UserRole) -> Self::Output {
        self | rhs as i64
    }
}

impl BitAnd<UserRole> for i64 {
    type Output = i64;

    fn bitand(self, rhs: UserRole) -> Self::Output {
        self & rhs as i64
    }
}

impl Not for UserRole {
    type Output = i64;

    fn not(self) -> Self::Output {
        !(self as i64)
    }
}
