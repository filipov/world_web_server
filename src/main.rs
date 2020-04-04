extern crate serde;

use rand::{thread_rng, Rng};
use actix_web::{App, HttpServer, web, Responder, HttpResponse, Error, HttpRequest};
use futures::future::{ready, Ready};
use serde::{Serialize};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

#[derive(Serialize)]
struct Data {
    items: Vec<Vec<Vec<f64>>>
}

// Responder
impl Responder for Data {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .header("Access-Control-Allow-Origin", "http://localhost:8082")
            .body(body)))
    }
}

async fn index() -> impl Responder {
    let reverse = false;

    let mut tiles = ground_tiles(0.0,-4.0,0.0,10, 2, reverse);

    tiles.append(&mut ground_tiles(0.0,    0.0,     -10.0,     10, 20, reverse));
    tiles.append(&mut ground_tiles(10.0,   0.0,     -10.0,     10, 2, reverse));
    tiles.append(&mut ground_tiles(-10.0,  0.0,     -10.0,     10, 15, reverse));
    tiles.append(&mut ground_tiles(10.0,   0.0,     0.0,       10, 2, reverse));
    tiles.append(&mut ground_tiles(-10.0,  0.0,     0.0,       10, 20, reverse));
    tiles.append(&mut ground_tiles(0.0,    0.0,     10.0,      10, 4, reverse));
    tiles.append(&mut ground_tiles(10.0,   0.0,     10.0,      10, 2, reverse));
    tiles.append(&mut ground_tiles(-10.0,  0.0,     10.0,      10, 10, reverse));
    tiles.append(&mut ground_tiles(20.0,   1.0,     10.0,      10, 4, reverse));
    tiles.append(&mut ground_tiles(20.0,   1.0,     0.0,       10, 2, reverse));
    tiles.append(&mut ground_tiles(20.0,   1.0,     -10.0,     10, 20, reverse));
    tiles.append(&mut ground_tiles(-20.0,  20.0,    10.0,      10, 40, reverse));
    tiles.append(&mut ground_tiles(-20.0,  10.0,    0.0,       10, 20, reverse));
    tiles.append(&mut ground_tiles(-20.0,  10.0,    -10.0,     10, 20, reverse));

    let tiles: Vec<Vec<Vec<f64>>> = colibrate_tiles(
        colibrate_tiles(
            colibrate_tiles(
                tiles
            )
        )
    );

    Data { items: tiles }
}

fn ground_tiles(x: f64, y: f64, z: f64, diametr: i32, diff: i32, reverse: bool) -> Vec<Vec<Vec<f64>>> {
    let radius: i64 = ((diametr as f64 / 2.0) as i64).abs();

    let mut tiles: Vec<Vec<Vec<f64>>> = vec!();

    let mut rng = thread_rng();

    for ix in (radius * -1)..radius {
        for iz in (radius * -1)..radius {
            let iy = rng.gen_range(diff as f64 / -2.0, diff as f64 / 2.0);

            {
                let mut triangle =vec![
                    vec![x + ix as f64,            y + iy,     z + iz as f64],
                    vec![x + ix as f64 + 1.0,      y + iy,     z + iz as f64],
                    vec![x + ix as f64 + 1.0,      y + iy,     z + iz as f64 + 1.0]
                ];

                if reverse {
                    triangle.reverse()
                }

                tiles.push(triangle)
            }

            {
                let mut triangle = vec![
                    vec![x + ix as f64,            y + iy,     z + iz as f64],
                    vec![x + ix as f64,            y + iy,     z + iz as f64 + 1.0],
                    vec![x + ix as f64 + 1.0,      y + iy,     z + iz as f64 + 1.0]
                ];

                if !reverse {
                    triangle.reverse()
                }

                tiles.push(triangle)
            }
        }
    }

    return tiles
}

    fn colibrate_tiles(some_tiles: Vec<Vec<Vec<f64>>>) -> Vec<Vec<Vec<f64>>> {
        let mut tiles = some_tiles;

        for irow in 0..tiles.len() {
            for itile in 0..tiles[irow].len() {
                for krow in 0..tiles.len() {
                    for ktile in 0..tiles[krow].len() {
                        if tiles[irow][itile][0] == tiles[krow][ktile][0] &&
                            tiles[irow][itile][1] != tiles[krow][ktile][1] &&
                            tiles[irow][itile][2] == tiles[krow][ktile][2]
                        {
                            let y = (tiles[irow][itile][1] + tiles[krow][ktile][1]) / 2.0;

                            tiles[irow][itile][1] = y.clone();
                            tiles[krow][ktile][1] = y.clone();
                        }
                    }
                }
            }
        }

        return tiles
    }
