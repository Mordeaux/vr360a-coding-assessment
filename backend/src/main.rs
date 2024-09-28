#[macro_use]
extern crate rocket;
mod models;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/device-info")]
fn device_info() -> &'static str {
    "Device info"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
