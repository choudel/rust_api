use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::books;
use crate::schema::books::dsl::books as all_books;



#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Book{
    pub id: i32,
    pub title:String,
    pub author:String,
    pub published:bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="books"]
pub struct NewBook{
    pub title:String,
    pub author:String,
    pub published: bool,
}
impl Book {
    pub fn show(id:i32,conn:&SqliteConnection)->Vec<Book>{
        all_books
        .find(id)
        .load::<Book>(conn)
        .expect("Error loading book")
    }
    pub fn all(conn:&SqliteConnection)->Vec<Book>{
        all_books
        .order(books::id.desc())
        .load::<Book>(conn)
        .expect("Error loading the books")
    }
    pub fn update_by_id(id:i32,conn:&SqliteConnection, book:NewBook)->bool{
        use crate::schema::books::dsl::{title as t, author as a, published as p};
        let NewBook {
            title,
            author,
            published,
        } =book;
        diesel::update(all_books.find(id)).set((a.eq(author),p.eq(published),t.eq(title)))
        .execute(conn)
            .is_ok()
    }
    pub fn insert(book:NewBook,conn:&SqliteConnection)->bool{
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }
    pub fn delete_by_id(id:i32,conn:&SqliteConnection)->bool{
        if Book::show(id,conn).is_empty(){
            return false;
        };
        diesel::delete(all_books.find(id)).execute(conn).is_ok()
    }
    pub fn all_by_author(author:String,conn:&SqliteConnection)->Vec<Book>{
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error loading Books")
    }
}