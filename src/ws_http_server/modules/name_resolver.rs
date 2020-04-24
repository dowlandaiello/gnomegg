use async_trait::async_trait;
use redis_async::{client::paired::PairedConnection, error::Error, resp::RespValue};
use diesel::MysqlConnection;

/// ProviderError represents any error emitted by a name resolution provider.
#[derive(Debug)]
pub enum ProviderError {
    RedisError(Error),
}

impl From<Error> for ProviderError {
    fn from(e: Error) -> Self {
        Self::RedisError(e)
    }
}

impl<T> From<Result<T, Error>> for Result<T, ProviderError> {
    fn from(r: Result<T, Error>) -> Self {
        r.map_err(|raw_err| raw_err.into())
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
impl<'a> Provider<'a> for Cache<'a> {
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
            .into()
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
            .into()
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
    /// let names = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// names.set_combination("MrMouton", 69410)
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
            .into()
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
        Self {
            connection
        }
    }
}

impl<'a> Provider for Persistent<'a> {
    /// Retreieves the user ID matching the provided username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which a corresponding user ID should
    /// be obtained
    async fn user_id_for(&self, username: &str) -> Result<Option<i32>, ProviderError> {
        
    }
}
