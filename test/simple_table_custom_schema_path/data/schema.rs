diesel::table! {
    todos (id) {
        id -> Int4,
        unsigned -> Unsigned<Integer>,
        text -> Text,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
