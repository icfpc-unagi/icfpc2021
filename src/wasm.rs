use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::util::SetMinMax;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    let mut x = 2;
    console::log_2(&x.setmin(1).into(), &x.into());
    console::log_2(&x.setmin(2).into(), &x.into());
    console::log_2(&x.setmax(3).into(), &x.into());
}

#[wasm_bindgen]
pub fn hello() {
    alert("hello");
}
