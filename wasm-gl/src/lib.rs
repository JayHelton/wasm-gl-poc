mod utils;

use crate::wasm_gl::do_stuff;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn do_it(s: &str) {
    do_stuff(s)
}

mod wasm_gl {
    use crate::alert;

    pub fn do_stuff(s: &str) {
        alert(s)
    }

    // #[cfg(test)]
    // mod ggg_translation_tests {
    //     use super::*;

    //     #[test]
    //     fn test_ggg_decode() {
    // }
}
