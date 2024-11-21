use libc::utsname;
use std::{ffi::CStr, mem};
type FieldAddr = [i8; 65];
struct UtsnameFieldsAddressPair<'a, 'b>(&'a str, &'b mut FieldAddr);
fn main() {
    let mut buf = unsafe { mem::zeroed::<utsname>() };
    let buf_raw = &mut buf as *mut utsname;
    let result_code = unsafe { libc::uname(buf_raw) };
    let utsname_fields = [
        UtsnameFieldsAddressPair("sysname", &mut buf.sysname),
        UtsnameFieldsAddressPair("nodename", &mut buf.nodename),
        UtsnameFieldsAddressPair("release", &mut buf.release),
        UtsnameFieldsAddressPair("domainname", &mut buf.domainname),
        UtsnameFieldsAddressPair("machine", &mut buf.machine),
        UtsnameFieldsAddressPair("version", &mut buf.version),
    ];
    if result_code < 0 {
        panic!("uname syscall occurs an error!")
    } else {
        unsafe {
            for UtsnameFieldsAddressPair(field, addr) in utsname_fields {
                let except_msg = String::from(field) + "convert to &str occurs an error";
                print_kv(
                    field,
                    CStr::from_ptr(addr.as_mut_ptr())
                        .to_str()
                        .expect(&except_msg),
                    "unknown",
                );
            }
        };
    }
}

fn print_kv<'a>(key: &str, mut value: &'a str, value_placeholder: &'a str) {
    if value.is_empty() {
        value = value_placeholder;
    }
    println!("🚀 {}: {}", key, value);
}
