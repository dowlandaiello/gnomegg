use diesel::{
    expression_methods::ExpressionMethods, result::Error as DieselError, QueryDsl, RunQueryDsl,
};

use super::{
    super::super::spec::{
        schema::{ids, users},
        user::NewIdMapping,
    },
    Cache, Persistent, ProviderError, Hybrid,
};

/// Provider represents an arbitrary backend for the name resolution service.
pub trait Provider {
    /// Retreieves the user ID matching the provided username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should
    /// be obtained
    fn user_id_for(&mut self, username: &str) -> Result<Option<u64>, ProviderError>;

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    fn username_for(&mut self, user_id: u64) -> Result<Option<String>, ProviderError>;

    /// Stores a username to user ID / user ID to username mapping in a
    /// provider.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should be
    /// obtained
    fn set_combination(&mut self, username: &str, user_id: u64) -> Result<(), ProviderError>;
}

impl<'a> Provider for Cache<'a> {
    /// Retreieves the user ID matching the provided username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should
    /// be obtained
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::name_resolver::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut names = Cache::new(&mut conn);
    /// names.set_combination("MrMouton", 69420)?;
    /// assert_eq!(names.user_id_for("MrMouton").unwrap(), Some(69420));
    /// Ok(())
    /// # }
    /// ```
    fn user_id_for(&mut self, username: &str) -> Result<Option<u64>, ProviderError> {
        redis::cmd("GET")
            .arg(format!("user_id::{}", username))
            .query(self.connection)
            .map_err(|e| e.into())
    }

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::name_resolver::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut names = Cache::new(&mut conn);
    /// names.set_combination("MrMouton", 69420)?;
    /// assert_eq!(names.username_for(69420).unwrap(), Some("MrMouton".to_owned()));
    /// Ok(())
    /// # }
    /// ```
    fn username_for(&mut self, user_id: u64) -> Result<Option<String>, ProviderError> {
        redis::cmd("GET")
            .arg(format!("username::{}", user_id))
            .query(self.connection)
            .map_err(|e| e.into())
    }

    /// Stores a username to user ID / user ID to username mapping in a
    /// provider.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should be
    /// obtained
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::name_resolver::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut names = Cache::new(&mut conn);
    /// names.set_combination("MrMouton", 69420)?;
    /// assert_eq!(names.username_for(69420).unwrap().unwrap(), "MrMouton".to_owned());
    /// Ok(())
    /// # }
    /// ```
    fn set_combination(&mut self, username: &str, user_id: u64) -> Result<(), ProviderError> {
        redis::cmd("MSET")
            .arg(format!("user_id::{}", username))
            .arg(user_id)
            .arg(format!("username::{}", user_id))
            .arg(username)
            .query(self.connection)
            .map_err(|e| e.into())
    }
}

impl<'a> Provider for Persistent<'a> {
    /// Retreieves the user ID matching the provided username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should
    /// be obtained
    fn user_id_for(&mut self, username: &str) -> Result<Option<u64>, ProviderError> {
        ids::dsl::ids
            .find(username)
            .select(ids::dsl::user_id)
            .first(self.connection)
            .map(Some)
            .or_else(|e| {
                // If we haven't immediately gotten a result from diesel, we can
                // check if no mute exists for the user, which would be
                // described as an error, but returned as None
                if let DieselError::NotFound = e {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            })
    }

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    fn username_for(&mut self, user_id: u64) -> Result<Option<String>, ProviderError> {
        users::dsl::users
            .find(user_id)
            .select(users::dsl::username)
            .first(self.connection)
            .map_err(|e| e.into())
    }

    /// Stores a username to user ID / user ID to username mapping in a
    /// provider.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should be
    /// obtained
    fn set_combination(&mut self, username: &str, user_id: u64) -> Result<(), ProviderError> {
        // The user must exist in order to set a mapping between them. As such,
        // we want to update existing user entries before adding or updating
        // secondary mappings
        diesel::update(users::dsl::users.find(user_id))
            .set(users::dsl::username.eq(username))
            .execute(self.connection)?;

        diesel::replace_into(ids::dsl::ids)
            .values(&NewIdMapping::new(username, user_id))
            .execute(self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}

impl<'a> Provider for Hybrid<'a> {
    /// Retreieves the user ID matching the provided username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should
    /// be obtained
    fn user_id_for(&mut self, username: &str) -> Result<Option<u64>, ProviderError> {
        self.cache.user_id_for(username).or_else(|_| {
            self.persistent.user_id_for(username).and_then(|id| {
                id.map_or(Ok(None), |id| {
                    self.cache.set_combination(username, id).and(Ok(Some(id)))
                })
            })
        })
    }

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    fn username_for(&mut self, user_id: u64) -> Result<Option<String>, ProviderError> {
        self.cache.username_for(user_id).or_else(|_| {
            self.persistent.username_for(user_id).and_then(|username| {
                username.map_or(Ok(None), |username| {
                    self.cache
                        .set_combination(&username, user_id)
                        .and(Ok(Some(username)))
                })
            })
        })
    }

    /// Stores a username to user ID / user ID to username mapping in a
    /// provider.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should be
    /// obtained
    fn set_combination(&mut self, username: &str, user_id: u64) -> Result<(), ProviderError> {
        self.cache
            .set_combination(username, user_id)
            .and(self.persistent.set_combination(username, user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::super::super::spec::user::NewUser, *};

    use diesel::connection::Connection;
    use dotenv;
    use std::{default::Default, env};

    #[test]
    fn test_hybrid() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

        let mut conn = redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
        let persistent_conn =
            MysqlConnection::establish(&env::var("DATABASE_URL").expect(
                "DATABASE_URL must be set in a .env file for test to complete successfully",
            ))?;

        // Register MrMouton as a user so that we can register a mapping
        // between the username and ID
        diesel::replace_into(users::table)
            .values(NewUser::default().with_username("MrMouton"))
            .execute(&persistent_conn)?;

        // Get MrMouton's ID for easy testing (so we can ensure that a
        // combination in the name resolver gets resolved correctly in the
        // future)
        let id = users::dsl::users
            .filter(users::dsl::username.eq("MrMouton"))
            .select(users::dsl::id)
            .first(&persistent_conn)?;

        let mut names = Hybrid::new(Cache::new(&mut conn), Persistent::new(&persistent_conn));
        names.set_combination("MrMouton", id)?;

        assert_eq!(names.username_for(id)?.unwrap(), "MrMouton");
        assert_eq!(names.user_id_for("MrMouton")?.unwrap(), id);

        Ok(())
    }

    #[test]
    fn test_cache() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

        let mut conn = redis::Client::open("redis://127.0.0.1/")?.get_connection()?;

        let mut names = Cache::new(&mut conn);
        names.set_combination("MrMouton", 42069)?;

        assert_eq!(names.username_for(42069)?.unwrap(), "MrMouton");
        assert_eq!(names.user_id_for("MrMouton")?.unwrap(), 42069);

        Ok(())
    }

    #[test]
    fn test_persistent() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

        // Open a connection with the MySQL server
        let persistent_conn =
            MysqlConnection::establish(&env::var("DATABASE_URL").expect(
                "DATABASE_URL must be set in a .env file for test to complete successfully",
            ))?;

        // Register MrMouton as a user so that we can register a mapping
        // between the username and ID
        diesel::replace_into(users::table)
            .values(NewUser::default().with_username("MrMouton"))
            .execute(&persistent_conn)?;

        // Get MrMouton's ID for easy testing (so we can ensure that a
        // combination in the name resolver gets resolved correctly in the
        // future)
        let id = users::dsl::users
            .filter(users::dsl::username.eq("MrMouton"))
            .select(users::dsl::id)
            .first(&persistent_conn)?;

        // Make a name resolver backend based on the MySQL database conn adapter
        let mut names = Persistent::new(&persistent_conn);
        names.set_combination("MrMouton", id)?;

        assert_eq!(names.username_for(id)?.unwrap(), "MrMouton");
        assert_eq!(names.user_id_for("MrMouton")?.unwrap(), id);

        Ok(())
    }
}
