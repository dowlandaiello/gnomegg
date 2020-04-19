use lru::LruCache;

/// Cache is an in-memory cache used in conjunction with gnomegg's redis and
/// postgres store capabilities. This cache is intended for only a small number
/// of users, and may not be used in place of the postgres long-term
/// persistence database.
pub struct Cache<'a> {
    /// Recent chatters who have been "muted"
    mutes: LruCache<&'a str, bool>,
}

impl<'a> Default for Cache<'a> {
    /// Creates an instance of the default mutes cache, where only 24 chatters
    /// will be stored concurrently in the aforementioned cache.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::Cache;
    /// use std::default::Default;
    ///
    /// let c = Cache::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Cache<'a> {
    /// Creates a new Cache, where the number of max stored mutes is 24, by
    /// default.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::Cache;
    ///
    /// let mutes = Cache::new();
    /// ```
    pub fn new() -> Self {
        Self {
            mutes: LruCache::new(24),
        }
    }

    /// Creates a new Cache according to the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The configuration to model the cache after
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::{Config, Cache};
    ///
    /// let cfg = Config::new(69);
    /// let muted = Cache::new_with_config(cfg);
    /// ```
    pub fn new_with_config(cfg: Config) -> Self {
        Self {
            mutes: LruCache::new(cfg.max_stored),
        }
    }

    /// Checks whether or not the user is muted in the cached mutes list.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user whose status should be checked
    /// in the cached mutes list.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::Cache;
    ///
    /// let mut mutes = Cache::new();
    ///
    /// assert_eq!(mutes.get("MrMouton"), None);
    /// ```
    pub fn get(&mut self, username: &'a str) -> Option<&bool> {
        self.mutes.get(&username)
    }

    /// Unmutes or mutes the specified user, returning the user's previous mute
    /// state.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the chatter who will be muted or unmuted
    /// * `muted` - Whether or not the user should be muted
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::Cache;
    ///
    /// let mut mutes = Cache::new();
    ///
    /// // Deplatformed by Nathan according to Destiny's deplatforming standards GODSTINY
    /// mutes.set("MrMouton", true);
    /// assert_eq!(*mutes.get("MrMouton").unwrap(), true);
    /// ```
    pub fn set(&mut self, username: &'a str, muted: bool) -> Option<bool> {
        self.mutes.put(username, muted)
    }
}

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
