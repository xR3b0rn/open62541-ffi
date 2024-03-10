extern crate open62541_sys;
use std::mem::{self, MaybeUninit};

fn main() {
    let server = unsafe {
        open62541_sys::UA_Server_new()
    };
}
