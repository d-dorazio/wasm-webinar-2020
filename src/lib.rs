use wasm_bindgen::prelude::*;
use web_sys::console;

use std::sync::atomic::{AtomicU32, Ordering};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}


// lib.rs
#[wasm_bindgen]
pub fn fast_add(a: u32, b: u32) -> u32 {
    a + b
}

// lib.rs
#[wasm_bindgen]
pub fn fast_sort(mut v: Vec<u8>, msg: &str) -> Vec<u8> {
    console::log_2(&JsValue::from_str("msg:"), &JsValue::from_str(msg));
    v.sort();
    v
}


static COUNTER: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn increment_counter() {
    let prev_val = COUNTER.fetch_add(1, Ordering::SeqCst);

    let document = web_sys::window().unwrap().document().unwrap();
    let num_div = document.get_element_by_id("num").expect("num div not found");

    num_div.set_inner_html(&format!("counter: {}", prev_val + 1));
}
