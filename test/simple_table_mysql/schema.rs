diesel::table! {
    todos (id) {
        id -> Int4,
        unsigned -> Unsigned<Integer>,
        unsigned_nullable -> Nullable<Unsigned<Integer>>,
        text -> Text,
        completed -> Bool,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        smallint -> Int2,
        bigint -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
