use std::os::nanvix::ffi::c_char;
use std::os::nanvix::ffi::c_int;
use std::os::nanvix::ffi::c_long;
use std::os::nanvix::ffi::c_uchar;
use std::os::nanvix::ffi::c_uint;
use std::os::nanvix::ffi::c_ulong;
use std::os::nanvix::ffi::c_void;

// Types in `time.h`.
pub use std::os::nanvix::syscall::time::time_t;

// Types in `sys/types.h`.
pub use std::os::nanvix::syscall::sys::types::clock_t;
pub use std::os::nanvix::syscall::sys::types::dev_t;
pub use std::os::nanvix::syscall::sys::types::mode_t;
pub use std::os::nanvix::syscall::sys::types::off_t;
pub use std::os::nanvix::syscall::sys::types::pthread_attr_t;
pub use std::os::nanvix::syscall::sys::types::pthread_cond_t;
pub use std::os::nanvix::syscall::sys::types::pthread_condattr_t;
pub use std::os::nanvix::syscall::sys::types::pthread_key_t;
pub use std::os::nanvix::syscall::sys::types::pthread_mutex_t;
pub use std::os::nanvix::syscall::sys::types::pthread_mutexattr_t;
pub use std::os::nanvix::syscall::sys::types::pthread_rwlock_t;
pub use std::os::nanvix::syscall::sys::types::pthread_rwlockattr_t;
pub use std::os::nanvix::syscall::sys::types::pthread_t;
pub use std::os::nanvix::syscall::sys::types::suseconds_t;

// Types in `sys/socket.h`.
pub use std::os::nanvix::syscall::sys::socket::sockaddr;
pub use std::os::nanvix::syscall::sys::socket::socklen_t;

// Types in `stddef.h`.
pub use std::os::nanvix::syscall::stddef::wchar_t;

// Types in `sys/stat.h`.
pub use std::os::nanvix::syscall::sys::stat::stat;

// Types in `netdb.h`.
pub use std::os::nanvix::syscall::netdb::addrinfo;

// Types in `dirent`.
pub use std::os::nanvix::syscall::dirent::dirent;

// Types in `sys/resource.h`.
pub use std::os::nanvix::syscall::sys::resource::rlim_t;

// Types in `dlfcn.h`.
pub type Dl_info = std::os::nanvix::syscall::dlfcn::DlInfo;

pub type sem_t = *mut c_void;

pub const FD_SETSIZE: usize = 1024;

pub type nfds_t = c_ulong;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct fd_set {
    fds_bits: [c_ulong; crate::FD_SETSIZE as usize / ::core::mem::size_of::<c_ulong>()],
}

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

pub struct passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: crate::uid_t,
    pub pw_gid: crate::gid_t,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}

pub type sigset_t = c_ulong;

pub type speed_t = c_uint;
pub type tcflag_t = c_uint;
pub type cc_t = c_uchar;

pub const NCCS: usize = 20;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; NCCS],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

pub struct lconv {
    pub currency_symbol: *const c_char,
    pub decimal_point: *const c_char,
    pub frac_digits: c_char,
    pub grouping: *const c_char,
    pub int_curr_symbol: *const c_char,
    pub int_frac_digits: c_char,
    pub mon_decimal_point: *const c_char,
    pub mon_grouping: *const c_char,
    pub mon_thousands_sep: *const c_char,
    pub negative_sign: *const c_char,
    pub n_cs_precedes: c_char,
    pub n_sep_by_space: c_char,
    pub n_sign_posn: c_char,
    pub positive_sign: *const c_char,
    pub p_cs_precedes: c_char,
    pub p_sep_by_space: c_char,
    pub p_sign_posn: c_char,
    pub thousands_sep: *const c_char,
}

pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct statvfs {
    pub f_bsize: c_ulong,
    pub f_frsize: c_ulong,
    pub f_blocks: crate::fsblkcnt_t,
    pub f_bfree: crate::fsblkcnt_t,
    pub f_bavail: crate::fsblkcnt_t,
    pub f_files: crate::fsfilcnt_t,
    pub f_ffree: crate::fsfilcnt_t,
    pub f_favail: crate::fsfilcnt_t,
    pub f_fsid: c_ulong,
    pub f_flag: c_ulong,
    pub f_namemax: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction {
    pub sa_sigaction: crate::sighandler_t,
    pub sa_flags: c_ulong,
    pub sa_restorer: Option<extern "C" fn()>,
    pub sa_mask: crate::sigset_t,
}
