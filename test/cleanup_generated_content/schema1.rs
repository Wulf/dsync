diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Text,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }

    books (id) {
        id -> Int4,
        title -> Text,
        year -> Int4,
        read -> Bool,
    }
}
