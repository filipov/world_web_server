use crate::logic::schema::ground_vertexes_with_deleted;

#[derive(Insertable)]
#[table_name = "ground_vertexes_with_deleted"]
pub struct GroundVertexInput {
    abscissa: f64,
    ordinate: f64,
    applicate: f64,
    horizontal_angle: f64,
    vertical_angle: f64,
    type_: String,
    // deleted_at: Option<i64>
}

impl GroundVertexInput {
    pub fn new_from_vec(coordinates: Vec<f64>, types: String) -> GroundVertexInput {
        GroundVertexInput {
            abscissa: coordinates[0],
            ordinate: coordinates[1],
            applicate: coordinates[2],
            horizontal_angle: coordinates[3],
            vertical_angle: coordinates[4],
            type_: types,
            // deleted_at: None
        }
    }
}
