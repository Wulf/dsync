diesel::table! {
    users (name, address) {
        name -> Text,
        address -> Text,
        secret -> Text,
    }
}
