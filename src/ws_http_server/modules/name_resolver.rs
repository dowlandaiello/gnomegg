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
    fn user_id_for(&self, username: &str) -> Result<Option<u64>, ProviderError>;

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    fn username_for(&self, user_id: u64) -> Result<Option<String>, ProviderError>;

    /// Stores a username to user ID / user ID to username mapping in a
    /// provider.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should be
    /// obtained
    fn set_combination(&self, username: &str, user_id: u64) -> Result<(), ProviderError>;
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
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::names::{Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let names = Cache::new(&conn).await.expect("a connection must be made to redis");
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
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::names::{Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let names = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// assert_eq!(names.user_id_for("MrMouton").await, None);
    /// # }
    /// ```
    fn user_id_for(&self, username: &str) -> Result<Option<u64>, ProviderError> {
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
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::names::{Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let names = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// assert_eq!(names.username_for("69420").await, None);
    /// # }
    /// ```
    fn username_for(&self, user_id: u64) -> Result<Option<String>, ProviderError> {
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
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::names::{Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let names = Cache::new(&conn);
    /// assert_eq!(names.set_combination("MrMouton", 69410).await, Ok(()));
    /// assert_eq!(names.username_for(69420).await, "MrMouton".to_owned());
    /// # }
    /// ```
    fn set_combination(&self, username: &str, user_id: u64) -> Result<(), ProviderError> {
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
    fn user_id_for(&self, username: &str) -> Result<Option<u64>, ProviderError> {
        ids::dsl::ids
            .find(username)
            .select(ids::dsl::user_id)
            .first(self.connection)
            .map(|ok| Some(ok))
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
    fn username_for(&self, user_id: u64) -> Result<Option<String>, ProviderError> {
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
    fn set_combination(&self, username: &str, user_id: u64) -> Result<(), ProviderError> {
        // The user must exist in order to set a mapping between them. As such,
        // we want to update existing user entries before adding or updating
        // secondary mappings
        diesel::update(users::dsl::users.find(user_id))
            .set(users::dsl::username.eq(username))
            .execute(self.connection)?;

        diesel::insert_into(ids::dsl::ids)
            .values(&NewIdMapping {
                username: username,
                user_id,
            })
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
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::names::{Hybrid, Cache, Persistent};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let cached_names = Cache::new(&conn);
    /// let persisted_names = Persistent::new(&conn);
    /// let all_names = Hybrid::new(cached_names, persisted_names);
    /// # }
    /// ```

    fn new(cache: Cache<'a>, persistent: Persistent<'a>) -> Self {
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
    fn user_id_for(&self, username: &str) -> Result<Option<u64>, ProviderError> {
        self.cache
            .user_id_for(username)
            .or(self.persistent.user_id_for(username))
    }

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    fn username_for(&self, user_id: u64) -> Result<Option<String>, ProviderError> {
        self.cache
            .username_for(user_id)
            .or(self.persistent.username_for(user_id))
    }

    /// Stores a username to user ID / user ID to username mapping in a
    /// provider.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should be
    /// obtained
    fn set_combination(&self, username: &str, user_id: u64) -> Result<(), ProviderError> {
        self.cache
            .set_combination(username, user_id)
            .and(self.persistent.set_combination(username, user_id))
    }
}
