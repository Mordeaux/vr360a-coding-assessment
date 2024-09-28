use std::env;

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use sysinfo::System;

pub fn get_update_interval() -> u64 {
    env::var("DATA_COLLECTOR_UPDATE_INTERVAL")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(10)
}

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    system_name: String,
    kernel_version: String,
    os_version: String,
    pub hostname: String,
    number_of_cpus: u32,
    timestamp: u64,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        let mut sys_info = System::new_all();
        sys_info.refresh_all();
        Self {
            total_memory: sys_info.total_memory(),
            used_memory: sys_info.used_memory(),
            total_swap: sys_info.total_swap(),
            used_swap: sys_info.used_swap(),
            system_name: System::name().unwrap(),
            kernel_version: System::kernel_version().unwrap(),
            os_version: System::os_version().unwrap(),
            hostname: System::host_name().unwrap(),
            number_of_cpus: sys_info.cpus().len() as u32,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

pub fn get_computer_info() -> DeviceInfo {
    DeviceInfo::default()
}
