#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
mod db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(database_url);
    rocket::build().manage(pool).mount("/", routes![index])
    
}

