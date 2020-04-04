table! {
    characters_with_deleted (id) {
        id -> Int4,
        name -> Varchar,
        abscissa -> Float8,
        ordinate -> Float8,
        applicate -> Float8,
        horizontal_angle -> Int2,
        vertical_angle -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    ground_tiles_with_deleted (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    ground_vertexes_with_deleted (id) {
        id -> Int8,
        abscissa -> Float8,
        ordinate -> Float8,
        applicate -> Float8,
        horizontal_angle -> Float8,
        vertical_angle -> Float8,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    characters_with_deleted,
    ground_tiles_with_deleted,
    ground_vertexes_with_deleted,
);
