// @generated automatically by Diesel CLI.

diesel::table! {
    notebooks (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    notes (id) {
        id -> Int4,
        content -> Text,
        notebook_id -> Int4,
    }
}

diesel::joinable!(notes -> notebooks (notebook_id));

diesel::allow_tables_to_appear_in_same_query!(
    notebooks,
    notes,
);
