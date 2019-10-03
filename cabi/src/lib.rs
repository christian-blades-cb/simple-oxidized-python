use simple_rs::hello_world as rust_hello;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn hello_world() -> *const c_char {
    let hello: &str = &rust_hello();
    CString::new(hello).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(x: *mut c_char) {
    let s = unsafe {
        CString::from_raw(x);
    };
    drop(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
