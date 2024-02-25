use guile_sys::SCM;

#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello 🦀");
}
