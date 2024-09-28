// @generated automatically by Diesel CLI.

diesel::table! {
    device (id) {
        id -> Int4,
        hostname -> Varchar,
    }
}

diesel::table! {
    device_info (id) {
        id -> Int4,
        total_memory -> Int8,
        used_memory -> Int8,
        total_swap -> Int8,
        used_swap -> Int8,
        system_name -> Varchar,
        kernel_version -> Varchar,
        os_version -> Varchar,
        hostname -> Varchar,
        number_of_cpus -> Int4,
        timestamp -> Int8,
        device_id -> Int4,
    }
}

diesel::joinable!(device_info -> device (device_id));

diesel::allow_tables_to_appear_in_same_query!(
    device,
    device_info,
);
