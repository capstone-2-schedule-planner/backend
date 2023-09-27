#[macro_use]
extern crate rocket;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::data::{FromData, Outcome};
use rocket::{Data, Request};
use rocket::response::content::RawJson;
use rocket_contrib::json::Json;
use serde_derive::Deserialize;

struct ClassInfo {
    name: String,
    id: u32,
    description: String,
}

#[derive(Deserialize)]
struct Schedule {
    classes: Vec<i32>,
}
// #[rocket::async_trait]
// impl FromData for Schedule {
//     type Error = ();
//
//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
//         todo!()
//     }
// }

// Database CRUD paths
#[post("/data", data = "<input>")]
fn data_post(input: String) -> Option<String> {
    Some("Data posted: ".to_owned() + &*input)
}


#[get("/data/<class_name>")]
fn data_get(class_name: String) -> Option<String> {
    Some("Requested data for: ".to_owned() + &*class_name)
}

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

#[post("/validate", data="<schedule>")]
fn validate_schedule(schedule: Json<Schedule>) -> Option<String> {
    Some("true".to_string())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/goodbye")]
fn goodbye() -> &'static str {
    "Goodbye World!"
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, goodbye, get_time, validate_schedule, data_delete, data_get, data_patch, data_post, data_put])
}
