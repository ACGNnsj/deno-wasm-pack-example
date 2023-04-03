use core::arch::wasm32::*;
use wasm_bindgen::prelude::*;
use crate::{alert, log};

#[wasm_bindgen]
pub fn info() -> () {
    log("wasm32");
    alert("wasm32");
    println!("wasm32: {}", "wasm32");
}