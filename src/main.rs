use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("std UNIX timestamp={} ms={}", std_get_timestamp(), std_get_timestamp_ms());

    println!("time crate UNIX timestamp={} ms={}", time::OffsetDateTime::now_utc().unix_timestamp(), time::OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000);

    println!("chrono crate UNIX timestamp={} ms={}", chrono::offset::Utc::now().timestamp(), chrono::offset::Utc::now().timestamp_millis());
}

fn std_get_timestamp() -> u64 {
    let start = SystemTime::now();
    let unix_timestamp = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs();
    unix_timestamp
}

fn std_get_timestamp_ms() -> u128 {
    let start = SystemTime::now();
    let unix_timestamp_ms = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_millis();
    unix_timestamp_ms
}
