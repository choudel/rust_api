#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{SqliteConnection, Connection};
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn =SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let book =models::NewBook{
        title: String::from("Gravey lord"),
        author: String::from("Thomas blond"),
        published: true,
    };
    if models::Book::insert(book,&conn){
        println!("success");
    } else {
        println!("failed");
    }


}
