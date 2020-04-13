use crate::logic::schema::ground_tiles_with_deleted;

#[derive(Insertable)]
#[table_name = "ground_tiles_with_deleted"]
pub struct GroundTileInput {
    type_: String,
    // deleted_at: Option<i64>
}

impl GroundTileInput {
    pub fn new(types: String) -> GroundTileInput {
        GroundTileInput {
            type_: types,
            // deleted_at: None
        }
    }
}
