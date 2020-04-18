/// Message is a message sent as text, rendered on the client.
pub struct Message {
    /// The contents of the message
    data: String,
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
        Self { data: contents }
    }

    /// Returns the contents of the message.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::event::Message;
    ///
    /// let msg = Message::new("Alright, you guys want to hear my most nuclear take?".to_owned());
    /// msg.contents(); // => "Alright, you guys want to hear my most nuclear take?"
    /// ````
    pub fn contents(&self) -> &str {
        &self.data
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
        self.message.contents()
    }
}
