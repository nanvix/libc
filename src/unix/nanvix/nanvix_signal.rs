use std::os::nanvix::ffi::c_ulong;

pub type sigset_t = c_ulong;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction {
    pub sa_sigaction: usize,
    pub sa_flags: c_ulong,
    pub sa_restorer: Option<extern "C" fn()>,
    pub sa_mask: crate::sigset_t,
}
