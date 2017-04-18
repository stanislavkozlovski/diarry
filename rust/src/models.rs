extern crate chrono;
extern crate diesel;
extern crate rocket_contrib;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use super::schema::{ diary_owner, diary_entries };
use self::chrono::{NaiveDate, NaiveTime};

// pub mod db_queries;
use db_queries::{ establish_connection, fetch_user_with_jwt };

 
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

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Insertable)]
#[table_name="diary_owner"]
pub struct NewDiaryOwner {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Queryable)]
pub struct DiaryOwner {
    id: i32,
    email: String,
    pub password: String,
    pub jwt: Option<String>
}

impl PartialEq for DiaryOwner {
    fn eq(&self, other: &DiaryOwner) -> bool {
        return self.id == other.id;
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for DiaryOwner {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DiaryOwner, ()> {
        let keys: Vec<_> = request.headers().get("jwt-auth").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let given_jwt = keys[0];
        // try to fetch a user with that JWT Token
        // TODO: JWT Expiration!
        let potential_owner: Option<DiaryOwner> = fetch_user_with_jwt(&establish_connection(), String::from(given_jwt));
        if potential_owner.is_none() {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        return Outcome::Success(potential_owner.unwrap());
    }
}