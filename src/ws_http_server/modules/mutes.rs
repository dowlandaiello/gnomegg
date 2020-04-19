use std::collections::HashMap;

/// CacheEntry represents an entry declaring whether or not a user is muted,
/// alongside the number of accesses.
type CacheEntry = (bool, usize);

/// Cache is an in-memory cache used in conjunction with gnomegg's redis and
/// postgres store capabilities. This cache is intended for only a small number
/// of users, and may not be used in place of the postgres long-term
/// persistence database.
pub struct Cache<'a> {
    /// Recent chatters who have been "muted"
    mutes: HashMap<String, (bool, usize)>,

    /// The name of the chatter who has interacted the least with this cache
    least_used: Option<(&'a str, usize)>,

    /// The maximum number of cached mutes
    max_stored: usize,
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
            mutes: HashMap::new(),
            least_used: None,
            max_stored: 24,
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
            mutes: HashMap::new(),
            least_used: None,
            max_stored: cfg.max_stored,
        }
    }

    /// Checks whether or not the user is muted in the cached mutes list.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user whose status should be checked
    /// in the cached mutes list.
    pub fn get(&mut self, username: &'a str) -> Option<bool> {
        let muted = self.mutes.get_mut(username)?;

        muted.1 += 1;

        if let Some(previously_least_used) = self.least_used {
            if muted.1 < previously_least_used.1 {
                self.least_used = Some((username, muted.1));
            }
        }

        Some(muted.0)
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
    /// mutes.set("MrMouton".to_owned(), true);
    /// assert_eq!(mutes.get("MrMouton").unwrap(), true);
    /// ```
    pub fn set(&mut self, username: String, muted: bool) -> Option<CacheEntry> {
        if self.mutes.len() >= self.max_stored {
            self.evict();
        }

        let prev = self.mutes.insert(username, (muted, 0));

        self.least_used = Some((prev, 0));

        prev
    }

    /// Evicts the least recently used mute entry in the cache.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Config};
    ///
    /// let mut mutes = Cache::new_with_config(Config::new(1));
    /// mutes.set("MrMouton".to_owned(), true);
    /// mutes.evict();
    ///
    /// assert_eq!(mutes.get("MrMouton"), None);
    /// ```
    pub fn evict(&mut self) -> Option<CacheEntry> {
        let least_used = self.least_used.take()?;
        self.mutes.remove(least_used.0)
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
