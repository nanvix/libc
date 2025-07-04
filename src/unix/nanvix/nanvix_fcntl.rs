use std::os::nanvix::ffi::c_int;

// from: i686-nanvix/include/sys/_default_fcntl.h
pub const LOCK_SH: c_int = 1;  // Shared lock
pub const LOCK_EX: c_int = 2;  // Exclusive lock  
pub const LOCK_NB: c_int = 4;  // Non-blocking
pub const LOCK_UN: c_int = 8;  // Unlock

// from: i686-nanvix/include/sys/errno.h
pub const EAGAIN: ::c_int = 11;
pub const EWOULDBLOCK: ::c_int = EAGAIN;

pub use std::os::nanvix::syscall::sysapi::fcntl::atflags::*;
pub use std::os::nanvix::syscall::sysapi::fcntl::file_access_mode::*;
pub use std::os::nanvix::syscall::sysapi::fcntl::file_advice::*;
pub use std::os::nanvix::syscall::sysapi::fcntl::file_control_request::*;
pub use std::os::nanvix::syscall::sysapi::fcntl::file_creation_flags::*;
