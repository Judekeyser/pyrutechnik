extern crate libc;

use libc::c_char;
use std::ffi::{ CString, CStr};

mod pyru;
use crate::pyru::{ add, Example };


#[no_mangle]
pub extern "C" fn add_example(c_s1: *const c_char, c_s2: *const c_char, c_sr: *mut *const c_char) -> *mut CString {
    // Convert raw pointers into valid CStr
    let cstr1: &CStr = unsafe { CStr::from_ptr(c_s1) };
    let cstr2: &CStr = unsafe { CStr::from_ptr(c_s2) };

    // Convert CStr to String
    let s1: String = cstr1.to_str().unwrap().to_owned();
    let s2: String = cstr2.to_str().unwrap().to_owned();

    // Internal conversion and method call
    let left = Example {
        s: s1,
        x: 2023
    };
    let right = Example {
        s: s2,
        x: 1
    };
    let result = add(left, right);

    // Convert back the Rust String into a CString, then leak it. result is partially moved
    let sr = CString::new(result.s).expect("CString::new failed");
    unsafe {
        *c_sr = sr.as_ptr()
    }
    Box::into_raw(Box::new(sr))
}

#[no_mangle]
pub extern "C" fn destroy_example(ptr: *mut CString) {
    unsafe { let _ = Box::from_raw(ptr); }
}