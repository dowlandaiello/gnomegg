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
