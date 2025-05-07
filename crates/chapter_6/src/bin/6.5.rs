// #![cfg(feature = "bin_6_5")]
use chrono::{Local, TimeZone};
use chrono_tz::Tz;
fn main() {
    let tz = iana_time_zone::get_timezone().unwrap();
    let tz = tz.parse::<Tz>().unwrap();
    let dt = tz.from_local_datetime(&Local::now().naive_local()).unwrap();
    println!("{}", dt.format("%a %b %e %r %Z %Y"));
}
