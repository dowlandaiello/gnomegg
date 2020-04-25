use async_trait::async_trait;
use diesel::{
    result::{Error as DieselError, QueryResult},
    MysqlConnection, QueryDsl, RunQueryDsl,
};
use redis_async::{client::paired::PairedConnection, error::Error, resp::RespValue};

use super::super::super::spec::{
    schema::{ids, users},
    user::NewIdMapping,
};

/// ProviderError represents any error emitted by a name resolution provider.
#[derive(Debug)]
pub enum ProviderError {
    RedisError(Error),
    DieselError(DieselError),
}

impl From<Error> for ProviderError {
    fn from(e: Error) -> Self {
        Self::RedisError(e)
    }
}

impl From<DieselError> for ProviderError {
    fn from(e: DieselError) -> Self {
        Self::DieselError(e)
    }
}

/// Provider represents an arbitrary backend for the name resolution service.
#[async_trait]
pub trait Provider {
    /// Retreieves the user ID matching the provided username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should
    /// be obtained
    async fn user_id_for(&self, username: &str) -> Result<Option<i32>, ProviderError>;

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    async fn username_for(&self, user_id: i32) -> Result<Option<String>, ProviderError>;

    /// Stores a username to user ID / user ID to username mapping in a
    /// provider.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should be
    /// obtained
    async fn set_combination(&self, username: &str, user_id: i32) -> Result<(), ProviderError>;
}

/// Cache implements a name resolver based on a locally or remotely-running
/// redis instance.
pub struct Cache<'a> {
    connection: &'a PairedConnection,
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
    pub fn new(connection: &'a PairedConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
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
    async fn user_id_for(&self, username: &str) -> Result<Option<i32>, ProviderError> {
        self.connection
            .send::<Option<i32>>(resp_array!["GET", username])
            .await
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
    async fn username_for(&self, user_id: i32) -> Result<Option<String>, ProviderError> {
        self.connection
            .send::<Option<String>>(resp_array!["GET", RespValue::Integer(user_id.into())])
            .await
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
    async fn set_combination(&self, username: &str, user_id: i32) -> Result<(), ProviderError> {
        self.connection
            .send::<()>(resp_array![
                "PUT",
                format!("name::{}", username),
                RespValue::Integer(user_id.into())
            ])
            .await?;

        self.connection
            .send::<()>(resp_array!["PUT", format!("id::{}", user_id), username])
            .await
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
    fn user_id_for(&self, username: &str) -> Result<Option<i32>, ProviderError> {
        ids::dsl::ids
            .find(username)
            .select(ids::dsl::user_id)
            .first(self.connection)
            .map_err(|e| e.into())
    }

    /// Retreives the username matching the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a corresponding username should be
    /// obtained
    fn username_for(&self, user_id: i32) -> Result<Option<String>, ProviderError> {
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
    fn set_combination(&self, username: &str, user_id: i32) -> Result<(), ProviderError> {
        diesel::update(users::dsl::users.find(user_id)).set(users::dsl::username.eq(username))?;
        diesel::insert_into(users::dsl::users).values(&NewIdMapping {
            username: username,
            user_id,
        })
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
    fn user_id_for(&self, username: &str) -> Result<Option<i32>, ProviderError> {
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
    fn username_for(&self, user_id: i32) -> Result<Option<String>, ProviderError> {
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
    fn set_combination(&self, username: &str, user_id: i32) -> Result<(), ProviderError> {
        self.cache
            .set_combination(username, user_id)
            .and(self.persistent.set_combination(username, user_id))
    }
}
