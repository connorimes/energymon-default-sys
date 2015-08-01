
extern crate libc;
extern crate energymon_sys;

use libc::{c_int};
use energymon_sys::em_impl;

extern "C" {
    /// Native C function that fills in the em_impl struct values and may allocate other resources.
    pub fn em_impl_get(em: *mut em_impl) -> c_int;
}
