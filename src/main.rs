#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

//Data Structs

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
fn get_search_results(input_string: String) -> Json<Vec<ClassInfo>> {
    // TODO: Connect to database

    // TODO: Get list of all class names and numbers

    // TODO: Search for the classes whose name or number includes the input string

    // TODO: Return filtered list like { class_id: class_name }

    todo!()
}

#[get("/get_class_information/<class_id>")]
fn get_class_information(class_id: u32) -> Json<ClassInfo> {
    // TODO: Connect to database

    // TODO: Get all information relating to given class_id

    // TODO: Return information as JSON

    todo!()
}

#[post("/validate", format = "application/json", data = "<schedule>")]
fn validate_schedule(schedule: Json<Schedule>) -> Json<bool> {
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
