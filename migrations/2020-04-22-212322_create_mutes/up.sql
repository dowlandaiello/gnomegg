-- User who have been muted
CREATE TABLE mutes (
       -- The ID of the gnomegg user who has been banned
       user_id INT PRIMARY KEY,

       -- The number of nanoseconds that the mute is active for
       duration BIGINT NOT NULL,

       -- The time at which the mute was issued
       initiated_at TIMESTAMP
);
