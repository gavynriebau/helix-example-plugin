use protobuf::Message;
use std::mem;
use std::os::raw::{c_char, c_void};

use crate::generated::messages::event::EventType;
use crate::generated::messages::{Event, KeyEvent};

mod generated;

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
    helix_log(&format!("allocate called with size: {}", size));
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut c_void, capacity: usize) {
    helix_log(&format!(
        "deallocate called with pointer '{:?}' and capacity '{}'",
        pointer, capacity
    ));

    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn handle_event(pointer: *mut u8, len: usize) {
    let bytes = unsafe { Vec::from_raw_parts(pointer, len, len) };
    let event = Event::parse_from_bytes(&bytes).unwrap();
    let event_type: EventType = event.event_type.enum_value().unwrap();

    match event_type {
        EventType::PLUGIN_STARTED => {
            helix_log("Received PLUGIN_STARTED event");
        }
        EventType::KEY_EVENT => {
            let key_event: KeyEvent = KeyEvent::parse_from_bytes(&event.payload).unwrap();
            helix_log(&format!("Received KEY_EVENT: {:#?}", key_event));
        }
    }
}
