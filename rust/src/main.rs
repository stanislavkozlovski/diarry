#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod cors;
extern crate jwt;

use std::env;
use std::io::Cursor;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use rocket::response::status;
use rocket::response::status::{Created};
use rocket::response::content;
use rocket_contrib::JSON;
use rocket::http::hyper::header::{Headers, AccessControlAllowOrigin, AccessControlAllowHeaders};
use rocket::http::{Status, ContentType};
use rocket::http::Method;
use rocket::Response;
use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{
    Header,
    Registered,
    Token,
};
use cors::{CORS, PreflightCORS};
use self::models::{DiaryEntry, NewDiaryEntry, ErrorDetails, DiaryEntryMetaInfo, DiaryOwner, NewDiaryOwner};
extern crate djangohashers;
extern crate crypto;
// Or, just what you need:
use djangohashers::{check_password, make_password_with_algorithm, Algorithm};

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

pub fn fetch_diary_entry(conn: &PgConnection, id: i32) -> Option<DiaryEntry> {
    /* Given the ID, queries the database for a DiaryEntry row and returns a DiaryEntry struct */
    use self::schema::diary_entries::dsl::diary_entries;
    let result = diary_entries.find(id).first(conn);
    match result {
        Ok(r) => Some(r),
        Err(r) => None
    }
}

pub fn fetch_last_five_diary_entries(conn: &PgConnection) -> Vec<DiaryEntry> {
    /* Query the database for the last 5 DiaryEntries and return them sorted by their creation date descending (newest first) */
    use self::schema::diary_entries::dsl::diary_entries;
    use self::schema::diary_entries::dsl::{creation_date, creation_time};
    
    
    let result = diary_entries.order((creation_date.desc(), creation_time.desc())).limit(5).load::<DiaryEntry>(conn).unwrap();
    return result;
}

pub fn fetch_all_diary_entries(conn: &PgConnection, order_by_date: bool) -> Vec<DiaryEntry> {
    /* Returns a Vector of all the Diary Entries
       param: order_by_date - if set, returns them ordered by their date and time descending
    */
    use self::schema::diary_entries::dsl::diary_entries;
    
    if order_by_date {
        use self::schema::diary_entries::dsl::{creation_date, creation_time};
        return diary_entries.order((creation_date.desc(), creation_time.desc())).load::<DiaryEntry>(conn).unwrap()
    } else {
        return diary_entries.load::<DiaryEntry>(conn).unwrap();
    }
}

pub fn seed_diary_owner() {
    use schema::diary_owner;

    let conn: PgConnection = establish_connection();
    let potential_owner: QueryResult<DiaryOwner> = diary_owner::dsl::diary_owner.find(1).first(&conn);
    if potential_owner.is_ok() {
        // Do not seed if an owner already exists
        return;
    }

    if let (Ok(email),Ok(password)) = (env::var("EMAIL"), env::var("PASSWORD")) {
        if (email.len() == 0 || password.len() == 0) {
            panic!("EMAIL or PASSWORD environment variables are set but empty! Please configure them in your .env file.")
        }
        let hashed_pwd = make_password_with_algorithm(&password.to_string(), Algorithm::BCryptSHA256);
        // convert to temp structure for easy inserting into DB
        let seeded_diary_owner = NewDiaryOwner { email: String::from(email), password: String::from(hashed_pwd) };
        // diesel::insert(&seeded_diary_owner).into(diary_owner::table)
                                        // .execute(&conn)
                                        // .expect("Error seeding owner");
    } else {
        panic!("EMAIL or PASSWORD environment variables are not set! Please configure them in your .env file.")
    }
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[post("/api/entries/new", format = "application/json", data = "<new_entry>")]
fn new_diary_controller(new_entry: JSON<NewDiaryEntry>) -> CORS<Result<JSON<DiaryEntryMetaInfo>, JSON<ErrorDetails>>> {
    if new_entry.body.len() <= 5 || new_entry.title.len() <= 10 {
        let json_err = JSON(ErrorDetails{ error_message: String::from("The length of the body and title must be greater than 5 and 10 characters!") });
        return CORS::any(Err(json_err))
            .status(Status::BadRequest);
    }
    let new_entry: DiaryEntry = create_diary_entry(&establish_connection(), new_entry.title.as_str(), new_entry.body.as_str());
    let json_entry_meta_info = JSON(DiaryEntryMetaInfo { title: new_entry.title.clone(), url: new_entry.get_react_url() });

    CORS::any(Ok(json_entry_meta_info))
        .status(Status::Created)
}

#[route(OPTIONS, "/api/entries/new")]
fn cors_preflight() -> PreflightCORS {
    CORS::preflight("*")
        .methods(&vec![Method::Options, Method::Post])
        .credentials(true)
        .headers(&vec!["Content-Type"])
}


#[get("/api/entries/<id>")]
fn diary_details_controller<'a>(id: i32) -> Response<'static> {
    let diary_entry: Option<DiaryEntry> = fetch_diary_entry(&establish_connection(), id);

    if diary_entry.is_none() {
        let error_message = ErrorDetails{ error_message: String::from(format!("Could not find a Diary Entry with ID {}", id)) };
        return Response::build()
                .status(Status::NotFound)
                .header(ContentType::JSON)
                .header(AccessControlAllowOrigin::Any)
                .sized_body(Cursor::new(
                    json!(error_message).to_string()
                    ))
                .finalize();
    }

    let response: Response = Response::build()
     .status(Status::Ok)
     .header(ContentType::JSON)
     .header(AccessControlAllowOrigin::Any)
     .sized_body(Cursor::new(
         json!(diary_entry.unwrap()).to_string()
         ))
     .finalize();

    return response
}

#[get("/api/entries/all")]
fn all_diary_entries_controller() -> CORS<JSON<Vec<DiaryEntry>>> {
    /* Return all the Diary Entries, ordered by their date descending */
    let connection: PgConnection = establish_connection();
    CORS::any(JSON(fetch_all_diary_entries(&connection, true)))
}

#[get("/api/entries/last_five")]
fn last_five_diary_entries_controller() -> CORS<JSON<Vec<DiaryEntryMetaInfo>>> {
    /* Returns meta information about the last five diary entries */
    let connection: PgConnection = establish_connection();
    let last_five_entries: Vec<DiaryEntry> = fetch_last_five_diary_entries(&connection);
    let mut entries_meta: Vec<DiaryEntryMetaInfo> = Vec::new();
    // fill up the entries
    for entry in last_five_entries {
        entries_meta.push(DiaryEntryMetaInfo { title: entry.title.clone(), url: entry.get_react_url() });
    }

    return CORS::any(JSON(entries_meta));
}

#[post("/api/authenticate", format = "application/json", data = "<owner>")]
fn login_auth_controller(owner: JSON<NewDiaryOwner>) -> CORS<String>{
    use schema::diary_owner::dsl::{diary_owner, email, password, jwt_token};
    let conn = establish_connection();

    // fetch the diary_owner with the email address and authenticate their passwords match
    let result = diary_owner.filter(email.eq(owner.email.clone())).select(password).first(&conn);
    if result.is_err() {
        return CORS::any(String::from("Wrong credentials")).status(Status::BadRequest);
    }
    let real_password: String = result.unwrap();
    if !check_password(owner.password.as_str(), real_password.as_str()).ok().unwrap() {
        return CORS::any(String::from("Wrong credentials")).status(Status::BadRequest);
    }

    // successful login, generate jwt, save it to the DB and return it back 
    let gen_jwt: String = generate_jwt_token(owner.email.clone());
    diesel::update(diary_owner.filter(email.eq(owner.email.clone()))).set(jwt_token.eq(gen_jwt.clone())).execute(&conn);

    return CORS::any(gen_jwt);
}

fn generate_jwt_token(email: String) -> String {
    let header: Header = Default::default();
    // TODO: iss, exp, iat etc
    let claims = Registered {
        sub: Some(email),
        ..Default::default()
    };

    let token = Token::new(header, claims);
    // Sign the token
    // TODO: Secret key
    if let Err(e) = env::var("SECRET_KEY") {
        panic!("Your SECRET_KEY environment variable is not set! Please configure it in your .env file.")
    }
    let secret_key: String = env::var("SECRET_KEY").ok().unwrap();
    let gen_jwt = token.signed("secret".as_bytes(), Sha256::new()).unwrap();
    gen_jwt
}

#[route(OPTIONS, "/api/authenticate")]
fn cors_preflight_auth() -> PreflightCORS {
    CORS::preflight("*")
        .methods(&vec![Method::Options, Method::Post])
        .credentials(true)
        .headers(&vec!["Content-Type"])
}

fn main() {
    dotenv().ok();
    seed_diary_owner();
    rocket::ignite()
        .mount("/", routes![new_diary_controller, diary_details_controller, all_diary_entries_controller, last_five_diary_entries_controller, cors_preflight, cors_preflight_auth, login_auth_controller])
        .launch();
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use establish_connection;
    
    #[test]
    fn test_connects_to_db() {
        dotenv().ok();
        // should not panic
        establish_connection();
    }
    extern crate serde_json;

    use {fetch_diary_entry, diary_details_controller};
    #[test]
    fn test_details_should_return_correct_entry_and_200() {
        dotenv().ok();
        let mut response: Response = diary_details_controller(1);

        let expected_entry = fetch_diary_entry(&establish_connection(), 1).unwrap();
        let received_entry: DiaryEntry = serde_json::from_str(
            &response.body().unwrap().into_string().unwrap()
            ).unwrap();

        assert_eq!(received_entry, expected_entry);
        assert_eq!(response.status().code, 200);
    }
    use rocket::Response;
    use models::ErrorDetails;
    #[test]
    fn test_details_should_return_error_message_and_404() {
        dotenv().ok();
        let mut response: Response = diary_details_controller(i32::max_value());
        let error_details: ErrorDetails = serde_json::from_str(
                &response.body().unwrap().into_string().unwrap()
            ).unwrap();
        let error_message = error_details.error_message;
        let expected_error_msg = format!("Could not find a Diary Entry with ID {}", i32::max_value());

        assert_eq!(response.status().code, 404);
        assert_eq!(error_message, expected_error_msg);
    }

    use schema::diary_entries::dsl::diary_entries;
    use diesel::prelude::*;
    
    use diesel::pg::PgConnection;
    use models::{DiaryEntry, DiaryEntryMetaInfo};
    #[test]
    fn test_fetch_diary_entry_should_return_corresponding_entry() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let expected_entry: QueryResult<DiaryEntry> = diary_entries.find(1).first(&connection);
        assert_eq!(fetch_diary_entry(&connection, 1).unwrap(), expected_entry.unwrap());
    }
    #[test]
    fn test_fetch_diary_entry_non_existing_id_should_return_none() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        assert!(fetch_diary_entry(&connection, i32::max_value()).is_none());
    }

    use fetch_all_diary_entries;
    use schema::diary_entries::dsl::{creation_date, creation_time};
    #[test]
    fn test_fetch_all_diary_entries_should_return_them_all() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let expected_entries: Vec<DiaryEntry> = diary_entries.load::<DiaryEntry>(&connection).unwrap();

        assert_eq!(fetch_all_diary_entries(&connection, false), expected_entries);
    }

    #[test]
    fn test_fetch_all_diary_entries_ordered_should_return_them_ordered() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let expected_entries: Vec<DiaryEntry> = diary_entries.order((creation_date.desc(), creation_time.desc())).load::<DiaryEntry>(&connection).unwrap();
        let received_entries = fetch_all_diary_entries(&connection, true);
        assert_eq!(received_entries, expected_entries);
    }

    use rocket_contrib::JSON;
    use all_diary_entries_controller;
    use std::ops::Deref;
    #[test]
    fn test_all_diary_entries_controller_should_return_all_entries_in_json_ordered() {
        /* the controller should return all the entries ordered by date and time desc */
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let expected_entries: JSON<Vec<DiaryEntry>> = JSON(diary_entries.order((creation_date.desc(), creation_time.desc())).load::<DiaryEntry>(&connection).unwrap());
        let response = all_diary_entries_controller();
        let received_entries = response.get_responder().deref().deref();
        assert_eq!(received_entries, expected_entries.into_inner().deref());
    }

    use fetch_last_five_diary_entries;
    #[test]
    fn test_fetch_last_five_diary_entries_should_be_sorted_and_not_more_than_five() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let received_entries: Vec<DiaryEntry> = fetch_last_five_diary_entries(&connection);

        assert!(received_entries.len() <= 5);
        // assert that each one's date/time is bigger than the next
        for i in 0..received_entries.len() {
            let ref newer_entry: DiaryEntry = received_entries[i];
            for j in i+1..received_entries.len() {
                let ref older_entry: DiaryEntry = received_entries[j];
                if newer_entry.date == older_entry.date {
                    assert!(newer_entry.time >= older_entry.time);
                } else {
                    assert!(newer_entry.date > older_entry.date);
                }
            }
        }
    }

    #[test]
    fn test_fetch_last_five_diary_entries_should_return_expected() {
        use schema::diary_entries::dsl::{creation_date, creation_time};
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let received_entries: Vec<DiaryEntry> = fetch_last_five_diary_entries(&connection);
        let expected_entries: Vec<DiaryEntry> = diary_entries.order((creation_date.desc(), creation_time.desc())).limit(5).load::<DiaryEntry>(&connection).unwrap();

        assert_eq!(received_entries, expected_entries);
    }

    use last_five_diary_entries_controller;
    #[test]
    fn test_last_five_diary_entries_controller_returns_correct_entries() {
        use schema::diary_entries::dsl::{creation_date, creation_time};
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let expected_entries: Vec<DiaryEntry> = diary_entries.order((creation_date.desc(), creation_time.desc())).limit(5).load::<DiaryEntry>(&connection).unwrap();
        let response = last_five_diary_entries_controller();
        let received_entries: &Vec<DiaryEntryMetaInfo> = response.get_responder().deref();

        assert_eq!(expected_entries.len(), received_entries.len());
        // assert each entry one by one
        for i in 0..expected_entries.len() {
            let ref exp_entry: DiaryEntry = expected_entries[i];
            let ref rec_entry: DiaryEntryMetaInfo = received_entries[i];

            assert_eq!(rec_entry.title, exp_entry.title);
            assert_eq!(rec_entry.url, exp_entry.get_react_url());
        }
    }

    use std::env;
    #[test]
    fn test_assert_secret_key_env_variable_is_set() {
        dotenv().ok();
        assert!(!env::var("SECRET_KEY").ok().is_none());  // should not panic
    }

    // use seed_diary_owner;
    // #[test]
    // #[should_panic(expected = "EMAIL or PASSWORD environment variables are set but empty! Please configure them in your .env file.")]
    // fn test_seed_diary_owner_no_email_env_var_should_panic() {
    //     dotenv().ok();
        
    //     println!("{:?}", env::var("EMAIL").ok());
    //     // env::set_var("EMAIL", "");
    //     // env::set_var("PASSWORD", "");
        
    //     seed_diary_owner()
    // }
}

