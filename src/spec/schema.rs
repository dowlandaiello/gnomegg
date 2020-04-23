table! {
    discord_connected (id) {
        id -> Integer,
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
    }
}

table! {
    google_connected (id) {
        id -> Integer,
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
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
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
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
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
    }
}

table! {
    twitter_connected (id) {
        id -> Integer,
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Nullable<Varchar>,
        verified -> Bool,
        nationality -> Nullable<Text>,
        accepts_gifts -> Nullable<Bool>,
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
