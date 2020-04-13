use crate::logic::models::ground_tile::GroundTile;
use crate::logic::models::ground_vertex::GroundVertex;
use crate::logic::schema::ground_tiles_vertexes_with_deleted;

#[derive(Insertable)]
#[table_name = "ground_tiles_vertexes_with_deleted"]
pub struct GroundTileVertexInput {
    tile_id: i64,
    vertex_id: i64,
}

impl GroundTileVertexInput {
    pub fn build(tile: GroundTile, vertexes: Vec<GroundVertex>) -> Vec<GroundTileVertexInput> {
        vertexes
            .into_iter()
            .map(|v| GroundTileVertexInput {
                tile_id: tile.id,
                vertex_id: v.id,
            })
            .rev()
            .collect()
    }
}
