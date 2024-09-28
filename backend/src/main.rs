use models::DeviceInfo;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;
mod models;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/device-info", format = "json", data = "<device_info>")]
fn device_info(device_info: Json<DeviceInfo>) -> &'static str {
    println!("Received device info: {:?}", device_info);

    "you posted it"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, device_info])
}
