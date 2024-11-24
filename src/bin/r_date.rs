use chrono::{Local, TimeZone};
use chrono_tz::Tz;
fn main() {
    let tz = iana_time_zone::get_timezone().unwrap();
    // warn: r_date's dependency iana_time_zone has an bug that get_timezone() can't get the right Tz. See [here](strawlab/iana-time-zone#118) for more details.
    println!("🚀 {:?}", tz); // 🚀 "Etc/UTC"
    let tz = tz.parse::<Tz>().unwrap();
    // let tz = Tz::Asia__Shanghai; // my real TZ
    let dt = tz.from_local_datetime(&Local::now().naive_local()).unwrap();
    println!("{}", dt.format("%a %b %e %r %Z %Y").to_string()); // Sun Nov 24 10:27:33 PM UTC 2024
}
