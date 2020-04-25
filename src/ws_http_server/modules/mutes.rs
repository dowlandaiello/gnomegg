use async_trait::async_trait;
use diesel::mysql::MysqlConnection;
use redis_async::{client::paired::PairedConnection, error::Error, resp::RespValue};
use serde_json::Error as SerdeError;

use std::{convert::TryInto, fmt};

use super::super::super::spec::{
    mute::Mute,
    schema::{ids, mutes},
};

/// Provider represents an arbitrary backend for the mutes service that may or
/// may not present an accurate or up to date view of the entire history of
/// mutes. Providers should be used in conjunction unless otherwise specified.
#[async_trait]
pub trait Provider {
    /// Sets a user's muted status in the active provider.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the chatter who will be muted by this command
    /// * `muted` - Whether or not this user should be muted
    /// * `duration` - (optional) The number of nanoseconds that the mute
    /// should be active for (this does not apply for unmuting a user)
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let mutes = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// mutes.set_muted("Harkdan", true).await.expect("harkdan should be muted");
    /// # }
    /// ```
    async fn set_muted(
        &self,
        user_id: i32,
        muted: bool,
        duration: Option<u64>,
    ) -> Result<Option<bool>, ProviderError>;

    /// Registers a gnomegg mute primitive in the active provider.
    ///
    /// # Arguments
    ///
    /// * `mute` - The mute primitive that should be used to modify the mutes
    /// state
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::{ws_http_server::modules::mutes::{Config, Cache, Provider}, spec::mute::Mute};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let mutes = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// let mute = Mute::new(0, 1024);
    ///
    /// mutes.register_mute(mute).await.expect("harkdan should be muted");
    /// # }
    /// ```
    async fn register_mute(&self, mute: Mute) -> Result<Option<bool>, ProviderError>;

    /// Gets the mute primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a mute primitive should be found in
    /// the caching database
    async fn get_mute(&self, mute: Mute) -> Result<Option<Mute>, ProviderError>;

    /// Checks whether or not a user with the given username has been muted
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID for which the "muted" value should be fetched
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let mutes = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// mutes.set_muted("Harkdan", true).await.expect("harkdan should be muted");
    /// assert_eq!(mutes.is_muted("Harkdan").await.unwrap().unwrap(), true);
    /// # }
    /// ```
    async fn is_muted(&self, user_id: &str) -> Result<Option<bool>, ProviderError>;
}

/// ProviderError represents any error emitted by a mute backend.
#[derive(Debug)]
pub enum ProviderError {
    RedisError(Error),
    SerdeError(SerdeError),
    MissingArgument { arg: &'static str },
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RedisError(err) => write!(f, "the provider encountered a redis error: {}", err),
        }
    }
}

impl From<Error> for ProviderError {
    /// Constructs a provider error from the given redis error.
    ///
    /// # Arguments
    ///
    /// * `e` - The redis error that should be wrapped in the ProviderError
    fn from(e: Error) -> Self {
        Self::RedisError(e)
    }
}

impl From<SerdeError> for ProviderError {
    /// Constructs a provider error from the given serde error.
    ///
    /// # Arguments
    ///
    /// * `e` - The serde error that should be wrapped in the ProviderError
    fn from(e: SerdeError) -> Self {
        Self::SerdeError(e)
    }
}

impl<Ok, Err: Into<ProviderError>> From<Result<Ok, Err>> for Result<Ok, ProviderError> {
    /// Constructs a result comprised by some type and a provider error from the
    /// to-provider implementing result (Err must be a type that can be
    /// converted to a ProviderError).
    fn from(r: Result<Ok, Error>) -> Self {
        r.map_err(|e| e.into())
    }
}

/// Cache is a connection helper to a redis database running remotely or
/// locally.
pub struct Cache<'a> {
    connection: &'a PairedConnection,
}

impl<'a> Cache<'a> {
    /// Creates a new cache connection with the given remote database address.
    ///
    /// # Arguments
    ///
    /// * `database_address` - The address corresponding to the remote redis
    /// session, formatted as such: 127.0.0.1:6379
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let cfg = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// # }
    /// ```
    pub fn new(connection: &'a PairedConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl<'a> Provider for Cache<'a> {
    /// Sets a user's muted status in the redis caching layer.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the chatter who will be muted by this command
    /// * `muted` - Whether or not this user should be muted
    /// * `duration` - (optional) The number of nanoseconds that the mute
    /// should be active for (this does not apply for unmuting a user)
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let mutes = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// mutes.set_muted("Harkdan", true).await.expect("harkdan should be muted");
    /// # }
    /// ```
    async fn set_muted(
        &self,
        user_id: i32,
        muted: bool,
        duration: Option<u64>,
    ) -> Result<Option<Mute>, ProviderError> {
        // If we're unmuting a user, we simply need to remove the redis entry
        if !muted {
            return self
                .connection
                .send::<Option<Mute>>(resp_array!["DEL", format!("muted:{}", user_id)])
                .await
                .into();
        }

        // Otherwise, insert a new mute into the redis database, and return any old entries
        self.connection
            .send::<Option<Mute>>(resp_array![
                "SET",
                format!("muted::{}", user_id),
                <Mute as TryInto<RespValue>>::try_into(Mute::new(
                    user_id,
                    duration.ok_or(ProviderError::MissingArgument { arg: "duration" })?
                ))?
            ])
            .await
            .into()
    }

    /// Registers a gnomegg mute primitive in the cache backend.
    ///
    /// # Arguments
    ///
    /// * `mute` - The mute primitive that should be used to modify the mutes
    /// state
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::{ws_http_server::modules::mutes::{Config, Cache, Provider}, spec::mute::Mute};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let mutes = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// let mute = Mute::new(0, 1024);
    ///
    /// mutes.register_mute(mute).await.expect("harkdan should be muted");
    /// # }
    /// ```
    async fn register_mute(&self, mute: Mute) -> Result<Option<bool>, ProviderError> {
        self.connection
            .send::<Option<bool>>(resp_array![
                "SET",
                format!("muted::{}", mute.concerns()),
                <Mute as TryInto<RespValue>>::try_into(mute)?
            ])
            .await
            .into()
    }

    /// Gets the mute primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a mute primitive should be found in
    /// the caching database
    async fn get_mute(&self, user_id: i32) -> Result<Option<Mute>, ProviderError> {
        self.connection
            .send::<Option<Mute>>(resp_array!["GET", format!("muted::{}", user_id)])
            .await
            .into()
    }

    /// Checks whether or not a user with the given username has been muted
    ///
    /// # Arguments
    ///
    /// * `username` - The username for which the "muted" value should be fetched
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    /// let conn = paired_connect(addr).await.expect("a connection to have been made to the redis server");
    ///
    /// let mutes = Cache::new(&conn).await.expect("a connection must be made to redis");
    /// mutes.set_muted("Harkdan", true).await.expect("harkdan should be muted");
    /// assert_eq!(mutes.is_muted("Harkdan").await.unwrap().unwrap(), true);
    /// # }
    /// ```
    async fn is_muted(&self, username: &str) -> Result<bool, ProviderError> {
        self.connection
            .send::<Option<bool>>(resp_array!["GET", format!("muted::{}", username)])
            .await
            .map(|raw| raw.unwrap_or_default())
            .into()
    }
}

/// Persistent is a mysql-based persistence layer for the gnomegg mutes backend.
pub struct Persistent<'a> {
    connection: &'a MysqlConnection,
}

impl<'a> Persistent<'a> {
    /// Creates a new connection to the mysql backend, and provides
    pub fn new(connection: &'a MysqlConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl<'a> Provider for Persistent<'a> {
    /// Sets a user's muted status in the redis caching layer.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the chatter who will be muted by this command
    /// * `muted` - Whether or not this user should be muted
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Persistent, Provider};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "mysql://localhost:3306/gnomegg";
    ///
    /// let mutes = Persistent::new(&addr).await.expect("a connection must be made to mariadb");
    /// mutes.set_muted("Harkdan", true).await.expect("harkdan should be muted");
    /// # }
    /// ```
    async fn set_muted(&self, username: &str, muted: bool) -> Result<Option<bool>, ProviderError> {}
}

/// Manages mutes across redis, postgres, and the LRU cache.
pub struct Manager<'a> {
    cache_conn: Cache<'a>,
}
