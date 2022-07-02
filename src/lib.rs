use protobuf::Message;
use std::mem;
use std::os::raw::c_void;

use crate::generated::messages::event::EventType;
use crate::generated::messages::{Event, KeyEvent};

mod generated;

// Plugins must provide allocate and deallocate functions that helix can use for passing data back/forth.
// See: https://github.com/wasmerio/wasmer/issues/1449

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut c_void {
    println!("allocate called with size: {}", size);
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut c_void, capacity: usize) {
    println!(
        "deallocate called with pointer '{:?}' and capacity '{}'",
        pointer, capacity
    );

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
            println!("Received PLUGIN_STARTED event");
        }
        EventType::KEY_EVENT => {
            let key_event: KeyEvent = KeyEvent::parse_from_bytes(&event.payload).unwrap();
            println!("Received KEY_EVENT: {:#?}", key_event);
        }
    }
}
