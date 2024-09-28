use diesel::prelude::*;
use models::{Device, DeviceInfo};
use rocket::serde::json::Json;
use schema::device::hostname;
use std::env;

#[macro_use]
extern crate rocket;
mod models;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn get_device(device_hostname: &str) -> Result<Device, diesel::result::Error> {
    use schema::device::dsl::*;
    let connection = &mut establish_connection();
    println!("device_hostname: {}", device_hostname);

    device
        .filter(hostname.eq(device_hostname))
        .select((id, hostname))
        .first(connection)
}

fn create_device(device_hostname: &str) -> Device {
    use schema::device;
    let connection = &mut establish_connection();

    diesel::insert_into(device::table)
        .values(hostname.eq(device_hostname.to_string()))
        .returning(Device::as_returning())
        .get_result(connection)
        .expect("Error saving new device info")
}

fn create_device_info(device_info: DeviceInfo) -> DeviceInfo {
    use schema::device_info;
    let connection = &mut establish_connection();

    diesel::insert_into(device_info::table)
        .values(&device_info)
        .returning(DeviceInfo::as_returning())
        .get_result(connection)
        .expect("Error saving new device info")
}

#[post("/device-info", format = "json", data = "<device_info>")]
fn device_info(device_info: Json<DeviceInfo>) -> Json<DeviceInfo> {
    println!("Received device info: {:?}", device_info);
    let device = get_device(&device_info.hostname).unwrap_or(create_device(&device_info.hostname));
    let device_info = device_info.set_device_id(&device);
    let device_info = create_device_info(device_info);

    println!("Device: {:?}", device);
    println!("Device info: {:?}", device_info);
    Json(device_info)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, device_info])
}
