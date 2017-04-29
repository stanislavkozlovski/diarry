#![feature(plugin)]
// #![feature(custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate djangohashers;
extern crate crypto;
extern crate jwt;
extern crate time;

pub mod models;
pub mod schema;
pub mod cors;
pub mod helpers;
pub mod db_queries;

use std::env;
use std::io::Cursor;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use djangohashers::{check_password, make_password_with_algorithm, Algorithm};

use rocket::response::status;
use rocket_contrib::JSON;
use rocket::http::hyper::header::{Headers, AccessControlAllowOrigin, AccessControlAllowHeaders};
use rocket::http::{Status, ContentType};
use rocket::http::Method;
use rocket::Response;
use rocket::data::Data;
use cors::{CORS, PreflightCORS};
use self::models::{DiaryEntry, NewDiaryEntry, ErrorDetails, DiaryEntryMetaInfo, DiaryOwner, NewDiaryOwner, DiaryComment, WholeDiaryEntry, DeserializableDiaryComment};
use std::io::Read;


fn create_whole_diary_entry(conn: &PgConnection, diary_entry: &DiaryEntry) -> WholeDiaryEntry {
    /* Creates a WholeDiaryEntry from a DiaryEntry struct*/
    let whole_diary = WholeDiaryEntry {
        id: diary_entry.id,
        title: diary_entry.title.clone(),
        body: diary_entry.body.clone(),
        creation_date: diary_entry.creation_date.clone(),
        creation_time: diary_entry.creation_time.clone(),
        comments:  db_queries::fetch_all_comments_belonging_to_diary_entry(conn, diary_entry)
    };
    return whole_diary;
}

#[post("/api/entries/new", format = "application/json", data = "<new_entry>")]
fn new_diary_controller(new_entry: JSON<NewDiaryEntry>, owner: DiaryOwner) -> CORS<Result<JSON<DiaryEntryMetaInfo>, JSON<ErrorDetails>>> {
    if new_entry.body.len() <= 5 || new_entry.title.len() <= 10 {
        let json_err = JSON(ErrorDetails{ error_message: String::from("The length of the body and title must be greater than 5 and 10 characters!") });
        return CORS::any(Err(json_err))
            .status(Status::BadRequest);
    }
    let new_entry: DiaryEntry = db_queries::create_diary_entry(&db_queries::establish_connection(), new_entry.title.as_str(), new_entry.body.as_str());
    let json_entry_meta_info = JSON(DiaryEntryMetaInfo { title: new_entry.title.clone(), url: new_entry.get_react_url() });

    CORS::any(Ok(json_entry_meta_info))
        .status(Status::Created)
}

#[post("/api/entries/<entry_id>/comments/new", format = "application/json", data = "<deser_comment>")]
fn new_comment_controller(entry_id: i32, deser_comment: JSON<DeserializableDiaryComment>, owner: DiaryOwner) -> CORS<Result<JSON<DiaryComment>, JSON<ErrorDetails>>> {
    let mut comment_body: String = String::from(deser_comment.into_inner().body);

    if comment_body.len() <= 3 || comment_body.len() > 1000 {
        let json_err = JSON(ErrorDetails{ error_message: String::from("The length of the comment body must be greater than 3 characters and less than 1000!") });
        return CORS::any(Err(json_err))
            .status(Status::BadRequest);
    }
    // get the diary entry
    let conn: PgConnection = db_queries::establish_connection();
    let entry = db_queries::fetch_diary_entry(&conn, entry_id);
    if entry.is_none() {
        let json_err = JSON(ErrorDetails{ error_message: String::from(format!("DiaryEntry with ID {} does not exist!!", entry_id)) });
        return CORS::any(Err(json_err))
            .status(Status::BadRequest);
    }

    let new_comment: DiaryComment = db_queries::create_diary_comment(&conn, &comment_body.as_str(), entry_id);

    CORS::any(Ok(JSON(new_comment)))
        .status(Status::Created)
}
#[route(OPTIONS, "/api/entries/new")]
fn cors_preflight() -> PreflightCORS {
    CORS::preflight("*")
        .methods(&vec![Method::Options, Method::Post])
        .credentials(true)
        .headers(&vec!["Content-Type", "jwt-auth"])
}


#[get("/api/entries/<id>")]
fn diary_details_controller<'a>(id: i32, owner: DiaryOwner) -> Response<'static> {
    // TODO: 
    /*
    Forwarding#
In this example above, what if id isn’t a i32? 
In this case, the request is forwarded to the next matching route, if there is any. 
This continues until a route doesn’t forward the request or there are no remaining routes to try. 
When there are no remaining matching routes, a customizable 404 error is returned.

rank route param
#[get("/user/<id>", rank = 2)]


    */
    let conn: PgConnection = db_queries::establish_connection();
    let diary_entry: Option<DiaryEntry> = db_queries::fetch_diary_entry(&conn, id);

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

    let whole_diary_entry: WholeDiaryEntry = create_whole_diary_entry(&conn, &diary_entry.unwrap());
    let response: Response = Response::build()
     .status(Status::Ok)
     .header(ContentType::JSON)
     .header(AccessControlAllowOrigin::Any)
     .sized_body(Cursor::new(
         json!(whole_diary_entry).to_string()
         ))
     .finalize();

    return response
}

#[get("/api/entries/all")]
fn all_diary_entries_controller(owner: DiaryOwner) -> CORS<JSON<Vec<DiaryEntry>>> {
    /* Return all the Diary Entries, ordered by their date descending */
    let connection: PgConnection = db_queries::establish_connection();
    CORS::any(JSON(db_queries::fetch_all_diary_entries(&connection, true)))
}

#[get("/api/entries/last_five")]
fn last_five_diary_entries_controller(owner: DiaryOwner) -> CORS<JSON<Vec<DiaryEntryMetaInfo>>> {
    /* Returns meta information about the last five diary entries */
    let connection: PgConnection = db_queries::establish_connection();
    let last_five_entries: Vec<DiaryEntry> = db_queries::fetch_last_five_diary_entries(&connection);
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
    let conn = db_queries::establish_connection();

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
    let gen_jwt: String = helpers::generate_jwt_token(owner.email.clone());
    diesel::update(diary_owner.filter(email.eq(owner.email.clone()))).set(jwt_token.eq(gen_jwt.clone())).execute(&conn);

    return CORS::any(gen_jwt);
}
// TODO: Split preflight handlers into GET and POST
use std::path::PathBuf;
#[route(OPTIONS, "/<empty..>")]
fn cors_preflight_auth(empty: PathBuf) -> PreflightCORS {
    CORS::preflight("*")
        .methods(&vec![Method::Options, Method::Post])
        .credentials(true)
        .headers(&vec!["Content-Type", "jwt-auth"])
}


#[error(401)]
fn fallback_unauthenticated() -> CORS<JSON<ErrorDetails>> {
    let json_err = JSON(ErrorDetails{ error_message: String::from("Missing or Invalid JWT Token in the 'jwt-auth' header!") });
    return CORS::any(json_err).status(Status::Unauthorized);
}

fn main() {
    dotenv().ok();
    db_queries::seed_diary_owner();
    rocket::ignite()
        .mount("/", routes![new_diary_controller, diary_details_controller, all_diary_entries_controller, last_five_diary_entries_controller, cors_preflight, cors_preflight_auth, login_auth_controller, new_comment_controller])
        .catch(errors![fallback_unauthenticated])
        .launch();
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use db_queries::establish_connection;
    
    #[test]
    fn test_connects_to_db() {
        dotenv().ok();
        // should not panic
        establish_connection();
    }
    extern crate serde_json;

    use db_queries::fetch_diary_entry;
    use diary_details_controller;
    #[test]
    fn test_details_should_return_correct_entry_and_200() {
        let _author = DiaryOwner {id: 1, email: String::from("manage@abv.bg"), password: String::from("123"), jwt: Some(String::from("W"))};
        dotenv().ok();
        let mut response: Response = diary_details_controller(1, _author);

        let expected_entry = fetch_diary_entry(&establish_connection(), 1).unwrap();
        let received_entry: WholeDiaryEntry = serde_json::from_str(
            &response.body().unwrap().into_string().unwrap()
            ).unwrap();

        assert_eq!(received_entry.id, expected_entry.id);
        assert_eq!(response.status().code, 200);
    }
    use rocket::Response;
    use models::ErrorDetails;
    #[test]
    fn test_details_should_return_error_message_and_404() {
        let _author = DiaryOwner {id: 1, email: String::from("manage@abv.bg"), password: String::from("123"), jwt: Some(String::from("W"))};
        dotenv().ok();
        let mut response: Response = diary_details_controller(i32::max_value(), _author);
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

    use db_queries::fetch_all_diary_entries;
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
        let _author = DiaryOwner {id: 1, email: String::from("manage@abv.bg"), password: String::from("123"), jwt: Some(String::from("W"))};
        
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let expected_entries: JSON<Vec<DiaryEntry>> = JSON(diary_entries.order((creation_date.desc(), creation_time.desc())).load::<DiaryEntry>(&connection).unwrap());
        let response = all_diary_entries_controller(_author);
        let received_entries = response.get_responder().deref().deref();
        assert_eq!(received_entries, expected_entries.into_inner().deref());
    }

    use db_queries::fetch_last_five_diary_entries;
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
                if newer_entry.creation_date == older_entry.creation_date {
                    assert!(newer_entry.creation_time >= older_entry.creation_time);
                } else {
                    assert!(newer_entry.creation_date > older_entry.creation_date);
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
        let _author = DiaryOwner {id: 1, email: String::from("manage@abv.bg"), password: String::from("123"), jwt: Some(String::from("W"))};
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let expected_entries: Vec<DiaryEntry> = diary_entries.order((creation_date.desc(), creation_time.desc())).limit(5).load::<DiaryEntry>(&connection).unwrap();
        let response = last_five_diary_entries_controller(_author);
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

    use db_queries::fetch_user_with_jwt;
    #[test]
    fn test_fetch_user_with_empty_jwt_returns_none() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        assert!(fetch_user_with_jwt(&connection, String::from("")).is_none());
    }
    use schema::diary_owner::dsl::{diary_owner};
    use models::DiaryOwner;
    #[test]
    fn test_fetch_user_with_jwt_returns_user() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();

        let real_owner: DiaryOwner = diary_owner.find(1).first(&connection).ok().unwrap();
        let received_owner: Option<DiaryOwner> = fetch_user_with_jwt(&connection, real_owner.jwt.clone().unwrap());
        assert!(received_owner.is_some());
        assert_eq!(real_owner, received_owner.unwrap());
    }
    use models::DiaryComment;
    use models::WholeDiaryEntry;
    use create_whole_diary_entry;
    #[test]
    fn test_create_whole_diary_entry_creates_entry_fills_comments() {
        dotenv().ok();
        let connection: PgConnection = establish_connection();
        let sample_entry: DiaryEntry = diary_entries.find(1).first(&connection).unwrap();
        let expected_commments: Vec<DiaryComment> = DiaryComment::belonging_to(&sample_entry).load(&connection).unwrap();

        let whole_entry: WholeDiaryEntry = create_whole_diary_entry(&connection, &sample_entry);

        assert_eq!(whole_entry.id, sample_entry.id);
        assert_eq!(whole_entry.title, sample_entry.title);
        assert_eq!(whole_entry.body, sample_entry.body);
        assert_eq!(whole_entry.creation_date, sample_entry.creation_date);
        assert_eq!(whole_entry.creation_time, sample_entry.creation_time);
        // assert that the comments are equal
        assert_eq!(whole_entry.comments.len(), expected_commments.len());
        for comment_idx in 0..expected_commments.len() {
            assert_eq!(whole_entry.comments[comment_idx], expected_commments[comment_idx]);
        }
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

