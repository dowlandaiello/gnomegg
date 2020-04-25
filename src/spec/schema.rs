table! {
    discord_connected (user_id) {
        user_id -> Unsigned<Bigint>,
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
    }
}

table! {
    google_connected (user_id) {
        user_id -> Unsigned<Bigint>,
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
    }
}

table! {
    ids (username) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        user_id -> Unsigned<Bigint>,
    }
}

table! {
    mutes (user_id) {
        user_id -> Unsigned<Bigint>,
        duration -> Bigint,
        initiated_at -> Nullable<Timestamp>,
    }
}

table! {
    reddit_connected (user_id) {
        user_id -> Unsigned<Bigint>,
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
    }
}

table! {
    roles (user_id) {
        user_id -> Unsigned<Bigint>,
        administrator -> Nullable<Bool>,
        moderator -> Nullable<Bool>,
        vip -> Nullable<Bool>,
        protected -> Nullable<Bool>,
        subscriber -> Nullable<Bool>,
        bot -> Nullable<Bool>,
    }
}

table! {
    twitch_connected (user_id) {
        user_id -> Unsigned<Bigint>,
        id_hash -> Nullable<Binary>,
        id_value -> Nullable<Text>,
    }
}

table! {
    twitter_connected (user_id) {
        user_id -> Unsigned<Bigint>,
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
    ids,
    mutes,
    reddit_connected,
    roles,
    twitch_connected,
    twitter_connected,
    users,
);
