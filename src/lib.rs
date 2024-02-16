use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[you_can::turn_off_the_borrow_checker]
pub fn main() {
    println!("Hello, world!");
}
