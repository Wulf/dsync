diesel::table! {
    todos (data) {
        data -> Binary,
        data_nullable -> Nullable<Binary>,
    }
}
