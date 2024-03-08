use std::{
    any::Any,
    ffi::{c_void, CStr, CString},
    ptr,
};

use guile_sys::{
    scm_c_define_gsubr, scm_from_signed_integer, scm_is_inexact, scm_is_integer, scm_list_1,
    scm_sum, SCM,
};

#[no_mangle]
pub unsafe extern "C" fn inc(input: SCM) -> SCM {
    let one = scm_from_signed_integer(1);

    if scm_is_integer(input) != 0 {
        let res = scm_sum(input, one);
        res
    } else {
        one
    }
}

#[no_mangle]
pub unsafe extern "C" fn hello() -> SCM {
    println!("Hello 🦀");

    // ptr::null::<SCM>() as *mut _
    scm_from_signed_integer(0)
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let name = CString::new("inc").unwrap();
    let x = ptr::NonNull::new_unchecked(inc as *mut _);
    scm_c_define_gsubr(name.as_ptr(), 1, 0, 0, x.as_ptr());

    let name = CString::new("hello").unwrap();
    let p = ptr::NonNull::new_unchecked(hello as *mut _);
    scm_c_define_gsubr(name.as_ptr(), 0, 0, 0, p.as_ptr());
}
