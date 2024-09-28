use models::{Device, DeviceInfo};
use rocket::serde::json::Json;
mod controller;
mod models;
mod schema;

#[macro_use]
extern crate rocket;

#[get("/device", format = "json")]
fn get_devices() -> Json<Vec<Device>> {
    let devices = controller::get_devices().unwrap();
    Json(devices)
}

#[get("/device/<device_id>/device_info", format = "json")]
fn get_device_info(device_id: i32) -> Json<Vec<DeviceInfo>> {
    let device_info = controller::get_device_info(device_id).unwrap();
    Json(device_info)
}

#[post("/device-info", format = "json", data = "<device_info>")]
fn device_info(device_info: Json<DeviceInfo>) -> Json<DeviceInfo> {
    let device = controller::get_device(&device_info.hostname)
        .unwrap_or_else(|_| controller::create_device(&device_info.hostname));

    let device_info = device_info.set_device_id(&device);
    let device_info = controller::create_device_info(device_info);

    Json(device_info)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_devices, device_info, get_device_info])
}
