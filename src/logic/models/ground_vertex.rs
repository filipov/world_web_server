#[derive(Debug, Clone, Queryable)]
// #[table_name="ground_vertexes_with_deleted"]
pub struct GroundVertex {
    pub id: i64,
    pub abscissa: f64,
    pub ordinate: f64,
    pub applicate: f64,
    pub horizontal_angle: f64,
    pub vertical_angle: f64,
    pub type_: String,
}
