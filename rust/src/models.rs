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

impl DiaryEntry {
    pub fn get_absolute_url(&self) -> String {
        return format!("/api/entries/{}", &self.id)
    }
    pub fn get_react_url(&self) -> String {
        // return the URL of the entry following the front-end routing
        return format!("/entry/{}", &self.id)
    }
}
#[derive(Deserialize)]
#[derive(Insertable)]
#[table_name="diary_entries"]
pub struct NewDiaryEntry {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct DiaryEntryMetaInfo {
    /* Meta information about a DiaryEntry */
    pub title: String,
    pub url: String
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ErrorDetails {
    pub error_message: String
}