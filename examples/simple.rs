extern crate open62541_ffi;

struct Server {
    server: *mut open62541_ffi::UA_Server,
}

impl Server {
    fn new() -> Server {
        Server {
            server: unsafe { open62541_ffi::UA_Server_new() },
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            open62541_ffi::UA_Server_delete(self.server);
        }
    }
}

fn main() {
    let _server = Server::new();
}
