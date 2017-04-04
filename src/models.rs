extern crate chrono;
extern crate diesel;
use super::schema::diary_entries;
use self::chrono::{NaiveDate, NaiveTime};


#[derive(Queryable)]
pub struct DiaryEntry {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub date: NaiveDate,
    pub time: NaiveTime
}


#[derive(Insertable)]
#[table_name="diary_entries"]
pub struct NewDiaryEntry<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
