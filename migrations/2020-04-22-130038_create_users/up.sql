-- Users who have signed up for gnome.gg
CREATE TABLE users (
	-- By default, a unique identifier is assigned to each user. This is the
	-- best way to search for users, as it is O(1).
	id SERIAL PRIMARY KEY,

        -- A user must have a screenname in order to use the gnomegg chat. You
        -- can use this to search for data in the database, but ID is usually
	-- the recommended mode of data filtering.
	username VARCHAR(20) UNIQUE KEY,

	-- Whether or not this user has a verified phone number or email attached
	-- to their account.
	verified BOOLEAN NOT NULL,

	-- A country this user most identifies with.
	nationality TEXT,

	-- Whether or not this user accepts gifts.
	accepts_gifts BOOLEAN,

	-- This user's minecraft username
	minecraft_name VARCHAR(16) UNIQUE KEY
);

-- IDs pertaining to each username in the database
CREATE TABLE ids (
	-- The ID of the mapping entry
	id SERIAL,

	-- The username of the gnomegg user
	username VARCHAR(20) PRIMARY KEY,

	-- The ID of the user
	user_id INT
);

-- Users who have used reddit to connect to their accounts.
CREATE TABLE reddit_connected (
	-- The ID assigned by gnomegg to the user
	user_id INT PRIMARY KEY,

	-- The ID assigned by reddit to the user, encoded as a BLAKE3 hash in
	-- base58
	id_hash BINARY(32) UNIQUE KEY,

	-- The raw ID assigned by reddit to the user
	id_value TEXT
);

-- Users who have used twitch to connect their accounts.
CREATE TABLE twitch_connected (
	-- The ID assigned by gnomegg to the user
	user_id INT PRIMARY KEY,

	-- The ID assigned by twitch to the user, encoded as a 32-byte BLAKE3
	-- hash
	id_hash BINARY(32) UNIQUE KEY,

	-- The raw ID assigned by twitch to the user
	id_value TEXT
);

-- Users who have used twitter to connect their accounts.
CREATE TABLE twitter_connected (
	-- The ID assigned by gnomegg to the user
	user_id INT PRIMARY KEY,

	-- The ID assigned by twitter to the user, encoded as a 32-byte BLAKE3
	-- hash
	id_hash BINARY(32) UNIQUE KEY,

	-- The raw ID assigned by twitter to the user
	id_value TEXT
);

-- Users who have used a google account to connect their accounts.
CREATE TABLE google_connected (
	-- The ID assigned by gnomegg to the user
	user_id INT PRIMARY KEY,

	-- The ID assigned by google to the user, encoded as a 32-byte BLAKE3
	-- hash
	id_hash BINARY(32) UNIQUE KEY,

	-- The raw ID assigned by google to the user
	id_value TEXT
);

-- Users who have used a discord account to connect their accounts.
CREATE TABLE discord_connected (
	-- The ID assigned by gnomegg to the user
	user_id INT PRIMARY KEY,

	-- The ID assigned by discord to the user, encoded as a 32-byte BLAKE3
	-- hash
	id_hash BINARY(32) UNIQUE KEY,

	-- The raw ID assigned by discord to the user
	id_value TEXT
);

-- Permissions for each user registered for gnome.gg
CREATE TABLE roles (
	-- The ID for the user whose roles should be noted
	user_id INT PRIMARY KEY,

	-- Whether or not this user is an administrator
	administrator BOOLEAN,

	-- Whether or not this user is a moderator
	moderator BOOLEAN,

	-- Whether or not this user is a VIP
	vip BOOLEAN,

	-- Whether or not this user is protected
	protected BOOLEAN,

	-- Whether or not this user is a subscriber
	subscriber BOOLEAN,

	-- Whether or not this user is a bot
	bot BOOLEAN
);
