#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

//Data Structs

#[derive(Serialize, Deserialize)]
struct SearchResult {
    id: u32,
    name: String,
    subject: String,
    catalog_num: u32,
}

#[derive(Serialize, Deserialize)]
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
    // TODO: Get correct IP
    let db = Surreal::new::<Ws>("127.0.0.1:4321").await.unwrap();

    let sql = "\
        SELECT id, name, subject, catalog_num \
        FROM classes \
        WHERE (class_name CONTAINS search_string) OR (catalog_num CONTAINS search_string)\
    ";
    let mut response = db
        .query(sql)
        .bind(("search_string", input_string))
        .await
        .unwrap();
    let classes: Vec<SearchResult> = response.take(1).unwrap();

    Ok(Json(classes))
}

#[get("/get_class_information/<class_id>")]
async fn get_class_information(class_id: u32) -> Result<Json<ClassInfo>, &'static str> {
    // TODO: Get correct IP
    let db = Surreal::new::<Ws>("127.0.0.1:4321").await.unwrap();

    let sql = "\
        SELECT * \
        FROM classes:id\
    ";
    let mut response = db.query(sql).bind(("id", class_id)).await.unwrap();
    let class: Option<ClassInfo> = response.take(0).unwrap();

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
