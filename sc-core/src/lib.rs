use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

fn get_unix_tx() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
