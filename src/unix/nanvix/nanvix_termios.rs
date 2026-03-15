use std::os::nanvix::ffi::c_uchar;
use std::os::nanvix::ffi::c_uint;

pub type speed_t = c_uint;
pub type tcflag_t = c_uint;
pub type cc_t = c_uchar;

pub const NCCS: usize = 20;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
