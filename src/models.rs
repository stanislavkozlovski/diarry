extern crate chrono;
extern crate diesel;
extern crate rocket_contrib;
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

#[derive(Deserialize)]
#[derive(Insertable)]
#[table_name="diary_entries"]
pub struct NewDiaryEntry {
    pub title: String,
    pub body: String,
}
