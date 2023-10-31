#[macro_use]
extern crate rocket;

use std::borrow::Cow;
use rocket::serde::json::{Json, json};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::http::ext::IntoCollection;
use rocket::http::private::SmallVec;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Number, Strand};
use surrealdb::Surreal;
use surrealdb::{
    sql::{self, Data},
    sql::{Thing, Value},
};
use surrealdb::dbs::{ Session, Response };
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
    id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    //id: Id,
    title: String,
    // subject: String,
    // catalog: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClassInfo {
    id: u32,
    name: String,
    min_units: u32,
    max_units: u32,
    subject: String,
    catalog_num: u32,
    req_group: u32,
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

//Real API paths

#[get("/get_search_results/<input_string>")]
async fn get_search_results(input_string: String) -> Result<Json<Vec<SearchResult>>, &'static str> {
    let db = Surreal::new::<Ws>("127.0.0.1:8001").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();

    let sql = format!("SELECT title FROM course WHERE (title CONTAINS \"{}\") OR (catalog CONTAINS \"{}\")", input_string, input_string);

    let mut response = db
        .query(sql)
        .await
        .unwrap();
    println!("{:?}", response);

    let classes: Option<SearchResult> = response.take(0).unwrap();
    println!("{:?}", classes);

    Ok(Json(vec!(classes.unwrap())))
    //Ok(Json(classes))
}

#[get("/get_class_information/<class_id>")]
async fn get_class_information(class_id: u32) -> Result<Json<ClassInfo>, &'static str> {
    let db = Surreal::new::<Ws>("127.0.0.1:8001").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let sql = "\
        SELECT * \
        FROM id\
    ";
    let mut response = db.query(sql).bind(("id", class_id)).await.unwrap();
    println!("{:?}", response);
    let class: Option<ClassInfo> = response.take(0).unwrap();
    println!("{:?}", class);

    Ok(Json(class.unwrap()))
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
