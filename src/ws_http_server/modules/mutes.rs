use redis_async::client::paired::paired_connect;

/// A configuration for the mutes cache.
pub struct Config {
    /// The maximum number of mutes stored in-memory
    max_stored: usize,
}

impl Config {
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
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache};
    ///
    /// let cfg = Config::new(69);
    /// let muted = Cache::new_with_config(cfg);
    /// ```
    pub fn new(max_stored: usize) -> Self {
        Self { max_stored }
    }
}

/// Cache is a connection helper to a redis database running remotely or
/// locally.
pub struct Cache {
    connection: PairedConnetion,
}

impl Cache {
    /// Creates a new cache connection with the given remote database address.
    ///
    /// # Arguments
    ///
    /// * `database_address` - The address corresponding to the remote redis
    /// session, formatted as such: 127.0.0.1:6379
    pub fn new(database_address: &str) -> Self {
        Self {
            connection: paired_connect(&database_address).await,
        }
    }
}

/// Manages mutes across redis, postgres, and the LRU cache.
pub struct Manager {
    cache_conn: Cache,
}
