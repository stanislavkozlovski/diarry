extern crate chrono;
use self::chrono::{DateTime, UTC};
extern crate diesel;
use self::diesel::types::Date;
use super::schema::diary_entries;

#[derive(Queryable)]
pub struct DiaryEntry {
    pub id: i32,
    pub title: String,
    pub body: String,
    // pub date: Date
}


#[derive(Insertable)]
#[table_name="diary_entries"]
pub struct NewDiaryEntry<'a> {
    pub title: &'a str,
    pub body: &'a str,
}