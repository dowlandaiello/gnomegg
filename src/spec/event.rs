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
    concerns: String,}

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
        Self {
            concerns: user
        }
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
            timeframe: duration
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
        Self {
            concerns: user
        }
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
    on: bool
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
        Self {
            on
        }
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
