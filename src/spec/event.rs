use chrono::{naive::NaiveDateTime, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Message is a message sent as text, rendered on the client.
#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
    /// The contents of the message
    contents: &'a str,
}

impl<'a> Message<'a> {
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
    /// let msg = Message::new("Mitta mitt mooowooo mitty mitta mitt mwoomooo");
    /// ```
    pub fn new(contents: &'a str) -> Self {
        Self { contents }
    }

    /// Returns the contents of the message.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Message;
    ///
    /// let msg = Message::new("Alright, you guys want to hear my most nuclear take?");
    /// msg.msg(); // => "Alright, you guys want to hear my most nuclear take?"
    /// ````
    pub fn msg(&self) -> &str {
        &self.contents
    }
}

/// PrivMessage is a message sent as text, rendered on the client corresponding
/// to the user that the message is targeting
#[derive(Serialize, Deserialize)]
pub struct PrivMessage<'a> {
    /// The username of the chatter that the message will be sent to
    concerns: &'a str,

    /// The contents of the private message
    message: Message<'a>,
}

impl<'a> PrivMessage<'a> {
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
    /// let msg = PrivMessage::new("essaywriter", "I have information concerning the murder of Jeffrey Epstein.");
    /// ```
    pub fn new(to: &'a str, contents: &'a str) -> Self {
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
    /// let msg = PrivMessage::new("essaywriter", "I have information concerning the murder of Jeffrey Epstein.");
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
    /// let msg = PrivMessage::new("essaywriter", "I have information concerning the murder of Jeffrey Epstein.");
    /// msg.contents(); // => "I have information concerning the murder of Jeffrey Epstein."
    /// ```
    pub fn contents(&self) -> &str {
        self.message.msg()
    }
}

/// Mute is a command issued to mute a particular user.
#[derive(Serialize, Deserialize)]
pub struct Mute<'a> {
    /// The user that will be muted by this command
    concerns: &'a str,

    /// The number of nanoseconds until the user will be unmuted
    duration: u64,
}

impl<'a> Mute<'a> {
    /// Creates a new mute command.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Mute;
    ///
    /// // Mute essaywriter for 666 nanoseconds for posting pepe cringe
    /// let mute = Mute::new("essaywriter", 666);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `user` - The username of the user who will be muted by this command
    /// * `duration` - The number of nanoseconds until the user will be unmuted
    pub fn new(user: &'a str, duration: u64) -> Self {
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
    /// let mute = Mute::new("essaywriter", 666);
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
    /// let mute = Mute::new("essaywriter", 666);
    /// mute.timeframe(); // => 666
    pub fn timeframe(&self) -> u64 {
        self.duration
    }
}

/// Unmute is a command used to unmute a particular chatter.
#[derive(Serialize, Deserialize)]
pub struct Unmute<'a> {
    /// The username of the user who will be unmuted by this command
    concerns: &'a str,
}

impl<'a> Unmute<'a> {
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
    /// let unmute = Unmute::new("essaywriter");
    /// ```
    pub fn new(user: &'a str) -> Self {
        Self { concerns: user }
    }

    /// Retreieves the username of the chatter who will be unmuted by this command.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Unmute;
    ///
    /// let unmute = Unmute::new("essaywriter");
    /// unmute.user(); // => "essaywriter"
    /// ```
    pub fn user(&self) -> &str {
        &self.concerns
    }
}

/// Ban is a command that bans a cringeposter.
#[derive(Serialize, Deserialize)]
pub struct Ban<'a> {
    /// The user that was banned
    concerns: &'a str,

    /// Why the user was banned
    reasoning: &'a str,

    /// The number of nanoseconds that the user will be banned for
    timeframe: u64,
}

impl<'a> Ban<'a> {
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
    /// let ban = Ban::new("RightToBearArmsLOL", "failing to falsify the Christian god", 1024);
    /// ```
    pub fn new(user: &'a str, reason: &'a str, duration: u64) -> Self {
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
    /// let ban = Ban::new("RightToBearArmsLOL", "failing to falsify the Christian god", 1024);
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
    /// let ban = Ban::new("RightToBearArmsLOL", "failing to falsify the Christian god", 1024);
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
    /// let ban = Ban::new("RightToBearArmsLOL", "failing to falsify the Christian god", 1024);
    /// ban.timeframe(); // => 1024
    /// ```
    pub fn timeframe(&self) -> u64 {
        self.timeframe
    }
}

/// Unban is a command used to unban a chatter.
#[derive(Serialize, Deserialize)]
pub struct Unban<'a> {
    /// The user who will be banned by this command
    concerns: &'a str,
}

impl<'a> Unban<'a> {
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
    /// let unban = Unban::new("essaywriter");
    /// ```
    pub fn new(user: &'a str) -> Self {
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
    /// let unban = Unban::new("essaywriter");
    /// unban.user(); // => "essaywriter"
    /// ```
    pub fn user(&self) -> &str {
        &self.concerns
    }
}

/// Subonly is a command used to set whether or not the chat is open only to
/// subscribers or not.
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Ping {
    /// The time at which the ping request was initiated by the user
    initiation_timestamp: NaiveDateTime,
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
            initiation_timestamp: Utc::now().naive_utc(),
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
            initiation_timestamp: initiation_timestamp.naive_utc(),
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
    pub fn started_at(&self) -> NaiveDateTime {
        self.initiation_timestamp
    }
}

/// Pong is an event representing a response to a ping request from the server.
#[derive(Serialize, Deserialize)]
pub struct Pong {
    /// The time at which the server responded to the user request for a ping
    response_timestamp: DateTime<Utc>,
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
            response_timestamp: Utc::now(),
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
#[derive(Serialize, Deserialize)]
pub struct Broadcast<'a> {
    /// The sender of the message
    sender: &'a str,

    /// The message sent in the broadcast event
    message: Message<'a>,
}

impl<'a> Broadcast<'a> {
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
    /// let broadcasted_msg = Broadcast::new("MrMouton", "I am a living meme holy shit. Hacked by a 7 year old.");
    /// ```
    pub fn new(sender: &'a str, message: &'a str) -> Self {
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
    /// let broadcasted_msg = Broadcast::new("MrMouton", "I am a living meme holy shit. Hacked by a 7 year old.");
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
    /// let broadcasted_msg = Broadcast::new("MrMouton", "I am a living meme holy shit. Hacked by a 7 year old.");
    /// broadcasted_msg.msg(); // => "I am a living meme holy shit. Hacked by a 7 year old."
    /// ```
    pub fn msg(&self) -> &str {
        self.message.msg()
    }
}

/// Error is an event representing a failure response from the server to a set
/// of clients.
#[derive(Serialize, Deserialize, Debug)]
pub struct Error<'a> {
    /// The users that this error will be communicated to
    concerns: EventTarget<'a>,

    /// The error that will be sent to each user
    error: &'a str,
}

impl<'a> Error<'a> {
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
    /// let err = Error::new(EventTarget::All, "mister mouton got evicted Slumlord");
    /// ```
    pub fn new(target: EventTarget<'a>, error: &'a str) -> Self {
        Self {
            concerns: target,
            error,
        }
    }

    /// Determines the users that will be affected by this error.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{Error, EventTarget};
    ///
    /// let err = Error::new(EventTarget::All, "mister mouton got evicted Slumlord");
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
    /// let err = Error::new(EventTarget::All, "mister mouton got evicted Slumlord");
    /// err.err_message(); // => "mister mouton got evicted Slumlord"
    /// ```
    pub fn err_message(&self) -> &str {
        &self.error
    }
}

/// CommandKind represents any one of the possible commands.
#[derive(Serialize, Deserialize)]
pub enum CommandKind<'a> {
    /// This command sends a message
    #[serde(borrow)]
    Message(Message<'a>),

    /// This command sends a message to one user
    PrivMessage(PrivMessage<'a>),

    /// This command mutes a user
    Mute(Mute<'a>),

    /// This command unmutes a user
    Unmute(Unmute<'a>),

    /// This command bans a user
    Ban(Ban<'a>),

    /// This command unbans a user
    Unban(Unban<'a>),

    /// This command makes the chat sub-only mode
    Subonly(Subonly),

    /// This command pings a user
    Ping(Ping),
}

/// Command represents any valid command, alongside the user issuing the
/// command.
#[derive(Serialize, Deserialize)]
pub struct Command<'a> {
    /// The issuer of the command
    issuer: &'a str,

    /// The type of command being issued
    #[serde(borrow)]
    kind: CommandKind<'a>,
}

impl<'a> Command<'a> {
    /// Creates a new command from the given issuer and individual commmand.
    ///
    /// # Arguments
    ///
    /// * `issuer` - The username of the chatter issuing the command
    /// * `cmd` - The underlying command, expressed as a CommandKind
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{CommandKind, Command, Message};
    ///
    /// let msg = Message::new("Hi nathanPepe dadd");
    /// let cmd_type = CommandKind::Message(msg);
    /// let cmd = Command::new("MrMouton", cmd_type);
    /// ```
    pub fn new(issuer: &'a str, cmd: CommandKind<'a>) -> Self {
        Self { issuer, kind: cmd }
    }

    /// Retreives the underlying command from the command.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{CommandKind, Command, Message};
    ///
    /// let msg = Message::new("Hi nathanPepe dadd");
    /// let cmd_type = CommandKind::Message(msg);
    /// let cmd = Command::new("MrMouton", cmd_type);
    ///
    /// cmd.command_type(); // => CommandKind::Message
    /// ```
    pub fn command_type(&self) -> &CommandKind {
        &self.kind
    }

    /// Retreieves the username associated with the issuer of the command.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{CommandKind, Command, Message};
    ///
    /// let msg = Message::new("Hi nathanPepe dadd");
    /// let cmd_type = CommandKind::Message(msg);
    /// let cmd = Command::new("MrMouton", cmd_type);
    ///
    /// cmd.command_type(); // => CommandKind::Message
    /// ```
    pub fn sent_by(&self) -> &str {
        &self.issuer
    }
}

/// EventTarget is a permissioning utility for events emitted by the server or a
/// client. Events will only be communicated to the specified target group.
#[derive(Serialize, Deserialize, Debug)]
pub enum EventTarget<'a> {
    /// This event targets all active chatters
    All,

    /// This event targets a specific user
    User(&'a str),

    /// This event is hidden, and will only be seen by the server
    Server,
}

/// EventKind represents any valid type of event.
#[derive(Serialize, Deserialize)]
pub enum EventKind<'a> {
    /// This event represents a new command being issued
    #[serde(borrow)]
    IssueCommand(Command<'a>),

    /// This event represents a response to a ping request from the server
    Pong,

    /// This event represents a new message being broadcasted
    Broadcast,

    /// This event represents a response to a client request with an error
    Error,
}

/// Event represents any action on gnomegg that might require a change in state.
#[derive(Serialize, Deserialize)]
pub struct Event<'a> {
    /// Users affected by this event
    concerns: EventTarget<'a>,

    /// The kind of event being emitted
    #[serde(borrow)]
    kind: EventKind<'a>,
}

impl<'a> Event<'a> {
    /// Creates a new event with the given target and underlying event.
    ///
    /// # Arguments
    ///
    /// * `target` - The users that will be affected by this event
    /// * `underlying_event` - The command, pong, broadcast, or error
    /// associated with this event
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{CommandKind, Command, Message, Event, EventTarget, EventKind};
    ///
    /// let msg = Message::new("Hi nathanPepe dadd");
    /// let cmd_type = CommandKind::Message(msg);
    /// let cmd = Command::new("MrMouton", cmd_type);
    /// let event = Event::new(EventTarget::User("Destiny"), EventKind::IssueCommand(cmd));
    /// ```
    pub fn new(target: EventTarget<'a>, underlying_event: EventKind<'a>) -> Self {
        Self {
            concerns: target,
            kind: underlying_event,
        }
    }

    /// Determines which set of users will be affected by this event.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{CommandKind, Command, Message, Event, EventTarget, EventKind};
    ///
    /// let msg = Message::new("Hi nathanPepe dadd");
    /// let cmd_type = CommandKind::Message(msg);
    /// let cmd = Command::new("MrMouton", cmd_type);
    /// let event = Event::new(EventTarget::User("Destiny"), EventKind::IssueCommand(cmd));
    /// event.targets(); // => EventTarget::User("Destiny")
    /// ```
    pub fn targets(&self) -> &EventTarget {
        &self.concerns
    }

    /// Determines what kind of event this is.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::{CommandKind, Command, Message, Event, EventTarget, EventKind};
    ///
    /// let msg = Message::new("Hi nathanPepe dadd");
    /// let cmd_type = CommandKind::Message(msg);
    /// let cmd = Command::new("MrMouton", cmd_type);
    /// let event = Event::new(EventTarget::User("Destiny"), EventKind::IssueCommand(cmd));
    /// event.targets(); // => EventTarget::User("Destiny")
    /// ```
    pub fn event_kind(&self) -> &EventKind {
        &self.kind
    }
}
