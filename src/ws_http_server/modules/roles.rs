use super::{
    super::super::spec::{
        schema::roles,
        user::{Role, RoleEntry},
    },
    Cache, Persistent, ProviderError,
};
use diesel::{
    OptionalExtension,
    QueryDsl, RunQueryDsl,
};

/// Provider represents an arbitrary provider of the roles lib API.
/// The roles API is responsible for managing roles corresponding to certain
/// users.
pub trait Provider {
    /// Determines whether or not a user with the given user ID has the given
    /// role.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn user_has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError>;

    /// Assigns the given role to a user without removing any existing roles
    /// from the aforementioned user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn give_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError>;

    /// Removes the given role from the user with the corresponding user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be removed
    /// * `role` - The role that should be removed from the user
    fn remove_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError>;

    /// Removes all of the roles corresponding to the given user, returning
    /// all roles that were removed.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be purged
    fn purge_roles(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError>;

    /// Obtains a list of the roles held by a certain user, indicated by the
    /// user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be determined
    fn roles_for_user(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError>;
}

impl<'a> Provider for Cache<'a> {
    /// Determines whether or not a user with the given user ID has the given
    /// role.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn user_has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError> {
        redis::cmd("SISMEMBER")
            .arg(format!("roles::{}", user_id))
            .arg(role.to_str())
            .query::<bool>(self.connection)
            .map_err(|e| e.into())
    }

    /// Assigns the given role to a user without removing any existing roles
    /// from the aforementioned user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn give_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError> {
        redis::cmd("SADD")
            .arg(format!("roles::{}", user_id))
            .arg(role.to_str())
            .query::<()>(self.connection)
            .map_err(|e| e.into())
    }

    /// Removes the given role from the user with the corresponding user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be removed
    /// * `role` - The role that should be removed from the user
    fn remove_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError> {
        redis::cmd("SREM")
            .arg(format!("roles::{}", user_id))
            .arg(role.to_str())
            .query::<()>(self.connection)
            .map_err(|e| e.into())
    }

    /// Removes all of the roles corresponding to the given user, returning
    /// all roles that were removed.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be purged
    fn purge_roles(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError> {
        // Get a list of the roles that the user once had
        let old = self.roles_for_user(user_id)?;

        // Purge all of the user's roles
        redis::cmd("DEL")
            .arg(format!("roles::{}", user_id))
            .query::<()>(self.connection)?;

        Ok(old)
    }

    /// Obtains a list of the roles held by a certain user, indicated by the
    /// user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be determined
    fn roles_for_user(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError> {
        redis::cmd("SMEMBERS")
            .arg(format!("roles::{}", user_id))
            .query::<Vec<String>>(self.connection)
            .map(|str_roles| {
                str_roles
                    .iter()
                    .filter_map(|str_role| str_role.parse().ok())
                    .collect()
            })
            .map_err(|e| e.into())
    }
}

impl<'a> Provider for Persistent<'a> {
    /// Determines whether or not a user with the given user ID has the given
    /// role.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn user_has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError> {
        roles::dsl::roles
            .find(user_id)
            .first::<RoleEntry>(self.connection)
            .map(|role_entry| role_entry.has_role(role))
            .map_err(|e| e.into())
    }

    /// Assigns the given role to a user without removing any existing roles
    /// from the aforementioned user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn give_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError> {
        role.construct_give_role_statement(user_id, true)
            .execute(self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    /// Removes the given role from the user with the corresponding user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be removed
    /// * `role` - The role that should be removed from the user
    fn remove_role(&mut self, user_id: u64, role: Role) -> Result<(), ProviderError> {
        role.construct_give_role_statement(user_id, false)
            .execute(self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    /// Removes all of the roles corresponding to the given user, returning
    /// all roles that were removed.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be purged
    fn purge_roles(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError> {
        let roles = self.roles_for_user(user_id)?;

        diesel::delete(roles::table.find(user_id))
            .execute(self.connection)
            .map(|_| roles)
            .map_err(|e| e.into())
    }

    /// Obtains a list of the roles held by a certain user, indicated by the
    /// user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be determined
    fn roles_for_user(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError> {
        Ok(<Vec<Role> as From<&RoleEntry>>::from(
            &roles::table
                .find(user_id)
                .first::<RoleEntry>(self.connection)
                .optional()?
                .unwrap_or_default(),
        ))
    }
}
