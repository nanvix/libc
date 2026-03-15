use std::os::nanvix::ffi::c_ulong;

pub use std::os::nanvix::syscall::sysapi::sys_types::*;

pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;
