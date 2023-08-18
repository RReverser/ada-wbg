use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

use ada_url::Url;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($tt:tt)*) => {
        log(&format!($($tt)*))
    };
}

#[wasm_bindgen(start)]
pub fn main() {
    extern "C" {
        fn __wasm_call_ctors();
    }

    unsafe {
        // Important to prevent all exports being wrapped into `__wasm_call_ctors`/`__wasm_call_dtors`.
        // See https://reviews.llvm.org/D81689.
        __wasm_call_ctors();
    }

    let mut u = Url::parse("http://www.google:8080/love#drug", None).expect("bad url");
    console_log!("port: {:?}", u.port());
    console_log!("hash: {:?}", u.hash());
    console_log!("pathname: {:?}", u.pathname());
    console_log!("href: {:?}", u.href());
    u.set_port("9999");
    console_log!("href: {:?}", u.href());
}
