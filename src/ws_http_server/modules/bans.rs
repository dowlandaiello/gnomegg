use chrono::Utc;
use diesel::{
    mysql::MysqlConnection, result::Error as DieselError, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use redis::{Connection, RedisError};
use serde_json::Error as SerdeError;

use std::{error::Error, fmt};

use super::super::super::spec::{
    ban::{Ban, NewBan},
    schema::bans,
};

/// BanQuery represents a query for a ban based on its IP or corresponding user
/// ID.
pub enum BanQuery<'a> {
    Address(&'a str),
    Id(u64),
}

/// Provider represents an arbitrary backend for the bans service that may or
/// may not present an accurate or up to date view of the entire history of
/// bans. Providers should be used in conjunction unless otherwise specified.
pub trait Provider {
    /// Sets a user's banned status in the active provider.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the chatter who will be banned by this command
    /// * `banned` - Whether or not this user should be banned
    /// * `duration` - (optional) The number of nanoseconds that the ban
    /// should be active for (this does not apply for unmuting a user)
    /// * `ip` - (optional) The IP of the user that should be registered as
    /// banned
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::bans::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut bans = Cache::new(&mut conn);
    /// bans.set_banned(1, true, None, None).expect("harkdan should be banned");
    /// Ok(())
    /// # }
    /// ```
    fn set_banned(
        &mut self,
        user_id: u64,
        banned: bool,
        duration: Option<u64>,
        ip: Option<&str>,
    ) -> Result<bool, ProviderError>;

    /// Registers a gnomegg ban primitive in the active provider.
    ///
    /// # Arguments
    ///
    /// * `ban` - The ban primitive that should be used to modify the bans
    /// state
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::{ws_http_server::modules::bans::{Cache, Provider}, spec::ban::NewBan};
    /// use chrono::offset::Utc;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut bans = Cache::new(&mut conn);
    /// bans.register_ban(&NewBan::new(1, None, Utc::now(), None));
    /// # Ok(())
    /// # }
    /// ```
    fn register_ban<'a>(&mut self, ban: &NewBan<'a>) -> Result<Option<Ban>, ProviderError>;

    /// Gets the ban primitive corresponding to the given user ID or IP address.
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::bans::{Cache, Provider, BanQuery};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(),Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut bans = Cache::new(&mut conn);
    /// bans.set_banned(1, true, None, None).expect("Dan should be banned");
    /// assert_eq!(bans.get_ban(&BanQuery::Id(1)).unwrap().unwrap().active(), true);
    /// # Ok(())
    /// # }
    /// ```
    fn get_ban(&mut self, query: &BanQuery) -> Result<Option<Ban>, ProviderError>;

    /// Checks whether or not a user with the given username or address has been
    /// banned.
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate tokio;
    /// use gnomegg::ws_http_server::modules::bans::{Cache, Provider, BanQuery};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut bans = Cache::new(&mut conn);
    /// bans.set_banned(1, true, None, None).expect("harkdan should be banned");
    /// assert_eq!(bans.is_banned(&BanQuery::Id(1)).unwrap(), true);
    /// # Ok(())
    /// # }
    /// ```
    fn is_banned(&mut self, query: &BanQuery) -> Result<bool, ProviderError>;
}

/// ProviderError represents any error emitted by a ban backend.
#[derive(Debug)]
pub enum ProviderError {
    RedisError(RedisError),
    SerdeError(SerdeError),
    DieselError(DieselError),
    MissingArgument { arg: &'static str },
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RedisError(err) => write!(f, "the provider encountered a redis error: {}", err),
            Self::SerdeError(err) => {
                write!(f, "the provider encountered a serialization error: {}", err)
            }
            Self::DieselError(err) => {
                write!(f, "the provider encountered a database error: {}", err)
            }
            Self::MissingArgument { arg } => {
                write!(f, "malformed query; missing argument: {}", arg)
            }
        }
    }
}

impl Error for ProviderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RedisError(e) => Some(e),
            Self::SerdeError(e) => Some(e),
            Self::DieselError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<RedisError> for ProviderError {
    /// Constructs a provider error from the given redis error.
    ///
    /// # Arguments
    ///
    /// * `e` - The redis error that should be wrapped in the ProviderError
    fn from(e: RedisError) -> Self {
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

impl From<DieselError> for ProviderError {
    /// Cosntructs a provider error from the given diesel error.
    ///
    /// # Arguments
    ///
    /// * `e` - The diesel error that should be wrapped in the ProviderError
    fn from(e: DieselError) -> Self {
        Self::DieselError(e)
    }
}

/// Cache is a connection helper to a redis database running remotely or
/// locally.
pub struct Cache<'a> {
    connection: &'a mut Connection,
}

impl<'a> Cache<'a> {
    /// Creates a new cache connection with the given remote database address.
    ///
    /// # Arguments
    ///
    /// * `database_address` - The address corresponding to the remote redis
    /// session, formatted as such: 127.0.0.1:6379
    pub fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }
}

impl<'a> Provider for Cache<'a> {
    /// Sets a user's banned status in the redis caching layer.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the chatter who will be banned by this command
    /// * `banned` - Whether or not this user should be banned
    /// * `duration` - (optional) The number of nanoseconds that the ban
    /// should be active for (this does not apply for unmuting a user)
    /// * `ip` - (optional) The IP of the user that should be banned
    fn set_banned(
        &mut self,
        user_id: u64,
        banned: bool,
        duration: Option<u64>,
        ip: Option<&str>,
    ) -> Result<bool, ProviderError> {
        // If we're unmuting a user, we simply need to remove the redis entry
        if !banned {
            if let Some(addr) = ip {
                redis::cmd("DEL")
                    .arg(format!("banned_addr::{}", addr))
                    .query(self.connection)
                    .map_err(<RedisError as Into<ProviderError>>::into)?;
            }

            return redis::cmd("DEL")
                .arg(format!("banned::{}", user_id))
                .query(self.connection)
                .map_err(|e| e.into());
        }

        // Otherwise, insert a new ban into the redis database, and return any old entries
        Ok(self
            .register_ban(&NewBan::new(user_id, duration, Utc::now(), ip))?
            .map_or(false, |ban| ban.active()))
    }

    /// Registers a gnomegg ban primitive in the cache backend.
    ///
    /// # Arguments
    ///
    /// * `ban` - The ban primitive that should be used to modify the bans
    /// state
    fn register_ban(&mut self, ban: &NewBan) -> Result<Option<Ban>, ProviderError> {
        if let Some(addr) = ban.address() {
            redis::cmd("SET")
                .arg(format!("banned_addr::{}", addr))
                .arg(serde_json::to_vec(ban)?)
                .query::<()>(self.connection)?;
        }

        redis::cmd("GETSET")
            .arg(format!("banned::{}", ban.concerns()))
            .arg(serde_json::to_vec(ban)?)
            .query::<Option<String>>(self.connection)
            .map_err(|e| e.into())
            .map(|raw| {
                raw.map(|str_data| serde_json::from_str::<Ban>(&str_data).map(Some))?
                    .unwrap_or(None)
            })
    }

    /// Gets the ban primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    fn get_ban(&mut self, query: &BanQuery) -> Result<Option<Ban>, ProviderError> {
        redis::cmd("GET")
            .arg(match query {
                BanQuery::Address(s) => format!("banned_addr::{}", s),
                BanQuery::Id(id) => format!("banned::{}", id),
            })
            .query::<Option<String>>(self.connection)
            .map_err(|e| e.into())
            .map(|raw| {
                raw.map(|str_data| serde_json::from_str::<Ban>(&str_data).map(Some))?
                    .unwrap_or(None)
            })
    }

    /// Checks whether or not a user with the given username has been banned
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    fn is_banned(&mut self, query: &BanQuery) -> Result<bool, ProviderError> {
        Ok(self.get_ban(query)?.map_or(false, |ban| ban.active()))
    }
}

/// Persistent is a mysql-based persistence layer for the gnomegg bans backend.
pub struct Persistent<'a> {
    connection: &'a MysqlConnection,
}

impl<'a> Persistent<'a> {
    /// Creates a new connection to the mysql backend, and provides
    pub fn new(connection: &'a MysqlConnection) -> Self {
        Self { connection }
    }
}

impl<'a> Provider for Persistent<'a> {
    /// Sets a user's banned status in the redis caching layer.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the chatter who will be banned by this command
    /// * `banned` - Whether or not this user should be banned
    /// * `duration` - (optional) The number of nanoseconds that the ban
    /// should be active for (this does not apply for unmuting a user)
    /// * `ip` - (optional) The IP of the user that should be banned
    fn set_banned(
        &mut self,
        user_id: u64,
        banned: bool,
        duration: Option<u64>,
        ip: Option<&str>,
    ) -> Result<bool, ProviderError> {
        let old = self.get_ban(&BanQuery::Id(user_id))?;

        // If the user is being unbanned, we simply need to delete the row
        // corresponding to the user's ban in the database
        if !banned {
            return diesel::delete(bans::dsl::bans.find(user_id))
                .execute(self.connection)
                .map(|_| old.map_or(false, |ban| ban.active()))
                .map_err(|e| e.into());
        }

        // Otherwise, insert a new ban entry
        Ok(self
            .register_ban(&NewBan::new(user_id, duration, Utc::now(), ip))?
            .map_or(false, |ban| ban.active()))
    }

    /// Registers a gnomegg ban primitive in the cache backend.
    ///
    /// # Arguments
    ///
    /// * `ban` - The ban primitive that should be used to modify the bans
    /// state
    fn register_ban(&mut self, ban: &NewBan) -> Result<Option<Ban>, ProviderError> {
        let old = self.get_ban(&BanQuery::Id(ban.concerns()))?;

        diesel::replace_into(bans::table)
            .values(ban)
            .execute(self.connection)?;

        Ok(old)
    }

    /// Gets the ban primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    fn get_ban(&mut self, query: &BanQuery) -> Result<Option<Ban>, ProviderError> {
        let ban = match query {
            BanQuery::Id(id) => bans::dsl::bans.find(id).first::<Ban>(self.connection),
            BanQuery::Address(address) => bans::dsl::bans
                .filter(bans::dsl::ip.eq(address))
                .first::<Ban>(self.connection),
        };

        ban.map(Some).or_else(|e| {
            if let DieselError::NotFound = e {
                Ok(None)
            } else {
                Err(<DieselError as Into<ProviderError>>::into(e))
            }
        })
    }

    /// Checks whether or not a user with the given username has been banned
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    fn is_banned(&mut self, query: &BanQuery) -> Result<bool, ProviderError> {
        Ok(self.get_ban(query)?.map_or(false, |ban| ban.active()))
    }
}

/// Hybrid manages bans across redis and MySQL.
pub struct Hybrid<'a> {
    /// The cached bans storage layer
    cache: Cache<'a>,

    /// The persistent bans storage layer
    persistent: Persistent<'a>,
}

impl<'a> Hybrid<'a> {
    /// Creates a new hybrid bans storage service with the provided persistent
    /// and cached bans helper layers.
    ///
    /// # Arguments
    ///
    /// * `cache` - The redis caching helper to use
    /// * `persistent` - The MySQL storage helper to use
    pub fn new(cache: Cache<'a>, persistent: Persistent<'a>) -> Self {
        Self { cache, persistent }
    }
}

impl<'a> Provider for Hybrid<'a> {
    /// Sets a user's banned status in the active provider.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the chatter who will be banned by this command
    /// * `banned` - Whether or not this user should be banned
    /// * `duration` - (optional) The number of nanoseconds that the ban
    /// should be active for (this does not apply for unmuting a user)
    /// * `ip` - (optional) The IP of the user that should be registered as
    /// banned
    fn set_banned(
        &mut self,
        user_id: u64,
        banned: bool,
        duration: Option<u64>,
        ip: Option<&str>,
    ) -> Result<bool, ProviderError> {
        self.cache
            .set_banned(user_id, banned, duration, ip)
            .and(self.persistent.set_banned(user_id, banned, duration, ip))
    }

    /// Registers a gnomegg ban primitive in the active provider.
    ///
    /// # Arguments
    ///
    /// * `ban` - The ban primitive that should be used to modify the bans
    /// state
    fn register_ban(&mut self, ban: &NewBan) -> Result<Option<Ban>, ProviderError> {
        self.cache
            .register_ban(ban)
            .and(self.persistent.register_ban(ban))
    }

    /// Gets the ban primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    fn get_ban(&mut self, query: &BanQuery) -> Result<Option<Ban>, ProviderError> {
        self.cache
            .get_ban(query)
            .or_else(|_| self.persistent.get_ban(query))
    }

    /// Checks whether or not a user with the given username has been banned
    ///
    /// # Arguments
    ///
    /// * `query` - A query containing an IP address or a user ID that should be
    /// searched for in the database
    fn is_banned(&mut self, query: &BanQuery) -> Result<bool, ProviderError> {
        self.cache
            .is_banned(query)
            .or_else(|_| self.persistent.is_banned(query))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::super::spec::{schema::users, user::NewUser},
        *,
    };

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

        // Ban MrMouton forever
        let mut bans = Hybrid::new(Cache::new(&mut conn), Persistent::new(&persistent_conn));
        bans.set_banned(id, true, None, None)?;

        assert_eq!(bans.is_banned(&BanQuery::Id(id))?, true);

        Ok(())
    }

    #[test]
    fn test_cache() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

        let mut conn = redis::Client::open("redis://127.0.0.1/")?.get_connection()?;

        // Ban MrMouton forever
        let mut bans = Cache::new(&mut conn);
        bans.set_banned(42069, true, None, None)?;

        assert_eq!(bans.is_banned(&BanQuery::Id(42069))?, true);

        Ok(())
    }

    #[test]
    fn test_persistent() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

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

        // Ban MrMouton forever
        let mut bans = Persistent::new(&persistent_conn);
        bans.set_banned(id, true, None, None)?;

        assert_eq!(bans.is_banned(&BanQuery::Id(id))?, true);

        Ok(())
    }
}
