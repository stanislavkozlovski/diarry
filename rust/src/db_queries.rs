// pub mod schema;
use diesel;
use schema;
use models::{DiaryEntry, NewDiaryEntry, NewDiaryOwner, DiaryOwner};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;   
use self::schema::diary_entries::dsl::{creation_date, creation_time};
use self::schema::diary_entries::dsl::diary_entries;
use schema::diary_owner::dsl::{diary_owner, jwt_token};

use djangohashers::{check_password, make_password_with_algorithm, Algorithm};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn fetch_all_diary_entries(conn: &PgConnection, order_by_date: bool) -> Vec<DiaryEntry> {
    /* Returns a Vector of all the Diary Entries
       param: order_by_date - if set, returns them ordered by their date and time descending
    */
    
    if order_by_date {
        return diary_entries.order((creation_date.desc(), creation_time.desc())).load::<DiaryEntry>(conn).unwrap()
    } else {
        return diary_entries.load::<DiaryEntry>(conn).unwrap();
    }
}

pub fn fetch_last_five_diary_entries(conn: &PgConnection) -> Vec<DiaryEntry> {
    /* Query the database for the last 5 DiaryEntries and return them sorted by their creation date descending (newest first) */
    return diary_entries.order((creation_date.desc(), creation_time.desc())).limit(5).load::<DiaryEntry>(conn).unwrap();
}

pub fn fetch_diary_entry(conn: &PgConnection, id: i32) -> Option<DiaryEntry> {
    /* Given the ID, queries the database for a DiaryEntry row and returns a DiaryEntry struct */
    let result = diary_entries.find(id).first(conn);
    match result {
        Ok(r) => Some(r),
        Err(r) => None
    }
}

pub fn create_diary_entry<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> DiaryEntry {
    /* Creates a new DiaryEntry in the database */
    use schema::diary_entries;

    let new_entry = NewDiaryEntry {
        title: String::from(title),
        body: String::from(body),
    };

    diesel::insert(&new_entry).into(diary_entries::table)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn fetch_user_with_jwt(conn: &PgConnection, given_jwt: String) -> Option<DiaryOwner> {
    if given_jwt.len() < 1 {
        return None
    }
    let result = diary_owner.filter(jwt_token.eq(given_jwt.clone())).first(conn);
    match result {
        Ok(r) => Some(r),
        Err(r) => None
    }
}

pub fn seed_diary_owner() {
    /* Seeds a DiaryOwner (one per app) using the environment variables for his credentials */
    use schema::diary_owner;
    
    let conn: PgConnection = establish_connection();
    let potential_owner: QueryResult<DiaryOwner> = diary_owner::dsl::diary_owner.find(1).first(&conn);
    if potential_owner.is_ok() {
        // Do not seed if an owner already exists
        return;
    }

    if let (Ok(email),Ok(password)) = (env::var("EMAIL"), env::var("PASSWORD")) {
        if email.len() == 0 || password.len() == 0 {
            panic!("EMAIL or PASSWORD environment variables are set but empty! Please configure them in your .env file.")
        }
        let hashed_pwd = make_password_with_algorithm(&password.to_string(), Algorithm::BCryptSHA256);
        // convert to temp structure for easy inserting into DB
        let seeded_diary_owner = NewDiaryOwner { email: String::from(email), password: String::from(hashed_pwd) };
        diesel::insert(&seeded_diary_owner).into(diary_owner::table)
                                        .execute(&conn)
                                        .expect("Error seeding owner");
    } else {
        panic!("EMAIL or PASSWORD environment variables are not set! Please configure them in your .env file.")
    }
}