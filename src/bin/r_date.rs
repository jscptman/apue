use chrono::{Local, TimeZone};
use chrono_tz::Tz;
fn main() {
    let tz = iana_time_zone::get_timezone().unwrap();
    println!("🚀 {:?}", tz); // 🚀 "Etc/UTC"
    let tz = tz.parse::<Tz>().unwrap();
    // let tz = Tz::Asia__Shanghai; // my real TZ
    let dt = tz.from_local_datetime(&Local::now().naive_local()).unwrap();
    println!("{}", dt.format("%a %b %e %r %Z %Y").to_string()); // Sun Nov 24 10:27:33 PM UTC 2024
}
