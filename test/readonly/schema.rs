diesel::table! {
    normal (id) {
        id -> Int4,
        testprop -> Int4,
    }
}

diesel::table! {
    prefixTable (id) {
        id -> Int4,
        testprop -> Int4,
    }
}

diesel::table! {
    tableSuffix (id) {
        id -> Int4,
        testprop -> Int4,
    }
}

diesel::table! {
    prefixTableSuffix (id) {
        id -> Int4,
        testprop -> Int4,
    }
}
