use std::{any::Any, ffi::{c_void, CStr, CString}, ptr};

use guile_sys::{scm_c_define_gsubr, scm_from_signed_integer, scm_is_inexact, scm_is_integer, scm_sum, SCM};

#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello 🦀");
}


#[no_mangle]
pub unsafe extern "C" fn inc(input: SCM) -> SCM {
    let one = scm_from_signed_integer(1);

    if scm_is_integer(input) != 0 {
        let res = scm_sum(input, one);
        return res;
    } else {
        return one;
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let name = CString::new("inc").unwrap();
    let x = ptr::NonNull::new_unchecked(inc as *mut _);
    scm_c_define_gsubr(name.as_ptr(), 1, 0, 0, x.as_ptr());
}
