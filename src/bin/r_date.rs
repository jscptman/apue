use chrono::Local;

fn main() {
    let local = Local::now();
    println!("{}", local.format("%a %b %e %r %Z %Y").to_string());
}
