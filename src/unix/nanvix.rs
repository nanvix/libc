use std::os::nanvix::ffi::c_char;
use std::os::nanvix::ffi::c_int;
use std::os::nanvix::ffi::c_long;
use std::os::nanvix::ffi::c_short;
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
pub use std::os::nanvix::syscall::sys::types::size_t;
pub use std::os::nanvix::syscall::sys::types::ssize_t;
pub use std::os::nanvix::syscall::sys::types::suseconds_t;

// Types in `arpa/inet.h`.
pub use std::os::nanvix::syscall::arpa::inet::*;

// `netinet/in.h`.
pub use std::os::nanvix::syscall::netinet::in_::bindings::*;

// `netinet/tcp.h`.
pub use std::os::nanvix::syscall::netinet::tcp::*;

// `sys/un.h`.
pub use std::os::nanvix::syscall::sys::un::bindings::*;

// `fcntl.h`.
pub use std::os::nanvix::syscall::fcntl::open_flags::*;
pub use std::os::nanvix::syscall::fcntl::F_DUPFD_CLOEXEC;
pub use std::os::nanvix::syscall::fcntl::F_GETFD;
pub use std::os::nanvix::syscall::fcntl::F_GETFL;
pub use std::os::nanvix::syscall::fcntl::F_SETFD;
pub use std::os::nanvix::syscall::fcntl::F_SETFL;

// Types in `sys/socket.h`.
pub use std::os::nanvix::syscall::sys::socket::family::*;
pub use std::os::nanvix::syscall::sys::socket::sockaddr;
pub use std::os::nanvix::syscall::sys::socket::socklen_t;
pub use std::os::nanvix::syscall::sys::socket::*;

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

// `errno.h`.
pub use std::os::nanvix::syscall::errno::*;

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

pub const IPV6_UNICAST_HOPS: c_int = 16;
pub const IPV6_MULTICAST_IF: c_int = 17;
pub const IPV6_MULTICAST_HOPS: c_int = 18;
pub const IPV6_MULTICAST_LOOP: c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;
pub const IPV6_V6ONLY: c_int = 26;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;

// poll.h
pub const POLLIN: c_short = 0x001;
pub const POLLPRI: c_short = 0x002;
pub const POLLOUT: c_short = 0x004;
pub const POLLERR: c_short = 0x008;
pub const POLLHUP: c_short = 0x010;
pub const POLLNVAL: c_short = 0x020;
pub const POLLRDNORM: c_short = 0x040;
pub const POLLRDBAND: c_short = 0x080;
pub const POLLWRNORM: c_short = 0x100;
pub const POLLWRBAND: c_short = 0x200;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ip_mreq {
    pub imr_multiaddr: crate::in_addr,
    pub imr_interface: crate::in_addr,
}

extern "C" {
    pub fn bind(
        socket: c_int,
        address: *const crate::sockaddr,
        address_len: crate::socklen_t,
    ) -> c_int;

    pub fn recvfrom(
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        addr: *mut crate::sockaddr,
        addrlen: *mut crate::socklen_t,
    ) -> ssize_t;

}
