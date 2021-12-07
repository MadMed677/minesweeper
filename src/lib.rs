mod battlefield;
mod mine_sweeper;

use js_sys;
use wasm_bindgen::prelude::*;
use web_sys::console;

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
pub fn hello_world() -> String {
    "Hello world!".into()
}

// #[wasm_bindgen]
// pub fn get_matrix(rows: usize, cols: usize) -> js_sys::Array {
//     mine_sweeper::matrix_array(rows, cols)
//         .clone()
//         .into_iter()
//         .map(|cell_vec| {
//             cell_vec
//                 .clone()
//                 .into_iter()
//                 .map(JsValue::from)
//                 .collect::<js_sys::Array>()
//         })
//         .collect()
// mine_sweeper::just_array(rows)
//     .clone()
//     .into_iter()
//     .map(|vec| JsValue::from(vec))
//     .collect::<js_sys::Array>()
// }
