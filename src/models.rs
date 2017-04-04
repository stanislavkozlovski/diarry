extern crate chrono;
extern crate diesel;
extern crate rocket_contrib;
use super::schema::diary_entries;
use self::chrono::{NaiveDate, NaiveTime};

 
#[derive(Debug)]
#[derive(Queryable)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub struct DiaryEntry {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub date: NaiveDate,
    pub time: NaiveTime
}

impl PartialEq for DiaryEntry {
    fn eq(&self, other: &DiaryEntry) -> bool {
        return self.id == other.id;
    }
}
#[derive(Deserialize)]
#[derive(Insertable)]
#[table_name="diary_entries"]
pub struct NewDiaryEntry {
    pub title: String,
    pub body: String,
}
