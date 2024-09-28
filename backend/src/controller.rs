use crate::models::{Device, DeviceInfo};
use crate::schema::device::hostname;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_device(device_hostname: &str) -> Result<Device, diesel::result::Error> {
    use crate::schema::device::dsl::*;
    let connection = &mut establish_connection();

    device
        .filter(hostname.eq(device_hostname))
        .select((id, hostname))
        .first(connection)
}

pub fn create_device(device_hostname: &str) -> Device {
    use crate::schema::device;
    let connection = &mut establish_connection();

    diesel::insert_into(device::table)
        .values(hostname.eq(device_hostname.to_string()))
        .returning(Device::as_returning())
        .get_result(connection)
        .expect("Error saving new device info")
}

pub fn create_device_info(device_info: DeviceInfo) -> DeviceInfo {
    use crate::schema::device_info;
    let connection = &mut establish_connection();

    diesel::insert_into(device_info::table)
        .values(&device_info)
        .returning(DeviceInfo::as_returning())
        .get_result(connection)
        .expect("Error saving new device info")
}

pub fn get_devices() -> Result<Vec<Device>, diesel::result::Error> {
    use crate::schema::device::dsl::*;
    let connection = &mut establish_connection();

    device.load(connection)
}

pub fn get_device_info(target_device_id: i32) -> Result<Vec<DeviceInfo>, diesel::result::Error> {
    use crate::schema::device_info::dsl::*;
    let connection = &mut establish_connection();

    device_info
        .filter(device_id.eq(target_device_id))
        .load(connection)
}
