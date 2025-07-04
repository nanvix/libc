extern "C" {
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
}

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
