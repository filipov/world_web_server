use diesel::prelude::*;
use diesel::PgConnection;
use serde::Serialize;

use crate::logic::models::ground_tile::GroundTile;
use crate::logic::models::ground_tile_input::GroundTileInput;
use crate::logic::models::ground_tiles_vertexes_input::GroundTileVertexInput;
use crate::logic::models::ground_vertex::GroundVertex;
use crate::logic::models::ground_vertex_input::GroundVertexInput;
use crate::logic::responders::grounds::Data;
use crate::LOCATION_RADIUS;

#[derive(Queryable, Serialize)]
pub struct Ground {
    pub id: i64,
    pub types: String,
    pub vertexes: serde_json::Value,
    pub angles: serde_json::Value,
    pub coordinates: serde_json::Value,
}

impl Ground {
    pub fn by_coord(x: f64, y: f64, z: f64, radius: f64, conn: &PgConnection) -> Data {
        use crate::logic::schema::grounds::dsl::*;
        use diesel::dsl::sql;

        let rad = radius;

        match grounds
            .filter(
                sql("(")
                    .sql(
                        &[
                            ground_query(0, x - rad, x + rad),
                            ground_query(1, y - rad, y + rad),
                            ground_query(2, z - rad, z + rad),
                        ]
                        .join(") AND ("),
                    )
                    .sql(")"),
            )
            .load::<Ground>(conn)
        {
            Ok(res) => Data { items: res },
            Err(e) => {
                println!("{:?}", e);
                Data { items: vec![] }
            }
        }
    }

    pub fn generate_by_tiles(tiles: Vec<Vec<Vec<f64>>>, conn: &PgConnection) -> Vec<i32> {
        use crate::logic::schema::ground_tiles_vertexes_with_deleted;
        use crate::logic::schema::ground_tiles_with_deleted;
        use crate::logic::schema::ground_vertexes_with_deleted;

        let mut tiles_ids: Vec<i32> = vec![];

        for tile in tiles {
            let points = vec![
                GroundVertexInput::new_from_vec(
                    vec![tile[0][0], tile[0][1], tile[0][2], 0.0, 0.0],
                    "ground".to_string(),
                ),
                GroundVertexInput::new_from_vec(
                    vec![tile[1][0], tile[1][1], tile[1][2], 0.0, 0.0],
                    "ground".to_string(),
                ),
                GroundVertexInput::new_from_vec(
                    vec![tile[2][0], tile[2][1], tile[2][2], 0.0, 0.0],
                    "ground".to_string(),
                ),
            ];

            let points: Vec<GroundVertex> =
                match diesel::insert_into(ground_vertexes_with_deleted::table)
                    .values(&points)
                    .get_results::<GroundVertex>(conn)
                {
                    Ok(res) => res,
                    Err(e) => panic!("{:?}", e),
                };

            let tile: GroundTileInput = GroundTileInput::new("stone".to_string());

            let tile: GroundTile = match diesel::insert_into(ground_tiles_with_deleted::table)
                .values(&tile)
                .get_result::<GroundTile>(conn)
            {
                Ok(res) => res,
                Err(e) => panic!("{:?}", e),
            };

            match diesel::insert_into(ground_tiles_vertexes_with_deleted::table)
                .values(&GroundTileVertexInput::build(tile.clone(), points.clone()))
                .execute(conn)
            {
                Ok(_) => (),
                Err(e) => panic!("{:?}", e),
            }

            println!("{:?}", points);
            println!("{:?}", tile);
            println!("#######################################");
        }

        tiles_ids
    }
}

fn ground_query(point_coordinate: i8, start: f64, end: f64) -> String {
    let q = |s: i8| -> String {
        format!(
            "(\
                (grounds.vertexes #>> '{{{}, {}}}')::float >= {} \
                AND\
                (grounds.vertexes #>> '{{{}, {}}}')::float <= {}
            )",
            s, point_coordinate, start, s, point_coordinate, end
        )
    };

    [q(0), q(1), q(2)].join(" OR ")
}
