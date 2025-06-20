use std::os::nanvix::ffi::c_int;

pub use std::os::nanvix::syscall::sysapi::netinet_in::in6_addr;
pub use std::os::nanvix::syscall::sysapi::netinet_in::in_addr;
pub use std::os::nanvix::syscall::sysapi::netinet_in::ip_option_names::*;
pub use std::os::nanvix::syscall::sysapi::netinet_in::message_flags::*;
pub use std::os::nanvix::syscall::sysapi::netinet_in::sockaddr_in;
pub use std::os::nanvix::syscall::sysapi::netinet_in::sockaddr_in6;
pub use std::os::nanvix::syscall::sysapi::netinet_in::socket_flags::*;
pub use std::os::nanvix::syscall::sysapi::netinet_in::sockopt_levels::*;

pub const IPV6_UNICAST_HOPS: c_int = 16;
pub const IPV6_MULTICAST_IF: c_int = 17;
pub const IPV6_MULTICAST_HOPS: c_int = 18;
pub const IPV6_MULTICAST_LOOP: c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;
pub const IPV6_V6ONLY: c_int = 26;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;
