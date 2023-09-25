use crate::db::*;

/// User is a struct that contains information about a user.
/// The User's internal id is the key in the users hashmap.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CoreUser {
    pub discord_id: u64,
    pub roles: HashSet<UserRole>,
}


impl CoreUser {
    pub fn new(id: u64,) -> Self {
        Self {
            discord_id: id,
            roles: HashSet::new(),
        }
    }

    /// Adds a new role to a user
    /// 
    /// returns true if user already had that role
    pub fn add_role(&mut self, role: UserRole) -> bool {
        !self.roles.insert(role)
    }

    /// Removes a role from a user (internal role)
    /// 
    /// returns true if user already had that role
    pub fn remove_role(&mut self, role: UserRole) -> bool {
        self.roles.remove(&role)
    }

    pub fn has_role(&mut self, role: UserRole) -> bool {
        self.roles.contains(&role)
    }

    /// Returns true if user already existed
    pub async fn save(&mut self, uid: UserInternalId, db: &mut Database) -> Result<bool> {
        let mut old = db.read_db().await?;
        let is_new = !old.users.insert(uid, self.clone()).is_none();
        db.save_to_cache(old);
        Ok(is_new)
    }
}