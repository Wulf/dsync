diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Text,
        completed -> Bool,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        smallint -> Int2,
        bigint -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamp,
    }
}
