use redis_async::{
    client::paired::{paired_connect, PairedConnection},
    error::Error,
};
use std::net::SocketAddr;

/// A configuration for the mutes cache.
pub struct Config<'a> {
    /// The address of the redis instance
    redis_address: &'a SocketAddr,
}

impl<'a> Config<'a> {
    /// Creats a new configuration for the mutes cache.
    ///
    /// # Arguments
    ///
    /// * `max_stored` - The maximum number of chatters stored in the cache.
    /// Chatters are evicted on an LRU basis.
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
    ///
    /// let cfg = Config::new(&addr);
    /// let muted = Cache::new_with_config(cfg).await.expect("a connection must be made to redis");
    /// # }
    /// ```
    pub fn new(redis_address: &'a SocketAddr) -> Self {
        Self { redis_address }
    }
}

/// Cache is a connection helper to a redis database running remotely or
/// locally.
pub struct Cache {
    connection: PairedConnection,
}

impl Cache {
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
    ///
    /// let cfg = Cache::new(&addr).await.expect("a connection must be made to redis");
    /// # }
    /// ```
    pub async fn new(database_address: &SocketAddr) -> Result<Self, Error> {
        Ok(Self {
            connection: paired_connect(&database_address).await?,
        })
    }

    /// Creates a new cache connection with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The configuration that should be used to created the cache
    /// connection
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
    ///
    /// let cfg = Config::new(&addr);
    /// let muted = Cache::new_with_config(cfg).await.expect("a connection must be made to redis");
    /// # }
    /// ```
    pub async fn new_with_config<'a>(cfg: Config<'a>) -> Result<Self, Error> {
        Self::new(cfg.redis_address).await
    }

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
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let addr = "127.0.0.1:6379".parse().expect("the redis address should have been parsed successfully");
    ///
    /// let mutes = Cache::new(&addr).await.expect("a connection must be made to redis");
    /// mutes.set_muted("Harkdan", true).await.expect("harkdan should be muted");
    /// # }
    /// ```
    pub async fn set_muted(&self, username: &str, muted: bool) -> Result<Option<bool>, Error> {
        self.connection
            .send::<String>(resp_array![
                "SET",
                format!("muted::{}", username),
                format!("{}", muted)
            ])
            .await
            .map(|raw| raw.parse::<bool>().ok())
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
    ///
    /// let mutes = Cache::new(&addr).await.expect("a connection must be made to redis");
    /// mutes.set_muted("Harkdan", true).await.expect("harkdan should be muted");
    /// assert_eq!(mutes.is_muted("Harkdan").await.unwrap().unwrap(), true);
    /// # }
    /// ```
    pub async fn is_muted(&self, username: &str) -> Result<Option<bool>, Error> {
        self.connection
            .send::<String>(resp_array!["GET", format!("muted::{}", username)])
            .await
            .map(|raw| raw.parse::<bool>().ok())
    }
}

/// Manages mutes across redis, postgres, and the LRU cache.
pub struct Manager {
    cache_conn: Cache,
}
