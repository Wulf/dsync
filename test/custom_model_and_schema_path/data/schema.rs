diesel::table! {
    tableA (_id) {
        _id -> Integer,
    }
}

diesel::table! {
    tableB (_id) {
        _id -> Integer,
        link -> Integer,
    }
}

diesel::joinable!(tableB -> tableA (link));
