// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        user_name -> Text,
        text -> Text,
        creation_date -> Timestamp,
    }
}
