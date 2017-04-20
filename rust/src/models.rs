extern crate chrono;
extern crate diesel;
extern crate rocket_contrib;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use super::schema::{ diary_owner, diary_entries };
use self::chrono::{NaiveDate, NaiveTime};
use time;
use jwt::Token;
use jwt::error::Error as JwtErrorEnum;
use jwt::header::Header as JwtHeader;
use jwt::claims::Claims as JwtClaims;
use jwt::{Component, Error as JwtError};
use diesel::associations::Identifiable;
use db_queries::{ establish_connection, fetch_user_with_jwt };
use schema::diary_comments;

#[derive(Debug)]
#[derive(Queryable)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Identifiable)]
#[table_name="diary_entries"]
#[has_many(diary_comments)]
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


#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(DiaryEntry)]
#[derive(Serialize)]
#[table_name="diary_comments"]
pub struct DiaryComment {
    id: i32,
    pub entry_id: i32,
    body: String,
    pub date: NaiveDate,
    pub time: NaiveTime
}

#[derive(Insertable)]
#[table_name="diary_comments"]
pub struct NewDiaryComment {
    pub entry_id: i32,
    pub body: String    
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

        let given_jwt: &str = keys[0];
        
        // check if the JWT is expired
        if given_jwt.len() > 1 {
            let token: Result<Token<JwtHeader, JwtClaims>, JwtErrorEnum> = Token::parse(given_jwt);
            if token.is_err() {
                println!("{:?}", token.err());
                println!("Token unpacking errored!");
                println!("{:?}", given_jwt);
                return Outcome::Failure((Status::Unauthorized, ()));
            }
            let expiryEpoch: Option<u64> = token.unwrap().claims.reg.exp;
            if expiryEpoch.is_none() {
                println!("Epoch errored!");
                return Outcome::Failure((Status::Unauthorized, ()));
            }
            let currentTime: u64 = time::get_time().sec as u64;
            if currentTime > expiryEpoch.unwrap() {
                // the Token has expired!
                println!("Token has expired!");
                
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        }
        // try to fetch a user with that JWT Token
        let potential_owner: Option<DiaryOwner> = fetch_user_with_jwt(&establish_connection(), String::from(given_jwt));
        if potential_owner.is_none() {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        return Outcome::Success(potential_owner.unwrap());
    }
}