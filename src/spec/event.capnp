@0xb2565d68d97959dd

# A message sent as text, rendered on the client
struct Message {
	contents @0 :Text;
}

# A message sent as text, rendered on the client corresponding to the user that
# the message is targeting
struct PrivMessage {

}

# A message issuing a command to mute a particular chatter
struct Mute {

}

# A message issuing a command to unmute a particular chatter
struct Unmute {

}

# A message issuing a command to ban a particular chatter
struct Ban {

}

# A message issuing a command to unban a particular chatter
struct Unban {

}

# A message issuing a command to make the chat sub-only
struct Subonly {

}

# A message issuing a command to ping the server
struct Ping {

}

# An event representing a response to the ping command from the server
struct Pong {

}

# An event representing an incomming message
struct Broadcast {

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
	}
}
