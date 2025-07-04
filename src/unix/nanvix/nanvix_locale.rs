use std::os::nanvix::ffi::c_char;

#[derive(Debug)]
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
