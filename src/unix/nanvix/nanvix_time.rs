use std::os::nanvix::ffi::c_char;
use std::os::nanvix::ffi::c_int;
use std::os::nanvix::ffi::c_long;

pub use std::os::nanvix::syscall::sysapi::time::clock_ids::*;
pub use std::os::nanvix::syscall::sysapi::time::timespec;

#[derive(Debug)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}
