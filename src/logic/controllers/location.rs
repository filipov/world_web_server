use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::conf::establish_connection;
use crate::generators::landscape::blank;
use crate::logic::models::ground::Ground;
use crate::LOCATION_RADIUS;

#[derive(Deserialize, Serialize)]
pub struct Params {
    x: Option<String>,
    y: Option<String>,
    z: Option<String>,

    radius: Option<String>,
}

pub async fn index(params: web::Query<Params>) -> impl Responder {
    let x: f64 = get_coordinates(params.x.to_owned());
    let y: f64 = get_coordinates(params.y.to_owned());
    let z: f64 = get_coordinates(params.z.to_owned());

    let radius: f64 = match params.radius.to_owned() {
        Some(val) => match from_str::<f64>(&val) {
            Ok(val) => val,
            Err(e) => panic!("{:?}", e),
        },
        None => LOCATION_RADIUS as f64,
    };

    Ground::by_coord(x, y, z, radius, &establish_connection())
}

pub async fn generate(params: web::Query<Params>) -> impl Responder {
    let x: f64 = get_coordinates(params.x.to_owned());
    let y: f64 = get_coordinates(params.y.to_owned());
    let z: f64 = get_coordinates(params.z.to_owned());

    blank::generate(x, y, z, &establish_connection());

    Ground::by_coord(x, y, z, 10.0, &establish_connection())
}

fn get_coordinates(c: Option<String>) -> f64 {
    match c {
        Some(val) => match from_str::<f64>(&val) {
            Ok(val) => val,
            Err(e) => panic!("{:?}", e),
        },
        None => 0.0,
    }
}
