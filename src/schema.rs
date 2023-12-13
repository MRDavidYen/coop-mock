// @generated automatically by Diesel CLI.

diesel::table! {
    endpoints_setting (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        path -> Text,
        options -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        method -> Varchar,
        enabled -> Bool,
    }
}