use guile_bindings::SCM;

#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello 🦀");
}
