use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{device, device_info};

#[derive(Queryable, Identifiable, Selectable, Debug, Insertable, Serialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = device)]
pub struct Device {
    pub id: i32,
    pub hostname: String,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Deserialize, Insertable, Serialize,
)]
#[diesel(belongs_to(Device))]
#[diesel(primary_key(id))]
#[diesel(table_name = device_info)]
pub struct DeviceInfo {
    #[diesel(deserialize_as = i32)]
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
    #[diesel(deserialize_as = i32)]
    pub device_id: Option<i32>,
}

impl DeviceInfo {
    pub fn set_device_id(&self, device: &Device) -> DeviceInfo {
        DeviceInfo {
            id: self.id,
            total_memory: self.total_memory,
            used_memory: self.used_memory,
            total_swap: self.total_swap,
            used_swap: self.used_swap,
            system_name: self.system_name.clone(),
            kernel_version: self.kernel_version.clone(),
            os_version: self.os_version.clone(),
            hostname: self.hostname.clone(),
            number_of_cpus: self.number_of_cpus,
            timestamp: self.timestamp,
            device_id: Some(device.id),
        }
    }
}
