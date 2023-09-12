diesel::table! {
    todos (text) {
        text -> Text,
        text_nullable -> Nullable<Text>,
        #[max_length = 255]
        varchar -> Varchar,
        varchar_nullable -> Nullable<Varchar>,
    }
}
