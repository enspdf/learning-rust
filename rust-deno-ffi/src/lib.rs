use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn add(x: f64, y: f64) -> f64 {
    x + y
}

#[no_mangle]
pub extern "C" fn print_string(str_ptr: *const i8) -> () {
    let my_string: &str;

    unsafe {
        my_string = CStr::from_ptr(str_ptr)
            .to_str()
            .expect("The string could not be parsed. ")
    }

    println!("The string passed to Rust: {}", my_string);
}
