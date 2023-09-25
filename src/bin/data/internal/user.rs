use std::{collections::HashSet, fmt::{Formatter, Display, self}, error::Error};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct UserInternalId(usize);
/// User is a struct that contains information about a user.
/// The User's internal id is the key in the users hashmap.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    discord_id: u64,
    roles: HashSet<Role>,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Role {
    Member,
    Staff,
    Moderator,
    Administator,
    Developer,
    Owner,
    BotOwner,
}

impl User {
    /// Create a new user with no roles.
    pub fn new(discord_id: u64) -> Self {
        Self {
            discord_id,
            roles: HashSet::new(),
        }
    }

    /// Create a new user with the given roles.
    pub fn new_with_roles<I>(discord_id: u64, roles: I) -> Self
    where I: IntoIterator<Item = Role> // Three cheers for trait bounds!
    {
        Self {
            discord_id,
            roles: {
                let mut roles_set = HashSet::new();
                for role in roles {
                    roles_set.insert(role);
                }
                roles_set
            },
        }
    }

    /// Returns the user's discord id.
    /// This is NOT the same as the internal id.
    pub fn discord_id(&self) -> u64 {
        self.discord_id
    }

    /// Adds a role to the user.
    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role);
    }

    /// Adds multiple roles to the user.
    /// Use this when you have a collection or iterator of roles.
    pub fn add_roles<I>(&mut self, roles: I)
    where I: IntoIterator<Item = Role> // Three cheers for trait bounds!
    {
        for role in roles {
            self.roles.insert(role);
        }
    }

    /// Returns true if the user has the given role.
    pub fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role)
    }

    /// Returns true if the user has any of the given roles.
    pub fn has_any_roles<I>(&self, roles: I) -> bool
    where I: IntoIterator<Item = Role> // Three cheers for trait bounds!
    {
        for role in roles {
            if self.roles.contains(&role) {
                return true;
            }
        }
        false
    }

    /// Returns true if the user has all of the given roles.
    pub fn has_all_roles<I>(&self, roles: I) -> bool
    where I: IntoIterator<Item = Role> // Three cheers for trait bounds!
    {
        for role in roles {
            if !self.roles.contains(&role) {
                return false;
            }
        }
        true
    }

    /// Returns an iterator over the user's roles.
    pub fn role_iter(&self) -> impl Iterator<Item = &Role> {
        self.roles.iter()
    }

    /// Removes a role from the user.
    pub fn remove_role(&mut self, role: Role) {
        self.roles.remove(&role);
    }

    /// Removes multiple roles from the user.
    /// Use this when you have a collection or iterator of roles.
    pub fn remove_roles<I>(&mut self, roles: I)
    where I: IntoIterator<Item = Role> // Three cheers for trait bounds!
    {
        for role in roles {
            self.roles.remove(&role);
        }
    }
}

/*
The existence of this error type requires the user of this code to do something like this:
if user.has_role(Role::Member) {
    user.remove_role(Role::Member);
}
Which is useless, since the error is dropped anyway. So we might as well just
not return the possibility of an error and just silently fail.
The net result is that the user no longer has the role, which is what we want.

#[derive(Debug)]
pub struct RoleNotPresentError {
    role: Role,
}

impl RoleNotPresentError {
    pub fn new(role: Role) -> Self {
        Self {
            role,
        }
    }
}

impl Display for RoleNotPresentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Role {:?} is not present", self.role)
    }
}

impl Error for RoleNotPresentError {}
 */