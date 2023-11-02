diesel::table! {
    todo (id) {
        id -> Int4,
        text -> Text,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
