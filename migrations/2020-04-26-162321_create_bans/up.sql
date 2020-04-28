CREATE TABLE bans (
       -- The ID of the gnomegg user who has been banned
       user_id BIGINT UNSIGNED NOT NULL UNIQUE PRIMARY KEY,

       -- (Optional) The number of nanoseconds that the ban is active for
       duration BIGINT UNSIGNED,

       -- The time at which the ban was issued
       initiated_at TIMESTAMP NOT NULL,

       -- (Optional) the IP of the user being banned
       ip TEXT 
);
