#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate diesel_codegen;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{DiaryEntry, NewDiaryEntry};

pub fn create_diary_entry<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> DiaryEntry {
    use schema::diary_entries;

    let new_entry = NewDiaryEntry {
        title: title,
        body: body,
    };

    diesel::insert(&new_entry).into(diary_entries::table)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn establish_connection() -> PgConnection {
    use schema::diary_entries::dsl::*;
    
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    let post: DiaryEntry = create_diary_entry(&establish_connection(), "What up", "Nothing");
    println!("{:?}", post.date);
    println!("{:?}", post.time);
    
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}

#[cfg(test)]
mod tests {
    use establish_connection;
    #[test]
    fn test_connects_to_db() {
        // should not panic
        establish_connection();
    }
}
