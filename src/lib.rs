use mallocator::Mallocator;

// It's important to ensure that C and Rust use the same allocator.
// Otherwise corresponding allocators might compete for and corrupt the same Wasm memory.
// One way to do that is to define custom `malloc`, `free` etc on the C side that invoke exported Rust equivalents.
// Another, and simpler, way is to override Rust's global allocator to one that uses C functions:
#[global_allocator]
static ALLOCATOR: Mallocator = Mallocator;

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
        // Important to call somewhere at startup to prevent all exports being wrapped into `__wasm_call_ctors`/`__wasm_call_dtors`.
        // See https://reviews.llvm.org/D81689.
        // Otherwise, if they're wrapped, you'll hit issue in wasm-bindgen preprocessor.
        // See https://github.com/rustwasm/wasm-bindgen/issues/2969 for prior example, but there are a lot more unsupported instructions it'll hit.
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
