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
    book_me (id) {
        id -> Text,
        user_id -> Text,
        email -> Text,
    }
}

table! {
    custom_migrations (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
    }
}

table! {
    photos (id) {
        id -> Text,
        album_id -> Text,
        user_id -> Text,
        index_in_album -> Integer,
        s3_id -> Text,
        src -> Text,
        main_color -> Text,
        is_favorite -> Bool,
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

joinable!(book_me -> users (user_id));
joinable!(photos -> albums (album_id));
joinable!(photos -> users (user_id));

allow_tables_to_appear_in_same_query!(albums, book_me, custom_migrations, photos, users,);
