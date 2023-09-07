#[macro_use] extern crate rocket;
use std::time::{SystemTime, UNIX_EPOCH};

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
    Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, goodbye, get_time])
}