table! {
    albums (id) {
        id -> Text,
        user_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

table! {
    photos (id) {
        id -> Text,
        album_id -> Text,
        user_id -> Text,
        index_in_album -> Integer,
        src -> Text,
        main_color -> Text,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

table! {
    users (id) {
        id -> Text,
        email -> Text,
        picture -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(photos -> albums (album_id));
joinable!(photos -> users (user_id));

allow_tables_to_appear_in_same_query!(
    albums,
    photos,
    users,
);
