use std::os::nanvix::ffi::c_ulong;

pub use std::os::nanvix::syscall::sysapi::sys_select::timeval;

pub const FD_SETSIZE: usize = 1024;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct fd_set {
    fds_bits: [c_ulong; crate::FD_SETSIZE as usize / ::core::mem::size_of::<c_ulong>()],
}
