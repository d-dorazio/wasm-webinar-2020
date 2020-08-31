use wasm_bindgen::prelude::*;
use web_sys::console;

use std::{
    f32::consts::PI,
    sync::atomic::{AtomicU32, Ordering},
};

use rand::prelude::*;
use serde::Serialize;

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

#[wasm_bindgen]
pub fn fast_add(a: u32, b: u32) -> u32 {
    a + b
}

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
    let num_div = document
        .get_element_by_id("num")
        .expect("num div not found");

    num_div.set_inner_html(&format!("counter: {}", prev_val + 1));
}

#[derive(Serialize)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

#[wasm_bindgen]
pub fn pack_circles(width: f32, height: f32, coverage: f32, minr: f32, padding: f32) -> JsValue {
    let circles = pack_circles_impl(width, height, coverage, minr, padding);
    JsValue::from_serde(&circles).unwrap()
}

fn pack_circles_impl(
    width: f32,
    height: f32,
    coverage: f32,
    minr: f32,
    padding: f32,
) -> Vec<Circle> {
    let mut circles: Vec<Circle> = vec![];

    let mut area = 0.0;
    let targert_area = width * height * coverage;

    let mut stall_i = 0;

    let mut rng = thread_rng();
    while area < targert_area {
        let x = rng.gen_range(0.0, 1.0) * width;
        let y = rng.gen_range(0.0, 1.0) * height;
        let mut r = f32::min(x.min(width - x), y.min(height - y)) - padding;

        if r >= minr {
            for cc in &circles {
                let d2 = (x - cc.x).powi(2) + (y - cc.y).powi(2);
                r = r.min(d2.sqrt() - cc.r - padding);

                if r < minr {
                    break;
                }
            }
        }

        if r < minr {
            stall_i += 1;
            if stall_i >= 1_000 {
                break;
            }
            continue;
        }

        stall_i = 0;

        circles.push(Circle { x, y, r });
        area += PI * r.powi(2);
    }

    circles
}
