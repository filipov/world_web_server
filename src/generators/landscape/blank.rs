use crate::logic::models::ground::Ground;
use crate::LOCATION_RADIUS;
use diesel::PgConnection;
use rand::{thread_rng, Rng};

pub fn generate(x: f64, y: f64, z: f64, conn: &PgConnection) {
    let rad = LOCATION_RADIUS;

    let tiles = ground_tiles(x, y, z, rad * 2, 2, false);

    Ground::generate_by_tiles(tiles, conn);
}

fn ground_tiles(
    x: f64,
    y: f64,
    z: f64,
    diametr: i32,
    diff: i32,
    reverse: bool,
) -> Vec<Vec<Vec<f64>>> {
    let radius: i64 = ((diametr as f64 / 2.0) as i64).abs();

    let mut tiles: Vec<Vec<Vec<f64>>> = vec![];

    let mut rng = thread_rng();

    for ix in (radius * -1)..radius {
        for iz in (radius * -1)..radius {
            let iy = rng.gen_range(diff as f64 / -2.0, diff as f64 / 2.0);

            {
                let mut triangle = vec![
                    vec![x + ix as f64, y + iy, z + iz as f64],
                    vec![x + ix as f64 + 1.0, y + iy, z + iz as f64],
                    vec![x + ix as f64 + 1.0, y + iy, z + iz as f64 + 1.0],
                ];

                if reverse {
                    triangle.reverse()
                }

                tiles.push(triangle)
            }

            {
                let mut triangle = vec![
                    vec![x + ix as f64, y + iy, z + iz as f64],
                    vec![x + ix as f64, y + iy, z + iz as f64 + 1.0],
                    vec![x + ix as f64 + 1.0, y + iy, z + iz as f64 + 1.0],
                ];

                if !reverse {
                    triangle.reverse()
                }

                tiles.push(triangle)
            }
        }
    }

    return tiles;
}
