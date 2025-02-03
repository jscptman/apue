use chrono::{Local, Timelike};
use std::{thread, time::Duration};

const SLEEP_TIME_SLICE: Duration = Duration::from_secs(5 * 60); // 5 minutes
fn main() {
    let mut loop_count = 1;
    loop {
        thread::sleep(SLEEP_TIME_SLICE);
        if loop_count % 5 == 0 {
            let dt = Local::now();
            println!("🚀 {}", dt.second());
        }
        loop_count += 1;
    }
}
