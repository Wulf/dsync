diesel::table! {
    /*
        CREATE TABLE user (
            id SERIAL PRIMARY KEY,
            phone_numbers TEXT[],
            optional_array_column TEXT[] NOT NULL
        );
     */

    user (id) {
        id -> Int4,
        // note: diesel will always generate a Array<Nullable<T>>
        phone_numbers -> Array<Nullable<Text>>,
        optional_array_column -> Nullable<Array<Nullable<Text>>>,
    }
}
