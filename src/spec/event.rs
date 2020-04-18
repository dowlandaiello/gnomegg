use chrono::{DateTime, Utc};

/// Message is a message sent as text, rendered on the client.
pub struct Message {
    /// The contents of the message
    contents: String,
}

impl Message {
    /// Creates a new owned message.
    ///
    /// # Arguments
    ///
    /// * `contents` - A string representing the contents of the message
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Message;
    ///
    /// let msg = Message::new("Mitta mitt mooowooo mitty mitta mitt mwoomooo".to_owned());
    /// ```
    pub fn new(contents: String) -> Self {
        Self { contents }
    }

    /// Returns the contents of the message.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Message;
    ///
    /// let msg = Message::new("Alright, you guys want to hear my most nuclear take?".to_owned());
    /// msg.msg(); // => "Alright, you guys want to hear my most nuclear take?"
    /// ````
    pub fn msg(&self) -> &str {
        &self.contents
    }
}

/// PrivMessage is a message sent as text, rendered on the client corresponding
/// to the user that the message is targeting
pub struct PrivMessage {
    /// The username of the chatter that the message will be sent to
    concerns: String,

    /// The contents of the private message
    message: Message,
}

impl PrivMessage {
    /// Creates a new owned private message.
    ///
    /// # Arguments
    ///
    /// * `to` - A string representing the username of the recipient of this
    /// message
    /// * `contents` - A string representing the contents of the message
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::PrivMessage;
    ///
    /// let msg = PrivMessage::new("essaywriter".to_owned(), "I have information concerning the murder of Jeffrey Epstein.".to_owned());
    /// ```
    pub fn new(to: String, contents: String) -> Self {
        Self {
            concerns: to,
            message: Message::new(contents),
        }
    }

    /// Retreives the username of the recipient of this message.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::PrivMessage;
    ///
    /// let msg = PrivMessage::new("essaywriter".to_owned(), "I have information concerning the murder of Jeffrey Epstein.".to_owned());
    /// msg.to(); // => "essaywriter"
    /// ```
    pub fn to(&self) -> &str {
        &self.concerns
    }

    /// Retrieves the contents of the private message.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::PrivMessage;
    ///
    /// let msg = PrivMessage::new("essaywriter".to_owned(), "I have information concerning the murder of Jeffrey Epstein.".to_owned());
    /// msg.contents(); // => "I have information concerning the murder of Jeffrey Epstein."
    /// ```
    pub fn contents(&self) -> &str {
        self.message.msg()
    }
}

/// Mute is a command issued to mute a particular user.
pub struct Mute {
    /// The user that will be muted by this command
    concerns: String,

    /// The number of nanoseconds until the user will be unmuted
    duration: u64,
}

impl Mute {
    /// Creates a new mute command.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Mute;
    ///
    /// // Mute essaywriter for 666 nanoseconds for posting pepe cringe
    /// let mute = Mute::new("essaywriter".to_owned(), 666);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `user` - The username of the user who will be muted by this command
    /// * `duration` - The number of nanoseconds until the user will be unmuted
    pub fn new(user: String, duration: u64) -> Self {
        Self {
            concerns: user,
            duration,
        }
    }

    /// Retreives the username of the user who will be muted by this command.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Mute;
    ///
    /// let mute = Mute::new("essaywriter".to_owned(), 666);
    /// mute.user(); // => "essaywriter"
    /// ```
    pub fn user(&self) -> &str {
        &self.concerns
    }

    /// Retreives the number of nanoseconds that the aforementioned user should
    /// be muted for.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Mute;
    ///
    /// let mute = Mute::new("essaywriter".to_owned(), 666);
    /// mute.timeframe(); // => 666
    pub fn timeframe(&self) -> u64 {
        self.duration
    }
}

/// Unmute is a command used to unmute a particular chatter.
pub struct Unmute {
    /// The username of the user who will be unmuted by this command
    concerns: String,
}

impl Unmute {
    /// Creates a new unmute command.
    ///
    /// # Arguments
    ///
    /// * `user` - The username of the chatter who will be unmuted by this command
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Unmute;
    ///
    /// // Reformed AngelThump
    /// let unmute = Unmute::new("essaywriter".to_owned());
    /// ```
    pub fn new(user: String) -> Self {
        Self { concerns: user }
    }

    /// Retreieves the username of the chatter who will be unmuted by this command.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Unmute;
    ///
    /// let unmute = Unmute::new("essaywriter".to_owned());
    /// unmute.user(); // => "essaywriter"
    /// ```
    pub fn user(&self) -> &str {
        &self.concerns
    }
}

/// Ban is a command that bans a cringeposter.
pub struct Ban {
    /// The user that was banned
    concerns: String,

    /// Why the user was banned
    reasoning: String,

    /// The number of nanoseconds that the user will be banned for
    timeframe: u64,
}

impl Ban {
    /// Creates a new ban command.
    ///
    /// # Arguments
    ///
    /// * `user` - The username of the chatter who will be banned by this command
    /// * `reason` - Why the aforementioned chatter was banned
    /// * `duration` - The number of nanoseconds that the user will be banned
    /// for
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ban;
    ///
    /// let ban = Ban::new("RightToBearArmsLOL".to_owned(), "failing to falsify the Christian god".to_owned(), 1024);
    /// ```
    pub fn new(user: String, reason: String, duration: u64) -> Self {
        Self {
            concerns: user,
            reasoning: reason,
            timeframe: duration,
        }
    }

    /// Retreieves the username of the chatter who will be banned.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ban;
    ///
    /// let ban = Ban::new("RightToBearArmsLOL".to_owned(), "failing to falsify the Christian god".to_owned(), 1024);
    /// ban.user(); // => "RightToBearArmsLOL"
    /// ```
    pub fn user(&self) -> &str {
        &self.concerns
    }

    /// Retreives the reason this user was banned.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ban;
    ///
    /// let ban = Ban::new("RightToBearArmsLOL".to_owned(), "failing to falsify the Christian god".to_owned(), 1024);
    /// ban.reason(); // => "failing to falsify the Christian god"
    /// ```
    pub fn reason(&self) -> &str {
        &self.reasoning
    }

    /// Retreieves the number of nanoseconds the user will be banned for.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ban;
    ///
    /// let ban = Ban::new("RightToBearArmsLOL".to_owned(), "failing to falsify the Christian god".to_owned(), 1024);
    /// ban.timeframe(); // => 1024
    /// ```
    pub fn timeframe(&self) -> u64 {
        self.timeframe
    }
}

/// Unban is a command used to unban a chatter.
pub struct Unban {
    /// The user who will be banned by this command
    concerns: String,
}

impl Unban {
    /// Creates a new unban command.
    ///
    /// # Arguments
    ///
    /// * `user` - The username of the user unbanned by this command
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Unban;
    ///
    /// // Pepega Clap
    /// let unban = Unban::new("essaywriter".to_owned());
    /// ```
    pub fn new(user: String) -> Self {
        Self { concerns: user }
    }

    /// Retreives the username of the chatter unbanned as a result of this
    /// command's execution.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Unban;
    ///
    /// let unban = Unban::new("essaywriter".to_owned());
    /// unban.user(); // => "essaywriter"
    /// ```
    pub fn user(&self) -> &str {
        &self.concerns
    }
}

/// Subonly is a command used to set whether or not the chat is open only to
/// subscribers or not.
pub struct Subonly {
    /// Whether or not the chat should be in subonly mode
    on: bool,
}

impl Subonly {
    /// Creates a new Subonly command.
    ///
    /// # Arguments
    ///
    /// * `on` - Whether or not the chat should be in subonly mode
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Subonly;
    ///
    /// // Slumlord
    /// let sub_only = Subonly::new(true);
    /// ```
    pub fn new(on: bool) -> Self {
        Self { on }
    }

    /// Determines whether or not subonly mode will be active once this command
    /// is executed.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Subonly;
    ///
    /// let sub_only = Subonly::new(true);
    /// sub_only.active(); // => true
    /// ```
    pub fn active(&self) -> bool {
        self.on
    }
}

/// Ping is a command used to initiate a client-server ping-pong loop.
pub struct Ping {
    /// The time at which the ping request was initiated by the user
    initiation_timestamp: DateTime<Utc>,
}

impl Default for Ping {
    /// Generates a ping command at the current timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ping;
    /// use std::default::Default;
    ///
    /// let current_time_ping = Ping::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Ping {
    /// Creates a new ping command at the current time.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ping;
    ///
    /// let ping_request = Ping::new();
    /// ```
    pub fn new() -> Self {
        Self {
            initiation_timestamp: Utc::now(),
        }
    }

    /// Creates a new ping command at the provided time.
    ///
    /// # Arguments
    ///
    /// * `initiation_timestamp` - The timestamp that the server will assume the
    /// ping request was made at
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ping;
    /// use chrono::{Utc, Duration};
    ///
    /// // Spectrum internet REE
    /// let ping_request = Ping::new_with_initiation_timestamp(Utc::now() - Duration::seconds(420));
    /// ```
    pub fn new_with_initiation_timestamp(initiation_timestamp: DateTime<Utc>) -> Self {
        Self {
            initiation_timestamp
        }
    }

    /// Retreieves the time at which this ping request was initiated.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Ping;
    ///
    /// let ping_request = Ping::new();
    /// ping_request.started_at(); // => Utc::now()
    /// ```
    pub fn started_at(&self) -> DateTime<Utc> {
        self.initiation_timestamp
    }
}

/// Pong is an event representing a response to a ping request from the server.
pub struct Pong {
    /// The time at which the server responded to the user request for a ping
   response_timestamp: DateTime<Utc>
}

impl Default for Pong {
    /// Generates a new pong response, assuming the server responded at the
    /// current UTC time.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Pong;
    /// use std::default::Default;
    ///
    /// let ping_response = Pong::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Pong {
    /// Creates a new pong response.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Pong;
    ///
    /// let ping_response = Pong::new();
    /// ```
    pub fn new() -> Self {
        Self {
            response_timestamp: Utc::now()
        }
    }

    /// Retreieves the time at which the server responded to the request for
    /// ping.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Pong;
    ///
    /// let ping_request = Pong::new();
    /// ping_request.responded_at(); // => Utc::now()
    /// ```
    pub fn responded_at(&self) -> DateTime<Utc> {
        self.response_timestamp
    }
}

/// Broadcast is an event representing an incoming message, intended for the
/// entire server.
pub struct Broadcast {
    /// The sender of the message
    sender: String,

    /// The message sent in the broadcast event
    message: Message,
}

impl Broadcast {
    /// Creates a new broadcast event with the given user and message.
    ///
    /// # Arguments
    ///
    /// * `sender` - The username of the sender of the message
    /// * `message` - The contents of the message to be broadcasted
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Broadcast;
    ///
    /// let broadcasted_msg = Broadcast::new("MrMouton".to_owned(), "I am a living meme holy shit. Hacked by a 7 year old.".to_owned());
    /// ```
    pub fn new(sender: String, message: String) -> Self {
        Self {
            sender,
            message: Message::new(message),
        }
    }

    /// Gets the username of the chatter that sent the message.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Broadcast;
    ///
    /// let broadcasted_msg = Broadcast::new("MrMouton".to_owned(), "I am a living meme holy shit. Hacked by a 7 year old.".to_owned());
    /// broadcasted_msg.sent_by(); // => "MrMouton"
    /// ```
    pub fn sent_by(&self) -> &str {
        &self.sender
    }

    /// Gets the contents of the broadcasted message.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Broadcast;
    ///
    /// let broadcasted_msg = Broadcast::new("MrMouton".to_owned(), "I am a living meme holy shit. Hacked by a 7 year old.".to_owned());
    /// broadcasted_msg.msg(); // => "I am a living meme holy shit. Hacked by a 7 year old."
    /// ```
    pub fn msg(&self) -> &str {
        self.message.msg()
    }
}

/// EventTarget is a permissioning utility for events emitted by the server or a
/// client. Events will only be communicated to the specified target group.
pub enum EventTarget {
    /// This event targets all active chatters
    All,

    /// This event targets a specific user
    User(String),

    /// This event is hidden, and will only be seen by the server
    Server
}

/// Error is an event representing a failure response from the server to a set
/// of clients.
pub struct Error {
    /// The users that this error will be communicated to
    concerns: EventTarget,

    /// The error that will be sent to each user
    error: String,
}

impl Error {
    /// Creates a new error with the given target and error message.
    ///
    /// # Arguments
    ///
    /// * `target` - The users the error will be sent to
    /// * `error` - The error message that will be sent to the aforementioned users
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{Error, EventTarget};
    ///
    /// let err = Error::new(EventTarget::All, "mister mouton got evicted Slumlord".to_owned());
    /// ```
    pub fn new(target: EventTarget, error: String) -> Self {
        Self {
            concerns: target,
            error
        }
    }

    /// Determines the users that will be affected by this error.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{Error, EventTarget};
    ///
    /// let err = Error::new(EventTarget::All, "mister mouton got evicted Slumlord".to_owned());
    /// err.targets(); // => EventTarget::All
    /// ```
    pub fn targets(&self) -> &EventTarget {
        &self.concerns
    }

    /// Retreieves the message corresponding to this error.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{Error, EventTarget};
    ///
    /// let err = Error::new(EventTarget::All, "mister mouton got evicted Slumlord".to_owned());
    /// err.err_message(); // => "mister mouton got evicted Slumlord"
    /// ```
    pub fn err_message(&self) -> &str {
        &self.error
    }
}
