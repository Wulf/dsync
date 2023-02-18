diesel::table! {
    todos (id) {
        id -> Int4,
        created_at -> Timestamp,
    }
}
