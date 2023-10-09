diesel::table! {
    normal (id) {
        id -> Int,
        testprop -> Int,
    }
}

diesel::table! {
    prefixTable (id) {
        id -> Int,
        testprop -> Int,
    }
}

diesel::table! {
    tableSuffix (id) {
        id -> Int,
        testprop -> Int,
    }
}

diesel::table! {
    prefixTableSuffix (id) {
        id -> Int,
        testprop -> Int,
    }
}
