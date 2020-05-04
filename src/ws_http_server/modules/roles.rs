use super::{
    super::super::spec::{
        schema::roles,
        user::{Role, RoleEntry},
    },
    Cache, Hybrid, Persistent, ProviderError,
};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};

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
    fn has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError>;

    /// Assigns the given role to a user without removing any existing roles
    /// from the aforementioned user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn give_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError>;

    /// Assigns multiple roles to a suer at once.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be set
    /// * `roles` - The roles that should be assigned to the user
    fn give_roles(&mut self, user_id: u64, roles: &[Role]) -> Result<(), ProviderError>;

    /// Removes the given role from the user with the corresponding user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be removed
    /// * `role` - The role that should be removed from the user
    fn remove_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError>;

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
    fn has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError> {
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
    fn give_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError> {
        self.give_roles(user_id, &[*role])
    }

    /// Assigns multiple roles to a user at once.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be set
    /// * `roles` - The roles that should be assigned to the user
    fn give_roles(&mut self, user_id: u64, roles: &[Role]) -> Result<(), ProviderError> {
        redis::cmd("SADD")
            .arg(format!("roles::{}", user_id))
            .arg(
                roles
                    .iter()
                    .map(|role| role.to_str())
                    .collect::<Vec<&str>>(),
            )
            .query::<()>(self.connection)
            .map_err(|e| e.into())
    }

    /// Removes the given role from the user with the corresponding user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be removed
    /// * `role` - The role that should be removed from the user
    fn remove_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError> {
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
    fn has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError> {
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
    fn give_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError> {
        role.construct_give_role_statement(user_id, true)
            .execute(self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    /// Assigns multiple roles to a suer at once.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be set
    /// * `roles` - The roles that should be assigned to the user
    fn give_roles(&mut self, user_id: u64, roles: &[Role]) -> Result<(), ProviderError> {
        println!(
            "IF EXISTS (SELECT * FROM roles WHERE user_id = {}) UPDATE roles SET {} WHERE user_id = {} ELSE INSERT INTO roles(user_id, {}) VALUES({}, {}) END",
            user_id,
            roles
                .iter()
                .map(|role| format!("{} = true", role))
                .collect::<Vec<String>>()
                .join(", "),
            user_id,
            roles
                .iter()
                .map(|role| role.to_str())
                .collect::<Vec<&str>>()
                .join(", "),
            user_id,
            roles
                .iter()
                .map(|_| "true")
                .collect::<Vec<&str>>()
                .join(", "),
        );
        diesel::sql_query(format!(
            "IF EXISTS (SELECT * FROM roles WHERE user_id = {}) UPDATE roles SET {} WHERE user_id = {} ELSE INSERT INTO roles(user_id, {}) VALUES({}, {}) END",
            user_id,
            roles
                .iter()
                .map(|role| format!("{} = true", role))
                .collect::<Vec<String>>()
                .join(", "),
            user_id,
            roles
                .iter()
                .map(|role| role.to_str())
                .collect::<Vec<&str>>()
                .join(", "),
            user_id,
            roles
                .iter()
                .map(|_| "true")
                .collect::<Vec<&str>>()
                .join(", "),
        ))
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
    fn remove_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError> {
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

impl<'a> Provider for Hybrid<'a> {
    /// Determines whether or not a user with the given user ID has the given
    /// role.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn has_role(&mut self, user_id: u64, role: &Role) -> Result<bool, ProviderError> {
        self.cache.has_role(user_id, role).or_else(|_| {
            self.persistent
                .has_role(user_id, role)
                .and_then(|has_role| {
                    {
                        if has_role {
                            self.cache.give_role(user_id, role)
                        } else {
                            self.cache.remove_role(user_id, role)
                        }
                    }
                    .map(|_| has_role)
                })
        })
    }

    /// Assigns the given role to a user without removing any existing roles
    /// from the aforementioned user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose role should be checked
    /// * `role` - The role that the user should have
    fn give_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError> {
        self.cache
            .give_role(user_id, role)
            .and(self.persistent.give_role(user_id, role))
    }

    /// Assigns multiple roles to a suer at once.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be set
    /// * `roles` - The roles that should be assigned to the user
    fn give_roles(&mut self, user_id: u64, roles: &[Role]) -> Result<(), ProviderError> {
        self.cache
            .give_roles(user_id, roles)
            .and(self.persistent.give_roles(user_id, roles))
    }

    /// Removes the given role from the user with the corresponding user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be removed
    /// * `role` - The role that should be removed from the user
    fn remove_role(&mut self, user_id: u64, role: &Role) -> Result<(), ProviderError> {
        self.cache
            .remove_role(user_id, role)
            .and(self.persistent.remove_role(user_id, role))
    }

    /// Removes all of the roles corresponding to the given user, returning
    /// all roles that were removed.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be purged
    fn purge_roles(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError> {
        self.cache
            .purge_roles(user_id)
            .and(self.persistent.purge_roles(user_id))
    }

    /// Obtains a list of the roles held by a certain user, indicated by the
    /// user_id.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose roles should be determined
    fn roles_for_user(&mut self, user_id: u64) -> Result<Vec<Role>, ProviderError> {
        self.cache.roles_for_user(user_id).or_else(|_| {
            self.persistent.roles_for_user(user_id).and_then(|roles| {
                self.cache
                    .give_roles(user_id, roles.as_slice())
                    .map(|_| roles)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::super::spec::{
            schema::users,
            user::{NewUser, Role},
        },
        *,
    };
    use diesel::{mysql::MysqlConnection, Connection, ExpressionMethods};

    use std::{env, error::Error};

    #[test]
    fn test_hybrid() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

        let mut conn = redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
        let persistent_conn =
            MysqlConnection::establish(&env::var("DATABASE_URL").expect(
                "DATABASE_URL must be set in a .env file for test to complete successfully",
            ))?;

        // Register MrMouton as a user so that we can specify his role
        diesel::replace_into(users::table)
            .values(NewUser::default().with_username("MrMouton"))
            .execute(&persistent_conn)?;

        // Get MrMouton's ID for easy testing
        let id = users::dsl::users
            .filter(users::dsl::username.eq("MrMouton"))
            .select(users::dsl::id)
            .first(&persistent_conn)?;

        // Key mout alert
        let mut roles = Hybrid::new(Cache::new(&mut conn), Persistent::new(&persistent_conn));
        roles.give_role(id, &Role::Protected)?;

        assert_eq!(roles.has_role(id, &Role::Protected)?, true);

        Ok(())
    }
}
