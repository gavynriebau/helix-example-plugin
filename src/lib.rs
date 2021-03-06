use std::mem;
use std::os::raw::{c_char, c_void};

#[link(wasm_import_module = "helix")]
extern "C" {
    pub fn log(pointer: *mut c_char, len: usize);
}

fn helix_log(msg: &str) {
    let text = String::from(msg);
    let ptr: *const u8 = text.as_ptr();
    let len: usize = text.len();

    unsafe {
        log(ptr as *mut i8, len);
    }
}

// Plugins must provide allocate and deallocate functions that helix can use for passing data back/forth.
// See: https://github.com/wasmerio/wasmer/issues/1449

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn on_start() {
    helix_log("on_start called");
}

#[no_mangle]
pub extern "C" fn on_key_press(pointer: *mut c_char, len: usize) {
    let event: String = unsafe { String::from_raw_parts(pointer as *mut u8, len, len) };
    helix_log(&format!("on_key_press '{}'", event));
}

#[no_mangle]
pub extern "C" fn on_mouse_event(pointer: *mut c_char, len: usize) {
    let event: String = unsafe { String::from_raw_parts(pointer as *mut u8, len, len) };
    helix_log(&format!("on_mouse_event '{}'", event));
}

#[no_mangle]
pub extern "C" fn on_resize(cols: u32, rows: u32) {
    helix_log(&format!(
        "Received resize event with cols '{}' and rows '{}'",
        cols, rows
    ));
}
