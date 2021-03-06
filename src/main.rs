#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

use actix_web::{web, App, HttpServer};
use clap::App as ClapApp;

use crate::logic::controllers::location;

mod conf;
mod generators;
mod logic;

const LOCATION_RADIUS: i32 = 10;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let cmd = ClapApp::from_yaml(yaml).get_matches();

    match cmd.subcommand().0 {
        _ => {
            HttpServer::new(|| {
                App::new()
                    .route("/index", web::get().to(location::index))
                    .route("/location", web::get().to(location::index))
                    .route("/location/generate", web::get().to(location::generate))
            })
                .bind("127.0.0.1:8088")?
                .run()
                .await
        }
    }
}
