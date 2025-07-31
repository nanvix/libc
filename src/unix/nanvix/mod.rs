use std::os::nanvix::ffi::{c_int, c_char, c_void, c_uint};
use crate::FILE;

extern "C" {
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;

    // From sys/uio.h
    pub fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn preadv(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn pwritev(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;

    // From sys/ioccom.h
    pub fn ioctl(fd: c_int, request: c_int, ...) -> c_int;
    
    // From sys/unistd.h
    pub fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int;

    #[link_name = "fopen"]
    pub fn fopen64(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    
    #[link_name = "fseeko"]
    pub fn fseeko64(stream: *mut FILE, offset: off64_t, whence: c_int) -> c_int;
    
    #[link_name = "ftello"]
    pub fn ftello64(stream: *mut FILE) -> off64_t;

    #[link_name = "vsscanf"]
    pub fn __isoc23_vsscanf(s: *const c_char, format: *const c_char, ap: va_list) -> c_int;
    
    #[link_name = "sscanf"]
    pub fn __isoc23_sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;

    #[link_name = "__assert_func"]
    pub fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

#[no_mangle]
#[used]
pub extern "C" static mut __dso_handle: usize = 0;

pub type off64_t = i64;
pub type va_list = *mut c_char;

// Common ioctl commands - these values are fairly standard across Unix systems
pub const FIONBIO: c_int = 0x5421;   // Set/clear non-blocking I/O
pub const FIONREAD: c_int = 0x541B;  // Get number of bytes to read

// Opcode type for ioctl operations
pub type _Opcode = c_int;

// Special handle used by dlsym, 0 is common for most Unix systems here.
pub const RTLD_DEFAULT: *mut c_void = 0 as *mut c_void;

// Map Linux-specific errors to closest Nanvix equivalents
pub const EHWPOISON: c_int = EIO;        // Hardware error -> I/O error
pub const EISNAM: c_int = EINVAL;        // Named type file -> Invalid argument
pub const EKEYEXPIRED: c_int = EACCES;   // Key expired -> Permission denied
pub const EKEYREJECTED: c_int = EACCES;  // Key rejected -> Permission denied
pub const EKEYREVOKED: c_int = EACCES;   // Key revoked -> Permission denied
pub const EMEDIUMTYPE: c_int = EINVAL;   // Wrong medium -> Invalid argument
pub const ENAVAIL: c_int = ENOSYS;       // XENIX not available -> Not implemented
pub const ENOKEY: c_int = ENOENT;        // No key -> No such entry
pub const ENOTNAM: c_int = EINVAL;       // Not XENIX file -> Invalid argument
pub const EREMOTEIO: c_int = EIO;        // Remote I/O error -> I/O error
pub const ERESTART: c_int = EINTR;       // Restart syscall -> Interrupted
pub const ERFKILL: c_int = EACCES;       // RF-kill -> Permission denied
pub const EUCLEAN: c_int = EIO;          // Needs cleaning -> I/O error

// from: i686-nanvix/include/sys/unistd.h
pub const _SC_CLK_TCK: c_int = 2;
pub const _SC_PAGESIZE: c_int = 8;
pub const _SC_PAGE_SIZE: c_int = _SC_PAGESIZE;
pub const F_OK: c_int = 0;  // File existence
pub const R_OK: c_int = 4;  // Read permission
pub const W_OK: c_int = 2;  // Write permission  
pub const X_OK: c_int = 1;  // Execute permission

mod nanvix_arpa_inet;
pub use nanvix_arpa_inet::*;

// `dirent`
mod nanvix_dirent;
pub use nanvix_dirent::*;

// `dlfcn.h`
mod nanvix_dlfcn;
pub use nanvix_dlfcn::*;

// `errno.h`
mod nanvix_errno;
pub use nanvix_errno::*;

// `fcntl.h`
mod nanvix_fcntl;
pub use nanvix_fcntl::*;

// `limits.h`
mod nanvix_limits;
pub use nanvix_limits::*;

// `locale.h`
mod nanvix_locale;
pub use nanvix_locale::*;

// `netdb.h`
mod nanvix_netdb;
pub use nanvix_netdb::*;

// `netinet/in.h`
mod nanvix_netinet_in;
pub use nanvix_netinet_in::*;

// `netinet/tcp.h`
mod nanvix_netinet_tcp;
pub use nanvix_netinet_tcp::*;

// `poll.h``
mod nanvix_poll;
pub use nanvix_poll::*;

// `pthread.h`
mod nanvix_pthread;
pub use nanvix_pthread::*;

// `pwd.h`
mod nanvix_pwd;
pub use nanvix_pwd::*;

// `sched.h`
mod nanvix_sched;
pub use nanvix_sched::*;

// `semaphore.h`
mod nanvix_semaphore;
pub use nanvix_semaphore::*;

// `signal.h`
mod nanvix_signal;
pub use nanvix_signal::*;

// `stddef.h`
mod nanvix_stddef;
pub use nanvix_stddef::*;

// `sys/resource.h`
mod nanvix_sys_resource;
pub use nanvix_sys_resource::*;

// `sys/select.h`
mod nanvix_sys_select;
pub use nanvix_sys_select::*;

// `sys/socket.h`
mod nanvix_sys_socket;
pub use nanvix_sys_socket::*;

// `sys/stat.h`
mod nanvix_sys_stat;
pub use nanvix_sys_stat::*;

// `sys/statvfs.h`
mod nanvix_sys_statvfs;
pub use nanvix_sys_statvfs::*;

// `sys/times.h`
mod nanvix_sys_times;
pub use nanvix_sys_times::*;

// `sys/types.h`
mod nanvix_sys_types;
pub use nanvix_sys_types::*;

// `sys/uio.h`
mod nanvix_sys_uio;
pub use nanvix_sys_uio::*;

// `sys/un.h`.
mod nanvix_sys_un;
pub use nanvix_sys_un::*;

// `termios.h``
mod nanvix_termios;
pub use nanvix_termios::*;

// `time.h`
mod nanvix_time;
pub use nanvix_time::*;

// `unistd.h`
mod nanvix_unistd;
pub use nanvix_unistd::*;

// TODO: I'm adding methods there to try to fulfill the symbols needed by rusty_v8, we should
// probably move them to a more appropriate place later.
mod nanvix_tmp;
pub use nanvix_tmp::*;