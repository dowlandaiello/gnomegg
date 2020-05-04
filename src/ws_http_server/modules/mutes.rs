use diesel::{result::Error as DieselError, QueryDsl, RunQueryDsl};
use redis::RedisError;

use super::{
    super::super::spec::{mute::Mute, schema::mutes},
    Cache, Hybrid, Persistent, ProviderError,
};

/// Provider represents an arbitrary backend for the mutes service that may or
/// may not present an accurate or up to date view of the entire history of
/// mutes. Providers should be used in conjunction unless otherwise specified.
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
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000));
    /// Ok(())
    /// # }
    /// ```
    fn set_muted(
        &mut self,
        user_id: u64,
        muted: bool,
        duration: Option<u64>,
    ) -> Result<bool, ProviderError>;

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
    /// use gnomegg::{ws_http_server::modules::mutes::{Cache, Provider}, spec::mute::Mute};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// let mute = Mute::new(0, 1024);
    ///
    /// mutes.register_mute(&mute);
    /// Ok(())
    /// # }
    /// ```
    fn register_mute(&mut self, mute: &Mute) -> Result<Option<Mute>, ProviderError>;

    /// Gets the mute primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a mute primitive should be found in
    /// the caching database
    fn get_mute(&mut self, user_id: u64) -> Result<Option<Mute>, ProviderError>;

    /// Checks whether or not a user with the given username has been muted
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID for which the "muted" value should be fetched
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000)).expect("harkdan should be muted");
    /// assert_eq!(mutes.is_muted(1).unwrap(), true);
    /// Ok(())
    /// # }
    /// ```
    fn is_muted(&mut self, user_id: u64) -> Result<bool, ProviderError>;
}

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
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000)).expect("harkdan should be muted");
    /// Ok(())
    /// # }
    /// ```
    fn set_muted(
        &mut self,
        user_id: u64,
        muted: bool,
        duration: Option<u64>,
    ) -> Result<bool, ProviderError> {
        // If we're unmuting a user, we simply need to remove the redis entry
        if !muted {
            let already_muted = self.is_muted(user_id)?;

            redis::cmd("DEL")
                .arg(format!("muted::{}", user_id))
                .query(self.connection)
                .map_err(<RedisError as Into<ProviderError>>::into)?;

            return Ok(already_muted);
        }

        // Otherwise, insert a new mute into the redis database, and return any old entries
        Ok(self
            .register_mute(&Mute::new(
                user_id,
                duration.ok_or(ProviderError::MissingArgument { arg: "duration" })?,
            ))?
            .map_or(false, |mute| mute.active()))
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
    /// use gnomegg::{ws_http_server::modules::mutes::{Cache, Provider}, spec::mute::Mute};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// let mute = Mute::new(0, 1024);
    ///
    /// mutes.register_mute(&mute).expect("harkdan should be muted");
    /// Ok(())
    /// # }
    /// ```
    fn register_mute(&mut self, mute: &Mute) -> Result<Option<Mute>, ProviderError> {
        redis::cmd("GETSET")
            .arg(format!("muted::{}", mute.concerns()))
            .arg(serde_json::to_string(mute)?)
            .query::<Option<String>>(self.connection)
            .map_err(|e| e.into())
            .map(|raw| {
                raw.map(|str_data| serde_json::from_str::<Mute>(&str_data).map(Some))?
                    .unwrap_or(None)
            })
    }

    /// Gets the mute primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a mute primitive should be found in
    /// the caching database
    fn get_mute(&mut self, user_id: u64) -> Result<Option<Mute>, ProviderError> {
        redis::cmd("GET")
            .arg(format!("muted::{}", user_id))
            .query::<Option<String>>(self.connection)
            .map_err(|e| e.into())
            .map(|raw| {
                raw.map(|str_data| serde_json::from_str::<Mute>(&str_data).map(Some))?
                    .unwrap_or(None)
            })
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
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000)).expect("harkdan should be muted");
    /// assert_eq!(mutes.is_muted(1).unwrap(), true);
    /// Ok(())
    /// # }
    /// ```
    fn is_muted(&mut self, user_id: u64) -> Result<bool, ProviderError> {
        Ok(self.get_mute(user_id)?.map_or(false, |mute| mute.active()))
    }
}

impl<'a> Provider for Persistent<'a> {
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
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000)).expect("harkdan should be muted");
    /// Ok(())
    /// # }
    /// ```
    fn set_muted(
        &mut self,
        user_id: u64,
        muted: bool,
        duration: Option<u64>,
    ) -> Result<bool, ProviderError> {
        let old = self.get_mute(user_id)?;

        // If the user is being unmuted, we simply need to delete the row
        // corresponding to the user's mute in the database
        if !muted {
            return diesel::delete(mutes::dsl::mutes.find(user_id))
                .execute(self.connection)
                .map(|_| old.map_or(false, |mute| mute.active()))
                .map_err(|e| e.into());
        }

        // Otherwise, insert a new mute entry
        Ok(self
            .register_mute(&Mute::new(
                user_id,
                duration.ok_or(ProviderError::MissingArgument { arg: "duration" })?,
            ))?
            .map_or(false, |mute| mute.active()))
    }

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
    /// use gnomegg::{ws_http_server::modules::mutes::{Cache, Provider}, spec::mute::Mute};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// let mute = Mute::new(1, 1024);
    ///
    /// mutes.register_mute(&mute).expect("harkdan should be muted");
    /// Ok(())
    /// # }
    /// ```
    fn register_mute(&mut self, mute: &Mute) -> Result<Option<Mute>, ProviderError> {
        let old = self.get_mute(mute.concerns())?;

        diesel::replace_into(mutes::table)
            .values(mute)
            .execute(self.connection)?;

        Ok(old)
    }

    /// Gets the mute primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a mute primitive should be found in
    /// the caching database
    fn get_mute(&mut self, user_id: u64) -> Result<Option<Mute>, ProviderError> {
        mutes::dsl::mutes
            .find(user_id)
            .first::<Mute>(self.connection)
            .map(Some)
            .or_else(|e| {
                if let DieselError::NotFound = e {
                    Ok(None)
                } else {
                    Err(<DieselError as Into<ProviderError>>::into(e))
                }
            })
    }

    /// Checks whether or not a user with the given username has been muted
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID for which the "muted" value should be fetched
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000)).expect("harkdan should be muted");
    /// assert_eq!(mutes.is_muted(1).unwrap(), true);
    /// Ok(())
    /// # }
    /// ```
    fn is_muted(&mut self, user_id: u64) -> Result<bool, ProviderError> {
        Ok(self.get_mute(user_id)?.map_or(false, |mute| mute.active()))
    }
}

impl<'a> Provider for Hybrid<'a> {
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
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000)).expect("harkdan should be muted");
    /// Ok(())
    /// # }
    /// ```
    fn set_muted(
        &mut self,
        user_id: u64,
        muted: bool,
        duration: Option<u64>,
    ) -> Result<bool, ProviderError> {
        self.cache
            .set_muted(user_id, muted, duration)
            .and(self.persistent.set_muted(user_id, muted, duration))
    }

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
    /// use gnomegg::{ws_http_server::modules::mutes::{Cache, Provider}, spec::mute::Mute};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// let mute = Mute::new(0, 1024);
    ///
    /// mutes.register_mute(&mute).expect("harkdan should be muted");
    /// Ok(())
    /// # }
    /// ```
    fn register_mute(&mut self, mute: &Mute) -> Result<Option<Mute>, ProviderError> {
        self.cache
            .register_mute(&mute)
            .and(self.persistent.register_mute(&mute))
    }

    /// Gets the mute primitive corresponding to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID for which a mute primitive should be found in
    /// the caching database
    fn get_mute(&mut self, user_id: u64) -> Result<Option<Mute>, ProviderError> {
        self.cache
            .get_mute(user_id)
            .or_else(|_| self.persistent.get_mute(user_id))
    }

    /// Checks whether or not a user with the given username has been muted
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID for which the "muted" value should be fetched
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::ws_http_server::modules::mutes::{Cache, Provider};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let client = redis::Client::open("redis://127.0.0.1/")?;
    /// let mut conn = client.get_connection()?;
    ///
    /// let mut mutes = Cache::new(&mut conn);
    /// mutes.set_muted(1, true, Some(1_000_000_000)).expect("harkdan should be muted");
    /// assert_eq!(mutes.is_muted(1).unwrap(), true);
    /// Ok(())
    /// # }
    /// ```
    fn is_muted(&mut self, user_id: u64) -> Result<bool, ProviderError> {
        self.cache
            .is_muted(user_id)
            .or_else(|_| self.persistent.is_muted(user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::super::spec::{schema::users, user::NewUser},
        *,
    };
    use diesel::{ExpressionMethods, Connection, mysql::MysqlConnection};
    use dotenv;

    use std::{default::Default, env, error::Error};

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

        // Mute MrMouton for 2048 nanoseconds
        let mut mutes = Hybrid::new(Cache::new(&mut conn), Persistent::new(&persistent_conn));
        mutes.set_muted(id, true, Some(1_000_000_000))?;

        assert_eq!(mutes.is_muted(id)?, true);

        Ok(())
    }

    #[test]
    fn test_cache() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

        let mut conn = redis::Client::open("redis://127.0.0.1/")?.get_connection()?;

        let mut mutes = Cache::new(&mut conn);
        mutes.set_muted(42069, true, Some(1_000_000))?;

        assert_eq!(mutes.is_muted(42069)?, true);

        Ok(())
    }

    #[test]
    fn test_persistent() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;

        // Open a connection with the MySQL server
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

        // Make a name resolver backend based on the MySQL database conn adapter
        let mut mutes = Persistent::new(&persistent_conn);
        mutes.set_muted(id, true, Some(1_000_000_000))?;

        assert_eq!(mutes.is_muted(id)?, true);

        Ok(())
    }
}
