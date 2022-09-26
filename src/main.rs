use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!(
        "{:>12}: UNIX timestamp={} ms={}",
        "std",
        std_get_timestamp(),
        std_get_timestamp_ms()
    );

    println!(
        "{:>12}: UNIX timestamp={} ms={}",
        "time crate",
        time::OffsetDateTime::now_utc().unix_timestamp(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000
    );

    println!(
        "{:>12}: UNIX timestamp={} ms={}",
        "chrono crate",
        chrono::offset::Utc::now().timestamp(),
        chrono::offset::Utc::now().timestamp_millis()
    );
}

fn std_get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn std_get_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}
