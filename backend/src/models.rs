use diesel::{prelude::*, sql_types::Timestamp};
use serde::Deserialize;

use crate::schema::{device, device_info};

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = device)]
pub struct Device {
    pub id: i32,
    pub hostname: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Deserialize)]
#[diesel(belongs_to(Device))]
#[diesel(table_name = device_info)]
pub struct DeviceInfo {
    pub id: Option<i32>,
    pub total_memory: i64,
    pub used_memory: i64,
    pub total_swap: i64,
    pub used_swap: i64,
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
    pub number_of_cpus: i32,
    pub timestamp: i64,
    pub device_id: Option<i32>,
}
