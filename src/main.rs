#[macro_use]
extern crate rocket;

use std::time::{ SystemTime, UNIX_EPOCH };
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ClassInfo {
    name: String,
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Schedule {
    classes: u8,
    valid: bool,
}

//Real API paths
#[get("/get_search_results/<input_string>")]
fn get_search_results(input_string: String) -> Option<String> {

    return Some("test".to_string())
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

#[post("/validate", format="json", data="<schedule>")]
fn validate_schedule(schedule: Json<Schedule>) -> Option<String> {
    Some("true".to_string())
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
    rocket::build().mount("/", routes![index, get_time, validate_schedule, data_delete, data_patch, data_put, get_search_results])
}
