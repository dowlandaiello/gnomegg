@0xb2565d68d97959dd

# A message sent as text, rendered on the client
struct Message {
	contents @0 :Text;
}

# A message sent as text, rendered on the client corresponding to the user that
# the message is targeting
struct PrivMessage {
  # The user that should be privately messaged
  concerns @0 :Text;

  # The message that should be sent to the aforementioned user
  message @1 :Message;
}

# A message issuing a command to mute a particular chatter
struct Mute {
  # The user that should be muted
  concerns @0 :Text;

  # The number of nanoseconds that the user should be muted for, from now
  duration @1 :UInt64;
}

# A message issuing a command to unmute a particular chatter
struct Unmute {
  # The user that should be unmuted
  concerns @0 :Text;
}

# A message issuing a command to ban a particular chatter
struct Ban {
  # The user that should be unbanned
  concerns @0 :Text;

  # The reason why the user is being banned
  reason @1 :Text;

  # The number of nanoseconds the user should be banned for, from now
  duration @2 :UInt64;
}

# A message issuing a command to unban a particular chatter
struct Unban {
  # The user that should be unbanned
  concerns @0 :Text; 
}

# A message issuing a command to toggle the chat's sub-only mode
struct Subonly {
  # Whether or not subonly mode should be on
  on @0 :Bool; 
}

# A message issuing a command to ping the server
struct Ping {
  # The time at which the ping request began
  initiationTimestamp @0 :Data;  
}

# An event representing a response to the ping command from the server
struct Pong {
  # The time at which the server accepted the ping request
  timestamp @0 :Data;
}

# An event representing an incomming message
struct Broadcast {
  # The chatter sending this message
  sender @0 :Text; 

  # The message being sent
  message @1 :Message;
}

# A notification sent by the server to the chatter issuing a command, usually
# with an error
struct Error {
  # The user that should be notified of the error response
  concerns @0 :Text;

  # The message sent in the error
  error @1 :Text;
}

# A parsed message
struct Command {
	# The chatter issuing this command
	issuer @0 :Text;

	# The type of command being issued
	type :union {
		# This is a raw text message, possibly containing specially formatted
		# text data handled by the client
		message @1 :Message;	

		# This is a raw text message sent to only to a specific client
		privMessage @2 :PrivMessage;

		# This command is muting a chatter
		mute @2 :Mute;

		# This command is unmuting a chatter
		unmute @3 :Unmute;

		# This command is banning a chatter
		ban @4 :Ban;

		# This command is unbanning a chatter
		unban @5 :Unban;

		# This command is making the stream sub-only
		subonly @6 :Subonly;

		# This command is initiating a server-client ping-pong feedback loop
		ping @7 :Ping;
	}
}

# Any operation on gnomegg that might require computation, or change state
# (e.g., yeeposting, /ping, /ban, etc...)
struct Event {
  # Events are scoped, and broadcasted only to the respective users
	concerns :union {
		# This event targets all users in gnomegg (e.g., broadcast)
		all @0 :Void;

		# This event targets a specific user
		user @1 :Text;

    # This event should not be sent to any client, and is only for the server to handle
    server @2 :Void;
	}

  # The type of event
	type :union {
    # This event represents a command that has been issued
    issueCommand @3 :Command;

    # The server is responding to a ping request
    pong @4 :Pong;

    # The server is handling a new message by broadcasting it
    broadcast @5 :Broadcast;
	}
}
