table! {
    grounds (id) {
        id -> Int8,
        types -> Varchar,
        vertexes -> Json,
        angles -> Json,
        vectors -> Json,
    }
}

allow_tables_to_appear_in_same_query!(
    grounds
);