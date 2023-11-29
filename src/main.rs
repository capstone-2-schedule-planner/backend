#[macro_use]
extern crate rocket;

use std::borrow::Cow;
use std::fmt::Debug;
use rocket::serde::json::{Json, json};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::form::validate::Contains;
use rocket::http::ext::IntoCollection;
use rocket::http::private::SmallVec;
use serde::de::DeserializeOwned;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Number, Strand};
use surrealdb::Surreal;
use surrealdb::{
    sql::{self, Data},
    sql::{Thing, Value},
};
use surrealdb::dbs::{ Session, Response };
use surrealdb::err::Error;
use surrealdb::kvs::Datastore;

//Data Structs

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
struct Id {
    tb: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    //tb: String,
    //id: String,
    title: String,
    subject: String,
    catalog: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClassInfo {
    //id: String,
    title: String,
    min_units: u32,
    max_units: u32,
    subject: String,
    catalog: String,
    req_group: Option<u32>,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Semester {
    classes: Vec<u32>,
    semester: u32,
    term: String,
}

#[derive(Serialize, Deserialize)]
struct Schedule {
    classes: Vec<Semester>,
    valid: bool,
}

#[derive(Serialize, Deserialize)]
struct ReqGroup {
    id: u32,
    effective_date: String,
    name: String,
    classes: Vec<u32>,
    union: bool,
}

//Helper Functions

async fn get_all_courses<T>() -> Option<Vec<T>> where T: DeserializeOwned, T: Debug {
    let db = Surreal::new::<Ws>("35.222.87.196:8000").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();

    // let input_string = "Cap";
    // let sql = "SELECT title FROM course WHERE (title CONTAINS \"Cap\") OR (catalog CONTAINS \"Cap\")".to_string();
    // let mut result = db.query(sql).await.unwrap();

    let result = db.select("course").await;
    println!("{:?}", result);
    match result {
        Ok(courses) => Some(courses),
        Err(_) => None,
    }
}

//Real API paths

#[get("/get_search_results/<input_string>")]
async fn get_search_results(input_string: String) -> Result<Json<Vec<SearchResult>>, &'static str> {
    let db_result = get_all_courses::<SearchResult>().await;
    let courses: Vec<SearchResult>;
    match db_result {
        Some(classes) => courses = classes,
        None => return Ok(Json(vec!())),
    }
    println!("{:?}", courses);

    let mut result: Vec<SearchResult> = vec!();
    for class in courses {
        if class.title.contains(input_string.as_str()) {
            result.push(class);
        }
    }

    Ok(Json(result))
}

#[get("/get_class_information/<subject>/<catalog>")]
async fn get_class_information(subject: String, catalog: String) -> Result<Json<ClassInfo>, &'static str> {
    let db_result = get_all_courses::<ClassInfo>().await;
    let courses: Vec<ClassInfo>;
    match db_result {
        Some(classes) => courses = classes,
        None => return Err("Given class does not exist"),
    }
    println!("{:?}", courses);

    println!("subject: {}, catalog: {}", subject.replace("%20", " "), catalog);

    for class in courses {
        if class.subject == subject.replace("%20", " ") && class.catalog == catalog {
            return Ok(Json(class));
        }
    }
    Err("Given class does not exist")
}

#[post("/validate", format = "application/json", data = "<schedule>")]
async fn validate_schedule(schedule: Json<Schedule>) -> Result<Json<bool>, &'static str> {
    // TODO: Connect to database

    // TODO: For each class in the schedule, ensure that all prereqs were taken before it

    // TODO: Return the result

    todo!()
}

// Example API paths

#[put("/data/<class_name>")] 
fn data_put(class_name: String) -> Option<String> {
    Some("Updating entire record for: ".to_owned() + &*class_name)
}

#[patch("/data/<class_number>")]
fn data_patch(class_number: i32) -> Option<String> {
    Some("New class_number will be: ".to_owned() + &*(class_number + 2).to_string())
}

#[delete("/data/<class_name>")]
fn data_delete(class_name: String) -> Option<String> {
    Some("Deleting class: ".to_owned() + &*class_name)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/time/now")]
fn get_time() -> Option<String> {
    Some(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string(),
    )
}

//Launching the API and setting the routes being used
#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            get_time,
            data_delete,
            data_patch,
            data_put,
            validate_schedule,
            get_search_results,
            get_class_information
        ],
    )
}
