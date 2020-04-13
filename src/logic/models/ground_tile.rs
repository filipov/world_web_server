#[derive(Debug, Clone, Queryable)]
// #[table_name="ground_tiles_with_deleted"]
pub struct GroundTile {
    pub id: i64,
    pub type_: String,
}
