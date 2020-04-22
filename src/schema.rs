table! {
    discord_connected (id) {
        id -> Integer,
        discord_id -> Nullable<Binary>,
    }
}

table! {
    google_connected (id) {
        id -> Integer,
        google_id -> Nullable<Binary>,
    }
}

table! {
    mutes (id) {
        id -> Integer,
        duration -> Bigint,
        initiated_at -> Timestamp,
    }
}

table! {
    reddit_connected (id) {
        id -> Integer,
        reddit_id -> Nullable<Binary>,
    }
}

table! {
    roles (id) {
        id -> Integer,
        administrator -> Nullable<Bool>,
        moderator -> Nullable<Bool>,
        vip -> Nullable<Bool>,
        protected -> Nullable<Bool>,
        subscriber -> Nullable<Bool>,
        bot -> Nullable<Bool>,
    }
}

table! {
    twitch_connected (id) {
        id -> Integer,
        twitch_id -> Nullable<Binary>,
    }
}

table! {
    twitter_connected (id) {
        id -> Integer,
        twitter_id -> Nullable<Binary>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Nullable<Varchar>,
        verified -> Bool,
        nationality -> Nullable<Text>,
        acepts_gifts -> Nullable<Bool>,
        minecraft_name -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    discord_connected,
    google_connected,
    mutes,
    reddit_connected,
    roles,
    twitch_connected,
    twitter_connected,
    users,
);
