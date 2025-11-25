#[no_mangle]
pub extern "C" fn frida_init() {
    println!("RustBrawl dylib loaded!");
}
