use diesel::{
    expression_methods::ExpressionMethods, result::Error as DieselError, MysqlConnection, QueryDsl,
    RunQueryDsl,
};
use redis::{Connection, RedisError};

use super::super::super::spec::{
    schema::{ids, users},
    user::NewIdMapping,
};

/// ProviderError represents any error emitted by a name resolution provider.
#[derive(Debug)]
pub enum ProviderError {
    RedisError(RedisError),
    DieselError(DieselError),
}

impl From<RedisError> for ProviderError {
    fn from(e: RedisError) -> Self {
        Self::RedisError(e)
    }
}

impl From<DieselError> for ProviderError {
    fn from(e: DieselError) -> Self {
        Self::DieselError(e)
    }
}

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

/// Cache implements a name resolver based on a locally or remotely-running
/// redis instance.
pub struct Cache<'a> {
    connection: &'a mut Connection,
}

impl<'a> Cache<'a> {
    /// Creates a new cache connection with the given remote database connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The redis connection over which data should be
    /// requested or sent
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::name_resolver::{Cache};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut names = Cache::new(&mut conn);
    /// Ok(())
    /// # }
    /// ```
    pub fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }
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
    /// assert_eq!(names.user_id_for("MrMouton").unwrap(), None);
    /// Ok(())
    /// # }
    /// ```
    fn user_id_for(&mut self, username: &str) -> Result<Option<u64>, ProviderError> {
        redis::cmd("GET")
            .arg(username)
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
    /// assert_eq!(names.username_for(69420).unwrap(), None);
    /// Ok(())
    /// # }
    /// ```
    fn username_for(&mut self, user_id: u64) -> Result<Option<String>, ProviderError> {
        redis::cmd("GET")
            .arg(user_id)
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
    /// assert_eq!(names.set_combination("MrMouton", 69410).unwrap(), ());
    /// assert_eq!(names.username_for(69420).unwrap().unwrap(), "MrMouton".to_owned());
    /// Ok(())
    /// # }
    /// ```
    fn set_combination(&mut self, username: &str, user_id: u64) -> Result<(), ProviderError> {
        redis::cmd("PUT")
            .arg(username)
            .arg(user_id)
            .query(self.connection)
            .map_err(|e| e.into())
    }
}

/// Persistent is a mysql-based persistence layer for the gnomegg name
/// resolution service.
pub struct Persistent<'a> {
    connection: &'a MysqlConnection,
}

impl<'a> Persistent<'a> {
    /// Creats a new persistence service helper with the given connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The SQL connection that should be used to send and
    /// retreieve data.
    pub fn new(connection: &'a MysqlConnection) -> Self {
        Self { connection }
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

        diesel::insert_into(ids::dsl::ids)
            .values(&NewIdMapping::new(username, user_id))
            .execute(self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}

/// Hybrid implements a provider utilizing both persistent and cached name
/// resolution.
pub struct Hybrid<'a> {
    /// The cached name storage layer
    cache: Cache<'a>,

    /// The persistent name storage layer
    persistent: Persistent<'a>,
}

impl<'a> Hybrid<'a> {
    /// Creates a new hybrid name resolution service with the provided
    /// persistent and cached helper layers.
    ///
    /// # Arguments
    ///
    /// * `cache` - The redis caching helper to use
    /// * `persistent` - The MySQL storage helper to use
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::name_resolver::{Hybrid, Cache, Persistent, Provider};
    /// # use diesel::{mysql::{MysqlConnection}, connection::Connection};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    /// let persistent_conn = MysqlConnection::establish("mysql://localhost:3306/gnomegg")?;
    ///
    /// let cached_names = Cache::new(&mut conn);
    /// let persisted_names = Persistent::new(&persistent_conn);
    /// let all_names = Hybrid::new(cached_names, persisted_names);
    /// Ok(())
    /// # }
    /// ```
    pub fn new(cache: Cache<'a>, persistent: Persistent<'a>) -> Self {
        Self { cache, persistent }
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
        self.cache
            .user_id_for(username)
            .or_else(|_| self.persistent.user_id_for(username))
    }

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    fn username_for(&mut self, user_id: u64) -> Result<Option<String>, ProviderError> {
        self.cache
            .username_for(user_id)
            .or_else(|_| self.persistent.username_for(user_id))
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
