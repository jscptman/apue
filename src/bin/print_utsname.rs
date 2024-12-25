use nix::sys::utsname as n_utsname;
use std::ffi::OsStr;
struct UtsnameFieldsAddressPair<'a, 'b>(&'a str, &'b OsStr);
fn main() {
    let utsname = n_utsname::uname().expect("uname occurs an error");
    let utsname_fields = [
        UtsnameFieldsAddressPair("sysname", utsname.sysname()),
        UtsnameFieldsAddressPair("nodename", utsname.nodename()),
        UtsnameFieldsAddressPair("release", utsname.release()),
        UtsnameFieldsAddressPair("domainname", utsname.domainname()),
        UtsnameFieldsAddressPair("machine", utsname.machine()),
        UtsnameFieldsAddressPair("version", utsname.version()),
    ];
    for UtsnameFieldsAddressPair(field, addr) in utsname_fields {
        print_kv(
            field,
            addr.to_str().expect("invalid utf-8 convert"),
            "unknown",
        );
    }
}

fn print_kv<'a>(key: &str, mut value: &'a str, value_placeholder: &'a str) {
    if value.is_empty() {
        value = value_placeholder;
    }
    println!("🚀 {}: {}", key, value);
}
