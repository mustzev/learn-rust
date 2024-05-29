use std::{thread::sleep, time::Duration};

pub async fn hello_world() -> &'static str {
    sleep(Duration::from_secs(10));
    "Hello, World!"
}
