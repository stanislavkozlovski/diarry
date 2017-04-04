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


#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// #[cfg(test)] mod tests;
use rocket::response::status;
use rocket::http::Status;
use rocket::response::content;
use rocket_contrib::{JSON, Value};
use rocket::response::status::{Created};

pub fn create_diary_entry<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> DiaryEntry {
    use schema::diary_entries;

    let new_entry = NewDiaryEntry {
        title: String::from(title),
        body: String::from(body),
    };

    diesel::insert(&new_entry).into(diary_entries::table)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn fetch_diary_entry(conn: &PgConnection, id: i32) -> Option<DiaryEntry> {
    use self::schema::diary_entries::dsl::*;
    let result = diary_entries.find(id).first(conn);
    match result {
        Ok(r) => Some(r),
        Err(r) => None
    }
    
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

#[post("/api/entries/new", format = "application/json", data = "<new_entry>")]
fn new(new_entry: JSON<NewDiaryEntry>) -> Result<Created<String>, String>{
    if (new_entry.body.len() <= 3 || new_entry.title.len() <= 3) {
        return Err(String::from("A"));
    }
    create_diary_entry(&establish_connection(), new_entry.title.as_str(), new_entry.body.as_str());

    Ok(Created(String::from("Created"), Some(String::from("Created"))))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, new])
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
