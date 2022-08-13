use diesel::prelude::*;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use rocket::outcome::Outcome;
use std::ops::Deref;

pub type Pool=r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool(db_url:String)->Pool{
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}
pub struct Conn (pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Conn{
    type Error= ();
    async fn from_request(request: &'r Request<'_>)-> request::Outcome<Conn,()>{
        let pool =request.guard::<&State<Pool>>().await;
       let pool= Success(Conn(conn));
       let pool= Failure((Status::ServiceUnavailable, ()));
           
    }
}

impl Deref for  Conn {
    type Target =SqliteConnection;
    #[inline(always)]
    fn deref(&self)-> &Self::Target{
        &self.0
    }
}